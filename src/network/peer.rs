use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::id::Id;

use super::peer_info::PeerInfo;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Peer {
    id: Id,
    addr: SocketAddr,
    info: PeerInfo,
}

impl Peer {
    pub fn new(addr: SocketAddr) -> Self {
        let id = Id::from_key(addr);
        Self { id, addr, info: PeerInfo::default() }
    }

    pub fn raw(id: Id, addr: SocketAddr) -> Self {
        Self { id, addr, info: PeerInfo::default() }
    }

    pub fn distance(&self, other: &Id) -> Id {
        self.id.distance(other)
    }
}