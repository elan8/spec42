#!/usr/bin/env python3
"""Report LSP edit-to-handler latency, including transport and queued obsolete work.

This is the first #192 baseline: it does not measure VS Code rendering, CPU/memory,
or claim version-correlated diagnostic latency. It never writes the edited file.
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


def distribution(values):
    ordered = sorted(values)
    return {name: ordered[max(0, math.ceil(len(ordered) * p) - 1)]
            for name, p in [("p50", .5), ("p95", .95), ("max", 1)]}


def end_position(text):
    return {"line": text.count("\n"),
            "character": len(text.rsplit("\n", 1)[-1].encode("utf-16-le")) // 2}


class Lsp:
    def __init__(self, command, cwd, stderr):
        self.messages = queue.Queue()
        self.events = []
        self.process = subprocess.Popen(command, cwd=cwd, stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=stderr)
        self.reader = threading.Thread(target=self.read, daemon=True)
        self.reader.start()

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
            if "error" in message:
                raise RuntimeError(message["error"])
            if predicate(message):
                return received, message

    def close(self):
        if self.process.poll() is None:
            self.process.kill()
        self.process.wait(timeout=10)
        self.reader.join(timeout=10)


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
    parser.add_argument("--change-kind", choices=["full", "incremental"], default="full")
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
    report = {"schema_version": 1, "benchmark": "lsp-edit-to-handler",
              "limitations": ["Not version-correlated diagnostic latency", "No view refresh or CPU/memory measurement",
                              "Client timestamps include transport, queueing and handler work",
                              "Workspace digest covers rg-visible source/manifests; external libraries identified by binary/config/environment"],
              "environment": {"platform": platform.platform(), "machine": platform.node(),
                              "processor": platform.processor(), "cpu_count": os.cpu_count(),
                              "profile": args.profile, "command": command,
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
            lsp = Lsp(command, root, stderr)
            lsp.send("initialize", {"processId": None, "rootUri": workspace.as_uri(), "capabilities": {},
                     "initializationOptions": {"performanceLogging": {"enabled": True}}}, 1)
            lsp.until(lambda m: m.get("id") == 1, args.timeout_seconds)
            lsp.send("initialized", {})
            lsp.until(lambda m: m.get("method") == "spec42/semanticIndexReady", args.timeout_seconds)
            lsp.send("textDocument/didOpen", {"textDocument": {"uri": uri, "languageId": "sysml", "version": 1, "text": text}})
            lsp.send("textDocument/hover", {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 0}}, 2)
            lsp.until(lambda m: m.get("id") == 2, args.timeout_seconds)
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
                            raw = message.get("params", {}).get("message", "")
                            if not raw.startswith("[SysML][perf] "):
                                return False
                            event = json.loads(raw[len("[SysML][perf] "):])
                            return event.get("event") == "backend:didChange" and event.get("version") in pending and event.get("uri", "").lower() == uri.lower()
                        received, message = lsp.until(completed, args.timeout_seconds)
                        event = json.loads(message["params"]["message"][len("[SysML][perf] "):])
                        completed_version = event["version"]
                        pending.remove(completed_version)
                        report["samples"].append({"workload": workload, "iteration": iteration, "version": completed_version,
                                                  "send_to_handler_complete_ms": (received - sent[completed_version]) / 1e6,
                                                  "handler_fields": event})
            report["summary_ms"] = {workload: distribution([sample["send_to_handler_complete_ms"] for sample in report["samples"] if sample["workload"] == workload])
                                    for workload in ["single", "burst"]}
            report["status"] = "complete"
        except Exception as error:
            report["status"] = "failed"
            report["error"] = repr(error)
        finally:
            if lsp:
                lsp.close()
                report["events"] = lsp.events
            args.output.write_text(json.dumps(report, indent=2, default=str) + "\n", encoding="utf-8")
    print(json.dumps({key: report.get(key) for key in ["status", "summary_ms", "error"]}))
    return int(report["status"] != "complete")


if __name__ == "__main__":
    raise SystemExit(main())
