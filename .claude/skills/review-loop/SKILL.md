---
name: review-loop
description: Run /code-review repeatedly on a target, fixing only findings that pass the review bar below, until three consecutive rounds accept nothing. Findings about device behavior are settled by read-only calls to real devices, not by the decompiled app. Ends with /simplify and a summary for review. Does not commit.
---

# Review Loop

Review a target repeatedly until the reviews stop producing findings worth fixing, without letting the fixes complicate the code or make the API worse to use.

The argument is the target, passed through to `/code-review` (a path, branch or PR number). With no argument, review the current diff, staged and unstaged. Never stage or unstage anything while the loop runs.

## Each round

1. **Review.** Run `/code-review high <target>`. Never pass `--fix`: nothing is applied before triage.
2. **Triage.** Judge every finding against the review bar below. Check the claimed failure against the actual code, and when it is a claim about what a device sends or does, against the evidence in "Device evidence". Do not accept a finding on the reviewer's word.
   - A finding that repeats one already rejected in this run, or one the user declined in an earlier review (recorded in memory), is rejected again without re-litigating it.
   - When unsure whether a failure is reachable, reject it and list it as "unconfirmed" in the summary.
3. **Fix.** Apply the accepted findings only, each with the smallest change that works. If a fix turns out to need more code than the problem deserves, revert it and move the finding to rejected.
   - A fix that changes a type or method exposed to Python is carried through `tapo-py` and its `.pyi` stubs in the same round. One that changes an MCP result is carried through `tapo-mcp`.
   - After a round with fixes, run `cargo check --workspace --all-features` and `cargo make test` before the next review. Never `cargo test --all-features`.
4. **Record.** Keep a running list for the round: accepted (with the fix made and the evidence it rests on) and rejected (with a one-line reason).

## Review bar

Accept a finding only if all of these hold:

- **It is real.** The failure can be reached through the public API of `tapo`, `tapo-py` or `tapo-mcp` with input or device behavior that actually occurs. A path that needs a response no device has been seen to send does not count.
- **It matters to a caller.** A wrong result, a panic, an error on a working device, a misleading doc, or a Rust, Python and MCP surface that disagree. Style preferences and "could be more defensive" do not count.
- **The fix does not cost the caller more than the problem.** Callers pay for every `Option`, every extra error variant and every fallback branch, on every call. A fix that makes the common case clumsier to guard against a case nobody has observed is rejected.

Reject, whatever the reviewer's confidence:

- Defensive handling for fields, values or error codes that appear only in the decompiled app, or only in the reviewer's imagination.
- Making a field optional, or adding `#[serde(default)]`, "to be safe". See the worked example below.
- Missing counterparts on another protocol, device family or handler that the change did not set out to cover.
- New test approaches. Tests follow the existing idioms: unit tests next to the code, and `tapo/tests/<model>.rs` against the fixture-driven simulator, where calling the method and getting `Ok` is the test.
- Behavior of the `_ => unreachable!()` arms that `tapo-py`, `tapo-mcp` and the examples carry for `#[non_exhaustive]` enums. They exist only to compile.
- Anything judged against an unmerged PR rather than `main`.

## Device evidence

Many findings in this repository are really claims about a device: "this field can be missing", "this value can be negative", "the device may answer with an error here". The code cannot settle those, and neither can the reviewer. Settle them with evidence, in this order:

1. **Real devices.** A read-only call to one of the user's devices, a fixture under `tapo/tests/fixtures/` (they are captured device answers), or a device log a tester posted in a GitHub issue.
2. **The decompiled Tapo app.** Use it only to fill gaps a device cannot show, such as what happens past a paging limit. The app carries code for other models, older firmware and paths it never takes, so "the app handles it" is not proof that a device sends it. When the app and a device disagree, the device wins.
3. **The reviewer's reasoning.** On its own it is not evidence for anything a device does.

### Asking a real device

Go and look instead of assuming. It is cheap, and it is the strongest evidence available.

- Find the device with the tapo MCP `list_devices` tool, which gives the model and IP. Credentials are in `docs/.env` (`set -a && . docs/.env && set +a`); never print them and never ask the user for them.
- For anything `get_device_state` does not show, write a small client in the scratchpad: a scratch crate that depends on `tapo` by path with the `debug` feature, or a Python script against the `tapo-py` venv. Print the raw response, so the fields are seen as the device sent them rather than as the library parsed them.
- **Read-only calls only**: login, device info, usage and energy data, child lists, logs and records, component lists, firmware state. The devices run a lived-in home. Never switch a plug, change a light, sound an alarm, move or reboot a camera, rename a device, or start a firmware update, and never run a stock example from `tapo/examples` or `tapo-py/examples` as it is, because most of them change state.
- Ask every model the finding concerns that the user owns, and note firmware versions. One model does not speak for another.
- If the finding can only be confirmed by changing device state, or the user owns no such device and no fixture or issue log covers it, do not guess: reject it as "unconfirmed" and say in the summary which call on which model would settle it.

### Worked example: optional fields

The reviewer says: "`rssi` is deserialized as a plain `i16`. The app's model class declares it nullable, so parsing fails if the device omits it. Make it `Option<i16>`."

- Query the models the struct serves and read the fixtures for them. Every one returns `rssi`.
- Reject. A field that every observed device always sends stays required. Marking it optional out of caution forces every caller to unwrap a value that is always there, in Rust and in Python, which is a worse library for all users in exchange for protection against a response nobody has seen.
- Had one model or firmware omitted it, the finding would be accepted, with that device named as the evidence, and with the narrowest fix: optional in the struct that device uses, not in every struct that shares the field.

The same reasoning runs the other way. If the code marks a field optional and every device and fixture sends it, a finding that asks to make it required is a good one.

When a device confirms something the code so far justified with the app, the comment is rewritten to cite the device.

## Stopping

Stop when either is true:

- three consecutive rounds accepted nothing, or
- 24 rounds have run.

"The reviewer found nothing" is not the condition. A reviewer can always produce something; only accepted findings count.

## After the last round

1. Run `/simplify` once if any fixes were applied.
2. Present a summary in chat:
   - rounds run, and why the loop stopped
   - a table of applied fixes: file, problem, change, evidence (device and firmware, fixture, issue log, or "code only")
   - a table of rejected findings: finding, reason (mark the "unconfirmed" ones, with the device call that would settle each)
   - the read-only device calls made: model, firmware, what was asked
3. Do not commit, and do not write changelog entries: `/commit` handles both. Wait for the user to review.
