# Contributing

Contributions are welcome and encouraged! See [/issues][issues] for ideas, or suggest your own!
If you're thinking to create a PR with large feature/change, please first discuss it in an issue.

[issues]: https://github.com/mihai-dinculescu/tapo/issues

## AI-assisted contributions

This repository's maintainer loves AI and is always excited to find new ways to leverage it. Even so, they are making the conscious choice to run this project with a human in the loop: every change is carefully reviewed by a human, particularly changes that affect the library API.

The maintainer has their own frontier model subscriptions, agent skills and well-curated local memory. A human-verified issue that the maintainer can quickly solve and test using their own tokens is a lot more valuable to this project than an AI-driven PR with no human in the loop.

With that in mind, issues and pull requests created with the help of AI agents are welcome, on one condition: a human has read, understood and verified the content, and takes ownership of it. **The agent is a tool; you are the contributor.**

Tokens are cheap; human attention is expensive. AI contributions that show very little human input will also get very little human attention, out of practicality. Fully automated accounts that open issues or pull requests without a human in the loop will get only the briefest of glances before being dismissed, with the help of AI.

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
  git push && git push origin vX.X.X
  ```

- Create the [release][releases].

[releases]: https://github.com/mihai-dinculescu/tapo/releases
