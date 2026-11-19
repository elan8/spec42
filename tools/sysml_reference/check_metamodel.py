"""Check the metamodel facts used by the item and redefinition regression cases."""

import argparse
import hashlib
from pathlib import Path
import re
from urllib.request import urlopen
import xml.etree.ElementTree as ET


# The OMG 20250201 XMI digests are pinned once, in the constraint manifest; they
# match Open-MBEE/sysml-toolkit@7d740b37 tools/fetch-spec-refs.pins.
MANIFEST = Path(__file__).resolve().parents[1] / "constraint_manifest" / "src" / "lib.rs"


def manifest_pins():
    """`{"KerML": sha256, "SysML": sha256}` from the constraint manifest's pinned specifications."""
    source = MANIFEST.read_text(encoding="utf-8")
    pins = {}
    for language in ("KerML", "SysML"):
        found = re.search(
            rf'name:\s*"{language}",.*?expected_sha256:\s*"([0-9a-f]{{64}})"',
            source,
            re.DOTALL,
        )
        if found is None:
            raise ValueError(f"no pinned {language} XMI digest in {MANIFEST.name}")
        pins[language] = found[1]
    return pins
XMI = "{http://www.omg.org/spec/XMI/20161101}"


def read_reference(directory, language, fetch, pins):
    path = directory / f"{language}.xmi"
    if fetch:
        url = f"https://www.omg.org/spec/{language}/20250201/{language}.xmi"
        with urlopen(url, timeout=30) as response:
            data = response.read()
    else:
        data = path.read_bytes()
    data = data.replace(b"\r\n", b"\n")
    if hashlib.sha256(data).hexdigest() != pins[language]:
        raise ValueError(f"SHA-256 mismatch for {path.name}")
    if fetch:
        path.write_bytes(data)
    return {element.get(XMI + "id"): element for element in ET.fromstring(data).iter()}


def check_metamodel(kerml, sysml):
    for identifier in ("Systems-Items-ItemDefinition", "Systems-Attributes-AttributeUsage"):
        if sysml[identifier].get(XMI + "type") != "uml:Class":
            raise ValueError(f"{identifier} must be a metaclass")
    abstract = kerml["Core-Types-Type"].find("./ownedAttribute[@name='isAbstract']/type")
    if abstract is None or not abstract.get("href", "").endswith("#Boolean"):
        raise ValueError("Type::isAbstract must be an owned Boolean property")
    target = kerml["Core-Features-Redefinition"].find("./ownedAttribute[@name='redefinedFeature']")
    target_type = target.find("type") if target is not None else None
    if target_type is None or target_type.get(XMI + "idref") != "Core-Features-Feature":
        raise ValueError("Redefinition::redefinedFeature must refer to Feature")
    if any(target.find(f"{bound}Value[@value='1']") is None for bound in ("lower", "upper")):
        raise ValueError("Redefinition::redefinedFeature must have multiplicity 1..1")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--fetch", action="store_true", help="download approved, pinned OMG files")
    args = parser.parse_args()
    try:
        if args.fetch:
            args.directory.mkdir(parents=True, exist_ok=True)
        pins = manifest_pins()
        refs = {name: read_reference(args.directory, name, args.fetch, pins) for name in pins}
        check_metamodel(refs["KerML"], refs["SysML"])
    except (OSError, ValueError, ET.ParseError, KeyError) as error:
        parser.exit(1, f"Metamodel reference check: {error}\n")
    print("Pinned OMG metamodel facts verified")


if __name__ == "__main__":
    main()
