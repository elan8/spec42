#!/usr/bin/env python3
"""Repeat the real snapshot and LSP integration workloads concurrently, with bounded failures."""
import argparse
import concurrent.futures
import json
import multiprocessing
import os
from pathlib import Path
import signal
import subprocess
import sys
import time


def stop_process_tree(process):
    if os.name == "nt":
        subprocess.run(["taskkill", "/T", "/F", "/PID", str(process.pid)],
                       capture_output=True, timeout=15, check=False)
    else:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
    process.wait(timeout=15)


def stalled_state(pid):
    lines = [f"stalled workload pid={pid}"]
    tasks = Path(f"/proc/{pid}/task")
    if tasks.is_dir():
        for task in sorted(tasks.iterdir()):
            try:
                lines.append(f"thread {task.name}: {task.joinpath('comm').read_text().strip()} "
                             f"wait={task.joinpath('wchan').read_text().strip()}")
            except OSError as error:
                lines.append(f"thread {task.name}: {error}")
    return "\n".join(lines)


def run_workload(name, command, timeout, log, cwd):
    started = time.monotonic()
    timed_out = False
    with log.open("wb") as output:
        process = subprocess.Popen(command, cwd=cwd, stdout=output, stderr=subprocess.STDOUT,
                                   start_new_session=os.name != "nt")
        try:
            code = process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            timed_out = True
            output.write(("\n" + stalled_state(process.pid) + "\n").encode())
            output.flush()
            stop_process_tree(process)
            code = 124
    return {"name": name, "command": [str(arg) for arg in command], "exit_code": code,
            "timed_out": timed_out, "seconds": round(time.monotonic() - started, 3),
            "log": str(log)}


def cpu_load(stop):
    value = 1
    while not stop.is_set():
        for _ in range(10000):
            value = (value * 1664525 + 1013904223) & 0xffffffff


def main():
    root = Path(__file__).resolve().parent.parent
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--snapshot-bin", type=Path,
                        default=root / "target/release" / ("spec42-snapshot.exe" if os.name == "nt" else "spec42-snapshot"))
    lsp = parser.add_mutually_exclusive_group(required=True)
    lsp.add_argument("--lsp-test-bin", type=Path)
    lsp.add_argument("--lsp-artifacts", type=Path, help="Cargo --message-format=json output from the --no-run LSP integration build")
    parser.add_argument("--repetitions", type=int, default=25)
    parser.add_argument("--jobs", type=int, default=2)
    parser.add_argument("--test-threads", type=int, default=4)
    parser.add_argument("--cpu-workers", type=int, default=0)
    parser.add_argument("--timeout-seconds", type=int, default=900)
    parser.add_argument("--output", type=Path, default=root / "target/publication-stress")
    args = parser.parse_args()
    if min(args.repetitions, args.jobs, args.test_threads) < 1 or args.timeout_seconds < 60 or args.cpu_workers < 0:
        parser.error("counts must be positive, CPU workers nonnegative, and timeout at least 60s")
    if args.lsp_artifacts:
        executables = set()
        for line in args.lsp_artifacts.read_text(encoding="utf-8-sig").splitlines():
            item = json.loads(line)
            if item.get("reason") == "compiler-artifact" and item.get("target", {}).get("name") == "lsp_integration" and item.get("executable"):
                executables.add(item["executable"])
        if len(executables) != 1:
            parser.error("Cargo artifact output must identify exactly one lsp_integration executable")
        args.lsp_test_bin = Path(executables.pop())
    args.snapshot_bin = args.snapshot_bin.resolve()
    args.lsp_test_bin = args.lsp_test_bin.resolve()
    for binary in [args.snapshot_bin, args.lsp_test_bin]:
        if not binary.is_file():
            parser.error(f"build the workload first: {binary}")
    args.output.mkdir(parents=True, exist_ok=True)
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
    summary = {"revision": revision, "platform": sys.platform, "cpu_count": os.cpu_count(),
               "repetitions": args.repetitions, "jobs": args.jobs, "test_threads": args.test_threads,
               "cpu_workers": args.cpu_workers, "timeout_seconds": args.timeout_seconds, "results": []}
    context = multiprocessing.get_context("spawn")
    stop = context.Event()
    load = [context.Process(target=cpu_load, args=(stop,)) for _ in range(args.cpu_workers)]
    for worker in load:
        worker.start()
    try:
        with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as executor:
            pending = []
            for iteration in range(1, args.repetitions + 1):
                workloads = [
                    (f"snapshot-{iteration:02}", [str(args.snapshot_bin), "check", "--timeout-seconds", str(args.timeout_seconds - 30)]),
                    (f"lsp-{iteration:02}", [str(args.lsp_test_bin), "--nocapture", f"--test-threads={args.test_threads}"]),
                ]
                for name, command in workloads:
                    pending.append(executor.submit(run_workload, name, command, args.timeout_seconds,
                                                   args.output / f"{name}.log", root))
            for future in concurrent.futures.as_completed(pending):
                result = future.result()
                summary["results"].append(result)
                summary["results"].sort(key=lambda result: result["name"])
                (args.output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
                print(f"{result['name']}: exit={result['exit_code']} seconds={result['seconds']} log={result['log']}", flush=True)
    finally:
        stop.set()
        for worker in load:
            worker.join(timeout=5)
            if worker.is_alive():
                worker.terminate()
                worker.join(timeout=5)
    return int(any(result["exit_code"] for result in summary["results"]))


if __name__ == "__main__":
    raise SystemExit(main())
