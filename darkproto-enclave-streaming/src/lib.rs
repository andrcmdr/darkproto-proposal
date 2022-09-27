pub mod cli;
pub mod cli_parser;
pub mod config;
pub mod vsock;

use tracing::debug;

use cli_parser::{ServerArgs, ClientArgs, RxTxArgs};
use config::AppConfig;
use vsock::{recv_loop, recv_u64, send_loop, send_u64};

use nix::sys::socket::listen as listen_vsock;
use nix::sys::socket::{accept, bind, connect, shutdown, socket};
use nix::sys::socket::{AddressFamily, Shutdown, SockAddr, SockFlag, SockType};
use nix::unistd::close;
use std::convert::TryInto;
use std::os::unix::io::{AsRawFd, RawFd};

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;
/// 32 or 64 KiB buffer size (depending on 32 or 64 bit architecture is used)
const BUF_MAX_LEN: usize = 8192;
/// Maximum number of outstanding connections in the socket's listen queue
const BACKLOG: usize = 128;
/// Maximum number of connection attempts
const MAX_CONNECTION_ATTEMPTS: usize = 5;

struct VsockSocket {
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
            .unwrap_or_else(|e| eprintln!("Failed to shut socket down: {:?}", e));
        close(self.socket_fd).unwrap_or_else(|e| eprintln!("Failed to close socket: {:?}", e));
    }
}

impl AsRawFd for VsockSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd
    }
}

/// Initiate a connection on an AF_VSOCK socket.
/// VSOCK Address:
/// The address for AF_VSOCK socket is defined as a combination of a
/// 32-bit Context Identifier (CID) and a 32-bit port number.
fn vsock_connect(cid: u32, port: u32) -> Result<VsockSocket, String> {
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
            .map_err(|err| format!("Failed to create the socket: {:?}", err))?,
        );
        match connect(vsocket.as_raw_fd(), &sockaddr) {
            Ok(_) => return Ok(vsocket),
            Err(e) => err_msg = format!("Failed to connect: {}", e),
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
    let len: u64 = buf.len().try_into().map_err(|err| format!("{:?}", err))?;
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
    .map_err(|err| format!("Create socket failed: {:?}", err))?;

    let sockaddr = SockAddr::new_vsock(VMADDR_CID_ANY, args.port);

    bind(socket_fd, &sockaddr).map_err(|err| format!("Bind failed: {:?}", err))?;

    listen_vsock(socket_fd, BACKLOG).map_err(|err| format!("Listen failed: {:?}", err))?;

    loop {
        let fd = accept(socket_fd).map_err(|err| format!("Accept failed: {:?}", err))?;

        // TODO: Replace this with actual server code
        let len = recv_u64(fd)?;
        let mut buf = [0u8; BUF_MAX_LEN];
        recv_loop(fd, &mut buf, len)?;
        println!(
            "{}",
            String::from_utf8(buf.to_vec())
                .map_err(|err| format!("The received bytes are not UTF-8: {:?}", err))?
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
    .map_err(|err| format!("Create rx socket failed: {:?}", err))?;

    let rx_sockaddr = SockAddr::new_vsock(VMADDR_CID_ANY, args.rx_port);

    bind(rx_socket_fd, &rx_sockaddr).map_err(|err| format!("Bind failed: {:?}", err))?;

    listen_vsock(rx_socket_fd, BACKLOG).map_err(|err| format!("Listen failed: {:?}", err))?;

    let tx_vsocket = vsock_connect(args.cid, args.tx_port)?;
    let tx_fd = tx_vsocket.as_raw_fd();

    loop {
        let rx_fd = accept(rx_socket_fd).map_err(|err| format!("Accept failed: {:?}", err))?;

        let rx_len = recv_u64(rx_fd)?;
        let mut buf = [0u8; BUF_MAX_LEN];
        recv_loop(rx_fd, &mut buf, rx_len)?;
        debug!(target: "darkproto", "{:?} bytes long data was received", rx_len);

        // encrypt buffer

        let tx_len: u64 = buf.len().try_into().map_err(|err| format!("{:?}", err))?;
        send_u64(tx_fd, tx_len)?;
        send_loop(tx_fd, &buf, tx_len)?;
        debug!(target: "darkproto", "{:?} bytes long data was transmitted", rx_len);
    }
}

/// Host mode handler:
/// connect to a given CID and port, stream data (to the enclave),
/// and listen on a given port to receive data (from inside the enclave),
/// then dump received data to persistent storage.
pub fn txrx(args: RxTxArgs, config: AppConfig) -> Result<(), String> {
    todo!("{:?}\n{:?}\n", args, config)
}
