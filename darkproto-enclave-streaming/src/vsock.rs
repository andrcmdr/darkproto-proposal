use tracing::error;
use byteorder::{ByteOrder, LittleEndian};
use nix::errno::Errno::EINTR;
use nix::sys::socket::MsgFlags;
use nix::sys::socket::{recv, send};
use std::convert::TryInto;
use std::mem::size_of;
use std::os::unix::io::RawFd;

/// Send the transferring data length in u64 though vsocket
pub fn send_u64(fd: RawFd, val: u64) -> Result<(), String> {
    let mut buf = [0u8; size_of::<u64>()];
    LittleEndian::write_u64(&mut buf, val);
    send_loop(fd, &buf, size_of::<u64>().try_into().unwrap())?;
    Ok(())
}

/// Take the receiving data length in u64 from vsocket
pub fn recv_u64(fd: RawFd) -> Result<u64, String> {
    let mut buf = [0u8; size_of::<u64>()];
    recv_loop(fd, &mut buf, size_of::<u64>().try_into().unwrap())?;
    let val = LittleEndian::read_u64(&buf);
    Ok(val)
}

/// Send exact data length in bytes from  buffer to a connection-oriented vsocket
pub fn send_loop(fd: RawFd, buf: &[u8], len: u64) -> Result<(), String> {
    let len: usize = len.try_into().map_err(|err| {
        error!(target: "darkproto", "{:?}", err);
        format!("{:?}", err)
    })?;
    let mut send_bytes = 0;

    while send_bytes < len {
        let size = match send(fd, &buf[send_bytes..len], MsgFlags::empty()) {
            Ok(size) => size,
            Err(nix::Error::Sys(EINTR)) => {
                error!(target: "darkproto", "EINTR error");
                0
            },
            Err(err) => {
                error!(target: "darkproto", "{:?}", err);
                return Err(format!("{:?}", err))
            },
        };
        send_bytes += size;
    }

    Ok(())
}

/// Receive exact data length in bytes to buffer from a connection-orriented vsocket
pub fn recv_loop(fd: RawFd, buf: &mut [u8], len: u64) -> Result<(), String> {
    let len: usize = len.try_into().map_err(|err| {
        error!(target: "darkproto", "{:?}", err);
        format!("{:?}", err)
}   )?;
    let mut recv_bytes = 0;

    while recv_bytes < len {
        let size = match recv(fd, &mut buf[recv_bytes..len], MsgFlags::empty()) {
            Ok(size) => size,
            Err(nix::Error::Sys(EINTR)) => {
                error!(target: "darkproto", "EINTR error");
                0
            },
            Err(err) => {
                error!(target: "darkproto", "{:?}", err);
                return Err(format!("{:?}", err))
            },
        };
        recv_bytes += size;
    }

    Ok(())
}
