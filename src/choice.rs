use std::{ops::Deref, rc::Rc};

pub struct Choice<T: Copy> {
    choices: Rc<Vec<T>>,
    index: usize,
}

impl<T: Copy> Choice<T> {
    pub fn new<'a, I>(vals: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Choice::new_with_initial(vals, 0)
    }

    pub fn new_with_initial<I>(vals: I, index: usize) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let choices: Vec<T> = vals.into_iter().collect();
        assert!(index < choices.len());
        Self {
            choices: Rc::new(choices),
            index,
        }
    }

    pub fn get(&self) -> T {
        self.choices[self.index]
    }

    #[must_use]
    pub fn next(&self) -> Self {
        let mut index = self.index;
        if index + 1 < self.choices.len() {
            index += 1;
        }
        self.make(index)
    }

    #[must_use]
    pub fn prev(&self) -> Self {
        let mut index = self.index;
        if index > 0 {
            index -= 1
        }
        self.make(index)
    }

    #[must_use]
    pub fn circular_next(&self) -> Self {
        self.make((self.index + 1) % self.choices.len())
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn circular_prev(&self) -> Self {
        self.make((self.index + self.choices.len() - 1) % self.choices.len())
    }

    fn make(&self, new_index: usize) -> Self {
        Self {
            choices: Rc::clone(&self.choices),
            index: new_index,
        }
    }
}

impl<T: Copy> Deref for Choice<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.choices[self.index]
    }
}

#[cfg(test)]
mod test {
    use super::Choice;

    #[test]
    fn choice() {
        let c: Choice<i32> = Choice::new([1, 2, 3, 4]);
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

        // Test deref trait.
        assert_eq!(c.trailing_zeros(), 2);
    }
}
