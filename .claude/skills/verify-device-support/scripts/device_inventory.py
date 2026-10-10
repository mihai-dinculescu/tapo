"""Cross-checks every place the repository states which devices are supported
and what each handler can do. Prints one line per mismatch and exits 1 when
any were found. Run from the repository root with `python -I`.

The checks here are mechanical (sets of models and method names). Wording,
links, and whether a model deserves to be called "tested" are left to the
skill that runs this script.
"""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
RUST_API = ROOT / "tapo" / "src" / "api"
PY_SRC_API = ROOT / "tapo-py" / "src" / "api"
PYI = ROOT / "tapo-py" / "tapo-py" / "tapo"

MODEL_RE = re.compile(r"\b(?:L|P|H|C|T|S|KE|TC)\d{2,3}[A-Z]{0,2}\b")
# Words that look like a model but are not one: the T310/T315 family
# placeholder used in type names, and video codecs.
NOT_MODELS = {"T31X", "H264", "H265"}

findings: list[str] = []


def finding(check: str, message: str) -> None:
    findings.append(f"[{check}] {message}")


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def models_in(text: str) -> set[str]:
    return set(MODEL_RE.findall(text)) - NOT_MODELS


def fmt(models: set[str] | list[str]) -> str:
    return ", ".join(sorted(models)) or "(none)"


# --------------------------------------------------------------------------
# 1. "Tested with" sentence
# --------------------------------------------------------------------------

TESTED_WITH_SOURCES = {
    "README.md": ROOT / "README.md",
    "tapo/Cargo.toml": ROOT / "tapo" / "Cargo.toml",
    "tapo/src/lib.rs": ROOT / "tapo" / "src" / "lib.rs",
    "tapo-py/pyproject.toml": ROOT / "tapo-py" / "pyproject.toml",
    "api_client.pyi": PYI / "api_client.pyi",
}


def tested_with_sentences(path: Path) -> list[str]:
    """Returns every "Tested with ... ." sentence in the file, joined across
    line breaks and stripped of comment markers."""
    text = read(path)
    text = re.sub(r"^\s*//!\s?", "", text, flags=re.MULTILINE)
    text = re.sub(r"^\s+", "", text, flags=re.MULTILINE)
    text = text.replace("\n", " ")
    text = re.sub(r"\s+", " ", text)
    return re.findall(r"Tested with .*?\)\.", text)


# The family words the sentence groups models into.
FAMILIES = (
    "light bulbs",
    "light strips",
    "plugs",
    "power strips",
    "hubs",
    "switches",
    "sensors",
    "cameras",
)
CHILD_FAMILIES = {"switches", "sensors"}


def check_tested_with() -> str | None:
    """Checks that every source carries the same sentence. Returns the
    reference sentence (the first one found), or None when no source has one."""
    reference: str | None = None
    reference_name = ""
    for name, path in TESTED_WITH_SOURCES.items():
        sentences = tested_with_sentences(path)
        if not sentences:
            finding("tested-with", f"{name}: no 'Tested with ...' sentence found")
            continue
        expected_count = 2 if name == "api_client.pyi" else 1
        if len(sentences) != expected_count:
            finding(
                "tested-with",
                f"{name}: expected {expected_count} 'Tested with' sentence(s), found {len(sentences)}",
            )
        for sentence in sentences:
            if reference is None:
                reference, reference_name = sentence, name
            elif sentence != reference:
                finding(
                    "tested-with",
                    f"{name} differs from {reference_name}:\n    {sentence}\n    {reference}",
                )
    return reference


def tested_with_groups(sentence: str) -> dict[str, set[str]]:
    """Returns {family: models} from a "Tested with" sentence. Each family
    name is anchored to the delimiter before it, so the sentence may list the
    families in any order; a family word not in FAMILIES is a finding."""
    groups: dict[str, set[str]] = {}
    for family, inner in re.findall(
        r"(?:^Tested with |, | and )([A-Za-z ]+?) \(([^)]*)\)", sentence
    ):
        family = family.strip().lower()
        if family not in FAMILIES:
            finding(
                "tested-with",
                f"'Tested with' groups models under '{family}', which is not a known family ({', '.join(FAMILIES)})",
            )
        groups[family] = models_in(inner)
    return groups


# --------------------------------------------------------------------------
# 2. Model -> DeviceType -> handler, from the discovery code
# --------------------------------------------------------------------------


# Strings DeviceType::from_model accepts that are not product names (some
# firmware reports them). They belong in the match and its unit test and in
# no doc; a new one goes here too.
ALIASES = {"L530 Series", "L535B"}


def device_type_mapping() -> dict[str, set[str]]:
    """Returns {DeviceType variant: models} from DeviceType::from_model."""
    text = read(RUST_API / "discovery" / "device_type.rs")
    body = text.split("pub fn from_model", 1)[1].split("#[cfg(test)]", 1)[0]
    mapping: dict[str, set[str]] = {}
    for arms, variant in re.findall(r'((?:"[^"]+"\s*\|?\s*)+)=>\s*\{?\s*DeviceType::(\w+)', body):
        models = set(re.findall(r'"([^"]+)"', arms))
        mapping.setdefault(variant, set()).update(models)
    return mapping


def device_type_docs_rs() -> dict[str, set[str]]:
    text = read(RUST_API / "discovery" / "device_type.rs")
    enum_body = text.split("pub enum DeviceType", 1)[1].split("}", 1)[0]
    docs: dict[str, set[str]] = {}
    for doc, variant in re.findall(r"///\s*(.*)\n\s*(\w+),", enum_body):
        docs[variant] = models_in(doc)
    return docs


def device_type_docs_pyi() -> dict[str, set[str]]:
    text = read(PYI / "device_type.pyi")
    docs: dict[str, set[str]] = {}
    for variant, doc in re.findall(r'(\w+): Final\[DeviceType\]\n\s*"""(.*?)"""', text):
        docs[variant] = models_in(doc)
    return docs


def discovery_handler_for_type() -> dict[str, str]:
    """Returns {DeviceType variant: handler struct} from DiscoveryResult::new."""
    text = read(RUST_API / "discovery" / "discovery_result.rs")
    body = text.split("pub(crate) async fn new", 1)[1]
    mapping: dict[str, str] = {}
    for variant, arm in re.findall(
        r"DeviceType::(\w+) => (.*?)(?=\n\s*DeviceType::|\n\s*\};)",
        body,
        flags=re.DOTALL,
    ):
        handler = re.search(r"(\w+Handler)", arm)
        if handler:
            mapping[variant] = handler.group(1)
    return mapping


def plug_child_handlers() -> dict[str, str]:
    """Returns {parent handler: child handler} from each power strip's plug() builder."""
    out: dict[str, str] = {}
    for path in RUST_API.glob("power_strip*_handler.rs"):
        text = read(path)
        parent = re.search(r"(\w+Handler)\(\w+\)", text)
        child = re.search(
            r"pub async fn plug\s*\([^)]*\)\s*->\s*Result<(\w+Handler)",
            text,
            flags=re.DOTALL,
        )
        if parent and child:
            out[parent.group(1)] = child.group(1)
    return out


def hub_child_mapping() -> dict[str, set[str]]:
    """Returns {child handler: models} from ChildDeviceHubResult's model match."""
    text = read(
        ROOT
        / "tapo"
        / "src"
        / "responses"
        / "child_device_list_hub_result"
        / "child_device_hub_result.rs"
    )
    mapping: dict[str, set[str]] = {}
    for arms, variant in re.findall(
        r'((?:"[^"]+"\s*\|?\s*)+)=>\s*serde_json::from_value\(value\)\s*\.map\(.*?ChildDeviceHubResult::(\w+)',
        text,
        flags=re.DOTALL,
    ):
        mapping.setdefault(f"{variant}Handler", set()).update(re.findall(r'"([^"]+)"', arms))
    return mapping


# --------------------------------------------------------------------------
# 3. ApiClient builders on every surface
# --------------------------------------------------------------------------

BUILDER_RE = re.compile(r"^[a-z]+\d+[a-z]*$")


def rust_builders() -> dict[str, str]:
    text = read(RUST_API / "api_client.rs")
    out: dict[str, str] = {}
    for name, ret in re.findall(
        r"pub async fn (\w+)\s*\([^)]*\)\s*->\s*Result<(\w+Handler)",
        text,
        flags=re.DOTALL,
    ):
        if BUILDER_RE.match(name):
            out[name] = ret
    return out


def py_rust_builders() -> dict[str, str]:
    text = read(PY_SRC_API / "api_client.rs")
    out: dict[str, str] = {}
    for name, ret in re.findall(
        r"pub async fn (\w+)\s*\([^)]*\)\s*->\s*PyResult<Py(\w+Handler)",
        text,
        flags=re.DOTALL,
    ):
        if BUILDER_RE.match(name):
            out[name] = ret
    return out


def pyi_builders() -> dict[str, tuple[str, set[str]]]:
    """Returns {builder: (handler, models named in its Returns: line)}."""
    text = read(PYI / "api_client.pyi")
    out: dict[str, tuple[str, set[str]]] = {}
    blocks = re.split(r"\n    async def ", text)
    for block in blocks[1:]:
        m = re.match(r"(\w+)\(self, ip_address: str\) -> (\w+Handler):", block)
        if not m:
            continue
        returns = re.search(r"Returns:\n(.*?)(?:\n\s*\n|Example:|\"\"\")", block, flags=re.DOTALL)
        models = models_in(returns.group(1)) if returns else set[str]()
        out[m.group(1)] = (m.group(2), models)
    return out


def builder_models(builder: str, all_models: set[str]) -> set[str]:
    """Models a builder serves: its own name, plus the `M` variant (p110 ->
    P110M, p304 -> P304M)."""
    upper = builder.upper()
    return {m for m in all_models if m == upper or m == upper + "M"}


# --------------------------------------------------------------------------
# 4. Handler method inventories (Rust and Python)
# --------------------------------------------------------------------------


@dataclass
class Handler:
    name: str
    rust_file: Path
    models_doc: set[str] = field(default_factory=set[str])
    # The Rust struct the handler's get_device_info returns (DeviceInfoHubResult).
    device_info: str = ""
    # The response file that defines it (device_info_result/hub), filled in by main().
    device_info_key: str = ""
    rust_methods: dict[str, bool] = field(default_factory=dict[str, bool])  # name -> debug gated
    py_methods: set[str] = field(default_factory=set[str])
    py_doc_models: set[str] = field(default_factory=set[str])


# A `#[cfg(...)]` attribute that requires the debug feature, including
# `#[cfg(all(feature = "debug", ...))]`. `#[cfg_attr(...)]` does not gate.
DEBUG_CFG_RE = re.compile(r'#\[cfg\([^\]]*feature = "debug"')
# A multi-line `#[...]` attribute, joined onto one line before scanning.
MULTILINE_ATTR_RE = re.compile(r"#\[[^\]]*\]")
PUB_FN_RE = re.compile(r"^\s*pub (?:async )?fn (\w+)\s*[<(]")
IMPL_RE = re.compile(r"^\s*impl\b.*\{\s*$")
# A `(... ) => { ... }` arm of a macro_rules! body.
MACRO_ARM_RE = re.compile(r"\n    \((.*?)\) => \{(.*?)\n    \};", re.DOTALL)
# A snake_case option on its own line (`on_off,`, `device_usage = T,`), in a
# macro arm head or in an invocation.
OPTION_RE = re.compile(r"^\s*([a-z_]+)\s*(?:=|,)", re.MULTILINE)
# An optional option in a macro arm head: `$(, child_device_list_note = ...)?`.
OPTIONAL_OPTION_RE = re.compile(r"\$\(\s*,?\s*([a-z_]+)\s*=")


def attributes_above(lines: list[str], index: int) -> str:
    """The `#[...]` attributes and `///` doc lines directly above `lines[index]`,
    in any order, joined into one string."""
    above: list[str] = []
    for line in reversed(lines[:index]):
        stripped = line.strip()
        if stripped.startswith("#[") or stripped.startswith("///"):
            above.append(stripped)
        else:
            break
    return "\n".join(above)


def debug_gated_ranges(lines: list[str]) -> list[tuple[int, int]]:
    """Line ranges of every `impl` block that is behind the debug feature."""
    ranges: list[tuple[int, int]] = []
    for i, line in enumerate(lines):
        if not IMPL_RE.match(line) or not DEBUG_CFG_RE.search(attributes_above(lines, i)):
            continue
        depth = 0
        for j in range(i, len(lines)):
            depth += lines[j].count("{") - lines[j].count("}")
            if depth <= 0:
                ranges.append((i, j))
                break
    return ranges


def methods_in(text: str) -> dict[str, bool]:
    """Returns {method: debug gated} for every `pub fn` in the text, skipping
    constructors. A method is gated when `#[cfg(feature = "debug")]` sits
    anywhere in the attribute and doc block above it, or on its `impl` block.
    Macro placeholders (`pub fn $method`) do not match."""
    text = MULTILINE_ATTR_RE.sub(lambda m: m.group(0).replace("\n", " "), text)
    lines = text.splitlines()
    gated_ranges = debug_gated_ranges(lines)
    out: dict[str, bool] = {}
    for i, line in enumerate(lines):
        m = PUB_FN_RE.match(line)
        if not m or m.group(1) == "new":
            continue
        gated = bool(DEBUG_CFG_RE.search(attributes_above(lines, i))) or any(
            start <= i <= end for start, end in gated_ranges
        )
        out[m.group(1)] = gated
    return out


@dataclass
class Macro:
    """A macro_rules! from tapo_handler.rs: which options each public arm
    takes, which internal `@arm`s it calls, and which methods each `@arm`
    generates."""

    name: str
    # Public arms: (required options, optional options, key into `arms`).
    entries: list[tuple[frozenset[str], frozenset[str], str]] = field(
        default_factory=list[tuple[frozenset[str], frozenset[str], str]]
    )
    # Every arm's inline methods and the `@arms` it calls, keyed by the
    # `@name` for internal arms and by position for public ones.
    arms: dict[str, dict[str, bool]] = field(default_factory=dict[str, dict[str, bool]])
    calls: dict[str, list[str]] = field(default_factory=dict[str, list[str]])

    def expand(self, arm: str) -> dict[str, bool]:
        """The methods an arm generates, including those of the arms it calls."""
        methods = dict(self.arms.get(arm, {}))
        for sub in self.calls.get(arm, []):
            methods.update(self.expand(sub))
        return methods

    def methods_for(self, options: set[str], where: str) -> dict[str, bool]:
        """The methods an invocation with these options generates."""
        for required, optional, arm in self.entries:
            if required <= options <= required | optional:
                return self.expand(arm)
        finding(
            "handlers",
            f"{where}: no {self.name}! arm takes the options {fmt(options)}",
        )
        return {}


def parse_macro(name: str) -> Macro:
    text = read(RUST_API / "tapo_handler.rs")
    m = re.search(r"macro_rules! " + name + r" \{\n(.*?)\n\}(?:\n|$)", text, flags=re.DOTALL)
    if m is None:
        raise SystemExit(f"tapo_handler.rs: macro_rules! {name} not found")
    macro = Macro(name)
    # The first arm may sit on the line after the brace; give it the leading
    # newline the arm pattern expects.
    for head, body in MACRO_ARM_RE.findall("\n" + m.group(1)):
        if head.startswith("@"):
            arm = head.split()[0][1:]
        else:
            arm = f"#{len(macro.entries)}"
            macro.entries.append(
                (
                    frozenset(OPTION_RE.findall(head)),
                    frozenset(OPTIONAL_OPTION_RE.findall(head)),
                    arm,
                )
            )
        macro.arms[arm] = methods_in(body)
        macro.calls[arm] = re.findall(name + r"!\(@(\w+)", body)
    return macro


TOP_MACRO = parse_macro("tapo_handler")
CHILD_MACRO = parse_macro("tapo_child_handler")
HUB_MACRO = parse_macro("hub_child_handlers")
# The child builders the hub_child_handlers! macro generates (ke100, s200, ...).
HUB_CHILD_BUILDERS = re.findall(
    r"hub_child_handlers!\(@checked (\w+),", read(RUST_API / "tapo_handler.rs")
)
# Builders for child handlers are not features in SUPPORTED_DEVICES.
NOT_FEATURES = (
    set(HUB_CHILD_BUILDERS)
    | {f"{b}_unchecked" for b in HUB_CHILD_BUILDERS}
    | {"plug", "plug_unchecked", "new"}
)


def strip_test_module(text: str) -> str:
    return text.split("#[cfg(test)]", 1)[0]


def parse_rust_handler(path: Path) -> Handler | None:
    text = strip_test_module(read(path))
    m = re.search(
        r"struct (\w+Handler)|tapo_(?:child_)?handler!\s*\{.*?(\w+Handler)\((\w+)\)",
        text,
        flags=re.DOTALL,
    )
    if not m:
        return None
    name = m.group(1) or m.group(2)
    h = Handler(name=name, rust_file=path)

    # Doc comment: the "Handler for ..." paragraph.
    doc = re.search(r"((?:\s*///[^\n]*\n)+)\s*(?:#\[derive|pub struct|" + name + r"\()", text)
    if doc:
        h.models_doc = models_in(doc.group(1))

    # Macro-generated methods, resolved through the macro's own arms.
    for macro, pattern in (
        (TOP_MACRO, r"tapo_handler!\s*\{(.*?)\n\}"),
        (CHILD_MACRO, r"tapo_child_handler!\s*\{(.*?)\n\}"),
    ):
        invocation = re.search(pattern, text, flags=re.DOTALL)
        if invocation:
            body = invocation.group(1)
            h.rust_methods.update(macro.methods_for(set(OPTION_RE.findall(body)), name))
            di = re.search(r"\w+Handler\((\w+)\)", body)
            h.device_info = di.group(1) if di else ""
    # Handlers with their own struct call the internal arms directly.
    for arm, info in re.findall(r"tapo_handler!\(@(\w+) \w+(?:\((\w+)\))?\)", text):
        h.rust_methods.update(TOP_MACRO.expand(arm))
        if info:
            h.device_info = info
    hub = re.search(r"hub_child_handlers!\((.*?)\);", text, flags=re.DOTALL)
    if hub:
        h.rust_methods.update(HUB_MACRO.methods_for(set(OPTION_RE.findall(hub.group(1))), name))
        for b in HUB_CHILD_BUILDERS:
            h.rust_methods[b] = False
            h.rust_methods[f"{b}_unchecked"] = False

    # Explicit methods, with their debug gating.
    h.rust_methods.update(methods_in(text))
    return h


def parse_pyi_handler(name: str) -> tuple[set[str], set[str]] | None:
    """Returns (methods including those of Ext bases, models in class doc)."""
    stub_name = (
        re.sub(r"(?<!^)(?=[A-Z])", "_", name)
        .lower()
        .replace("t31_x", "t31x")
        .replace("k_e100", "ke100")
    )
    path = PYI / f"{stub_name}.pyi"
    if not path.exists():
        return None
    text = read(path)
    cls = re.search(r"class " + name + r"\((.*?)\):\n\s*\"\"\"(.*?)\"\"\"", text, flags=re.DOTALL)
    if not cls:
        return None
    bases = [b.strip() for b in cls.group(1).replace("\n", " ").split(",") if b.strip()]
    methods = set(re.findall(r"\n    (?:async )?def (\w+)\(", text)) - {"__init__"}
    for base in bases:
        ext_path = PYI / (re.sub(r"(?<!^)(?=[A-Z])", "_", base).lower() + ".pyi")
        if ext_path.exists():
            methods |= set(re.findall(r"\n    (?:async )?def (\w+)\(", read(ext_path)))
    return methods, models_in(cls.group(2))


def all_handlers() -> dict[str, Handler]:
    handlers: dict[str, Handler] = {}
    for path in sorted(
        list(RUST_API.glob("*_handler.rs"))
        + list((RUST_API / "child_devices").glob("*_handler.rs"))
    ):
        if path.name == "tapo_handler.rs":
            continue
        h = parse_rust_handler(path)
        if h is None:
            finding("handlers", f"{path.relative_to(ROOT)}: could not find a handler struct")
            continue
        py = parse_pyi_handler(h.name)
        if py is None:
            finding("handlers", f"{h.name}: no matching Python stub class found")
        else:
            h.py_methods, h.py_doc_models = py
        handlers[h.name] = h
    return handlers


# --------------------------------------------------------------------------
# 5. Device info docs
# --------------------------------------------------------------------------

DEVICE_INFO_DIRS = [
    "device_info_result",
    "child_device_list_hub_result",
    "child_device_list_power_strip_result",
]


def device_info_docs() -> dict[str, tuple[set[str], set[str]]]:
    """Returns {relative file stem: (rust doc models, pyi doc models)} for every
    response file whose doc starts with "Device info of"."""
    out: dict[str, tuple[set[str], set[str]]] = {}
    for d in DEVICE_INFO_DIRS:
        for rs in sorted((ROOT / "tapo" / "src" / "responses" / d).glob("*.rs")):
            text = read(rs)
            doc = re.search(
                r"///\s*((?:Device info of|[A-Z0-9]+ and [A-Z0-9]+ power strip child plugs)[^\n]*)",
                text,
            )
            if not doc:
                continue
            pyi = PYI / "responses" / d / (rs.stem + ".pyi")
            py_models: set[str] = set()
            if pyi.exists():
                py_doc = re.search(
                    r'"""\s*((?:Device info of|[A-Z0-9]+ and [A-Z0-9]+ power strip child plugs)[^\n]*)',
                    read(pyi),
                )
                py_models = models_in(py_doc.group(1)) if py_doc else set()
            else:
                finding("device-info", f"{d}/{rs.stem}.pyi: missing Python stub")
            out[f"{d}/{rs.stem}"] = (models_in(doc.group(1)), py_models)
    return out


def device_info_type_for_struct(struct: str) -> str | None:
    """Finds the response file that defines `struct`."""
    for d in DEVICE_INFO_DIRS:
        for rs in (ROOT / "tapo" / "src" / "responses" / d).glob("*.rs"):
            if re.search(r"pub struct " + struct + r"\b", read(rs)):
                return f"{d}/{rs.stem}"
    return None


# --------------------------------------------------------------------------
# 6. SUPPORTED_DEVICES.md
# --------------------------------------------------------------------------


@dataclass
class Table:
    title: str
    columns: list[set[str]]  # models per column
    rows: dict[str, tuple[bool, list[str]]]  # feature -> (debug flagged, marker per column)


def parse_supported_devices() -> list[Table]:
    text = read(ROOT / "SUPPORTED_DEVICES.md")
    tables: list[Table] = []
    for title, body in re.findall(r"\n## ([^\n]+)\n(.*?)(?=\n## |\Z)", text, flags=re.DOTALL):
        lines = [l for l in body.splitlines() if l.startswith("|")]
        if len(lines) < 3:
            continue
        header = [c.strip() for c in lines[0].strip("|").split("|")]
        columns = [models_in(c.replace("<br/>", " ")) for c in header[1:]]
        rows: dict[str, tuple[bool, list[str]]] = {}
        for line in lines[2:]:
            cells = [c.strip() for c in line.strip("|").split("|")]
            feature = cells[0]
            debug = "(d)" in feature
            feature = re.sub(r"\s*\(d\)|\s*\\\*|\(\) API", "", feature).strip()
            rows[feature] = (debug, cells[1:])
        tables.append(Table(title, columns, rows))
    return tables


def expected_marker(rust: bool, python: bool) -> str:
    if rust and python:
        return "&#x2705;"
    if rust:
        return "&check;"
    return ""


# --------------------------------------------------------------------------
# 7. MCP crate
# --------------------------------------------------------------------------


def markdown_section(text: str, heading: str) -> str:
    """The body of a markdown section, up to the next heading of the same or a
    higher level. A `#` line inside a fenced code block (a shell comment) is
    not a heading. Empty when the heading is not there."""
    level = len(heading) - len(heading.lstrip("#"))
    lines = text.splitlines()
    if heading not in lines:
        return ""
    body: list[str] = []
    in_fence = False
    for line in lines[lines.index(heading) + 1 :]:
        if line.startswith("```"):
            in_fence = not in_fence
        elif not in_fence and re.match(r"#{1,%d} " % level, line):
            break
        body.append(line)
    return "\n".join(body)


def check_mcp(child_mapping: dict[str, set[str]]) -> None:
    get_devices = read(ROOT / "tapo-mcp" / "src" / "requests" / "get_devices.rs")
    models_rs = read(ROOT / "tapo-mcp" / "src" / "models.rs")
    readme = read(ROOT / "tapo-mcp" / "README.md")
    skill = read(ROOT / "tapo-mcp" / "openclaw-skill" / "SKILL.md")

    def variants_for(capability: str) -> set[str]:
        arm = re.search(
            r"((?:ChildDeviceHubResult::\w+\(_\)\s*\|?\s*)+)=>\s*\{?\s*get_capabilities\.push\(GetCapability::"
            + capability,
            get_devices,
        )
        if not arm:
            return set()
        models: set[str] = set()
        for variant in re.findall(r"ChildDeviceHubResult::(\w+)", arm.group(1)):
            models |= child_mapping.get(f"{variant}Handler", set())
        return models

    def enum_doc_models(enum: str, capability: str) -> set[str] | None:
        """Models in the whole `///` block above the variant of the enum."""
        body = re.search(r"pub enum " + enum + r" \{\n(.*?)\n\}", models_rs, flags=re.DOTALL)
        if body is None:
            finding("mcp", f"tapo-mcp/src/models.rs: enum {enum} not found")
            return None
        block = re.search(r"((?:[ \t]*///[^\n]*\n)+)[ \t]*" + capability + r"\b", body.group(1))
        return models_in(block.group(1)) if block else None

    def readme_table_models(capability: str) -> set[str] | None:
        """Models in the capability's row of the README's Get Capabilities table."""
        body = markdown_section(readme, "### Get Capabilities")
        row = re.search(r"^\|\s*`" + capability + r"`\s*\|([^\n]*)", body, flags=re.MULTILINE)
        return models_in(row.group(1)) if row else None

    def skill_example_models(capability: str) -> set[str] | None:
        """Models in the `# ...` comment that spells the capability out in words
        above its example in the skill's get_device_state section."""
        body = markdown_section(skill, "### get_device_state")
        words = re.findall(r"[A-Z][a-z]+", capability)
        line = re.search(
            r"^#[^\n]*?" + r"[\s/_]*".join(words) + r"[^\n]*",
            body,
            flags=re.IGNORECASE | re.MULTILINE,
        )
        return models_in(line.group(0)) if line else None

    for capability in ("TriggerLogs", "TemperatureHumidityRecords"):
        code = variants_for(capability)
        surfaces: list[tuple[str, set[str] | None]] = [
            (
                "tapo-mcp/src/models.rs GetCapability",
                enum_doc_models("GetCapability", capability),
            ),
            (
                "tapo-mcp/src/models.rs GetCapabilityRequest",
                enum_doc_models("GetCapabilityRequest", capability),
            ),
            ("tapo-mcp/README.md Get Capabilities", readme_table_models(capability)),
            ("openclaw-skill/SKILL.md get_device_state", skill_example_models(capability)),
        ]
        for label, documented in surfaces:
            if documented is None:
                finding("mcp", f"{label}: no mention of {capability}")
                continue
            # Docs name the handler family (S200), code names the models (S200B, S200D).
            documented_expanded = set[str]()
            for m in documented:
                documented_expanded |= {c for c in code if c.startswith(m)} or {m}
            if documented_expanded != code:
                finding(
                    "mcp",
                    f"{label} {capability}: documents {fmt(documented)} but the code grants it to {fmt(code)}",
                )


# --------------------------------------------------------------------------
# 8. Discovery result docs, child builder docs, examples, README symlinks
# --------------------------------------------------------------------------


def check_discovery_result_docs(models_for_handler: dict[str, set[str]]) -> None:
    rs = read(RUST_API / "discovery" / "discovery_result.rs")
    enum_body = rs.split("pub enum DiscoveryResult", 1)[1].split("\n}\n", 1)[0]
    for variant_doc, variant, body in re.findall(
        r"((?:\s*///[^\n]*\n)+)\s*(\w+) \{(.*?)\n    \},", enum_body, flags=re.DOTALL
    ):
        handler = re.search(r"handler: (\w+Handler)", body)
        if not handler:
            continue
        info_doc, _, handler_doc = body.partition("device_info:")
        handler_doc = handler_doc.split("handler:", 1)[0]
        compare_discovery_docs(
            "discovery_result.rs",
            variant,
            handler.group(1),
            models_for_handler,
            variant_doc,
            info_doc,
            handler_doc,
        )

    pyi = read(PYI / "discovery_result.pyi")
    for cls, body in re.findall(
        r"\nclass (\w+)\(DiscoveryResultExt\):(.*?)(?=\nclass |\Z)",
        pyi,
        flags=re.DOTALL,
    ):
        handler = re.search(r"handler: (\w+Handler)", body)
        if not handler:
            continue
        class_doc, _, rest = body.partition("device_info:")
        info_doc, _, handler_doc = rest.partition("handler:")
        handler_doc = handler_doc.split("__match_args__", 1)[0]
        compare_discovery_docs(
            "discovery_result.pyi",
            cls,
            handler.group(1),
            models_for_handler,
            class_doc,
            info_doc,
            handler_doc,
        )


def compare_discovery_docs(
    file: str,
    variant: str,
    handler: str,
    models_for_handler: dict[str, set[str]],
    variant_doc: str,
    info_doc: str,
    handler_doc: str,
) -> None:
    expected = models_for_handler.get(handler, set())
    for label, doc in (("variant", variant_doc), ("handler field", handler_doc)):
        if models_in(doc) != expected:
            finding(
                "discovery-docs",
                f"{file} {variant} {label} doc names {fmt(models_in(doc))}, {handler} serves {fmt(expected)}",
            )
    info_models = models_in(info_doc)
    if not expected <= info_models:
        finding(
            "discovery-docs",
            f"{file} {variant} device_info doc names {fmt(info_models)}, {handler} serves {fmt(expected)}",
        )
    elif info_models - expected:
        print(
            f"note: {file} {variant} device_info doc also names {fmt(info_models - expected)} (mirrors the device info type's doc?)"
        )


def check_child_builder_docs(models_for_handler: dict[str, set[str]]) -> None:
    """Every `Returns:\\n  XHandler: Handler for ...` block in a stub must name
    the models discovery maps to that handler."""
    for stub in sorted(PYI.glob("*_handler.pyi")):
        for handler, doc in re.findall(
            r"Returns:\n\s*(\w+Handler): (Handler for[^\n]*(?:\n(?!\s*\n)[^\n]*)*)",
            read(stub),
        ):
            expected = models_for_handler.get(handler)
            if expected is not None and models_in(doc) != expected:
                finding(
                    "handler-docs",
                    f"{stub.name} builder Returns: names {fmt(models_in(doc))}, {handler} serves {fmt(expected)}",
                )


def check_examples() -> None:
    rust = {p.stem for p in (ROOT / "tapo" / "examples").glob("tapo_*.rs")}
    py = {p.stem for p in (ROOT / "tapo-py" / "examples").glob("tapo_*.py")}
    for name in sorted(rust - py):
        finding("examples", f"tapo/examples/{name}.rs has no tapo-py/examples/{name}.py")
    for name in sorted(py - rust):
        finding("examples", f"tapo-py/examples/{name}.py has no tapo/examples/{name}.rs")
    for name in sorted(rust & py):
        rs_models = models_in(read(ROOT / "tapo" / "examples" / f"{name}.rs").split("\n", 1)[0])
        py_models = models_in(read(ROOT / "tapo-py" / "examples" / f"{name}.py").split("\n", 1)[0])
        if rs_models != py_models:
            finding(
                "examples",
                f"{name}: Rust header names {fmt(rs_models)}, Python header names {fmt(py_models)}",
            )


def check_readme_symlinks() -> None:
    for crate in ("tapo", "tapo-py"):
        path = ROOT / crate / "README.md"
        if path.is_symlink():
            continue
        if not path.exists() or path.read_text(encoding="utf-8").strip() != "../README.md":
            finding(
                "readme",
                f"{crate}/README.md is no longer a symlink to the root README; its 'Tested with' sentence would need separate verification",
            )


# --------------------------------------------------------------------------
# Main
# --------------------------------------------------------------------------


def report() -> int:
    if findings:
        print(f"\n{len(findings)} finding(s):")
        for f in findings:
            print("  " + f)
        return 1
    print("\nno mechanical mismatches found")
    return 0


def main() -> int:
    # 1. Tested with
    sentence = check_tested_with()
    if sentence is None:
        return report()
    groups = tested_with_groups(sentence)
    tested_models: set[str] = set[str]().union(*groups.values()) if groups else set()
    tested_children = set[str]().union(*(groups.get(f, set()) for f in CHILD_FAMILIES))
    tested_top = tested_models - tested_children

    # 2. Discovery mapping
    type_models = device_type_mapping()
    discovered_models = set[str]().union(*type_models.values())
    aliases = discovered_models & ALIASES
    canonical_discovered = discovered_models - aliases
    for m in sorted(tested_models & ALIASES):
        finding(
            "discovery",
            f"'Tested with' lists {m}, which is a model alias rather than a product name",
        )
    for m in sorted(canonical_discovered - tested_top):
        finding(
            "discovery",
            f"DeviceType::from_model knows {m} but the 'Tested with' sentence does not list it",
        )
    for m in sorted(tested_top - canonical_discovered):
        finding(
            "discovery",
            f"'Tested with' lists {m} but DeviceType::from_model does not map it",
        )
    if aliases:
        print(f"note: DeviceType::from_model also accepts model aliases: {fmt(aliases)}")

    rs_docs, pyi_docs = device_type_docs_rs(), device_type_docs_pyi()
    for variant, models in type_models.items():
        models_canonical = models - aliases
        if rs_docs.get(variant, set()) != models_canonical:
            finding(
                "discovery",
                f"DeviceType::{variant} doc (device_type.rs) names {fmt(rs_docs.get(variant, set()))}, from_model maps {fmt(models_canonical)}",
            )
        if pyi_docs.get(variant, set()) != models_canonical:
            finding(
                "discovery",
                f"DeviceType.{variant} doc (device_type.pyi) names {fmt(pyi_docs.get(variant, set()))}, from_model maps {fmt(models_canonical)}",
            )

    handler_for_type = discovery_handler_for_type()
    models_for_handler: dict[str, set[str]] = {}
    for variant, models in type_models.items():
        handler = handler_for_type.get(variant)
        if handler:
            models_for_handler.setdefault(handler, set()).update(models - aliases)
    child_mapping = hub_child_mapping()
    models_for_handler.update(child_mapping)
    for parent, child in plug_child_handlers().items():
        if parent in models_for_handler:
            models_for_handler[child] = set(models_for_handler[parent])
    for m in sorted(set[str]().union(*child_mapping.values()) - tested_children):
        finding(
            "discovery",
            f"ChildDeviceHubResult decodes {m} but the 'Tested with' sentence does not list it as a switch or sensor",
        )
    for m in sorted(tested_children - set[str]().union(*child_mapping.values())):
        finding(
            "discovery",
            f"'Tested with' lists {m} as a switch or sensor but ChildDeviceHubResult does not decode it",
        )

    # 3. Builders
    rust_b, py_rust_b, pyi_b = rust_builders(), py_rust_builders(), pyi_builders()
    for name in sorted(set(rust_b) - set(py_rust_b)):
        finding(
            "builders",
            f"ApiClient::{name} exists in Rust but not in tapo-py/src/api/api_client.rs",
        )
    for name in sorted(set(py_rust_b) - set(rust_b)):
        finding(
            "builders",
            f"ApiClient.{name} exists in tapo-py but not in the Rust ApiClient",
        )
    for name in sorted(set(rust_b) - set(pyi_b)):
        finding("builders", f"ApiClient.{name} is missing from api_client.pyi")
    for name in sorted(set(pyi_b) - set(rust_b)):
        finding(
            "builders",
            f"api_client.pyi declares ApiClient.{name} which the Rust ApiClient does not have",
        )
    for name, ret in rust_b.items():
        if name in py_rust_b and py_rust_b[name] != ret:
            finding(
                "builders",
                f"ApiClient::{name} returns {ret} in Rust but {py_rust_b[name]} in tapo-py",
            )
        if name in pyi_b and pyi_b[name][0] != ret:
            finding(
                "builders",
                f"ApiClient.{name} returns {ret} in Rust but api_client.pyi says {pyi_b[name][0]}",
            )
        served = builder_models(name, tested_top)
        if not served:
            finding(
                "builders",
                f"ApiClient::{name} serves no model named in the 'Tested with' sentence",
            )
        expected = models_for_handler.get(ret, set())
        if name in pyi_b and pyi_b[name][1] not in (expected, served):
            finding(
                "builders",
                f"api_client.pyi {name} Returns: names {fmt(pyi_b[name][1])}; expected the {ret} models {fmt(expected)} or just {fmt(served)}",
            )
    for m in sorted(tested_top):
        if not any(m in builder_models(b, tested_top) for b in rust_b):
            finding(
                "builders",
                f"'Tested with' lists {m} but no ApiClient builder serves it",
            )

    # 4. Handlers
    handlers = all_handlers()
    for h in handlers.values():
        handler_models = models_for_handler.get(h.name)
        if handler_models is None:
            finding(
                "handlers",
                f"{h.name}: not reachable from DeviceType::from_model or ChildDeviceHubResult; add a mapping in this script if it is a new kind of handler",
            )
            continue
        if h.models_doc != handler_models:
            finding(
                "handler-docs",
                f"{h.name} Rust doc names {fmt(h.models_doc)}, discovery maps {fmt(handler_models)} to it",
            )
        if h.py_doc_models != handler_models:
            finding(
                "handler-docs",
                f"{h.name} Python stub doc names {fmt(h.py_doc_models)}, discovery maps {fmt(handler_models)} to it",
            )
        rust_feature_names = {m for m in h.rust_methods if m not in NOT_FEATURES}
        py_feature_names = {m for m in h.py_methods if m not in NOT_FEATURES and m != "to_dict"}
        for m in sorted(rust_feature_names - py_feature_names):
            finding(
                "handler-methods",
                f"{h.name}.{m} exists in Rust but not in the Python stub (expected: Python ships every method, including debug ones)",
            )
        for m in sorted(py_feature_names - rust_feature_names):
            finding(
                "handler-methods",
                f"{h.name}.{m} is in the Python stub but not in the Rust handler",
            )
        if h.device_info:
            h.device_info_key = device_info_type_for_struct(h.device_info) or ""

    # Power strip child handlers and hub child handlers: builders must exist on both sides.
    for h in handlers.values():
        child_builders = {m for m in h.rust_methods if m in NOT_FEATURES}
        py_child_builders = {m for m in h.py_methods if m in NOT_FEATURES}
        for m in sorted(child_builders - py_child_builders):
            finding(
                "handler-methods",
                f"{h.name}.{m} child builder exists in Rust but not in the Python stub",
            )
        for m in sorted(py_child_builders - child_builders):
            finding(
                "handler-methods",
                f"{h.name}.{m} child builder is in the Python stub but not in Rust",
            )

    # 5. Device info docs
    info_docs = device_info_docs()
    for key, (rs_models, py_models) in info_docs.items():
        if rs_models != py_models:
            finding(
                "device-info",
                f"{key}: Rust doc names {fmt(rs_models)}, Python stub names {fmt(py_models)}",
            )
    for h in handlers.values():
        if h.device_info_key in info_docs:
            rs_models, _ = info_docs[h.device_info_key]
            served = models_for_handler.get(h.name, set())
            if not served <= rs_models:
                finding(
                    "device-info",
                    f"{h.device_info_key}: doc names {fmt(rs_models)} but {h.name} (which returns it) serves {fmt(served)}",
                )
            if rs_models - served:
                # Shared types (e.g. power strip info for both strip handlers) and
                # looser camera docs are legitimate; report for a human decision.
                print(
                    f"note: {h.device_info_key} doc also names {fmt(rs_models - served)}, which {h.name} does not serve (shared type or deliberate wording?)"
                )

    # 6. SUPPORTED_DEVICES.md
    tables = parse_supported_devices()
    covered: set[str] = set()
    for table in tables:
        columns_per_handler: dict[str, set[str]] = {}
        for col_models in table.columns:
            covered |= col_models
            candidates = [n for n, ms in models_for_handler.items() if col_models <= ms]
            if table.title.startswith("Power Strip Child"):
                candidates = [c for c in candidates if "Plug" in c and "PowerStrip" in c]
            else:
                candidates = [c for c in candidates if not (c.startswith("PowerStripPlug"))]
            if len(candidates) != 1:
                finding(
                    "supported-devices",
                    f"'{table.title}' column {fmt(col_models)}: cannot map to exactly one handler ({candidates})",
                )
                continue
            handler = candidates[0]
            column_handler = handlers.get(handler)
            if column_handler is None:
                continue
            columns_per_handler.setdefault(handler, set()).update(col_models)
            idx = table.columns.index(col_models)
            rust_gating = {
                m: d for m, d in column_handler.rust_methods.items() if m not in NOT_FEATURES
            }
            for feature, (debug, markers) in table.rows.items():
                marker = markers[idx] if idx < len(markers) else ""
                expected_cell = expected_marker(
                    feature in rust_gating, feature in column_handler.py_methods
                )
                if marker != expected_cell:
                    finding(
                        "supported-devices",
                        f"'{table.title}' {feature} x {fmt(col_models)}: table says '{marker or 'blank'}', code says '{expected_cell or 'blank'}'",
                    )
                if marker and debug != rust_gating.get(feature, False):
                    finding(
                        "supported-devices",
                        f"'{table.title}' {feature}: '(d)' flag is {'set' if debug else 'missing'} but the Rust method is {'' if rust_gating.get(feature) else 'not '}debug-gated",
                    )
        table_methods: set[str] = set()
        for handler, models in columns_per_handler.items():
            if models != models_for_handler[handler]:
                finding(
                    "supported-devices",
                    f"'{table.title}' columns for {handler} list {fmt(models)}, the handler serves {fmt(models_for_handler[handler])}",
                )
            table_handler = handlers[handler]
            table_methods |= set(table_handler.rust_methods) | table_handler.py_methods
            rust_feature_names = {m for m in table_handler.rust_methods if m not in NOT_FEATURES}
            for m in sorted(rust_feature_names - set(table.rows)):
                finding("supported-devices", f"'{table.title}': {handler}.{m} has no row")
        if columns_per_handler:
            for feature in table.rows:
                if feature not in table_methods:
                    finding(
                        "supported-devices",
                        f"'{table.title}': row {feature} is not a method of {', '.join(sorted(columns_per_handler))}",
                    )
    for m in sorted(tested_models - covered):
        finding(
            "supported-devices",
            f"{m} is in 'Tested with' but in no SUPPORTED_DEVICES.md column",
        )
    for m in sorted(covered - tested_models):
        finding(
            "supported-devices",
            f"{m} is in SUPPORTED_DEVICES.md but not in 'Tested with'",
        )

    # 7. MCP
    check_mcp(child_mapping)

    # 8. Remaining doc surfaces
    check_discovery_result_docs(models_for_handler)
    check_child_builder_docs(models_for_handler)
    check_examples()
    check_readme_symlinks()
    return report()


if __name__ == "__main__":
    sys.exit(main())
