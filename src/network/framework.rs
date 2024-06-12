use std::{net::SocketAddr, sync::{Arc, RwLock}, thread};

use anyhow::{bail, Ok};

use super::{config::Config, packet::Packet, peer::Peer, routing::routing_table::RoutingTable, Network};

#[derive(Debug)]
pub struct Framework 
{
    network: Arc<RwLock<Network>>,
    running: Arc<RwLock<bool>>,
    threads: Vec<thread::JoinHandle<()>>,
}

impl Framework {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self {
            network: Arc::new(RwLock::new(Network::new(config)?)),
            running: Arc::new(RwLock::new(false)),
            threads: Vec::new(),
        })
    }

    fn handle_packet(network: Arc<RwLock<Network>>, packet: Packet, addr: SocketAddr) -> anyhow::Result<()> {
        match packet {
            Packet::JoinRequest => 
            {
                let peer = Peer::new(addr);
                let packet = Packet::PeerIsJoining { applicant: peer, hop_count: 0 };
                let network = network.read().unwrap();
                let next_hop = network.route(&peer.id())?;
                if let Some(next_hop) = next_hop {
                    network.send(packet, next_hop.addr())?;
                }
                else
                {
                    let leaves = network.get_routing_table().unwrap().leaves_to_vec();
                    let routing_table_row = network.get_routing_table().unwrap().row(0);
                    let packet = Packet::JoinResponse { applicant_id: peer.id(), routing_table_row, leaves, hop_count: 0 };
                    network.send(packet, addr)?;
                }
            },
            Packet::PeerIsJoining { applicant, hop_count } => {
                let network = network.read().unwrap();
                let next_hop = network.route(&applicant.id())?;
                if let Some(next_hop) = next_hop {
                    let next_hop_count = match hop_count.checked_add(1) {
                        Some(count) => count,
                        None => bail!("Hop count overflow"),
                    };
                    let packet = Packet::PeerIsJoining { applicant, hop_count: next_hop_count};
                    network.send(packet, next_hop.addr())?;
                    let routing_table_row = network.get_routing_table().unwrap().row(hop_count as usize);
                    let leaves = network.get_routing_table().unwrap().leaves_to_vec();
                    let packet = Packet::JoinResponse { applicant_id: applicant.id(), routing_table_row, leaves, hop_count };
                    network.send(packet, addr)?;
                }
                else
                {
                    let leaves = network.get_routing_table().unwrap().leaves_to_vec();
                    let routing_table_row = network.get_routing_table().unwrap().row(hop_count as usize);
                    let packet = Packet::JoinResponse { applicant_id: applicant.id(), routing_table_row, leaves, hop_count };
                    network.send(packet, addr)?;
                }
            },
            Packet::JoinResponse { applicant_id, routing_table_row, leaves, hop_count } => {
                let mut network = network.write().unwrap();
                if network.get_routing_table().is_none() {
                    let mut routing_table = RoutingTable::empty(applicant_id);
                    routing_table.set_row(routing_table_row, hop_count as usize);
                    routing_table.add_leaves(leaves);
                    network.set_routing_table(routing_table);
                }
            },
            Packet::Ping { nonce } => {
                let packet = Packet::Pong { nonce };
                let network = network.read().unwrap();
                network.send(packet, addr)?;
            },
            Packet::Pong { nonce } => todo!(),
            Packet::Message { key, payload } => {
                // the message_is_for_me variable trick is to unlock the network before handling the message
                let message_is_for_me = {
                    let network = network.read().unwrap();
                    let next_hop = network.route(&key)?;
                    if let Some(next_hop) = next_hop {
                        let packet = Packet::Message { key, payload };
                        network.send(packet, next_hop.addr())?;
                        false
                    }
                    else
                    {
                        true
                    }
                };
                if message_is_for_me {
                    todo!("Handle message")
                }
            },
        }
        Ok(())
    }

    fn run(network: Arc<RwLock<Network>>, running: Arc<RwLock<bool>>) {
        while *running.read().unwrap() {
            let packet = network.read().unwrap().recv();
            if let std::result::Result::Ok((packet, addr)) = packet
            {
                if let Err(e) = Self::handle_packet(network.clone(), packet, addr)
                {
                    // TODO: maybe log the error
                }
            }
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        {
            let mut running = self.running.write().unwrap();
            match *running {
                false => *running = true,
                true => bail!("Framework is already running"),
            }
        }
        
        let _running = self.running.clone();
        let _network = self.network.clone();
        self.threads.push(
        thread::Builder::new().name("network".to_string()).spawn(move || {
            Self::run(_network, _running);
        })?);

        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        {
            let mut running = self.running.write().unwrap();
            match *running {
                true => *running = false,
                false => bail!("Framework is not running"),
            }
        }
        
        for thread in self.threads.drain(..) {
            let name = thread.thread().name().unwrap_or("Unknown").to_string();
            if let Err(_) = thread.join() {
                bail!("Failed to join thread: {}", name);
            }
        }

        Ok(())
    }
}