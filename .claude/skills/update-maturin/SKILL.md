---
name: update-maturin
description: Bump maturin to the latest release and regenerate `.github/workflows/tapo-py.yml`, preserving local customizations
---

# Update Maturin

Bump the `maturin` build tool to the latest version and regenerate `.github/workflows/tapo-py.yml` from `maturin generate-ci github`, re-applying all local customizations.

## Steps

1. **Bump the maturin dependency.** From `tapo-py/`:

   ```bash
   uv lock --upgrade-package maturin
   uv sync
   uv run maturin --version
   ```

   Confirm the new version in the last command's output. If it didn't change, maturin is already at the latest — stop and tell the user.

2. **Regenerate the workflow baseline.** From `tapo-py/`, run `uv run maturin generate-ci github` and capture the stdout (skip the `📦`/`🍹`/`🔗` progress lines). This is the baseline — do NOT overwrite `.github/workflows/tapo-py.yml` with it directly.

3. **Apply the baseline to `.github/workflows/tapo-py.yml`, re-applying each customization below.** After editing, diff the updated file against the baseline — the only remaining differences should be exactly the items listed here. Anything in the committed file that is **not** on this list should be replaced with the baseline value, even if the committed file differs; treat those as stale upstream defaults, not deliberate customizations.

   - **Workflow name**: `name: "Tapo Python"` (baseline: `name: CI`).
   - **Triggers** under `on:`:
     - `push.branches`: only `main` (baseline includes `master`).
     - `push.tags`: `"v*"` (baseline: `'*'`).
     - `pull_request.paths`: keep the existing list (`tapo/**`, `tapo-py/**`, `Cargo.lock`, `Cargo.toml`, `.github/workflows/tapo-py.yml`). Baseline has no path filter.
   - **`--manifest-path ./tapo-py/Cargo.toml`**: append this flag to every `args:` line in each `PyO3/maturin-action@v1` step (linux, musllinux, windows, macos, and sdist jobs).
   - **`manylinux: 2_28`**: in the `linux` job (baseline: `auto`).
   - **GitHub Actions versions**: never downgrade. For each `uses: <action>@vN` line, keep the higher of the two versions between the current file and the baseline (e.g. if the file pins `actions/upload-artifact@v7` but the baseline emits `@v6`, keep `@v7`). To make this mechanical, compare with `diff <(grep 'uses: actions/' .github/workflows/tapo-py.yml) <(grep 'uses: actions/' <baseline-file>)` and reconcile each differing line by picking the higher `@vN`.
   - **Job display names**: each job has a `name:` using the `Python / ...` prefix convention. The baseline emits no `name:` on the matrix jobs and `name: Release` on the release job — re-add these after regeneration:
     - `linux`: `name: Python / Build wheel (linux, ${{ matrix.platform.target }})`
     - `musllinux`: `name: Python / Build wheel (musllinux, ${{ matrix.platform.target }})`
     - `windows`: `name: Python / Build wheel (windows, ${{ matrix.platform.target }}, ${{ matrix.platform.python_arch }})`
     - `macos`: `name: Python / Build wheel (macos, ${{ matrix.platform.target }})`
     - `sdist`: `name: Python / Build sdist`
     - `release`: `name: Python / Publish to PyPI` (baseline: `Release`)
   - **`Test sdist` step**: keep this step between `Build sdist` and `Upload sdist` in the `sdist` job:

     ```yaml
     - name: Test sdist
       run: |
         pip install --force-reinstall --verbose dist/*.tar.gz
         python -c 'from tapo import ApiClient'
     ```

4. **Verify.** Run `git diff .github/workflows/tapo-py.yml`. Every line change should be either (a) the `maturin v<old>` → `v<new>` version bump in the header comment, (b) an upstream structural change introduced by the new maturin release, or (c) one of the preserved customizations above. Flag anything else before handing back.
