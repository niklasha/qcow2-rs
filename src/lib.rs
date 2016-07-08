extern crate byteorder;
extern crate positioned_io;
extern crate num;

mod error;
mod extension;
mod feature;
mod header;
mod int;
pub use error::Error;

use std::fmt::{self, Debug, Formatter};
use std::result;

use byteorder::BigEndian;
use positioned_io::{ReadAt, ByteIo};


pub struct Qcow2<I>
    where I: ReadAt
{
    pub header: header::Header,
    io: ByteIo<I, BigEndian>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl<I> Qcow2<I>
    where I: ReadAt
{
    pub fn open(io: I) -> Result<Self> {
        let io: ByteIo<_, BigEndian> = ByteIo::new(io);
        let mut q = Qcow2 {
            header: Default::default(),
            io: io,
        };
        try!(q.header.read(&mut q.io));
        Ok(q)
    }
}

impl<I> Debug for Qcow2<I> where I: ReadAt {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        f.debug_struct("Qcow2")
            .field("header", &self.header)
            .finish()
    }
}
