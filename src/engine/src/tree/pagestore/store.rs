use crate::tree::{page::PagePtr, Result};

#[derive(Copy, Clone, Debug)]
pub struct PageInfo {
    pub ver: u64,
    pub len: u8,
    pub is_leaf: bool,
}

pub struct PageStore {}

#[allow(dead_code)]
impl PageStore {
    pub fn open() -> Result<Self> {
        Ok(Self {})
    }

    pub fn page_info(&self, _addr: u64) -> Option<PageInfo> {
        todo!()
    }

    pub fn load_page(&self, _addr: u64) -> Result<PagePtr> {
        todo!()
    }

    pub fn acquire_page(&self) -> u64 {
        todo!()
    }

    pub fn release_page(&self, _addr: u64) {
        todo!()
    }
}
