pub mod cycle;
pub mod naive;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Digest([u8; 7]);

impl Digest {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Digest> for [u8; 7] {
    #[inline]
    fn from(digest: Digest) -> Self {
        digest.0
    }
}

pub fn custom_md5<T: AsRef<[u8]>>(data: T) -> Digest {
    let mut result = [0; 7];
    result.copy_from_slice(&md5::compute(md5::compute(data).0).0[..7]);
    Digest(result)
}

pub fn append_prefix(prefix: &[u8], input: &[u8]) -> Vec<u8> {
    let mut result = prefix.to_vec();
    result.extend_from_slice(input);
    result
}

pub fn compute_function(prefix: &[u8], input: &[u8]) -> Vec<u8> {
    custom_md5(append_prefix(prefix, input)).as_bytes().to_vec()
}
