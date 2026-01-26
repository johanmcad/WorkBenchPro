# Claude Instructions for WorkBench-Pro

## Build Command

When the user types "build", perform the following steps:

### 1. Rust Desktop App
- Build release: `cargo build --release`
- Upload to GitHub release: `gh release upload v1.1.0 target/release/workbench-pro.exe --clobber`

### 2. Web App
- Commit and push changes - Vercel auto-deploys from main branch

### 3. Database (if schema changes)
- Apply migrations via Supabase MCP tool

## Version Updates (for new releases)

When creating a new version:
1. Update version in `Cargo.toml`
2. Update version in `build.rs` (FileVersion, ProductVersion)
3. Create new GitHub release: `gh release create vX.X.X --title "WorkBench vX.X.X" --notes "Release notes here" target/release/workbench-pro.exe`
