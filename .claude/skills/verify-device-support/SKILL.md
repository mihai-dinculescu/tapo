---
name: verify-device-support
description: Verify that every statement of which devices are supported and what each handler can do is correct, complete, and in sync across SUPPORTED_DEVICES.md, the "Tested with" sentences, the ApiClient builders, the handler and device-info docs, the Python stubs, the examples, and tapo-mcp
---

# Verify Device Support

Device support is stated in many places and none of them is generated from the others. This skill checks that they all agree with the code, which is the source of truth:

- `DeviceType::from_model` in `tapo/src/api/discovery/device_type.rs` says which model strings the library knows and which category each belongs to
- `DiscoveryResult::new` in `tapo/src/api/discovery/discovery_result.rs` says which handler each category gets
- the `model` match in `tapo/src/responses/child_device_list_hub_result/child_device_hub_result.rs` says which hub child models are decoded, and so which child handler each gets
- each handler's `pub` methods (explicit ones plus the ones `tapo_handler!`, `tapo_child_handler!` and `hub_child_handlers!` generate) say what it can do, and `#[cfg(feature = "debug")]` says which of them need the `debug` feature

Run the whole skill whenever a device model is added or removed, a handler gains or loses a method, a method's `debug` gating changes, or on request. On its own, this skill reports and does not change anything. When `/pre-commit` invokes it, or the user asked for the findings to be fixed, fix every finding, rerun the script until it exits 0, and report what was changed. Never post anywhere.

## Step 1: Run the mechanical checks

```bash
python -I .claude/skills/verify-device-support/scripts/device_inventory.py
```

The script reads the sources above and compares them with every documentation surface listed in Step 2. It prints one line per mismatch, prefixed with the check that found it, and exits 1 when there are any. Lines prefixed `note:` are not findings. They point at wording that is a superset of what the code serves (a shared device-info type, the camera doc naming untested models, a model alias the device reports) and need a human decision, which Step 3 covers.

If the script crashes or reports that it cannot find a handler, a table column, or a mapping, the code moved in a way the script does not understand. Fix the script in the same change, then rerun it. Do not work around it by reading the files by hand only; the script is what keeps the next run honest.

Every finding the script prints is a real mismatch unless Step 3 says the wording is deliberate. Carry each one into the report.

## Step 2: What the script covers, and what to read when it reports something

### The "Tested with" sentence

One sentence, identical in all of these:

- `README.md` (`tapo/README.md` and `tapo-py/README.md` are symlinks to it; the script checks that they still are)
- `tapo/Cargo.toml` `description`
- `tapo/src/lib.rs` crate doc (split over several `//!` lines)
- `tapo-py/pyproject.toml` `description`
- `tapo-py/tapo-py/tapo/api_client.pyi` module docstring and `ApiClient` class docstring

The script checks that the sentences are identical after joining lines, and that the models it names are exactly the models `DeviceType::from_model` maps (top-level devices) plus the models `ChildDeviceHubResult` decodes (switches and sensors). A model the code accepts but the sentence omits, or the other way round, is a finding.

### The ApiClient builders

- Rust: `pub async fn l510(...) -> Result<LightHandler, Error>` and friends in `tapo/src/api/api_client.rs`
- Python binding: the same names in `tapo-py/src/api/api_client.rs`, returning the `Py*` handler
- Python stub: the same names in `api_client.pyi`, returning the stub handler, with a `Returns:` line that names the models the handler serves

The script checks that the three lists match, that each builder returns the same handler on every surface, that every top-level model in "Tested with" has a builder (a builder serves its own model and the `M` variant: `p110` serves P110 and P110M, `p304` serves P304M), and that each stub `Returns:` line names either every model of that handler or only the builder's own model.

### Handler docs

Every handler has a "Handler for the ..." paragraph that names its models with links:

- Rust: the doc comment on the `tapo_handler!` / `tapo_child_handler!` invocation, or on the struct for `CameraHubHandler`
- Python stub: the class docstring in `tapo-py/tapo-py/tapo/<handler>.pyi`
- the `handler:` field docs of every `DiscoveryResult` variant in `discovery_result.rs`, and the matching class in `discovery_result.pyi`
- the `Returns:` line of every child builder (`ke100`, `s200`, ..., `plug`) in `hub_handler.pyi`, `camera_hub_handler.pyi`, and the two power strip stubs

The script checks that each of these names exactly the models discovery maps to that handler. The `DeviceType` variant docs in `device_type.rs` and `device_type.pyi` are checked the same way.

### Handler methods

For each handler, the script collects the Rust methods (explicit `pub fn`s plus the macro-generated ones, with their `debug` gating) and the Python stub methods (the class's own `def`s plus those of its `*Ext` Protocol bases). It reports a method present on one side only. Python always has the `debug` feature on, so a debug-gated Rust method must still be in the stub.

### Device info docs

Every "Device info of Tapo ..." doc, and the "... power strip child plugs" docs, in `tapo/src/responses/{device_info_result,child_device_list_hub_result,child_device_list_power_strip_result}/` and the stub with the same name under `tapo-py/tapo-py/tapo/responses/`. The script checks that Rust and Python name the same models and that the handler returning the type is covered by them.

### SUPPORTED_DEVICES.md

Each table column is a group of models. The script maps the group to its handler through discovery, then checks every cell:

- `&#x2705;` when the method exists in Rust and in the Python stub
- `&check;` when it exists in Rust only
- blank when the handler does not have it
- `(d)` on the row when, and only when, the Rust method is behind `#[cfg(feature = "debug")]`

It also checks that every method of the handler has a row, that the columns for one handler list every model discovery maps to it, and that the set of models across all tables equals the "Tested with" set. Child builders (`ke100`, `plug`, `*_unchecked`) are not rows.

### Examples

`tapo/examples/tapo_*.rs` and `tapo-py/examples/tapo_*.py` must come in pairs with the same name, and their first line (`/// P304M and P316M Example`) must name the same models.

### tapo-mcp

`GetCapability::TriggerLogs` and `GetCapability::TemperatureHumidityRecords` are granted per child type in `tapo-mcp/src/requests/get_devices.rs`. The script checks that the models named in the `GetCapability` docs in `tapo-mcp/src/models.rs`, in the Get Capabilities table of `tapo-mcp/README.md`, and in the `get_device_state` examples of `tapo-mcp/openclaw-skill/SKILL.md` match what the code grants.

## Step 3: Judgement checks the script cannot make

Read these by hand every time, and when the script printed a `note:`.

- **Wording and links.** In every "Handler for" and "Device info of" paragraph the models should be comma separated with "and" before the last, each linked as `[MODEL](https://www.tapo.com/en/search/?q=MODEL)` (TP-Link-branded models such as the P300 and P304M link to `tp-link.com`). Rust and Python should read the same. Check that a model added in one place got the same link style as its neighbours.
- **Supersets the script only notes.** `DeviceInfoPowerStripResult` is shared by both power strip handlers, so its doc names all four power strips. Decide whether each note is still the intended wording and, if it is not, report it as a finding. A device-info doc that names a model `DeviceType::from_model` does not map is a finding, not a note to keep.
- **Model aliases.** `DeviceType::from_model` accepts strings the device reports that are not product names (`L530 Series`, `L535B`). They belong in the match, its unit test, and the `ALIASES` set in the script, and nowhere in the docs. The script reports an alias that reached the "Tested with" sentence and treats any other string in the match as a product name, so a new alias that is not in `ALIASES` shows up as an undocumented model. Report an alias that has leaked into a doc, and a doc model that has been added to the match as if it were an alias.
- **"Tested" means tested.** A model goes into the "Tested with" sentence only when someone has run the current code against that device. `SUPPORTED_DEVICES.md`, the handler docs and the changelog say what is supported, which is a weaker claim. If the change adds a model, ask where the test evidence is (an issue comment, `docs/TESTERS.md`, which is local-only, or the user's own device). Do not add a model to "Tested with" on the strength of the device sharing a handler.
- **Family words.** `tapo-mcp/src/server.rs` `with_instructions(...)` and the `description:` in `tapo-mcp/openclaw-skill/SKILL.md` list device families in words ("plugs, lights, power strips, hubs and their child sensors, cameras"). A new `DeviceType` category needs a word in both. The "Tested with" sentence groups models into families too; a new model must land in the right group (light bulbs, light strips, plugs, power strips, hubs, switches, sensors, cameras).
- **Table shape.** A handler whose models no longer fit one header cell is split over several columns (the cameras). The columns must be adjacent, and the legend lines above each table must be the same three lines.
- **Changelog.** A new model or method needs a changelog entry on both the Rust and the Python side; run `/changelog` for that rather than editing by hand. Do not verify historical entries.
- **TROUBLESHOOTING.md.** It names models only inside issue reports ("plugs (P100, P110, P115) upgraded to firmware 1.4.0"). Those are historical and are not device-support claims; leave them alone.

## Step 4: Report

Present the findings as one table with these columns: surface, what it says, what the code says, severity. Order by severity:

- **High**: a surface claims support the code does not have, or omits a model or method the code has (a user relying on it would be misled)
- **Medium**: Rust and Python disagree with each other but both are plausible
- **Low**: link style, wording, or a `note:` the user may want to keep

Then list the `note:` lines the user has to decide on, with your recommendation for each in one sentence. If the script reported nothing and the judgement checks found nothing, say so plainly.
