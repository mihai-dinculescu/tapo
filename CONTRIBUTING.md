# Contributing

Contributions are welcome and encouraged! See [/issues][issues] for ideas, or suggest your own!
If you're thinking to create a PR with large feature/change, please first discuss it in an issue.

[issues]: https://github.com/mihai-dinculescu/tapo/issues

## Releasing new versions

- Update version in `tapo/Cargo.toml`
- Update version in `tapo-py/pyproject.toml` (two places)
- Update CHANGELOG.md
- Commit
- Add tag

  ```bash
  git tag -a vX.X.X -m "vX.X.X"
  ```

- Push

  ```bash
  git push --follow-tags
  ```

- Create the [release][releases].

[releases]: https://github.com/mihai-dinculescu/tapo/releases
