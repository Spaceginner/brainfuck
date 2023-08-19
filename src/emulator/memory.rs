use std::ops::{Index, IndexMut};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Memory {
    positive: Vec<u8>,
    negative: Vec<u8>,
}


impl Default for Memory {
    fn default() -> Self {
        Self {
            positive: vec![0],
            negative: vec![0],
        }
    }
}

impl Memory {
    fn ensure(&mut self, index: isize) {
        match index.cmp(&0) {
            Ordering::Equal | 
            Ordering::Greater => {
                let delta = -(self.positive.len() as isize - 1 - index).min(0) as usize;
                self.positive.reserve(delta);
                for _ in 0..delta {
                    self.positive.push(0);
                };
            },
            Ordering::Less => {
                let delta = -(self.negative.len() as isize - 1 + index).min(0) as usize;
                self.negative.reserve(delta);
                for _ in 0..delta {
                    self.negative.push(0)
                };
            },
        };
    }
}

impl Index<isize> for Memory {
    type Output = u8;

    fn index(&self, index: isize) -> &Self::Output {
        match index.cmp(&0) {
            Ordering::Equal | 
            Ordering::Greater  => {
                if self.positive.len() < index as usize + 1 {
                    &0
                } else {
                    &self.positive[index as usize]
                }
            },
            Ordering::Less => {
                if self.negative.len() < -index as usize - 1 {
                    &0
                } else {
                    &self.negative[-(index + 1) as usize]
                }
            },
        }
    }
}

impl IndexMut<isize> for Memory {
    fn index_mut(&mut self, index: isize) -> &mut u8 {
        self.ensure(index);

        match index.cmp(&0) {
            Ordering::Equal | 
            Ordering::Greater => &mut self.positive[index as usize],
            Ordering::Less => &mut self.negative[-(index + 1) as usize],
        }
    }
}
