// Local Imports
use community::Community;
use dst_as_path;
use error;
use ipaddress;
use types::ReadSeeker;
use utils::ReadBytesLocal;

// Std Lib Imports
use std::io::SeekFrom;

#[derive(Debug, Clone)]
pub enum FlowRecord {
    SampledHeader(SampledHeader), // Format 1
    SampledIpv4(SampledIpv4), // Format 3
    ExtendedSwitch(ExtendedSwitch), // Format 1001
    ExtendedRouter(ExtendedRouter), // Format 1002
    ExtendedGateway(ExtendedGateway), // Format 1003
    ExtendedUrl(ExtendedUrl), // Format 1005
    ExtendedMpls(ExtendedMpls), // Format 1006
    ExtendedMplsTunnel(ExtendedMplsTunnel), // Format 1008
}

impl ::utils::Decodeable for FlowRecord {
    fn read_and_decode(stream: &mut ReadSeeker) -> Result<FlowRecord, error::Error> {
        let format = try!(stream.be_read_u32());
        let length = try!(stream.be_read_u32());

        match format {
            1 => {
                let e = try!(SampledHeader::read_and_decode(stream));
                return Ok(FlowRecord::SampledHeader(e));
            }
            3 => {
                let e = try!(SampledIpv4::read_and_decode(stream));
                return Ok(FlowRecord::SampledIpv4(e));
            }
            1001 => {
                let e = try!(ExtendedSwitch::read_and_decode(stream));
                return Ok(FlowRecord::ExtendedSwitch(e));
            }
            1002 => {
                let e = try!(ExtendedRouter::read_and_decode(stream));
                return Ok(FlowRecord::ExtendedRouter(e));
            }
            1003 => {
                let e = try!(ExtendedGateway::read_and_decode(stream));
                return Ok(FlowRecord::ExtendedGateway(e));
            }
            1005 => {
                let e = try!(ExtendedUrl::read_and_decode(stream));
                return Ok(FlowRecord::ExtendedUrl(e));
            }
            1006 => {
                let e = try!(ExtendedMpls::read_and_decode(stream));
                return Ok(FlowRecord::ExtendedMpls(e));
            }
            1008 => {
                let e = try!(ExtendedMplsTunnel::read_and_decode(stream));
                return Ok(FlowRecord::ExtendedMplsTunnel(e));
            }
            _ => {
                println!("DEBUG: Unknown FlowRecord type {0} skipping {1} bytes.",
                         format,
                         length);
                try!(stream.seek(SeekFrom::Current(length as i64)));
                return Err(error::Error::UnknownType(format!("Unknown FlowRecord type {0} \
                                                              skipping {1} bytes.",
                                                             format,
                                                             length)));
            }
        }
    }
}

add_decoder!{
#[derive(Debug, Clone, Default)]
pub struct ExtendedGateway {
    pub next_hop: ipaddress::IPAddress,
    pub asn: u32, // Autonomous system number of router
    pub src_as: u32, // Autonomous system number of source
    pub src_peer_as: u32,   /* Autonomous system number of source peer */
    pub dst_as_path: Vec<dst_as_path::DstASPath>, /* Autonomous system path to the destination */
    pub communities: Vec<Community>, // Communities associated with this route
    pub localpref: u32, // LocalPref associated with this route
}
}

add_decoder!{
#[derive(Debug, Clone)]
pub struct ExtendedUrl {
   pub directoin: u32,   /* Direction of connection */
   pub url: String,               /* The HTTP request-line (see RFC 2616) */
   pub host: String,              /* The host field from the HTTP header */
}
}

add_decoder!{
#[derive(Debug, Clone)]
pub struct SampledIpv4 {
   pub length: u32,     /* The length of the IP packet excluding
                                  lower layer encapsulations */
   pub protocol: u32,   /* IP Protocol type
                                   (for example, TCP = 6, UDP = 17) */
   pub src_ip: ipaddress::IPAddress,            /* Source IP Address */
   pub dst_ip: ipaddress::IPAddress,            /* Destination IP Address */
   pub src_port: u32,   /* TCP/UDP source port number or equivalent */
   pub dst_port: u32,   /* TCP/UDP destination port number or equivalent */
   pub tcp_flags: u32,  /* TCP flags */
   pub tos: u32,        /* IP type of service */
}
}

add_decoder!{
#[derive(Debug, Clone)]
pub struct ExtendedSwitch {
   pub src_vlan: u32,     /* The 802.1Q VLAN id of incoming frame */
   pub src_priority: u32, /* The 802.1p priority of incoming frame */
   pub dst_vlan: u32,     /* The 802.1Q VLAN id of outgoing frame */
   pub dst_priority: u32, /* The 802.1p priority of outgoing frame */
}
}

add_decoder!{
#[derive(Debug, Clone)]
pub struct ExtendedRouter {
   pub nexthop: ipaddress::IPAddress,            /* IP address of next hop router */
   pub src_mask_len: u32,   /* Source address prefix mask
                                   (expressed as number of bits) */
   pub dst_mask_len: u32,   /* Destination address prefix mask
                                   (expressed as number of bits) */
}
}

add_decoder!{
#[derive(Debug, Clone)]
pub struct SampledHeader {
   // TODO: header_protocol should be an enum...
   pub protocol: u32,       /* Format of sampled header */
   pub frame_length: u32, /* Original length of packet before sampling. Note: For a layer 2
                             header_protocol, length is total number of octets of data received on
                             the network (excluding framing bits but including FCS octets).
                             Hardware limitations may prevent an exact reporting of the underlying
                             frame length, but an agent should attempt to be as accurate as
                             possible. Any octets added to the frame_length to compensate for
                             encapsulations removed by the underlying hardware must also be added
                             to the stripped count. */

   pub stripped: u32, /* The number of octets removed from the packet before extracting the
                         header<> octets. Trailing encapsulation data corresponding to any leading
                         encapsulations that were stripped must also be stripped. Trailing
                         encapsulation data for the outermost protocol layer included in the
                         sampled header must be stripped.

                         In the case of a non-encapsulated 802.3 packet stripped >= 4 since VLAN
                         tag information might have been stripped off in addition to the FCS.

                         Outer encapsulations that are ambiguous, or not one of the standard
                         header_protocol must be stripped. */
   pub header: Vec<u8>, /* Header bytes */
}
}

add_decoder!{
#[derive(Debug, Clone)]
pub struct ExtendedMplsTunnel {
   pub tunnel_lsp_name: String, /* Tunnel name */
   pub tunnel_id: u32,     /* Tunnel ID */
   pub tunnel_cos: u32,    /* Tunnel COS value */
}
}

add_decoder!{
#[derive(Debug, Clone)]
pub struct ExtendedMpls {
   pub nexthop: ipaddress::IPAddress,           /* Address of the next hop */
   pub in_stack: Vec<i32>,       /* Label stack of received packet */
   pub out_stack: Vec<i32>,      /* Label stack for transmitted packet */
}
}
