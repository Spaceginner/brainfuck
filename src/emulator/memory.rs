use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct Memory(Vec<u8>);

impl Memory {
    fn ensure(&mut self, index: usize) {
        while self.0.len() < index + 1 {
            self.0.push(0);
        }
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if index + 1 > self.0.len() {
            &0
        } else {
            &self.0[index]
        }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        self.ensure(index);
        &mut self.0[index]
    }
}
