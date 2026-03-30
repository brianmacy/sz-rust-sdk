# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp"]
# ///
"""Senzing Rust SDK FAQ MCP Server — queryable FAQ for project knowledge.

Uses BM25 (Okapi BM25) ranking for search relevance. No external
dependencies beyond the `mcp` package.
"""

import math
import re
import sys
from pathlib import Path

try:
    from mcp.server.fastmcp import FastMCP
except ImportError:
    print(
        "ERROR: The 'mcp' package is not installed.\n"
        "\n"
        "This script is designed to be run via uv with PEP 723 inline metadata:\n"
        "    uv run --script .claude/faq_server.py\n"
        "\n"
        "If you don't have uv installed:\n"
        "    pip install uv   # or: brew install uv\n"
        "\n"
        "Alternatively, install the dependency manually:\n"
        "    pip install mcp\n"
        "    python .claude/faq_server.py",
        file=sys.stderr,
    )
    sys.exit(1)

FAQ_DIR = Path(__file__).parent / "faqs"

# ---------------------------------------------------------------------------
# BM25 index
# ---------------------------------------------------------------------------

_K1 = 1.2
_B = 0.75
_TOKEN_RE = re.compile(r"[a-z0-9_]+")


def _tokenize(text: str) -> list[str]:
    """Lowercase tokenization — keeps alphanumerics and underscores."""
    return _TOKEN_RE.findall(text.lower())


class _Document:
    """A single FAQ document with precomputed token frequencies."""

    __slots__ = ("category", "title", "content", "tokens", "tf", "length")

    def __init__(self, category: str, title: str, content: str) -> None:
        self.category = category
        self.title = title
        self.content = content
        self.tokens = _tokenize(title + " " + content)
        self.length = len(self.tokens)
        self.tf: dict[str, int] = {}
        for tok in self.tokens:
            self.tf[tok] = self.tf.get(tok, 0) + 1


class _BM25Index:
    """Okapi BM25 index over a collection of FAQ documents."""

    def __init__(self) -> None:
        self.docs: list[_Document] = []
        self.df: dict[str, int] = {}
        self.avgdl: float = 0.0

    def add(self, doc: _Document) -> None:
        self.docs.append(doc)
        for term in set(doc.tf):
            self.df[term] = self.df.get(term, 0) + 1

    def finalize(self) -> None:
        if self.docs:
            self.avgdl = sum(d.length for d in self.docs) / len(self.docs)

    def search(
        self,
        query: str,
        category: str | None = None,
        max_results: int = 5,
    ) -> list[tuple[_Document, float]]:
        terms = _tokenize(query)
        if not terms:
            return []
        n = len(self.docs)
        scores: list[tuple[_Document, float]] = []
        for doc in self.docs:
            if category and doc.category != category:
                continue
            score = 0.0
            for t in terms:
                df = self.df.get(t, 0)
                if df == 0:
                    continue
                idf = math.log((n - df + 0.5) / (df + 0.5) + 1.0)
                tf = doc.tf.get(t, 0)
                if tf == 0:
                    continue
                numerator = tf * (_K1 + 1.0)
                denominator = tf + _K1 * (
                    1.0 - _B + _B * doc.length / max(self.avgdl, 1.0)
                )
                score += idf * numerator / denominator
            if score > 0:
                scores.append((doc, score))
        scores.sort(key=lambda x: x[1], reverse=True)
        return scores[:max_results]


# ---------------------------------------------------------------------------
# Load FAQs at import time
# ---------------------------------------------------------------------------

_faqs: dict[str, dict[str, str]] = {}
_index = _BM25Index()


def _load_faqs() -> None:
    """Walk faqs/<category>/*.md and populate the _faqs dict + BM25 index."""
    if not FAQ_DIR.is_dir():
        return
    for cat_dir in sorted(FAQ_DIR.iterdir()):
        if not cat_dir.is_dir():
            continue
        category = cat_dir.name
        _faqs[category] = {}
        for md in sorted(cat_dir.glob("*.md")):
            title = md.stem.replace("-", " ")
            content = md.read_text(encoding="utf-8")
            _faqs[category][title] = content
            _index.add(_Document(category, title, content))
    _index.finalize()


_load_faqs()

# ---------------------------------------------------------------------------
# MCP server
# ---------------------------------------------------------------------------

mcp = FastMCP(
    "sz-rust-sdk-faq",
    instructions=(
        "Senzing Rust SDK project FAQ server. Consult these tools BEFORE making "
        "design decisions about error handling, FFI bindings, flags, initialization "
        "patterns, or test architecture. Critical design rules — like the flat error "
        "enum requirement, thread pool architecture (no async), and error code mapping "
        "from szerrors.json — are documented here. If FAQ search returns no useful "
        "results, TELL THE USER and recommend adding a FAQ."
    ),
)


@mcp.tool()
def get_faq_categories() -> str:
    """List all FAQ categories with the number of articles in each."""
    if not _faqs:
        return (
            "No FAQ categories found. "
            "Ensure .claude/faqs/ contains category directories with .md files."
        )
    lines = []
    for cat in sorted(_faqs):
        count = len(_faqs[cat])
        titles = ", ".join(sorted(_faqs[cat]))
        lines.append(f"**{cat}** ({count}): {titles}")
    return "\n".join(lines)


@mcp.tool()
def search_faqs(
    query: str, category: str | None = None, max_results: int = 5
) -> str:
    """Search FAQs using BM25 ranking. Returns titles + matching excerpts.

    Args:
        query: keyword(s) to search for
        category: optional category filter
        max_results: max results to return (default 5)
    """
    results = _index.search(query, category=category, max_results=max_results)
    if not results:
        return f"No results for '{query}'."

    lines = []
    for doc, score in results:
        query_lower = query.lower()
        content_lower = doc.content.lower()
        idx = content_lower.find(query_lower)
        matched_len = len(query)
        if idx < 0:
            for term in _tokenize(query):
                idx = content_lower.find(term)
                if idx >= 0:
                    matched_len = len(term)
                    break
        if idx >= 0:
            start = max(0, idx - 80)
            end = min(len(doc.content), idx + matched_len + 120)
            excerpt = (
                ("..." if start > 0 else "")
                + doc.content[start:end].strip()
                + ("..." if end < len(doc.content) else "")
            )
        else:
            excerpt = doc.content[:200].strip() + (
                "..." if len(doc.content) > 200 else ""
            )
        lines.append(
            f"### [{doc.category}] {doc.title} (score: {score:.2f})\n{excerpt}\n"
        )
    return "\n".join(lines)


@mcp.tool()
def get_faq(title: str, category: str | None = None) -> str:
    """Get full content of a specific FAQ by title.

    Args:
        title: FAQ title (use dashes or spaces, case-insensitive)
        category: optional category to narrow the search
    """
    title_normalized = title.lower().replace("-", " ")

    cats = [category] if category and category in _faqs else sorted(_faqs)
    for cat in cats:
        for faq_title, content in _faqs.get(cat, {}).items():
            if faq_title.lower() == title_normalized:
                return f"# [{cat}] {faq_title}\n\n{content}"

    # Fuzzy fallback: partial match
    for cat in cats:
        for faq_title, content in _faqs.get(cat, {}).items():
            if (
                title_normalized in faq_title.lower()
                or faq_title.lower() in title_normalized
            ):
                return f"# [{cat}] {faq_title}\n\n{content}"

    return f"FAQ '{title}' not found. Use get_faq_categories() to see available FAQs."


if __name__ == "__main__":
    mcp.run()
