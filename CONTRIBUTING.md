# Contributing

Contributions are welcome and encouraged! See [/issues][issues] for ideas, or suggest your own!
If you're thinking to create a PR with large feature/change, please first discuss it in an issue.

## Releases

### Rust

- Update version in `tapo/Cargo.toml`
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

- Release\
  Create a [new release][releases]. \
  The `publish.yml` GitHub Action will pick it up and do the actual release to [crates.io][crates_io].

### Python

Until it reaches feature parity with the Rust library, the Python library will be released separately and have a independent version.

- Update version in `tapo-py/pyproject.toml`
- Update CHANGELOG.md
- Commit
- Add tag

  ```bash
  git tag -a py-vX.X.X -m "py-vX.X.X"
  ```

- Push

  ```bash
  git push --follow-tags
  ```

- Release\
  Create a [new release][releases].


[issues]: https://github.com/mihai-dinculescu/tapo/issues
[releases]: https://github.com/mihai-dinculescu/tapo/releases
[crates_io]: https://crates.io
