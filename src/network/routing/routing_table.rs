use crate::{id::Id, network::peer::Peer};

use super::routing_table_row::RoutingTableRow;

const HALF_LEAVES: usize = 0x4;
const ROWS: usize = 0x8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingTable {
    node_id: Id,
    leaves: [Option<Peer>; HALF_LEAVES*2],
    table_rows: [RoutingTableRow; ROWS],
}

impl RoutingTable {

    /// Find the next hop to reach the closest peer to the target.
    /// If the result is None, the closest peer is the current node or the network has failed.
    pub fn route(&self, target: &Id) -> Option<&Peer> {
        // long jump
        for (i,row) in self.table_rows.iter().enumerate() {
            if target.get_digit(i) != self.node_id.get_digit(i) {
                match row[target.get_digit(i)].as_ref() {
                    Some(peer) => return Some(peer),
                    None => break,
                }
            }
        }

        // short jump
        let mut closest_peer = None;
        let mut closest_distance = self.node_id.distance(target);
        for leaf in &self.leaves {
            if let Some(leaf) = leaf {
                let distance = leaf.distance(target);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_peer = Some(leaf);
                }
            }
        }

        closest_peer
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::id::Id;

    #[test]
    fn test_empty_routing_table() {
        let node_id = Id::from_key("node");
        let table = RoutingTable {
            node_id,
            leaves: [None; HALF_LEAVES*2],
            table_rows: [RoutingTableRow::empty(); ROWS],
        };

        let target = Id::from_key("target");
        assert_eq!(table.route(&target), None);
    }

    #[test]
    fn test_short_jump() {
        let node_id = Id::from_str("2000-0000-0000-0000").unwrap();
        let mut leaves = [None; HALF_LEAVES*2];
        leaves[HALF_LEAVES] = Some(Peer::new(Id::from_str("1000-0000-0000-0000").unwrap()));
        let mut table_rows = [RoutingTableRow::empty(); ROWS];
        table_rows[0][0] = Some(Peer::new(Id::from_str("0000-0000-0000-0000").unwrap()));
        let table = RoutingTable {
            node_id,
            leaves,
            table_rows,
        };

        let target = Id::from_str("1200-1000-0000-0000").unwrap();
        assert_eq!(table.route(&target), Some(&Peer::new(Id::from_str("1000-0000-0000-0000").unwrap())));
    }

    #[test]
    fn test_long_jump() {
        let node_id = Id::from_str("2000-0000-0000-0000").unwrap();
        let mut table_rows = [RoutingTableRow::empty(); ROWS];
        table_rows[0][0] = Some(Peer::new(Id::from_str("0000-0000-0000-0000").unwrap()));
        table_rows[0][1] = Some(Peer::new(Id::from_str("1000-0000-0000-0000").unwrap()));
        table_rows[1][1] = Some(Peer::new(Id::from_str("2100-0000-0000-0000").unwrap()));
        table_rows[1][2] = Some(Peer::new(Id::from_str("2200-0000-0000-0000").unwrap()));
        table_rows[2][2] = Some(Peer::new(Id::from_str("2020-0000-0000-0000").unwrap()));
        let table = RoutingTable {
            node_id,
            leaves: [None; HALF_LEAVES*2],
            table_rows,
        };

        let target = Id::from_str("1200-1000-0000-0000").unwrap();
        assert_eq!(table.route(&target), Some(&Peer::new(Id::from_str("1000-0000-0000-0000").unwrap())));

        let target = Id::from_str("2100-1000-0000-0000").unwrap();
        assert_eq!(table.route(&target), Some(&Peer::new(Id::from_str("2100-0000-0000-0000").unwrap())));

        let target = Id::from_str("2200-1000-0000-0000").unwrap();
        assert_eq!(table.route(&target), Some(&Peer::new(Id::from_str("2200-0000-0000-0000").unwrap())));

        let target = Id::from_str("2020-1000-0000-0000").unwrap();
        assert_eq!(table.route(&target), Some(&Peer::new(Id::from_str("2020-0000-0000-0000").unwrap())));

        let target = Id::from_str("2000-0000-0000-0000").unwrap();
        assert_eq!(table.route(&target), None);
    }
}