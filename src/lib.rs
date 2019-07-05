pub struct SelectingPermutation<T>
where
    T: Ord + Clone,
{
    original: Vec<T>,
    possition: Vec<usize>,
    len: usize,
    started: bool,
}

impl<T> SelectingPermutation<T>
where
    T: Ord + Clone,
{
    pub fn new(mut original: Vec<T>, len: usize) -> Self {
        match original.len() > len && len >= 1 {
            true => {
                original.sort();
                Self {
                    original,
                    possition: (0..len).collect(),
                    len,
                    started: false,
                }
            }
            false => panic!("the length has to be smaller then the datasets len"),
        }
    }

    #[inline]
    fn insert(&self, col: &mut Vec<T>) {
        col.clear();
        self.possition
            .iter()
            .enumerate()
            .for_each(|(p, n)| col.insert(p, self.original[*n].clone()))
    }

    pub fn next_perm(&mut self, mut col: &mut Vec<T>) -> bool {
        let ans = match self.started {
            false => {
                self.started = true;
                self.insert(&mut col);
                true
            }
            true => {
                let mut change = false;
                let org_len = self.original.len();
                if self.possition[self.len - 1] != org_len - 1 {
                    for i in 2..=self.len {
                        if self.possition[self.len - i] != org_len - i {
                            let val = &self.original[self.possition[self.len - i]];
                            for var in iterable {}
                            break true;
                        }
                    }
                } else {
                    self.possition[self.len - 1] += 1;
                    true
                }
            }
        };
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
