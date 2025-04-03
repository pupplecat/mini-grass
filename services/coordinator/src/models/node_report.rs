use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeReport {
    node_id: u64,
    bandwidth: u64,
}
