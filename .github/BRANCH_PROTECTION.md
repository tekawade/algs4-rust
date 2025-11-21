# Branch Protection Rules for algs4-rust

This document outlines the recommended branch protection settings for the algs4-rust repository to ensure code quality and require owner approval for all changes to the main/master branch.

## Recommended Settings for Main/Master Branch

### 1. General Protection Rules

Navigate to: `Settings` → `Branches` → `Branch protection rules` → Add rule for `main` (or `master`)

### 2. Required Settings

#### Require pull request reviews before merging
- [x] **Enable**: Require a pull request before merging
- [x] **Required approvals**: 1
- [x] **Dismiss stale pull request approvals when new commits are pushed**
- [x] **Require review from Code Owners** (uses `.github/CODEOWNERS` file)
- [ ] **Restrict who can dismiss pull request reviews** (optional, only needed for teams)

#### Require status checks to pass before merging
- [x] **Enable**: Require status checks to pass before merging
- [x] **Require branches to be up to date before merging**

**Required status checks** (must pass before merge):
- `test` - Run all unit and integration tests
- `fmt` - Check code formatting with rustfmt
- `clippy` - Run clippy linter with -D warnings
- `doc` - Check documentation builds without warnings
- `build-release` - Ensure release build succeeds

#### Other Protection Rules
- [x] **Require conversation resolution before merging** - All PR comments must be resolved
- [x] **Require signed commits** (optional but recommended for security)
- [x] **Require linear history** (optional, requires squash or rebase merging instead of merge commits)
- [x] **Include administrators** - Apply these rules to repository admins too
- [ ] **Allow force pushes** - DISABLED (prevents history rewriting)
- [ ] **Allow deletions** - DISABLED (prevents branch deletion)

### 3. CODEOWNERS Configuration

The `.github/CODEOWNERS` file has been created with `@tekawade` as the default owner for all files. This means:

- Any pull request to main/master will automatically request review from @tekawade
- The PR cannot be merged until @tekawade approves it
- This ensures you maintain control over what gets merged to the main branch

### 4. Recommended Workflow

1. **Development**: All work should be done on feature branches (`feature/`, `fix/`, `docs/`, etc.)
2. **Pull Requests**: Create a PR to merge changes into main/master
3. **CI Checks**: Wait for all GitHub Actions CI checks to pass (test, fmt, clippy, doc, build-release)
4. **Code Review**: Review your own PR or have a team member review
5. **Approval**: Approve the PR (as code owner)
6. **Merge**: Merge only after approval and passing CI

### 5. How to Enable These Rules

1. Go to your repository on GitHub
2. Click **Settings** (requires admin access)
3. Click **Branches** in the left sidebar
4. Click **Add branch protection rule**
5. In "Branch name pattern", enter: `main` (or `master` if that's your default branch)
6. Enable the checkboxes as described in section 2 above
7. In the "Require status checks to pass" section, search for and select:
   - `test`
   - `fmt`
   - `clippy`
   - `doc`
   - `build-release`
8. Click **Create** or **Save changes**

### 6. Additional Security Recommendations

- **Enable Dependabot**: Automatically create PRs for dependency updates
  - Go to `Settings` → `Security & analysis` → Enable "Dependabot alerts" and "Dependabot security updates"

- **Enable Secret Scanning**: Detect accidentally committed secrets
  - Go to `Settings` → `Security & analysis` → Enable "Secret scanning"

- **Require 2FA**: Require two-factor authentication for all contributors
  - Go to `Settings` → `Options` → Enable "Require two-factor authentication"

### 7. Testing the Configuration

After setting up branch protection:

1. Create a test branch: `git checkout -b test/branch-protection`
2. Make a small change and push: `git push -u origin test/branch-protection`
3. Create a pull request on GitHub
4. Verify that:
   - CI checks run automatically
   - You are requested as a reviewer (via CODEOWNERS)
   - You cannot merge until approval + CI passes
5. Approve and merge the PR
6. Delete the test branch

## Summary

With these settings in place:

✅ All changes to main/master require a pull request
✅ All PRs require your approval (@tekawade)
✅ All PRs must pass comprehensive CI checks (tests, linting, docs, build)
✅ No direct pushes to main/master are allowed
✅ Force pushes and branch deletion are disabled
✅ This ensures complete control over what gets merged to production

This configuration provides excellent protection for a solo developer or small team while maintaining code quality standards.
