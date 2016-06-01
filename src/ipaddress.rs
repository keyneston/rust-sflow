use std::net;
use std::io;

use types::*;
use error::Error;
use utils::ReadBytesLocal;
use byteorder::ReadBytesExt;

#[derive(Debug, Clone, Copy)]
pub enum IPAddress {
    IPv4(net::Ipv4Addr),
    IPv6(net::Ipv6Addr),
}

impl Default for IPAddress {
    fn default() -> IPAddress {
        IPAddress::IPv4(net::Ipv4Addr::new(0, 0, 0, 0))
    }
}

/// decode_ip_address will read from the stream and decode an IPAddress. Either an IPv4 or an IPv6
/// address. This also has a side effect of progressing the stream forward to the next data to be
/// decoded.
impl ::utils::Decodeable for IPAddress {
    fn read_and_decode(stream: &mut ReadSeeker) -> Result<IPAddress, Error> {
        let ip_version = try!(stream.be_read_u32());

        let ip: IPAddress;

        match ip_version {
            1 => ip = try!(decode_ipv4(stream)),
            2 => ip = try!(decode_ipv6(stream)),
            _ => {
                let err_string = format!("Unknown sflow ip type {}", ip_version);
                return Err(Error::Io(io::Error::new(io::ErrorKind::InvalidData, err_string)));
            }

        }

        Ok(ip)
    }
}

fn decode_ipv4(stream: &mut ReadSeeker) -> Result<IPAddress, Error> {
    let mut b: [u8; 4] = [0; 4];
    for i in 0..4 {
        b[i] = try!(stream.read_u8());
    }

    Ok(IPAddress::IPv4(net::Ipv4Addr::new(b[0], b[1], b[2], b[3])))
}

fn decode_ipv6(stream: &mut ReadSeeker) -> Result<IPAddress, Error> {
    let mut b: [u16; 8] = [0; 8];
    for i in 0..8 {
        b[i] = try!(stream.be_read_u16())
    }

    Ok(IPAddress::IPv6(net::Ipv6Addr::new(b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7])))
}
