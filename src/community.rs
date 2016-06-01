use std::fmt;
use error;

// Local Imports
use types;
use utils::Decodeable;
use utils::ReadBytesLocal;

/// Community represents a BGP community. While normally a community is a u32 with the first
/// half being the asn, and the second half being a tag or value, in this case we are storing the
/// asn as a u32 in order to support extended asns.
#[derive(Debug, Copy, Clone, Default)]
pub struct Community {
    pub asn: u32,
    pub tag: u16,
}

impl fmt::Display for Community {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.asn, self.tag)
    }
}

impl Decodeable for Community {
    #[inline]
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<Community, error::Error> {
        let r = Community {
            asn: try!(stream.be_read_u16()) as u32,
            tag: try!(stream.be_read_u16()),
        };

        Ok(r)
    }
}
