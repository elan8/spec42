#!/usr/bin/env python3
"""Report LSP edit ingestion and version-correlated diagnostic latency.

It records Windows process CPU/memory counters after completions, but does not measure
VS Code rendering. It never writes the edited file.
"""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import queue
import subprocess
import threading
import time


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def library_inputs(status):
    libraries = [("stdlib", status["stdlib"])] + [(item["id"], item) for item in status["kparLibraries"]]
    result = {}
    for name, library in libraries:
        if not library["available"]:
            continue
        if not library.get("resolvedPath"):
            raise RuntimeError(f"Available library {name} has no resolved input path")
        root = Path(library["resolvedPath"])
        if root.is_file():
            files = {root.name: digest(root)}
        elif root.is_dir():
            files = {str(path.relative_to(root)): digest(path) for path in sorted(root.rglob("*"))
                     if path.is_file() and path.suffix.lower() in (".sysml", ".kerml", ".kpar")}
        else:
            raise RuntimeError(f"Available library {name} input path does not exist: {root}")
        if not files:
            raise RuntimeError(f"No source/archive inputs found for available library {name}: {root}")
        result[name] = {"root": str(root), "files": files,
                        "digest": hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest()}
    return result


def distribution(values):
    ordered = sorted(values)
    return {name: ordered[max(0, math.ceil(len(ordered) * p) - 1)]
            for name, p in [("p50", .5), ("p95", .95), ("max", 1)]}


def performance_event(message):
    if message.get("method") != "window/logMessage":
        return None
    raw = message["params"]["message"]
    if raw.startswith("[SysML][perf] "):
        return json.loads(raw[len("[SysML][perf] "):])
    return None


def end_position(text):
    return {"line": text.count("\n"),
            "character": len(text.rsplit("\n", 1)[-1].encode("utf-16-le")) // 2}


class WindowsResources:
    """Read only the child process: GetProcessTimes and GetProcessMemoryInfo.

    API contracts: learn.microsoft.com/windows/win32/api/{processthreadsapi,psapi}/
    nf-processthreadsapi-getprocesstimes and nf-psapi-getprocessmemoryinfo.
    """
    def __init__(self, pid):
        import ctypes
        from ctypes import wintypes
        self.ctypes, self.wintypes = ctypes, wintypes

        class MemoryCounters(ctypes.Structure):
            _fields_ = [("cb", wintypes.DWORD), ("page_faults", wintypes.DWORD)] + [
                (name, ctypes.c_size_t) for name in ["peak_rss_bytes", "rss_bytes", "peak_paged_pool",
                "paged_pool", "peak_nonpaged_pool", "nonpaged_pool", "commit_bytes", "peak_commit_bytes"]]

        self.memory_type = MemoryCounters
        self.kernel = ctypes.WinDLL("kernel32", use_last_error=True)
        self.psapi = ctypes.WinDLL("psapi", use_last_error=True)
        self.kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
        self.kernel.OpenProcess.restype = wintypes.HANDLE
        self.kernel.CloseHandle.argtypes = [wintypes.HANDLE]
        self.kernel.CloseHandle.restype = wintypes.BOOL
        self.kernel.GetProcessTimes.argtypes = [wintypes.HANDLE] + [ctypes.POINTER(wintypes.FILETIME)] * 4
        self.kernel.GetProcessTimes.restype = wintypes.BOOL
        self.psapi.GetProcessMemoryInfo.argtypes = [wintypes.HANDLE, ctypes.POINTER(MemoryCounters), wintypes.DWORD]
        self.psapi.GetProcessMemoryInfo.restype = wintypes.BOOL
        self.handle = self.kernel.OpenProcess(0x1000, False, pid)  # PROCESS_QUERY_LIMITED_INFORMATION
        if not self.handle:
            raise ctypes.WinError(ctypes.get_last_error())

    def sample(self):
        ctypes, wintypes = self.ctypes, self.wintypes
        created, exited, kernel, user = (wintypes.FILETIME() for _ in range(4))
        if not self.kernel.GetProcessTimes(self.handle, *[ctypes.byref(item) for item in [created, exited, kernel, user]]):
            raise ctypes.WinError(ctypes.get_last_error())
        memory = self.memory_type()
        memory.cb = ctypes.sizeof(memory)
        if not self.psapi.GetProcessMemoryInfo(self.handle, ctypes.byref(memory), memory.cb):
            raise ctypes.WinError(ctypes.get_last_error())
        seconds = lambda value: ((value.dwHighDateTime << 32) | value.dwLowDateTime) / 10_000_000
        return {"status": "measured", "observed_ns": time.perf_counter_ns(),
                "cpu_user_s": seconds(user), "cpu_kernel_s": seconds(kernel),
                "rss_bytes": memory.rss_bytes, "peak_rss_bytes": memory.peak_rss_bytes,
                "commit_bytes": memory.commit_bytes, "peak_commit_bytes": memory.peak_commit_bytes}

    def close(self):
        if self.handle:
            self.kernel.CloseHandle(self.handle)
            self.handle = None


class Lsp:
    def __init__(self, command, cwd, stderr, allow_missing_libraries=False):
        self.messages = queue.Queue()
        self.events = []
        self.publications = {}
        self.diagnostics = {}
        self.allow_missing_libraries = allow_missing_libraries
        self.process = subprocess.Popen(command, cwd=cwd, stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=stderr)
        self.reader = threading.Thread(target=self.read, daemon=True)
        self.reader.start()
        self.resources = None
        self.resource_error = "Process resource counters are currently implemented for Windows only"
        if os.name == "nt":
            try:
                self.resources = WindowsResources(self.process.pid)
            except OSError as error:
                self.resource_error = repr(error)

    def resource_sample(self):
        if not self.resources:
            return {"status": "unavailable", "reason": self.resource_error}
        try:
            return self.resources.sample()
        except OSError as error:
            return {"status": "unavailable", "reason": repr(error)}

    def read(self):
        try:
            while True:
                headers = {}
                while True:
                    line = self.process.stdout.readline()
                    if not line:
                        raise EOFError("server stdout closed")
                    if line == b"\r\n":
                        break
                    key, value = line.decode("ascii").split(":", 1)
                    headers[key.lower()] = value.strip()
                length = int(headers["content-length"])
                body = bytearray()
                while len(body) < length:
                    chunk = self.process.stdout.read(length - len(body))
                    if not chunk:
                        raise EOFError("truncated server frame")
                    body.extend(chunk)
                self.messages.put((time.perf_counter_ns(), json.loads(body)))
        except Exception as error:
            self.messages.put(error)

    def send(self, method, params, request_id=None):
        message = {"jsonrpc": "2.0", "method": method, "params": params}
        if request_id is not None:
            message["id"] = request_id
        body = json.dumps(message, ensure_ascii=False).encode("utf-8")
        sent = time.perf_counter_ns()
        self.process.stdin.write(f"Content-Length: {len(body)}\r\n\r\n".encode() + body)
        self.process.stdin.flush()
        return sent

    def until(self, predicate, timeout):
        deadline = time.monotonic() + timeout
        while True:
            try:
                item = self.messages.get(timeout=max(0, deadline - time.monotonic()))
            except queue.Empty as error:
                raise TimeoutError(f"No matching LSP message within {timeout:g}s") from error
            if isinstance(item, Exception):
                raise item
            received, message = item
            self.events.append({"received_ns": received, "message": message})
            if message.get("method") == "textDocument/publishDiagnostics":
                params = message["params"]
                if isinstance(params.get("version"), int):
                    self.diagnostics[(params["uri"].lower(), params["version"])] = received
            if message.get("method") == "textDocument/publishDiagnostics" and not self.allow_missing_libraries:
                if any(diagnostic.get("code") == "missing_library_context" for diagnostic in message["params"]["diagnostics"]):
                    raise RuntimeError("Missing library context: this run cannot be used as a representative editor baseline")
            event = performance_event(message)
            if event is not None:
                if event.get("event") == "backend:publication":
                    versions = event.get("editorVersions")
                    if versions is None:
                        versions = {event["uri"]: event["version"]}  # retained handler-only baseline contract
                    for source_uri, version in versions.items():
                        self.publications[(source_uri.lower(), version)] = event
            if "error" in message:
                raise RuntimeError(message["error"])
            if predicate(message):
                return received, message

    def close(self):
        if self.process.poll() is None:
            self.process.kill()
        self.process.wait(timeout=10)
        self.reader.join(timeout=10)
        if self.resources:
            self.resources.close()
        for pipe in [self.process.stdin, self.process.stdout]:
            try:
                pipe.close()
            except BrokenPipeError:
                pass


def main():
    root = Path(__file__).resolve().parent.parent
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--server", type=Path, required=True)
    parser.add_argument("--server-arg", action="append", default=[])
    parser.add_argument("--profile", choices=["release", "debug", "profiling"], required=True)
    parser.add_argument("--workspace", type=Path, required=True)
    parser.add_argument("--document", type=Path, required=True)
    parser.add_argument("--iterations", type=int, default=20)
    parser.add_argument("--burst-size", type=int, default=10)
    parser.add_argument("--completion-scope", choices=["diagnostics", "handler"], default="diagnostics",
                        help="Require final versioned diagnostics, or explicitly retain the older handler-only baseline scope")
    parser.add_argument("--change-kind", choices=["full", "incremental"], default="full")
    parser.add_argument("--allow-missing-libraries", action="store_true",
                        help="Permit a discovery run without library admission evidence; not a representative editor baseline")
    parser.add_argument("--timeout-seconds", type=float, default=120)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if min(args.iterations, args.burst_size, args.timeout_seconds) <= 0:
        parser.error("counts and timeout must be positive")
    server, workspace, document = (path.resolve() for path in
                                    [args.server, args.workspace, args.document])
    if not server.is_file() or not document.is_file() or not document.is_relative_to(workspace):
        parser.error("server/document must exist and document must belong to workspace")
    text = document.read_text(encoding="utf-8-sig").replace("\r\n", "\n")
    uri = document.as_uri()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    command = [str(server), *args.server_arg]
    report = {"schema_version": 3, "benchmark": "lsp-edit-ingestion-and-diagnostics",
              "scope": "discovery-with-missing-libraries-permitted" if args.allow_missing_libraries else "editor-with-library-admission",
              "limitations": ["No view refresh measurement",
                              "Unobserved intermediate burst diagnostics are not assigned the final version latency",
                              "CPU times are cumulative child-process times; resource snapshots are taken after completion receipt, not at a semantic barrier",
                              "Client timestamps include transport, queueing and handler work",
                              "Workspace digest covers rg-visible source/manifests; external libraries identified by binary/config/environment"],
              "environment": {"platform": platform.platform(), "machine": platform.node(),
                              "processor": platform.processor(), "cpu_count": os.cpu_count(),
                              "profile": args.profile, "command": command,
                              "command_cwd": str(root), "runner_sha256": digest(Path(__file__)),
                              "server_sha256": digest(server),
                              "spec42_environment": {k: os.environ[k] for k in [
                                  "SPEC42_LSP_TEST_STDLIB", "SPEC42_LSP_TEST_NO_STDLIB",
                                  "SPEC42_LIBRARY_FULL_SCAN", "SPEC42_PARALLEL_STARTUP_PARSE",
                                  "SPEC42_PARALLEL_STARTUP_PARSE_MIN_FILES"] if k in os.environ}},
              "revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
              "working_tree": subprocess.check_output(["git", "status", "--short"], cwd=root, text=True),
              "configuration": vars(args).copy(), "inputs": {}, "samples": [], "status": "running"}
    files = subprocess.check_output(["rg", "--files", "--hidden", "-g", "*.sysml", "-g", "*.kerml",
                                    "-g", ".project*.json", "-g", "sysand*.json"], cwd=workspace, text=True).splitlines()
    report["inputs"] = {name: digest(workspace / name) for name in sorted(files)}
    report["document_sha256"] = digest(document)
    report["pins"] = {str(path.relative_to(root)): digest(path) for path in
                      [root / "Cargo.lock", root / "config/standard-library.json", *sorted((root / "config/libraries").glob("*.json"))]}
    lsp = None
    with args.output.with_suffix(".stderr.log").open("wb") as stderr:
        try:
            lsp = Lsp(command, root, stderr, args.allow_missing_libraries)
            lsp.send("initialize", {"processId": None, "rootUri": workspace.as_uri(), "capabilities": {},
                     "initializationOptions": {"performanceLogging": {"enabled": True}}}, 1)
            lsp.until(lambda m: m.get("id") == 1, args.timeout_seconds)
            if not args.allow_missing_libraries:
                lsp.send("sysml/libraryStatus", {}, 3)
                _, status = lsp.until(lambda m: m.get("id") == 3, args.timeout_seconds)
                report["library_status"] = status["result"]
                standard = report["library_status"]["stdlib"]
                if not standard["available"] or not standard["versionMatches"]:
                    raise RuntimeError("The host must admit the pinned standard library before editor benchmarking")
                report["library_inputs"] = library_inputs(report["library_status"])
            lsp.send("initialized", {})
            lsp.until(lambda m: m.get("method") == "spec42/semanticIndexReady", args.timeout_seconds)
            lsp.send("textDocument/didOpen", {"textDocument": {"uri": uri, "languageId": "sysml", "version": 1, "text": text}})
            if args.completion_scope == "diagnostics":
                lsp.until(lambda m: m.get("method") == "textDocument/publishDiagnostics"
                          and m["params"].get("uri", "").lower() == uri.lower()
                          and m["params"].get("version") == 1, args.timeout_seconds)
            else:
                lsp.send("textDocument/hover", {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 0}}, 2)
                lsp.until(lambda m: m.get("id") == 2, args.timeout_seconds)
            report["resources_before_edits"] = lsp.resource_sample()
            version = 1
            current_text = text
            for workload, count in [("single", 1), ("burst", args.burst_size)]:
                for iteration in range(args.iterations):
                    sent = {}
                    for _ in range(count):
                        version += 1
                        suffix = f"\n// editor benchmark revision {version}\n"
                        changed = text + suffix
                        change = {"text": changed}
                        if args.change_kind == "incremental":
                            change = {"range": {"start": end_position(text),
                                                "end": end_position(current_text)}, "text": suffix}
                        sent[version] = lsp.send("textDocument/didChange", {"textDocument": {"uri": uri, "version": version},
                                                                         "contentChanges": [change]})
                        current_text = changed
                    pending = set(sent)
                    while pending:
                        def completed(message):
                            event = performance_event(message)
                            if event is None:
                                return False
                            return event.get("event") == "backend:didChange" and event.get("version") in pending and event.get("uri", "").lower() == uri.lower()
                        received, message = lsp.until(completed, args.timeout_seconds)
                        event = performance_event(message)
                        completed_version = event["version"]
                        pending.remove(completed_version)
                        report["samples"].append({"workload": workload, "iteration": iteration, "version": completed_version,
                                                  "send_to_handler_complete_ms": (received - sent[completed_version]) / 1e6,
                                                  "handler_fields": event,
                                                  "resources": lsp.resource_sample(),
                                                  "publication_fields": lsp.publications.get((uri.lower(), completed_version))})
                    if args.completion_scope == "diagnostics":
                        latest = (uri.lower(), version)
                        if latest not in lsp.diagnostics:
                            lsp.until(lambda _: latest in lsp.diagnostics, args.timeout_seconds)
                        if latest not in lsp.publications:
                            lsp.until(lambda _: latest in lsp.publications, args.timeout_seconds)
                    for sample in report["samples"][-count:]:
                        key = (uri.lower(), sample["version"])
                        observed = lsp.diagnostics.get(key)
                        sample["send_to_diagnostics_ms"] = ((observed - sent[sample["version"]]) / 1e6
                                                            if observed is not None else None)
                        sample["analysis_observation"] = "versioned_diagnostics" if observed is not None else "coalesced_or_unobserved"
                        sample["publication_fields"] = lsp.publications.pop(key, sample["publication_fields"])
                        sample["resources_after_batch"] = lsp.resource_sample()
            report["diagnostic_summary_ms"] = {}
            for workload in ["single", "burst"]:
                values = [sample["send_to_diagnostics_ms"] for sample in report["samples"]
                          if sample["workload"] == workload and sample["send_to_diagnostics_ms"] is not None]
                report["diagnostic_summary_ms"][workload] = distribution(values) if values else None
            report["summary_ms"] = {workload: distribution([sample["send_to_handler_complete_ms"] for sample in report["samples"] if sample["workload"] == workload])
                                    for workload in ["single", "burst"]}
            report["phase_summary_ms"] = {}
            for workload in ["single", "burst"]:
                phases = {}
                for sample in report["samples"]:
                    if sample["workload"] != workload:
                        continue
                    for record in [sample["handler_fields"], sample["publication_fields"] or {}]:
                        for name, value in record.items():
                            if name.endswith("Ms") and isinstance(value, (float, int)):
                                phases.setdefault(name, []).append(value)
                report["phase_summary_ms"][workload] = {name: distribution(values) for name, values in phases.items()}
            report["status"] = "complete"
        except Exception as error:
            report["status"] = "failed"
            report["error"] = repr(error)
        finally:
            if lsp:
                try:
                    lsp.close()
                except Exception as error:
                    report["status"] = "failed"
                    report["cleanup_error"] = repr(error)
                report["events"] = lsp.events
            args.output.write_text(json.dumps(report, indent=2, default=str) + "\n", encoding="utf-8")
    print(json.dumps({key: report.get(key) for key in ["status", "summary_ms", "error"]}))
    return int(report["status"] != "complete")


if __name__ == "__main__":
    raise SystemExit(main())
