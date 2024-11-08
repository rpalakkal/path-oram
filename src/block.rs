#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Block<B> {
    pub data: B,
    pub idx: usize,
}

impl<B> Block<B> {
    pub fn new(data: B, idx: usize) -> Self {
        Self { data, idx }
    }

    pub fn is_empty(&self) -> bool {
        self.idx == usize::MAX
    }
}

impl<B: Default> Default for Block<B> {
    fn default() -> Self {
        Self {
            data: B::default(),
            idx: usize::MAX,
        }
    }
}
