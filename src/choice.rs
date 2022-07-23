use std::{ops::Deref, rc::Rc};

#[derive(Clone, PartialEq, Eq)]
pub struct ChoiceSet<T: Copy>(Rc<Vec<T>>);

impl<T: Copy> ChoiceSet<T> {
    pub fn new<I>(vals: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(Rc::new(vals.into_iter().collect()))
    }

    pub fn by_index(&self, index: usize) -> Choice<T> {
        assert!(index < self.0.len());
        Choice {
            choice_set: self.clone(),
            index,
        }
    }

    #[allow(dead_code)]
    pub fn by_value(&self, value: T) -> Choice<T>
    where
        T: PartialEq,
    {
        let index = self
            .iter()
            .position(|v| *v == value)
            .expect("value not in choice set");

        self.by_index(index)
    }
}

impl<T: Copy> Deref for ChoiceSet<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Choice<T: Copy> {
    choice_set: ChoiceSet<T>,
    index: usize,
}

impl<T: Copy> Choice<T> {
    pub fn get(&self) -> T {
        self.choice_set[self.index]
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn choice_set(&self) -> ChoiceSet<T> {
        self.choice_set.clone()
    }

    #[must_use]
    pub fn next(&self) -> Self {
        let mut index = self.index;
        if index + 1 < self.choice_set.len() {
            index += 1;
        }
        self.choice_set.by_index(index)
    }

    #[must_use]
    pub fn prev(&self) -> Self {
        let mut index = self.index;
        if index > 0 {
            index -= 1
        }
        self.choice_set.by_index(index)
    }

    #[must_use]
    pub fn circular_next(&self) -> Self {
        let index = (self.index + 1) % self.choice_set.len();
        self.choice_set.by_index(index)
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn circular_prev(&self) -> Self {
        let index = (self.index + self.choice_set.len() - 1) % self.choice_set.len();
        self.choice_set.by_index(index)
    }
}

impl<T: Copy> Deref for Choice<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.choice_set[self.index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn choice() {
        let cs: ChoiceSet<i32> = ChoiceSet::new([1, 2, 3, 4]);
        let c: Choice<i32> = cs.by_index(0);
        assert_eq!(c.get(), 1);
        let c = c.next();
        assert_eq!(c.get(), 2);
        let c = c.prev();
        assert_eq!(c.get(), 1);
        let c = c.prev();
        let c = c.prev();
        assert_eq!(c.get(), 1);
        let c = c.next();
        let c = c.next();
        let c = c.next();
        let c = c.next();
        assert_eq!(c.get(), 4);
        let c = c.circular_next();
        assert_eq!(c.get(), 1);
        let c = c.circular_prev();
        assert_eq!(c.get(), 4);

        // Test by_index.
        let c = cs.by_index(3);
        assert_eq!(c.get(), 4);

        // Test by_value.
        let c = cs.by_value(2);
        assert_eq!(c.get(), 2);

        // Test deref trait.
        let c = cs.by_value(4);
        assert_eq!(c.trailing_zeros(), 2);
    }
}
