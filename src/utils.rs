use sha3::Digest;

/// Dummy trait used to map a generic type to a u8.
pub trait AlwaysU8 {
    type Type;
}

impl<T> AlwaysU8 for T {
    type Type = u8;
}

/// A types binary encoding.
pub(crate) type BinEncoding<T> = Vec<<T as AlwaysU8>::Type>;

// A Keccak256 hash.
pub(crate) type Keccak256 = Vec<u8>;

// A Keccak256 hash of a senders public key.
pub(crate) type Sender = Keccak256;

/// Creates a Keccak256 hash of the given data.
pub(crate) fn hash<T: AsRef<[u8]>>(data: T) -> Keccak256 {
    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    hasher.finalize().as_slice().to_vec()
}
