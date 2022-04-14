use alloc::vec::Vec;
use primitive_types::{H160, H256};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHash(pub H256);

impl AsRef<[u8]> for BlockHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<[u8; 32]> for BlockHash {
    fn from(b: [u8; 32]) -> Self {
        Self(H256(b))
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Txhash(pub H256);

impl From<[u8; 32]> for Txhash {
    fn from(b: [u8; 32]) -> Self {
        Self(H256(b))
    }
}

impl AsRef<[u8]> for Txhash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Debug, Default)]
pub struct MemoOperation(pub u32);

#[derive(Debug, Default)]
pub struct Memo {
    pub operation: MemoOperation,
    pub data: Vec<u8>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct OutputOperation(pub u32);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Amount(pub u64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHeight(pub i64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MerkleHash(pub H256);

impl From<[u8; 32]> for MerkleHash {
    fn from(b: [u8; 32]) -> Self {
        Self(H256(b))
    }
}

impl AsRef<[u8]> for MerkleHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Timestamp(pub i64);

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NodeAddress(pub H160);

impl From<[u8; 20]> for NodeAddress {
    fn from(b: [u8; 20]) -> Self {
        Self(H160(b))
    }
}

impl AsRef<[u8]> for NodeAddress {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
