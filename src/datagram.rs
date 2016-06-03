// Internal Imports
use sample::SampleRecord;
use ipaddress::IPAddress;

add_decoder!{
#[derive(Debug, Clone)]
pub struct Datagram {
    pub sflow_version: u32,
    pub agent_address: IPAddress,
    pub sub_agent_id: u32,
    pub sequence_number: u32,
    pub uptime: u32,
    pub sample_record: Vec<SampleRecord>,
}
}
