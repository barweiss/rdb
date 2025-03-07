use bytes::{Buf, BufMut};

pub trait Row {
    fn serialize<T: BufMut>(&self, into: &mut T);
    fn deserialize<T: Buf>(from: &T);
}
