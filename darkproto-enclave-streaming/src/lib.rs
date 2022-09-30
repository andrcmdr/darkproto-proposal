pub mod cli;
pub mod cli_parser;
pub mod config;
pub mod vsock;

use serde::{Deserialize, Serialize};

use tracing::{debug, error};

use cli_parser::{ServerArgs, ClientArgs, RxTxArgs};
use config::AppConfig;
use vsock::{recv_loop, recv_u64, send_loop, send_u64};

use nix::sys::socket::listen as listen_vsock;
use nix::sys::socket::{accept, bind, connect, shutdown, socket};
use nix::sys::socket::{AddressFamily, Shutdown, SockAddr, SockFlag, SockType};
use nix::unistd::close;
use std::convert::TryInto;
use std::os::unix::io::{AsRawFd, RawFd};

use std::io::Read;

use blake2::{Blake2b512, Digest};

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;
/// 32 or 64 KiB buffer size (depending on 32 or 64 bit architecture is used)
const BUF_MAX_LEN: usize = 8192;
/// Maximum number of outstanding connections in the socket's listen queue
const BACKLOG: usize = 128;
/// Maximum number of connection attempts
const MAX_CONNECTION_ATTEMPTS: usize = 10;

pub struct VsockSocket {
    socket_fd: RawFd,
}

impl VsockSocket {
    fn new(socket_fd: RawFd) -> Self {
        VsockSocket { socket_fd }
    }
}

impl Drop for VsockSocket {
    fn drop(&mut self) {
        shutdown(self.socket_fd, Shutdown::Both)
            .unwrap_or_else(|err| error!(target: "darkproto", "Failed to shut socket down: {:?}", err));
        close(self.socket_fd).unwrap_or_else(|err| error!(target: "darkproto", "Failed to close socket: {:?}", err));
    }
}

impl AsRawFd for VsockSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd
    }
}

#[derive(Debug, Clone)]
pub struct ChunkSource<T> where T: Read {
    source: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    data: Vec<u8>,
    hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkEncrypted {
    data: Vec<u8>,
    hash_plaintext: String,
    hash_ciphertext: String,
}

impl<T> Iterator for ChunkSource<T> where T: Read {
    type Item = Chunk;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer  = [0; 1024*1024];
        let read_bytes_len = self.source.read(&mut buffer[..])
            .unwrap_or_else(|err| {
                error!(target: "darkproto", "Error while reading the bytes chunk of data from the source: {:?}", err);
                0_usize
            });
        Some(produce_chunk(&buffer[..read_bytes_len]))
    }
}

pub fn produce_chunk(in_data: &[u8]) -> Chunk {
    Chunk { 
        data: in_data.to_vec(),
        hash: hash_data(in_data),
    }
}

pub fn produce_encrypted_chunk(in_data: &Chunk) -> ChunkEncrypted {
    let encrypted_data = encrypt_data(in_data.data.as_slice());
    ChunkEncrypted { 
        data: encrypted_data.clone(),
        hash_plaintext: in_data.hash.clone(),
        hash_ciphertext: hash_data(encrypted_data.as_slice()),
    }
}

pub fn hash_data(in_data: &[u8]) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(in_data);
    let hash = hasher.finalize();
    format!("{:16x}", hash).trim().to_string()
}

pub fn encrypt_data(in_data: &[u8]) -> Vec<u8> {
    todo!("{:?}", in_data)
}

/// Initiate a connection on an AF_VSOCK socket.
/// VSOCK Address:
/// The address for AF_VSOCK socket is defined as a combination of a
/// 32-bit Context Identifier (CID) and a 32-bit port number.
pub fn vsock_connect(cid: u32, port: u32) -> Result<VsockSocket, String> {
    let sockaddr = SockAddr::new_vsock(cid, port);
    let mut err_msg = String::new();

    for i in 0..MAX_CONNECTION_ATTEMPTS {
        let vsocket = VsockSocket::new(
            socket(
                AddressFamily::Vsock,
                SockType::Stream,
                SockFlag::empty(),
                None,
            )
            .map_err(|err| {
                error!(target: "darkproto", "Failed to create socket: {:?}", err);
                format!("Failed to create socket: {:?}", err)
            })?,
        );
        match connect(vsocket.as_raw_fd(), &sockaddr) {
            Ok(_) => return Ok(vsocket),
            Err(e) => {
                error!(target: "darkproto", "Failed to connect: {:?}", e);
                err_msg = format!("Failed to connect: {:?}", e)
            },
        }

        // Exponentially backoff before retrying to connect to the socket
        std::thread::sleep(std::time::Duration::from_secs(1 << i));
    }

    Err(err_msg)
}

/// Send data in a message to the server (enclave):
/// i.e. connect to a given CID and port, stream data (to the enclave).
pub fn client(args: ClientArgs) -> Result<(), String> {
    let vsocket = vsock_connect(args.cid, args.port)?;
    let fd = vsocket.as_raw_fd();

    // TODO: Replace this with actual client code
    let data = "Hello, world!".to_string();
    let buf = data.as_bytes();
    let len: u64 = buf.len().try_into().map_err(|err| {
        error!(target: "darkproto", "{:?}", err);
        format!("{:?}", err)
    })?;
    send_u64(fd, len)?;
    send_loop(fd, buf, len)?;

    Ok(())
}

/// Accept connections on a certain port and receive data in a message from client (sender from inside the enclave):
/// i.e. listen on a given port to receive data (from inside the enclave),
/// then dump received data to persistent storage.
pub fn server(args: ServerArgs) -> Result<(), String> {
    let socket_fd = socket(
        AddressFamily::Vsock,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .map_err(|err| {
        error!(target: "darkproto", "Create socket failed: {:?}", err);
        format!("Create socket failed: {:?}", err)
    })?;

    let sockaddr = SockAddr::new_vsock(VMADDR_CID_ANY, args.port);

    bind(socket_fd, &sockaddr).map_err(|err| {
        error!(target: "darkproto", "Bind failed: {:?}", err);
        format!("Bind failed: {:?}", err)
    })?;

    listen_vsock(socket_fd, BACKLOG).map_err(|err| {
        error!(target: "darkproto", "Listen failed: {:?}", err);
        format!("Listen failed: {:?}", err)
    })?;

    loop {
        let fd = accept(socket_fd).map_err(|err| {
            error!(target: "darkproto", "Accept failed: {:?}", err);
            format!("Accept failed: {:?}", err)
        })?;

        // TODO: Replace this with actual server code
        let len = recv_u64(fd)?;
        let mut buf = [0u8; BUF_MAX_LEN];
        recv_loop(fd, &mut buf, len)?;
        println!(
            "{}",
            String::from_utf8(buf.to_vec())
                .map_err(|err| {
                    error!(target: "darkproto", "The received bytes are not UTF-8: {:?}", err);
                    format!("The received bytes are not UTF-8: {:?}", err)
                })?
        );
    }
}

/// Enclave mode handler:
/// listen on a given port, receive data (from host),
/// then connect to a given CID and port and stream data (to host) from inside the enclave.
pub fn rxtx(args: RxTxArgs, _config: AppConfig) -> Result<(), String> {
    let rx_socket_fd = socket(
        AddressFamily::Vsock,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .map_err(|err| {
        error!(target: "darkproto", "Create rx socket failed: {:?}", err);
        format!("Create rx socket failed: {:?}", err)
    })?;

    let rx_sockaddr = SockAddr::new_vsock(VMADDR_CID_ANY, args.rx_port);

    bind(rx_socket_fd, &rx_sockaddr).map_err(|err| {
        error!(target: "darkproto", "Bind failed: {:?}", err);
        format!("Bind failed: {:?}", err)
    })?;

    listen_vsock(rx_socket_fd, BACKLOG).map_err(|err| {
        error!(target: "darkproto", "Listen failed: {:?}", err);
        format!("Listen failed: {:?}", err)
    })?;

    let tx_vsocket = vsock_connect(args.cid, args.tx_port)?;
    let tx_fd = tx_vsocket.as_raw_fd();

    let (mut rx_block_num, mut tx_block_num) = (0_usize, 0_usize);
    loop {
        let rx_fd = accept(rx_socket_fd).map_err(|err| {
            error!(target: "darkproto", "Accept failed: {:?}", err);
            format!("Accept failed: {:?}", err)
        })?;

        let rx_len = recv_u64(rx_fd)?;
        let mut buf = [0u8; BUF_MAX_LEN];
        recv_loop(rx_fd, &mut buf, rx_len)?;
        rx_block_num+=1;
        debug!(target: "darkproto", "{:?} bytes long data was received in data block #{:?}", rx_len, rx_block_num);

        // TODO: buffer encryption

        let tx_len: u64 = buf.len().try_into().map_err(|err| {
            error!(target: "darkproto", "{:?}", err);
            format!("{:?}", err)
        })?;
        send_u64(tx_fd, tx_len)?;
        send_loop(tx_fd, &buf, tx_len)?;
        tx_block_num+=1;
        debug!(target: "darkproto", "{:?} bytes long data was transmitted in data block #{:?}", tx_len, tx_block_num);
    }
}

/// Host mode handler:
/// connect to a given CID and port, stream data (to the enclave),
/// and listen on a given port to receive data (from inside the enclave),
/// then dump received data to persistent storage.
pub fn txrx(args: RxTxArgs, config: AppConfig) -> Result<(), String> {
    todo!("{:?}\n{:?}\n", args, config)
}
