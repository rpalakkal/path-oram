use eyre::OptionExt;

use crate::block::Block;

#[derive(Debug, Clone)]
pub struct Bucket<B, const Z: usize> {
    pub blocks: Box<[Block<B>; Z]>,
}

impl<B: Default + Clone, const Z: usize> Bucket<B, Z> {
    pub fn new() -> eyre::Result<Self> {
        let blocks: Vec<Block<B>> = (0..Z).map(|_| Block::default()).collect();
        Ok(Self {
            blocks: blocks.try_into().ok().ok_or_eyre("too many blocks")?,
        })
    }

    pub fn from_blocks(mut blocks: Vec<Block<B>>) -> eyre::Result<Self> {
        if blocks.len() > Z {
            return Err(eyre::eyre!("too many blocks"));
        }
        blocks.resize(Z, Block::default());
        let arr: [Block<B>; Z] = blocks
            .try_into()
            .ok()
            .ok_or_eyre("failed to convert vec to array")?;
        Ok(Self {
            blocks: Box::new(arr),
        })
    }
}
