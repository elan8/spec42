import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest


spec = importlib.util.spec_from_file_location("benchmark_editor", Path(__file__).with_name("benchmark-editor.py"))
benchmark = importlib.util.module_from_spec(spec)
spec.loader.exec_module(benchmark)


class EditorBenchmarkTests(unittest.TestCase):
    def test_library_identity_changes_when_bytes_change_with_unchanged_metadata(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / "library.kerml"
            source.write_bytes(b"package A;")
            stamp = source.stat()
            status = {"stdlib": {"available": True, "resolvedPath": directory}, "kparLibraries": []}
            before = benchmark.library_inputs(status)
            source.write_bytes(b"package B;")
            os.utime(source, ns=(stamp.st_atime_ns, stamp.st_mtime_ns))
            after = benchmark.library_inputs(status)
            self.assertNotEqual(before["stdlib"]["digest"], after["stdlib"]["digest"])
            self.assertEqual(set(after["stdlib"]["files"]), {"library.kerml"})

    def test_an_available_library_without_sources_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            status = {"stdlib": {"available": True, "resolvedPath": directory}, "kparLibraries": []}
            with self.assertRaisesRegex(RuntimeError, "No source/archive inputs"):
                benchmark.library_inputs(status)

    def test_missing_library_diagnostic_cannot_be_a_successful_baseline(self):
        message = json.dumps({"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics",
                              "params": {"uri": "file:///model.sysml", "diagnostics": [{"code": "missing_library_context"}]}})
        code = "import sys,time; body=" + repr(message.encode()) + "; sys.stdout.buffer.write(('Content-Length: %d\\r\\n\\r\\n' % len(body)).encode()+body); sys.stdout.buffer.flush(); time.sleep(30)"
        with tempfile.TemporaryDirectory() as directory, tempfile.TemporaryFile() as stderr:
            lsp = benchmark.Lsp([sys.executable, "-c", code], directory, stderr)
            try:
                with self.assertRaisesRegex(RuntimeError, "Missing library context"):
                    lsp.until(lambda _: False, 5)
            finally:
                lsp.close()
            self.assertIsNotNone(lsp.process.returncode)
            self.assertFalse(lsp.reader.is_alive())

    def test_stalled_server_is_bounded_and_reaped(self):
        with tempfile.TemporaryDirectory() as directory, tempfile.TemporaryFile() as stderr:
            lsp = benchmark.Lsp([sys.executable, "-c", "import time; time.sleep(30)"], directory, stderr)
            try:
                with self.assertRaises(TimeoutError):
                    lsp.until(lambda _: False, .1)
            finally:
                lsp.close()
            self.assertIsNotNone(lsp.process.returncode)
            self.assertFalse(lsp.reader.is_alive())

    @unittest.skipUnless(os.name == "nt", "Windows process resource API contract")
    def test_windows_resource_counters_are_monotonic_and_have_valid_memory_units(self):
        resources = benchmark.WindowsResources(os.getpid())
        try:
            before = resources.sample()
            after = resources.sample()
            self.assertEqual(after["status"], "measured")
            self.assertGreater(after["rss_bytes"], 0)
            self.assertGreaterEqual(after["peak_rss_bytes"], after["rss_bytes"])
            self.assertGreaterEqual(after["cpu_user_s"], before["cpu_user_s"])
            self.assertGreaterEqual(after["cpu_kernel_s"], before["cpu_kernel_s"])
        finally:
            resources.close()


if __name__ == "__main__":
    unittest.main()
