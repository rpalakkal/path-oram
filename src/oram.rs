use std::{collections::HashMap, hash::Hash};

use eyre::OptionExt;
use rand::Rng;

use crate::{block::Block, bucket::Bucket};

pub struct PathORAM<B, const Z: usize, const N: usize> {
    pub buckets: Box<[Bucket<B, Z>]>,
    pub stash: HashMap<usize, B>,
    pub position_map: Box<[usize]>,
}

impl<B: Default + Copy + Eq + Hash, const Z: usize, const N: usize> PathORAM<B, Z, N> {
    pub const L: usize = (N - 1).ilog2() as usize + 2;
    pub const NUM_LEAVES: usize = 1 << (Self::L - 1);
    pub const NUM_BUCKETS: usize = (1 << Self::L) - 1;

    pub fn init() -> eyre::Result<Self> {
        let buckets: Vec<Bucket<B, Z>> = (0..Self::NUM_BUCKETS)
            .map(|_| Bucket::new().expect("failed to create bucket"))
            .collect();

        let mut rng = rand::thread_rng();
        let position_map: Vec<usize> = (0..Self::NUM_BUCKETS)
            .map(|_| rng.gen_range(0..Self::NUM_LEAVES))
            .collect();

        Ok(Self {
            buckets: buckets.try_into().ok().ok_or_eyre("too many buckets")?,
            stash: HashMap::new(),
            position_map: position_map
                .try_into()
                .ok()
                .ok_or_eyre("too many buckets")?,
        })
    }

    fn p(&self, x: usize, l: usize) -> usize {
        (1 << l) - 1 + (x >> (Self::L - l - 1))
    }

    pub fn access(&mut self, a: usize, data: Option<B>) -> eyre::Result<B> {
        let mut rng = rand::thread_rng();
        let x = self.position_map[a];
        self.position_map[a] = rng.gen_range(0..Self::NUM_LEAVES);

        for l in 0..Self::L {
            let p = self.p(x, l);
            for block in self.buckets[p].blocks.iter() {
                if !block.is_empty() {
                    self.stash.insert(block.idx, block.data);
                }
            }
        }

        let prev_data = self.stash.get(&a).copied().unwrap_or_default();
        if let Some(data) = data {
            self.stash.insert(a, data);
        }

        for l in (0..Self::L).rev() {
            let p = self.p(x, l);
            let mut blocks: Vec<Block<B>> = Vec::new();
            let mut bucket_idx = 0;

            let mut to_remove = Vec::new();
            for (idx, data) in self.stash.iter() {
                if bucket_idx >= Z {
                    break;
                }
                if p == self.p(self.position_map[*idx], l) {
                    blocks.push(Block::new(*data, *idx));
                    to_remove.push(*idx);
                    bucket_idx += 1;
                }
            }
            for idx in to_remove {
                self.stash.remove(&idx);
            }

            self.buckets[p] = Bucket::from_blocks(blocks)?;
        }

        Ok(prev_data)
    }
}
