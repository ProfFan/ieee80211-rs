use scroll::Endian;

use crate::common::read_iterator::ReadIterator;

use super::EncodedRate;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RatesReadIterator<'a> {
    pub(crate) bytes: ReadIterator<'a, Endian, u8>,
}
impl<'a> RatesReadIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes: ReadIterator::new(bytes),
        }
    }
}
impl<'a> Iterator for RatesReadIterator<'a> {
    type Item = EncodedRate;
    fn next(&mut self) -> Option<Self::Item> {
        self.bytes.next().map(EncodedRate)
    }
}
