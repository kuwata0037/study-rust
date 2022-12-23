#[derive(Debug)]
pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> Default for ToyVec<T> {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl<T: Default> ToyVec<T> {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(size),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, element: T) {
        if self.len() == self.capacity() {
            self.glow();
        }
        self.elements[self.len()] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    // ライフタイムは以下のようにwhere節でも使える
    pub fn get_or<'a, 'b>(&'a self, index: usize, default: &'b T) -> &'a T
    where
        'b: 'a,
    {
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
    }

    fn glow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    // 可変の借用(&mut)経由では値の所有権を一方的に奪うことはできないが、所有権を交換することなら可能
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::take(&mut self.elements[self.len()]);
            Some(elem)
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            elements: &self.elements,
            len: self.len(),
            pos: 0,
        }
    }
}

pub struct Iter<'a, T> {
    elements: &'a [T],
    len: usize,
    pos: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toy_vec01() {
        let mut v = ToyVec::default();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());
        let e = v.get(1);
        assert_eq!(e, Some(&"Budgerigar".to_string()));
    }

    #[test]
    fn test_vec03() {
        let mut v = ToyVec::default();
        v.push("Java Finch".to_string());
        v.push("Budgerigar".to_string());

        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
        v.push("Canary".to_string());
    }
}
