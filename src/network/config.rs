use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub entry_addr: SocketAddr,
    pub socket_read_timeout: std::time::Duration,
    pub socket_write_timeout: std::time::Duration,
}