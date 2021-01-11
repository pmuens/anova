use sha3::Digest;

/// A types binary encoding.
pub(crate) type BinEncoding = Vec<u8>;

// A Keccak256 hash.
pub(crate) type Keccak256 = Vec<u8>;

/// Creates a Keccak256 hash of the given data.
pub(crate) fn hash<T: AsRef<[u8]>>(data: T) -> Vec<u8> {
    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    hasher.finalize().as_slice().to_vec()
}
