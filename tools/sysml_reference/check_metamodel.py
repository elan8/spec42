"""Check the metamodel facts used by the item and redefinition regression cases."""

import argparse
import hashlib
from pathlib import Path
from urllib.request import urlopen
import xml.etree.ElementTree as ET


# OMG 20250201; normalized SHA-256 from Open-MBEE/sysml-toolkit@7d740b37,
# tools/fetch-spec-refs.pins. These are separate from the official corpus pin.
PINS = {
    "KerML": "45b18775afe2b2fcdc70e24f37c6d2f344defcc3f38a02075a193354e2d7b466",
    "SysML": "caa65d54f56798bf7582d173f7567e1eea37a49c45984f8bd7df145011cf8c6f",
}
XMI = "{http://www.omg.org/spec/XMI/20161101}"


def read_reference(directory, language, fetch):
    path = directory / f"{language}.xmi"
    if fetch:
        url = f"https://www.omg.org/spec/{language}/20250201/{language}.xmi"
        with urlopen(url, timeout=30) as response:
            data = response.read()
    else:
        data = path.read_bytes()
    data = data.replace(b"\r\n", b"\n")
    if hashlib.sha256(data).hexdigest() != PINS[language]:
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
        refs = {name: read_reference(args.directory, name, args.fetch) for name in PINS}
        check_metamodel(refs["KerML"], refs["SysML"])
    except (OSError, ValueError, ET.ParseError, KeyError) as error:
        parser.exit(1, f"Metamodel reference check: {error}\n")
    print("Pinned OMG metamodel facts verified")


if __name__ == "__main__":
    main()
