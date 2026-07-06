use std::sync::Arc;
use std::time::Duration;

use workspace_session::{RelinkToken, SessionActor, TracksRelink};

#[derive(Clone, Default)]
struct CounterState {
    value: u64,
}

impl TracksRelink for CounterState {
    fn is_token_current(&self, _token: &RelinkToken) -> bool {
        true
    }
}

/// Proves the actual "readers never block on writers" property with real concurrent tokio
/// tasks, not just a type-level check: one writer drives a burst of sequential mutations
/// while many readers hammer `current()`, and every read must both stay monotonic (single
/// writer) and complete promptly — if a future regression reintroduces a lock that readers
/// must wait on, this test times out instead of silently passing.
#[tokio::test]
async fn concurrent_reads_never_block_on_in_flight_mutations() {
    let (actor, snapshot) = SessionActor::spawn(CounterState::default());
    let actor = Arc::new(actor);

    let writer = {
        let actor = actor.clone();
        tokio::spawn(async move {
            for i in 1..=500u64 {
                actor.mutate(move |s| s.value = i).await.unwrap();
            }
        })
    };

    let mut readers = Vec::new();
    for _ in 0..16 {
        let snapshot = snapshot.clone();
        readers.push(tokio::spawn(async move {
            let mut last = 0u64;
            for _ in 0..2000 {
                let v = snapshot.current().value;
                assert!(v >= last, "reads must be monotonic under a single writer");
                last = v;
            }
        }));
    }

    tokio::time::timeout(Duration::from_millis(2000), writer)
        .await
        .expect("writer must not be starved by readers")
        .unwrap();
    for r in readers {
        tokio::time::timeout(Duration::from_millis(200), r)
            .await
            .expect("reads must never block on in-flight mutations")
            .unwrap();
    }
}
