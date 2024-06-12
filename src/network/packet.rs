use anyhow::Ok;
use serde::{Deserialize, Serialize};

use crate::id::Id;

use super::{peer::Peer, routing::routing_table_row::RoutingTableRow};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Packet {
    /// Send this to a peer to ask them to join the network
    JoinRequest,
    
    /// Send this to a peer to let them know that they are the next hop in a join request
    /// The peer should forward the correct RoutingTableRow to the new peer and continue the join process
    PeerIsJoining {
        applicant: Peer,
        hop_count: u8,
    },

    /// Send this to a peer that required to join the network and you received a PeerIsJoining packet
    JoinResponse {
        routing_table_row: RoutingTableRow,
        hop_count: u8,
    },

    Ping {
        nonce: u64,
    },

    Pong {
        nonce: u64,
    },

    /// Send this to send a generic message to a peer, 
    /// keep in mind that the closest peer to the key will receive the message,
    /// not necessarily the peer with the exact key
    Message {
        key: Id, 
        payload: Vec<u8>
    },
}

impl Packet {
    pub fn serialize(&self) -> anyhow::Result<Vec<u8>> {
        let payload = bincode::serialize(self)?;
        Ok(payload)
    }

    pub fn deserialize(data: &[u8]) -> anyhow::Result<Self> {
        let packet = bincode::deserialize(data)?;
        Ok(packet)
    }
}


