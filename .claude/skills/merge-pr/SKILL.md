---
name: merge-pr
description: Merge a GitHub PR by fast-forwarding main to its head, so every commit lands unchanged with its signature and no merge commit is created
---

# Merge PR

Merge PR `<N>` by pushing its head commit to `main` as a fast-forward. GitHub's "Rebase and merge" rewrites commits without signing them, "Squash" collapses them, and "Merge commit" adds a merge commit, so none of the merge buttons fit.

Usage: `/merge-pr <N>`. If no number is given, ask for it.

## Steps

Run every check before touching anything. On any failed check, stop and report it; do not work around it.

1. **Read the PR**

   ```sh
   gh pr view <N> --json number,title,state,isDraft,baseRefName,headRefName,headRefOid,isCrossRepository,headRepositoryOwner,reviewDecision,statusCheckRollup,commits
   ```

   - `state` must be `OPEN` and `isDraft` false.
   - `baseRefName` must be `main`.

2. **Fetch the head and check it fast-forwards `main`**

   ```sh
   git fetch origin main pull/<N>/head
   git rev-parse FETCH_HEAD   # must equal headRefOid
   git merge-base --is-ancestor origin/main <headRefOid>
   ```

   If it is not a descendant of `origin/main`, stop. Rebasing would replace the author's signatures, so ask the user whether to have the author rebase or to rebase and re-sign the commits themselves. Never rebase on your own.

3. **Verify every commit is signed**

   ```sh
   gh api repos/mihai-dinculescu/tapo/pulls/<N>/commits --paginate \
     --jq '.[] | "\(.sha[0:7]) \(.commit.verification.verified) \(.commit.verification.reason) \(.commit.message | split("\n")[0])"'
   ```

   Every commit must show `true`. GitHub's verification is the source of truth; a local `git log --show-signature` fails for keys not in the local keyring.

4. **Require green CI**

   Every entry in `statusCheckRollup` must have concluded `SUCCESS`, `NEUTRAL` or `SKIPPED`, and the required checks `Rust / Checks` and `Rust / Checks on minimum supported version` must be among them. If checks are still running, say so and stop; don't poll.

5. **Approve**

   If `reviewDecision` is not `APPROVED`, ask the user to confirm, then run `gh pr review <N> --approve`. Approving is outward-facing, so never do it without the user's go-ahead.

6. **Confirm the push**

   Show the user:
   - the commit list from step 3 (short SHA, subject, verified)
   - the exact command: `git push origin <headRefOid>:refs/heads/main`

   Wait for explicit confirmation. Pushing to `main` cannot be undone.

7. **Push**

   ```sh
   git push origin <headRefOid>:refs/heads/main
   ```

   Never use `--force`. If the push is rejected because `main` moved, stop and report it; don't retry.

8. **Confirm the merge**

   ```sh
   gh pr view <N> --json state,mergedAt,mergeCommit
   ```

   `state` should be `MERGED` and `mergeCommit.oid` should equal `headRefOid`. GitHub can take a few seconds to notice; check once more if it still shows `OPEN`, then report.

9. **Sync the local checkout**

   If the current branch is `main` and the working tree is clean, run `git pull --ff-only`. Otherwise just tell the user `main` moved.

10. **Head branch**

    For a same-repo PR (`isCrossRepository` false), ask whether to delete the head branch (`git push origin --delete <headRefName>`). Never delete it without asking. For a fork PR, leave it alone.

## Notes

- Branch protection on `main` requires an approving review and passing checks, but `enforce_admins` is off, so an admin's direct push is accepted. Steps 4 and 5 enforce those requirements, since the push itself would skip them.
- Issues referenced by the PR stay open until release; don't close them here.
