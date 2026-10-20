import importlib.util
from pathlib import Path
import sys
import tempfile
import time
import unittest

spec = importlib.util.spec_from_file_location("stress_publication", Path(__file__).with_name("stress-publication.py"))
stress = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stress)


class StressRunnerTests(unittest.TestCase):
    def test_stuck_workload_is_terminated_with_a_diagnostic_log(self):
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "timeout.log"
            started = time.monotonic()
            result = stress.run_workload("blocked", [sys.executable, "-c", "import time; time.sleep(60)"], .1, log, directory)
            self.assertTrue(result["timed_out"])
            self.assertEqual(result["exit_code"], 124)
            self.assertLess(time.monotonic() - started, 20)
            self.assertIn("stalled workload pid=", log.read_text())

    def test_real_failure_preserves_exit_code_and_stderr(self):
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "failure.log"
            result = stress.run_workload("failed", [sys.executable, "-c", "import sys; print('evidence', file=sys.stderr); sys.exit(7)"], 5, log, directory)
            self.assertFalse(result["timed_out"])
            self.assertEqual(result["exit_code"], 7)
            self.assertEqual(log.read_text().strip(), "evidence")


if __name__ == "__main__":
    unittest.main()
