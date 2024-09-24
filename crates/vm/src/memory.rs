pub struct Memory {
    bytes: Vec<u8>,
    size: usize,
}

impl Memory {
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; n],
            size: n,
        }
    }
}
