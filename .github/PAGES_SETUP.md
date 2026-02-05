# GitHub Pages Setup Instructions

## Manual Setup Required

After pushing the repository to GitHub, you'll need to configure Pages in the repository settings:

### Steps:

1. **Go to Repository Settings**
   - Navigate to your repository on GitHub
   - Click on "Settings" tab
   - Scroll down to "Pages" section in the left sidebar

2. **Configure Source**
   - Under "Source", select "GitHub Actions"
   - This will enable the workflow in `.github/workflows/docs.yml`

3. **Verify Workflow Permissions**
   - Go to "Settings" > "Actions" > "General"
   - Under "Workflow permissions", ensure "Read and write permissions" is selected
   - Check "Allow GitHub Actions to create and approve pull requests"

4. **Enable Pages**
   - The workflow will automatically deploy docs on every push to main
   - Documentation will be available at: `https://[username].github.io/sz-rust-sdk/`

### What the Workflow Does:

- **Builds documentation** using `cargo doc --all-features --document-private-items`
- **Runs quality checks** (formatting, clippy, tests)
- **Creates redirect page** at root for easy navigation
- **Deploys to GitHub Pages** automatically on main branch pushes

### Manual Verification:

After setup, you can verify the workflow works by:

1. Pushing a change to the main branch
2. Checking the "Actions" tab for workflow runs
3. Visiting the Pages URL once deployment completes

### Troubleshooting:

- If workflow fails, check the Actions tab for error details
- Ensure repository has Pages enabled in settings
- Verify workflow has proper permissions to deploy
