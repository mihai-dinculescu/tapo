# Contributing

Contributions are welcome and encouraged! See [/issues][issues] for ideas, or suggest your own!
If you're thinking to create a PR with large feature/change, please first discuss it in an issue.

## Releases

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

[issues]: https://github.com/mihai-dinculescu/tapo/issues
[releases]: https://github.com/mihai-dinculescu/tapo/releases
[crates_io]: https://crates.io
