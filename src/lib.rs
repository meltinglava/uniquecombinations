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
                original.sort_unstable();
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
        match self.started {
            false => { // first pass throught
                self.started = true;
                self.insert(&mut col);
                true
            }
            true => {
                let org_len = self.original.len();
                // check if we cant bump the back number

                if self.original[self.possition[self.len - 1]] == self.original[org_len - 1] {
                    // locate the number closest behind that needs to be bumped
                    for i in 2..=self.len {
                        if self.original[self.possition[self.len - i]] < self.original[org_len - i] {
                            //find the value of the
                            let lastpos = self.possition[self.len - i];
                            let val = &self.original[lastpos];
                            for j in lastpos+1..org_len {
                                if *val < self.original[j] {
                                    for k in 0..i {
                                        self.possition[self.len - i + k] = j + k;
                                    }
                                    self.insert(&mut col);
                                    return true;
                                }
                            }
                        }
                    }
                    return false;
                } else {
                    let mut i = self.possition[self.len - 1];
                    let current = &self.original[i];
                    let mut next = current;
                    while current == next {
                        i += 1;
                        next = &self.original[i];
                    }
                    self.possition[self.len - 1] = i;
                    self.insert(&mut col);
                    true
                }
            }
        }
    }
}

impl<T> Iterator for SelectingPermutation<T>
where
    T: Ord + Clone
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vals = Vec::with_capacity(self.len);
        match self.next_perm(&mut vals) {
            false => None,
            true => Some(vals),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals() {
        assert!(SelectingPermutation::new(vec![2, 2, 2], 2).next().unwrap() == vec![2, 2])
    }

    #[test]
    fn t_123() {
        assert!(
            dbg!(SelectingPermutation::new(vec![1, 2, 3], 2).take(10).collect::<Vec<_>>()) == vec![vec![1, 2], vec![1, 3], vec![2, 3]])
    }

    #[test]
    fn complicated() {
        let actual: Vec<_> = SelectingPermutation::new(vec![1, 2, 2, 3, 4], 3).collect();
        let expected = vec![
            vec![1, 2, 2],
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 2, 3],
            vec![2, 2, 4],
            vec![2, 3, 4],
        ];
        assert!(actual == expected)
    }
}
