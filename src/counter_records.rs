// Local Imports
use error;
use types::ReadSeeker;
use utils::ReadBytesLocal;

// Std Lib imports
use std::io::SeekFrom;

#[derive(Debug, Clone)]
pub enum CounterRecord {
    InterfaceCounters(InterfaceCounters),
    EthernetCounters(EthernetCounters),
    Tokenringcounters,
    VGCounters,
    VLANCounters,
}

add_decoder!{
#[derive(Debug, Clone, Default)]
pub struct InterfaceCounters {
    pub if_index: u32,
    pub if_type: u32,
    pub if_speed: u64,

    // derived from MAU MIB (RFC 2668)
    // 0 = unkown, 1=full-duplex, 2=half-duplex,
    // 3 = in, 4=out
    pub if_direction: u32,

    // bit field with the following bits assigned
    // bit 0 = ifAdminStatus (0 = down, 1 = up)
    // bit 1 = ifOperStatus (0 = down, 1 = up)
    pub if_status: u32,
    pub if_in_octets: u64,
    pub if_in_ucast_pkts: u32,
    pub if_in_multicast_pkts: u32,
    pub if_in_broadcast_pkts: u32,
    pub if_in_discards: u32,
    pub if_in_errors: u32,
    pub if_in_unknown_protos: u32,
    pub if_out_octets: u64,
    pub if_out_ucast_pkts: u32,
    pub if_out_multicast_pkts: u32,
    pub if_out_broadcast_pkts: u32,
    pub if_out_discards: u32,
    pub if_out_errors: u32,
    pub if_promiscuous_mode: u32,
}
}

add_decoder!{
#[derive(Debug, Clone, Default)]
pub struct EthernetCounters {
   pub dot3_stats_alignment_errors: u32,
   pub dot3_stats_fcs_errors: u32,
   pub dot3_stats_single_collision_frames: u32,
   pub dot3_stats_multiple_collision_frames: u32,
   pub dot3_stats_sqe_test_errors: u32,
   pub dot3_stats_deferred_transmissions: u32,
   pub dot3_stats_late_collisions: u32,
   pub dot3_stats_excessive_collisions: u32,
   pub dot3_stats_internal_mac_transmit_errors: u32,
   pub dot3_stats_carrier_sense_errors: u32,
   pub dot3_stats_frame_too_longs: u32,
   pub dot3_stats_internal_mac_receive_errors: u32,
   pub dot3_stats_symbol_errors: u32,
}
}

impl ::utils::Decodeable for CounterRecord {
    fn read_and_decode(stream: &mut ReadSeeker) -> Result<CounterRecord, error::Error> {
        let format = try!(stream.be_read_u32());
        let length = try!(stream.be_read_u32());

        match format {
            1 => {
                let e = try!(InterfaceCounters::read_and_decode(stream));
                return Ok(CounterRecord::InterfaceCounters(e));
            }
            2 => {
                let e = try!(EthernetCounters::read_and_decode(stream));
                return Ok(CounterRecord::EthernetCounters(e));
            }
            _ => {
                println!(
                    "DEBUG Unknown CounterRecord type {0} skipping {1} bytes.",
                    format,
                    length
                );
                try!(stream.seek(SeekFrom::Current(length as i64)));
                return Err(error::Error::UnknownType(format!(
                    "Unknown CounterRecord type {0} skipping {1} bytes",
                    format,
                    length
                )));
            }
        }
    }
}
