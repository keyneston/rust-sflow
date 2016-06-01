

add_decoder!{
#[derive(Debug, Clone)]
pub struct DstASPath {
    pub ordered: u32,
    pub elements: Vec<u32>,
}
}
