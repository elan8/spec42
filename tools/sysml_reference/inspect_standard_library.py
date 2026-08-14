#!/usr/bin/env python3
"""Inspect an explicitly supplied standard-library source or ZIP/TAR archive.

All operations are read-only. Archive members are never extracted and symbolic
links in directory inputs are ignored, so inspection stays within the supplied
library input.
"""
from __future__ import annotations

import argparse
import io
import os
from pathlib import Path
import tarfile
import zipfile

MAX_CONTENT_BYTES = 5 * 1024 * 1024


def unavailable(message: str, strict: bool) -> int:
    prefix = "ERROR" if strict else "SKIP"
    print(f"{prefix}: {message}")
    return 2 if strict else 0


def directory_members(root: Path) -> list[tuple[str, Path]]:
    members = []
    for directory, dirs, files in os.walk(root, followlinks=False):
        dirs[:] = [name for name in dirs if not (Path(directory) / name).is_symlink()]
        for name in files:
            path = Path(directory) / name
            if path.is_symlink() or not path.is_file():
                continue
            members.append((path.relative_to(root).as_posix(), path))
    return sorted(members, key=lambda member: member[0])


def archive_members(path: Path) -> tuple[str, list[str]] | None:
    if zipfile.is_zipfile(path):
        with zipfile.ZipFile(path) as archive:
            return "zip", sorted(info.filename for info in archive.infolist() if not info.is_dir())
    if tarfile.is_tarfile(path):
        with tarfile.open(path) as archive:
            return "tar", sorted(member.name for member in archive.getmembers() if member.isfile())
    return None


def archive_text(path: Path, kind: str, member: str, limit: int) -> str | None:
    if kind == "zip":
        with zipfile.ZipFile(path) as archive:
            info = archive.getinfo(member)
            if info.file_size > limit:
                return None
            return archive.read(info).decode("utf-8", errors="replace")
    with tarfile.open(path) as archive:
        info = archive.getmember(member)
        if info.size > limit:
            return None
        extracted = archive.extractfile(info)
        if extracted is None:
            return None
        with extracted:
            return io.TextIOWrapper(extracted, encoding="utf-8", errors="replace").read()


def directory_text(path: Path, limit: int) -> str | None:
    if path.stat().st_size > limit:
        return None
    return path.read_text(encoding="utf-8", errors="replace")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--library", type=Path, help="path to a source file, directory, or ZIP/TAR archive")
    parser.add_argument("--strict-input", action="store_true", help="fail instead of skipping an unavailable input")
    parser.add_argument("--max-content-bytes", type=int, default=MAX_CONTENT_BYTES, help="largest file searched for text")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("list", help="list source files or archive members")
    search = sub.add_parser("search", help="case-insensitively search member paths and UTF-8 text")
    search.add_argument("text")
    args = parser.parse_args()

    if args.max_content_bytes < 0:
        parser.error("--max-content-bytes must not be negative")
    if args.library is None:
        return unavailable("no standard-library input supplied; pass --library /path/to/library", args.strict_input)
    if not args.library.exists():
        return unavailable(f"standard-library input is unavailable: {args.library}", args.strict_input)
    if args.library.is_symlink():
        print(f"ERROR: symbolic-link library inputs are not supported: {args.library}")
        return 2

    archive = None if args.library.is_dir() else archive_members(args.library)
    if args.library.is_dir():
        members = directory_members(args.library)
        kind = "directory"
    elif archive is not None:
        kind, names = archive
        members = [(name, None) for name in names]
    elif args.library.is_file():
        members = [(args.library.name, args.library)]
        kind = "file"
    else:
        print(f"ERROR: library input is neither a source file, directory, nor ZIP/TAR archive: {args.library}")
        return 2

    if args.command == "list":
        for name, _ in members:
            print(name)
        return 0

    needle = args.text.casefold()
    matches = []
    skipped = 0
    for name, path in members:
        if kind in {"directory", "file"}:
            text = directory_text(path, args.max_content_bytes)
        else:
            text = archive_text(args.library, kind, name, args.max_content_bytes)
        if text is None:
            skipped += 1
            if needle in name.casefold():
                matches.append(name)
        elif needle in name.casefold() or needle in text.casefold():
            matches.append(name)
    for name in matches:
        print(name)
    if skipped:
        print(f"SKIP: {skipped} member(s) exceeded --max-content-bytes={args.max_content_bytes}; searched paths only")
    return 0 if matches else 1


if __name__ == "__main__":
    raise SystemExit(main())
