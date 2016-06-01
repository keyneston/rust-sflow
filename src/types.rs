//! Types is a small wrapper module for storing types that are shared amongst multiple other record
//! and sample types.

use std::io::{Read, Seek};

pub type Interface = u32;
pub type SourceID = u32;

pub trait ReadSeeker: Read + Seek {}
impl<T> ReadSeeker for T where T: Read + Seek {}
