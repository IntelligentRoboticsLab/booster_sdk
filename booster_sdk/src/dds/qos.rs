//! QoS helpers aligned with the Booster DDS reference.

use rustdds::{
    Duration, QosPolicies, QosPolicyBuilder,
    policy::{History, Reliability},
};

pub fn qos_best_effort_keep_last(depth: i32) -> QosPolicies {
    QosPolicyBuilder::new()
        .reliability(Reliability::BestEffort)
        .history(History::KeepLast { depth })
        .build()
}

pub fn qos_reliable_keep_last(depth: i32) -> QosPolicies {
    QosPolicyBuilder::new()
        .reliability(Reliability::Reliable {
            max_blocking_time: Duration::from_millis(100),
        })
        .history(History::KeepLast { depth })
        .build()
}

pub fn qos_reliable_keep_all() -> QosPolicies {
    QosPolicyBuilder::new()
        .reliability(Reliability::Reliable {
            max_blocking_time: Duration::from_millis(100),
        })
        .history(History::KeepAll)
        .build()
}
