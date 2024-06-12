use std::net::{SocketAddr, UdpSocket};

use anyhow::{bail, Ok};

use crate::id::Id;

use super::{config::Config, packet::Packet, peer::Peer, routing::routing_table::RoutingTable};

const MTU: usize = 1500;

#[derive(Debug)]
pub struct Network {
    socket: UdpSocket,
    routing_table: Option<RoutingTable>,
    config: Config,
}

impl Network {
    pub fn bootstrap(&mut self, public_addr: SocketAddr) -> anyhow::Result<()> {
        let peer = Peer::new(public_addr);
        let routing_table = RoutingTable::empty(peer.id());
        self.set_routing_table(routing_table);
        Ok(())
    }

    pub fn new(config: Config) -> anyhow::Result<Self> {
        let socket = UdpSocket::bind(config.bind_addr)?;
        socket.set_read_timeout(Some(config.socket_read_timeout))?;
        socket.set_write_timeout(Some(config.socket_write_timeout))?;
        Ok(Self {
            socket,
            routing_table: None,
            config,
        })
    }

    pub fn send(&self, packet: Packet, addr: SocketAddr) -> anyhow::Result<()> {
        let buf = packet.serialize()?;
        self.socket.send_to(&buf, addr)?;
        Ok(())
    }

    pub fn recv(&self) -> anyhow::Result<(Packet, SocketAddr)> {
        let mut buf = [0; MTU];
        let (len, addr) = self.socket.recv_from(&mut buf)?;
        let packet = Packet::deserialize(&buf[..len])?;
        Ok((packet, addr))
    }

    pub fn route(&self, id: &Id) -> anyhow::Result<Option<&Peer>> {
        if let Some(routing_table) = &self.routing_table {
            Ok(routing_table.route(id))
        } else {
            bail!("Routing table is not initialized")
        }
    }

    pub fn get_routing_table(&self) -> Option<&RoutingTable> {
        self.routing_table.as_ref()
    }

    pub fn set_routing_table(&mut self, routing_table: RoutingTable) {
        self.routing_table = Some(routing_table);
    }
}