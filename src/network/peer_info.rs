use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerInfo {
    pub physical_distance_index: Option<u64>, 
}

impl Default for PeerInfo {
    fn default() -> Self {
        Self {
            physical_distance_index: None,
        }
    }
}