//! Bound the complete snapshot operation and report which fixture workers are still active.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Stage {
    Discovery,
    LibraryAdmission,
    Evaluation,
    Publication,
}

struct Progress {
    stage: Stage,
    completed: usize,
    total: usize,
    active: BTreeMap<PathBuf, Instant>,
}

impl Progress {
    fn report(&self, budget: Duration) -> String {
        let mut report = format!(
            "snapshot operation timed out after {}s: pid={} stage={:?}, completed={}/{}\n",
            budget.as_secs(),
            std::process::id(),
            self.stage,
            self.completed,
            self.total,
        );
        for (path, started) in &self.active {
            report.push_str(&format!(
                "  active {} ({}ms)\n",
                path.display(),
                started.elapsed().as_millis()
            ));
        }
        report
    }
}

/// Disconnecting the sender cancels the watchdog without waiting for its entire budget.
pub(crate) struct Watchdog {
    progress: Arc<Mutex<Progress>>,
    _finished: mpsc::Sender<()>,
}

impl Watchdog {
    pub(crate) fn start(budget: Duration, total: usize) -> Self {
        Self::with_timeout_action(budget, total, |report| {
            eprintln!("{report}");
            // A panic on this thread would leave deadlocked workers alive indefinitely.
            std::process::exit(124);
        })
    }

    fn with_timeout_action(
        budget: Duration,
        total: usize,
        on_timeout: impl FnOnce(String) + Send + 'static,
    ) -> Self {
        let progress = Arc::new(Mutex::new(Progress {
            stage: Stage::Discovery,
            completed: 0,
            total,
            active: BTreeMap::new(),
        }));
        let observed = Arc::clone(&progress);
        let (finished, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            if receiver.recv_timeout(budget) == Err(mpsc::RecvTimeoutError::Timeout) {
                let progress = observed
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                let mut report = progress.report(budget);
                drop(progress);
                append_thread_states(&mut report);
                on_timeout(report);
            }
        });
        Self {
            progress,
            _finished: finished,
        }
    }

    pub(crate) fn total(&self, total: usize) {
        self.progress
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .total = total;
    }

    pub(crate) fn stage(&self, stage: Stage) {
        self.progress
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .stage = stage;
    }

    pub(crate) fn fixture(&self, path: &Path) -> ActiveFixture {
        self.progress
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .active
            .insert(path.to_path_buf(), Instant::now());
        ActiveFixture {
            progress: Arc::clone(&self.progress),
            path: path.to_path_buf(),
        }
    }
}

pub(crate) struct ActiveFixture {
    progress: Arc<Mutex<Progress>>,
    path: PathBuf,
}

impl Drop for ActiveFixture {
    fn drop(&mut self) {
        let mut progress = self
            .progress
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        progress.active.remove(&self.path);
        progress.completed += 1;
    }
}

fn append_thread_states(report: &mut String) {
    if let Ok(tasks) = std::fs::read_dir("/proc/self/task") {
        let mut threads: Vec<_> = tasks
            .filter_map(Result::ok)
            .map(|task| task.path())
            .collect();
        threads.sort();
        for thread in threads {
            let name = std::fs::read_to_string(thread.join("comm")).unwrap_or_default();
            let wait = std::fs::read_to_string(thread.join("wchan")).unwrap_or_default();
            report.push_str(&format!(
                "  thread {}: {} wait={}\n",
                thread.display(),
                name.trim(),
                wait.trim()
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_reports_a_bounded_operation_failure() {
        let (sender, receiver) = mpsc::channel();
        let _watchdog =
            Watchdog::with_timeout_action(Duration::from_millis(20), 3, move |report| {
                sender.send(report).unwrap()
            });
        let report = receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("bounded timeout");
        assert!(report.contains("snapshot operation timed out"), "{report}");
    }

    #[test]
    fn report_names_the_active_fixture_and_owning_stage() {
        let watchdog = Watchdog::with_timeout_action(Duration::from_secs(30), 3, |_| {
            panic!("unexpected timeout")
        });
        watchdog.stage(Stage::Evaluation);
        let active = watchdog.fixture(Path::new("stuck.md"));
        let report = watchdog
            .progress
            .lock()
            .unwrap()
            .report(Duration::from_secs(30));
        assert!(
            report.contains("stage=Evaluation, completed=0/3"),
            "{report}"
        );
        assert!(report.contains("active stuck.md"), "{report}");
        drop(active);
        assert_eq!(watchdog.progress.lock().unwrap().completed, 1);
    }

    #[test]
    fn completing_the_operation_cancels_the_timeout() {
        let (sender, receiver) = mpsc::channel();
        let watchdog = Watchdog::with_timeout_action(Duration::from_secs(30), 1, move |report| {
            sender.send(report).unwrap()
        });
        drop(watchdog);
        assert_eq!(
            receiver.recv_timeout(Duration::from_secs(5)),
            Err(mpsc::RecvTimeoutError::Disconnected)
        );
    }
}
