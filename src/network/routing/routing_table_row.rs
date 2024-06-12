use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::network::peer::Peer;

const ROW_SIZE: usize = 0x10;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoutingTableRow {
    peers: [Option<Peer>; ROW_SIZE]
}

impl RoutingTableRow {
    pub fn empty() -> Self {
        Self {
            peers: [None; ROW_SIZE]
        }
    }
}

impl Index<u8> for RoutingTableRow {
    type Output = Option<Peer>;

    fn index(&self, index: u8) -> &Self::Output {
        &self.peers[index as usize]
    }
}

impl IndexMut<u8> for RoutingTableRow {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.peers[index as usize]
    }
}