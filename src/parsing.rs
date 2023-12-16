// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

use core::convert::TryInto;
pub struct CStr<'a>(&'a [u8]);

impl<'a> CStr<'a> {
    pub fn new(data: &'a [u8]) -> Option<Self> {
        let end = data.iter().position(|&b| b == 0)?;
        Some(Self(&data[..end]))
    }

    /// Does not include the null terminating byte
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_str(&self) -> Option<&'a str> {
        core::str::from_utf8(self.0).ok()
    }
}

pub fn u32_from_be_byte_slice(bytes: &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes(bytes.get(..4)?.try_into().unwrap()))
}

pub fn u64_from_be_byte_slice(bytes: &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(bytes.get(..8)?.try_into().unwrap()))
}

#[derive(Debug, Clone, Copy)]
pub struct FdtData<'a> {
    bytes: &'a [u8],
}

impl<'a> FdtData<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn u32(&mut self) -> Option<u32> {
        let ret = u32_from_be_byte_slice(self.bytes)?;
        self.skip(4);

        Some(ret)
    }

    pub fn u64(&mut self) -> Option<u64> {
        let ret = u64_from_be_byte_slice(self.bytes)?;
        self.skip(8);

        Some(ret)
    }

    pub fn skip(&mut self, n_bytes: usize) {
        self.bytes = self.bytes.get(n_bytes..).unwrap_or_default()
    }

    pub fn remaining(&self) -> &'a [u8] {
        self.bytes
    }

    pub fn peek_u32(&self) -> Option<u32> {
        Self::new(self.remaining()).u32()
    }

    pub fn is_empty(&self) -> bool {
        self.remaining().is_empty()
    }

    pub fn skip_nops(&mut self) {
        while let Some(crate::node::FDT_NOP) = self.peek_u32() {
            let _ = self.u32();
        }
    }

    pub fn take(&mut self, bytes: usize) -> Option<&'a [u8]> {
        if self.bytes.len() >= bytes {
            let ret = &self.bytes[..bytes];
            self.skip(bytes);

            return Some(ret);
        }

        None
    }
}
