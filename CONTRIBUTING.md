# Contributing

Contributions are welcome and encouraged! See [/issues][issues] for ideas, or suggest your own!
If you're thinking to create a PR with large feature/change, please first discuss it in an issue.

[issues]: https://github.com/mihai-dinculescu/tapo/issues

## Reverse Engineering the Tapo API
The Tapo API is not documented, but it can be discovered by reverse engineering the Android app.

- Download the APK (i.e. from [APKMirror](https://www.apkmirror.com/?&s=tapo))
- Decompile it using APK Studio or JADX

## Releasing new versions

- Update version in `tapo/Cargo.toml`
- Update version in `tapo-py/pyproject.toml`
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

- Create the [release][release].

[releases]: https://github.com/mihai-dinculescu/tapo/releases
