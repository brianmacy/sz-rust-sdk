//! Flag definitions for Senzing SDK operations
//!
//! This module defines the bitflags used to control the behavior of various
//! Senzing SDK operations, matching the C# SDK's SzFlag enumeration exactly.
//!
//! For comprehensive flag documentation and usage examples, see:
//! <https://www.senzing.com/docs/flags/4/>

use bitflags::bitflags;

bitflags! {
    /// Bitflags for controlling Senzing SDK operations
    ///
    /// These flags match the C# SDK SzFlag enum exactly.
    /// For detailed flag descriptions and usage patterns, see the official documentation:
    /// <https://www.senzing.com/docs/flags/4/>
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct SzFlags: u64 {
        // =================================================================
        // Export flags (bits 0-5)
        // =================================================================
        const EXPORT_INCLUDE_MULTI_RECORD_ENTITIES = 1 << 0;
        const EXPORT_INCLUDE_POSSIBLY_SAME = 1 << 1;
        const EXPORT_INCLUDE_POSSIBLY_RELATED = 1 << 2;
        const EXPORT_INCLUDE_NAME_ONLY = 1 << 3;
        const EXPORT_INCLUDE_DISCLOSED = 1 << 4;
        const EXPORT_INCLUDE_SINGLE_RECORD_ENTITIES = 1 << 5;

        // =================================================================
        // Entity relation flags (bits 6-9)
        // =================================================================
        const ENTITY_INCLUDE_POSSIBLY_SAME_RELATIONS = 1 << 6;
        const ENTITY_INCLUDE_POSSIBLY_RELATED_RELATIONS = 1 << 7;
        const ENTITY_INCLUDE_NAME_ONLY_RELATIONS = 1 << 8;
        const ENTITY_INCLUDE_DISCLOSED_RELATIONS = 1 << 9;

        // =================================================================
        // Entity feature flags (bits 10-11)
        // =================================================================
        const ENTITY_INCLUDE_ALL_FEATURES = 1 << 10;
        const ENTITY_INCLUDE_REPRESENTATIVE_FEATURES = 1 << 11;

        // =================================================================
        // Entity name and record flags (bits 12-16, 18)
        // =================================================================
        const ENTITY_INCLUDE_ENTITY_NAME = 1 << 12;
        const ENTITY_INCLUDE_RECORD_SUMMARY = 1 << 13;
        const ENTITY_INCLUDE_RECORD_DATA = 1 << 14;
        const ENTITY_INCLUDE_RECORD_MATCHING_INFO = 1 << 15;
        const ENTITY_INCLUDE_RECORD_JSON_DATA = 1 << 16;
        // Note: bit 17 is unused
        const ENTITY_INCLUDE_RECORD_FEATURES = 1 << 18;

        // =================================================================
        // Related entity flags (bits 19-22)
        // =================================================================
        const ENTITY_INCLUDE_RELATED_ENTITY_NAME = 1 << 19;
        const ENTITY_INCLUDE_RELATED_MATCHING_INFO = 1 << 20;
        const ENTITY_INCLUDE_RELATED_RECORD_SUMMARY = 1 << 21;
        const ENTITY_INCLUDE_RELATED_RECORD_DATA = 1 << 22;

        // =================================================================
        // Internal/feature flags (bits 23-24)
        // =================================================================
        const ENTITY_INCLUDE_INTERNAL_FEATURES = 1 << 23;
        const ENTITY_INCLUDE_FEATURE_STATS = 1 << 24;

        // =================================================================
        // Find path flags (bits 25, 30)
        // =================================================================
        const FIND_PATH_STRICT_AVOID = 1 << 25;
        const FIND_PATH_INCLUDE_MATCHING_INFO = 1 << 30;

        // =================================================================
        // Scoring and stats flags (bits 26-27)
        // =================================================================
        const INCLUDE_FEATURE_SCORES = 1 << 26;
        const SEARCH_INCLUDE_STATS = 1 << 27;

        // =================================================================
        // Record type flags (bits 28-29)
        // =================================================================
        const ENTITY_INCLUDE_RECORD_TYPES = 1 << 28;
        const ENTITY_INCLUDE_RELATED_RECORD_TYPES = 1 << 29;

        // =================================================================
        // Additional entity record flags (bits 31, 35-36, 39)
        // =================================================================
        const ENTITY_INCLUDE_RECORD_UNMAPPED_DATA = 1 << 31;
        const ENTITY_INCLUDE_RECORD_FEATURE_DETAILS = 1 << 35;
        const ENTITY_INCLUDE_RECORD_FEATURE_STATS = 1 << 36;
        const ENTITY_INCLUDE_RECORD_DATES = 1 << 39;

        // =================================================================
        // Search flags (bits 32, 37-38)
        // Note: bits 0-3 are aliased as search flags (same as export flags)
        // =================================================================
        const SEARCH_INCLUDE_ALL_CANDIDATES = 1 << 32;
        const FIND_NETWORK_INCLUDE_MATCHING_INFO = 1 << 33;
        const INCLUDE_MATCH_KEY_DETAILS = 1 << 34;
        const SEARCH_INCLUDE_REQUEST = 1 << 37;
        const SEARCH_INCLUDE_REQUEST_DETAILS = 1 << 38;

        // =================================================================
        // With info flag (bit 62)
        // =================================================================
        const WITH_INFO = 1 << 62;

        // =================================================================
        // Search flags that alias export flags (bits 0-3)
        // These have the same bit values as their export counterparts
        // =================================================================
        const SEARCH_INCLUDE_RESOLVED = 1 << 0;  // Same as EXPORT_INCLUDE_MULTI_RECORD_ENTITIES
        const SEARCH_INCLUDE_POSSIBLY_SAME = 1 << 1;  // Same as EXPORT_INCLUDE_POSSIBLY_SAME
        const SEARCH_INCLUDE_POSSIBLY_RELATED = 1 << 2;  // Same as EXPORT_INCLUDE_POSSIBLY_RELATED
        const SEARCH_INCLUDE_NAME_ONLY = 1 << 3;  // Same as EXPORT_INCLUDE_NAME_ONLY

        // =================================================================
        // Composite flags - Export
        // =================================================================
        const EXPORT_INCLUDE_ALL_ENTITIES = Self::EXPORT_INCLUDE_MULTI_RECORD_ENTITIES.bits()
            | Self::EXPORT_INCLUDE_SINGLE_RECORD_ENTITIES.bits();

        const EXPORT_INCLUDE_ALL_HAVING_RELATIONSHIPS = Self::EXPORT_INCLUDE_POSSIBLY_SAME.bits()
            | Self::EXPORT_INCLUDE_POSSIBLY_RELATED.bits()
            | Self::EXPORT_INCLUDE_NAME_ONLY.bits()
            | Self::EXPORT_INCLUDE_DISCLOSED.bits();

        // =================================================================
        // Composite flags - Entity relations
        // =================================================================
        const ENTITY_INCLUDE_ALL_RELATIONS = Self::ENTITY_INCLUDE_POSSIBLY_SAME_RELATIONS.bits()
            | Self::ENTITY_INCLUDE_POSSIBLY_RELATED_RELATIONS.bits()
            | Self::ENTITY_INCLUDE_NAME_ONLY_RELATIONS.bits()
            | Self::ENTITY_INCLUDE_DISCLOSED_RELATIONS.bits();

        // =================================================================
        // Composite flags - Search
        // =================================================================
        const SEARCH_INCLUDE_ALL_ENTITIES = Self::SEARCH_INCLUDE_RESOLVED.bits()
            | Self::SEARCH_INCLUDE_POSSIBLY_SAME.bits()
            | Self::SEARCH_INCLUDE_POSSIBLY_RELATED.bits()
            | Self::SEARCH_INCLUDE_NAME_ONLY.bits();

        // =================================================================
        // Composite flags - Record
        // =================================================================
        const RECORD_ALL_FLAGS = Self::ENTITY_INCLUDE_INTERNAL_FEATURES.bits()
            | Self::ENTITY_INCLUDE_RECORD_FEATURES.bits()
            | Self::ENTITY_INCLUDE_RECORD_FEATURE_DETAILS.bits()
            | Self::ENTITY_INCLUDE_RECORD_FEATURE_STATS.bits()
            | Self::ENTITY_INCLUDE_RECORD_DATES.bits()
            | Self::ENTITY_INCLUDE_RECORD_JSON_DATA.bits()
            | Self::ENTITY_INCLUDE_RECORD_UNMAPPED_DATA.bits();

        const RECORD_PREVIEW_ALL_FLAGS = Self::ENTITY_INCLUDE_INTERNAL_FEATURES.bits()
            | Self::ENTITY_INCLUDE_RECORD_FEATURES.bits()
            | Self::ENTITY_INCLUDE_RECORD_FEATURE_DETAILS.bits()
            | Self::ENTITY_INCLUDE_RECORD_FEATURE_STATS.bits()
            | Self::ENTITY_INCLUDE_RECORD_JSON_DATA.bits()
            | Self::ENTITY_INCLUDE_RECORD_UNMAPPED_DATA.bits();

        // =================================================================
        // Core entity flags (used in defaults)
        // =================================================================
        const ENTITY_CORE_FLAGS = Self::ENTITY_INCLUDE_REPRESENTATIVE_FEATURES.bits()
            | Self::ENTITY_INCLUDE_ENTITY_NAME.bits()
            | Self::ENTITY_INCLUDE_RECORD_SUMMARY.bits()
            | Self::ENTITY_INCLUDE_RECORD_DATA.bits()
            | Self::ENTITY_INCLUDE_RECORD_MATCHING_INFO.bits();
    }
}

impl Default for SzFlags {
    fn default() -> Self {
        SzFlags::empty()
    }
}

impl SzFlags {
    /// No flags set
    pub const NO_FLAGS: SzFlags = SzFlags::empty();

    // =========================================================================
    // Record defaults
    // =========================================================================

    /// Default flags for record operations
    pub const RECORD_DEFAULT_FLAGS: SzFlags = SzFlags::ENTITY_INCLUDE_RECORD_JSON_DATA;

    /// Default flags for record preview operations
    pub const RECORD_PREVIEW_DEFAULT_FLAGS: SzFlags =
        SzFlags::ENTITY_INCLUDE_RECORD_FEATURE_DETAILS;

    // =========================================================================
    // Entity defaults
    // =========================================================================

    /// Default flags for entity operations
    pub const ENTITY_DEFAULT_FLAGS: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::ENTITY_CORE_FLAGS.bits()
            | SzFlags::ENTITY_INCLUDE_ALL_RELATIONS.bits()
            | SzFlags::ENTITY_INCLUDE_RELATED_ENTITY_NAME.bits()
            | SzFlags::ENTITY_INCLUDE_RELATED_RECORD_SUMMARY.bits()
            | SzFlags::ENTITY_INCLUDE_RELATED_MATCHING_INFO.bits(),
    );

    /// Brief default flags for entity operations
    pub const ENTITY_BRIEF_DEFAULT_FLAGS: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::ENTITY_INCLUDE_ALL_RELATIONS.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_MATCHING_INFO.bits()
            | SzFlags::ENTITY_INCLUDE_RELATED_MATCHING_INFO.bits(),
    );

    // =========================================================================
    // Export defaults
    // =========================================================================

    /// Default flags for export operations
    pub const EXPORT_DEFAULT_FLAGS: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::EXPORT_INCLUDE_ALL_ENTITIES.bits() | SzFlags::ENTITY_DEFAULT_FLAGS.bits(),
    );

    // =========================================================================
    // Find path defaults
    // =========================================================================

    /// Default flags for find path operations
    pub const FIND_PATH_DEFAULT_FLAGS: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::FIND_PATH_INCLUDE_MATCHING_INFO.bits()
            | SzFlags::ENTITY_INCLUDE_ENTITY_NAME.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY.bits(),
    );

    // =========================================================================
    // Find network defaults
    // =========================================================================

    /// Default flags for find network operations
    pub const FIND_NETWORK_DEFAULT_FLAGS: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::FIND_NETWORK_INCLUDE_MATCHING_INFO.bits()
            | SzFlags::ENTITY_INCLUDE_ENTITY_NAME.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY.bits(),
    );

    // =========================================================================
    // Search by attributes defaults
    // =========================================================================

    /// All search by attributes flags
    pub const SEARCH_BY_ATTRIBUTES_ALL: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::SEARCH_INCLUDE_ALL_ENTITIES.bits()
            | SzFlags::SEARCH_INCLUDE_STATS.bits()
            | SzFlags::ENTITY_INCLUDE_REPRESENTATIVE_FEATURES.bits()
            | SzFlags::ENTITY_INCLUDE_ENTITY_NAME.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY.bits()
            | SzFlags::INCLUDE_FEATURE_SCORES.bits(),
    );

    /// Strong search by attributes flags
    pub const SEARCH_BY_ATTRIBUTES_STRONG: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::SEARCH_INCLUDE_RESOLVED.bits()
            | SzFlags::SEARCH_INCLUDE_POSSIBLY_SAME.bits()
            | SzFlags::SEARCH_INCLUDE_STATS.bits()
            | SzFlags::ENTITY_INCLUDE_REPRESENTATIVE_FEATURES.bits()
            | SzFlags::ENTITY_INCLUDE_ENTITY_NAME.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY.bits()
            | SzFlags::INCLUDE_FEATURE_SCORES.bits(),
    );

    /// Minimal all search by attributes flags
    pub const SEARCH_BY_ATTRIBUTES_MINIMAL_ALL: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::SEARCH_INCLUDE_ALL_ENTITIES.bits() | SzFlags::SEARCH_INCLUDE_STATS.bits(),
    );

    /// Minimal strong search by attributes flags
    pub const SEARCH_BY_ATTRIBUTES_MINIMAL_STRONG: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::SEARCH_INCLUDE_RESOLVED.bits()
            | SzFlags::SEARCH_INCLUDE_POSSIBLY_SAME.bits()
            | SzFlags::SEARCH_INCLUDE_STATS.bits(),
    );

    /// Default flags for search by attributes operations
    pub const SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS: SzFlags = Self::SEARCH_BY_ATTRIBUTES_ALL;

    // =========================================================================
    // Why defaults
    // =========================================================================

    /// Default flags for why entities operations
    pub const WHY_ENTITIES_DEFAULT_FLAGS: SzFlags = SzFlags::INCLUDE_FEATURE_SCORES;

    /// Default flags for why records operations
    pub const WHY_RECORDS_DEFAULT_FLAGS: SzFlags = SzFlags::INCLUDE_FEATURE_SCORES;

    /// Default flags for why record in entity operations
    pub const WHY_RECORD_IN_ENTITY_DEFAULT_FLAGS: SzFlags = SzFlags::INCLUDE_FEATURE_SCORES;

    /// Default flags for why search operations
    pub const WHY_SEARCH_DEFAULT_FLAGS: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::INCLUDE_FEATURE_SCORES.bits()
            | SzFlags::SEARCH_INCLUDE_REQUEST_DETAILS.bits()
            | SzFlags::SEARCH_INCLUDE_STATS.bits(),
    );

    // =========================================================================
    // How defaults
    // =========================================================================

    /// Default flags for how entity operations
    pub const HOW_ENTITY_DEFAULT_FLAGS: SzFlags = SzFlags::INCLUDE_FEATURE_SCORES;

    /// All flags for how entity operations
    pub const HOW_ALL_FLAGS: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::INCLUDE_MATCH_KEY_DETAILS.bits() | SzFlags::INCLUDE_FEATURE_SCORES.bits(),
    );

    // =========================================================================
    // Virtual entity defaults
    // =========================================================================

    /// Default flags for virtual entity operations
    pub const VIRTUAL_ENTITY_DEFAULT_FLAGS: SzFlags = SzFlags::ENTITY_CORE_FLAGS;

    /// All flags for virtual entity operations
    pub const VIRTUAL_ENTITY_ALL_FLAGS: SzFlags = SzFlags::from_bits_truncate(
        SzFlags::ENTITY_INCLUDE_ALL_FEATURES.bits()
            | SzFlags::ENTITY_INCLUDE_REPRESENTATIVE_FEATURES.bits()
            | SzFlags::ENTITY_INCLUDE_ENTITY_NAME.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_TYPES.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_DATA.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_MATCHING_INFO.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_DATES.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_JSON_DATA.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_UNMAPPED_DATA.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_FEATURES.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_FEATURE_DETAILS.bits()
            | SzFlags::ENTITY_INCLUDE_RECORD_FEATURE_STATS.bits()
            | SzFlags::ENTITY_INCLUDE_INTERNAL_FEATURES.bits()
            | SzFlags::ENTITY_INCLUDE_FEATURE_STATS.bits(),
    );

    // =========================================================================
    // Add/Delete/Reevaluate defaults
    // =========================================================================

    /// Default flags for adding records
    pub const ADD_RECORD_DEFAULT_FLAGS: SzFlags = SzFlags::empty();

    /// All flags for adding records (with info)
    pub const ADD_RECORD_ALL_FLAGS: SzFlags = SzFlags::WITH_INFO;

    /// Default flags for deleting records
    pub const DELETE_RECORD_DEFAULT_FLAGS: SzFlags = SzFlags::empty();

    /// All flags for deleting records (with info)
    pub const DELETE_RECORD_ALL_FLAGS: SzFlags = SzFlags::WITH_INFO;

    /// Default flags for reevaluating records
    pub const REEVALUATE_RECORD_DEFAULT_FLAGS: SzFlags = SzFlags::empty();

    /// All flags for reevaluating records (with info)
    pub const REEVALUATE_RECORD_ALL_FLAGS: SzFlags = SzFlags::WITH_INFO;

    /// Default flags for reevaluating entities
    pub const REEVALUATE_ENTITY_DEFAULT_FLAGS: SzFlags = SzFlags::empty();

    /// All flags for reevaluating entities (with info)
    pub const REEVALUATE_ENTITY_ALL_FLAGS: SzFlags = SzFlags::WITH_INFO;

    /// Default flags for redo operations
    pub const REDO_DEFAULT_FLAGS: SzFlags = SzFlags::empty();

    /// All flags for redo operations (with info)
    pub const REDO_ALL_FLAGS: SzFlags = SzFlags::WITH_INFO;

    // =========================================================================
    // Find interesting entities defaults
    // =========================================================================

    /// Default flags for find interesting entities operations
    pub const FIND_INTERESTING_ENTITIES_DEFAULT_FLAGS: SzFlags = SzFlags::empty();

    /// All flags for find interesting entities operations
    pub const FIND_INTERESTING_ENTITIES_ALL_FLAGS: SzFlags = SzFlags::empty();

    // =========================================================================
    // Utility methods
    // =========================================================================

    /// Converts optional flags to i64 for FFI calls
    ///
    /// This helper consolidates the common pattern of converting `Option<SzFlags>`
    /// to the i64 value expected by FFI functions.
    #[inline]
    pub fn to_ffi(flags: Option<SzFlags>, default: SzFlags) -> i64 {
        flags.unwrap_or(default).bits() as i64
    }
}
