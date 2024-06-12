use std::{net::SocketAddr, sync::{atomic::AtomicBool, Arc, RwLock, RwLockWriteGuard}, thread};

use anyhow::{bail, Ok};

use super::{config::Config, packet::{self, Packet}, Network};

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
            Packet::JoinRequest => todo!(),
            Packet::PeerIsJoining { applicant, hop_count } => todo!(),
            Packet::JoinResponse { routing_table_row, hop_count } => todo!(),
            Packet::Ping { nonce } => todo!(),
            Packet::Pong { nonce } => todo!(),
            Packet::Message { key, payload } => todo!(),
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
                    continue;
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