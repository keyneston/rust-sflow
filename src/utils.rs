use byteorder::{self, ByteOrder, BigEndian, ReadBytesExt};
use error;
use types;

use std::io::{self, SeekFrom, Read};
use std::mem::size_of;
use std::vec::Vec;

pub trait ReadBytesLocal: io::Read {
    #[inline]
    /// be_read_u32 will read 32 bits in *b*ig *e*dian format.
    fn be_read_u32(&mut self) -> Result<u32, byteorder::Error> {
        return self.read_u32::<BigEndian>();
    }

    #[inline]
    /// be_read_u16 will read 16 bits in *b*ig *e*dian format.
    fn be_read_u16(&mut self) -> Result<u16, byteorder::Error> {
        return self.read_u16::<BigEndian>();
    }

    #[inline]
    /// be_read_i32 will read 32 bits in *b*ig *e*dian format.
    fn be_read_i32(&mut self) -> Result<i32, byteorder::Error> {
        return self.read_i32::<BigEndian>();
    }

    #[inline]
    /// be_read_i16 will read 16 bits in *b*ig *e*dian format.
    fn be_read_i16(&mut self) -> Result<i16, byteorder::Error> {
        return self.read_i16::<BigEndian>();
    }
}

impl<R: io::Read + ?Sized> ReadBytesLocal for R {}

// The add_decoder macro is super super brittle. For example it only works if the struct and every
// field is public.
//
// It would be possible to build a second matching case to match non public structs, but I don't
// know of anyway to match public and non-public fields in the same struct.
#[macro_use(try)]
macro_rules! add_decoder {
    ( $( #[$struct_attr:meta] )*
    pub struct $name:ident {
        $(pub $field_name:ident: $field_type:ty,)*
    }) => {
        $( #[$struct_attr] )*
        pub struct $name {
            $(pub $field_name: $field_type,)*
        }

        impl ::utils::Decodeable for $name {
// decode is an automatically generated function from the add_decoder macro.
            fn read_and_decode(stream: &mut ::types::ReadSeeker) -> ::std::result::Result<$name, ::error::Error> {
                let s: $name =  $name{
                $($field_name : try!(::utils::Decodeable::read_and_decode(stream))),+
                };

                Ok(s)
            }
        }
    };
}

pub trait Decodeable {
    fn read_and_decode(&mut types::ReadSeeker) -> Result<Self, ::error::Error> where Self: Sized;
}

impl Decodeable for u8 {
    #[inline]
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<Self, error::Error> {
        let r = try!(stream.read_u8());

        Ok(r)
    }
}

impl Decodeable for u32 {
    #[inline]
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<u32, error::Error> {
        let r = try!(stream.be_read_u32());

        Ok(r)
    }
}

impl Decodeable for u16 {
    #[inline]
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<Self, error::Error> {
        let r = try!(stream.be_read_u16());

        Ok(r)
    }
}

impl Decodeable for i8 {
    #[inline]
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<Self, error::Error> {
        let r = try!(stream.read_i8());

        Ok(r)
    }
}

impl Decodeable for i32 {
    #[inline]
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<i32, error::Error> {
        let r = try!(stream.be_read_i32());

        Ok(r)
    }
}

impl Decodeable for i16 {
    #[inline]
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<Self, error::Error> {
        let r = try!(stream.be_read_i16());

        Ok(r)
    }
}

impl Decodeable for String {
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<Self, error::Error> {
        // Get the XDR length
        let length: usize = try!(stream.be_read_u32()) as usize;

        // Create a buffer to read the buf.
        let mut buf: Vec<u8> = Vec::with_capacity(length);
        unsafe {
            buf.set_len(length);
        }

        try!(stream.read_exact(&mut buf));
        let s = try!(String::from_utf8(buf));

        // We need to figure out how much padding will be needed.
        let mut padding = (4 - (length as i64)) % 4;
        if padding < 0 {
            padding += 4
        }
        if padding != 0 {
            try!(stream.seek(SeekFrom::Current(padding as i64)));
        }

        Ok(s)
    }
}

impl<T: Decodeable> Decodeable for Vec<T> {
    fn read_and_decode(stream: &mut types::ReadSeeker) -> Result<Vec<T>, error::Error> {
        // First we need to figure out how many samples there are.
        let count = try!(stream.be_read_u32());
        let mut results: Vec<T> = Vec::new();

        // We need to figure out how much padding will be needed.
        let total_size = ((count as usize) * size_of::<T>()) as i64;

        let mut padding = (4 - total_size) % 4;
        if padding < 0 {
            padding += 4
        }

        for _ in 0..count {
            let x: Result<T, error::Error> = ::utils::Decodeable::read_and_decode(stream);

            match x {
                Ok(x) => results.push(x),
                Err(error::Error::UnknownType(_)) => continue,
                Err(e) => return Err(e),
            }

        }

        try!(stream.seek(SeekFrom::Current(padding as i64)));

        Ok(results)
    }
}
