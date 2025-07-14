


pub trait Location<T> {
    fn advance(&mut self, item: Option<T>);
}







pub struct Peekable<T, L> {
    inner: Vec<T>,
    index: usize,
    pub location: L
}



impl<T: Clone, L: Default + Location<T>> Peekable<T, L> {
    pub fn new() -> Self {
        return Self {
            inner: Vec::new(),
            index: 0,
            location: L::default()
        };
    }

    pub fn from_string(s: String) -> Self where T: From<char> {
        return Self {
            inner: s.chars().map(T::from).collect(),
            index: 0,
            location: L::default()
        };
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        return Self {
            inner: v,
            index: 0,
            location: L::default()
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.index >= self.inner.len();
    }

    pub fn consume(&mut self, n: usize) {
        self.location.advance(self.peek().cloned());
        self.index += n;
    }

    pub fn peek(&self) -> Option<&T> {
        return self.inner.get(self.index);
    }

    pub fn peek_n(&self, n: usize) -> Option<&[T]> {
        return self.inner.get(self.index..self.index+n);
    }

    pub fn next_char_is(&self, c: T) -> bool where T: PartialEq {
        return self.peek().is_some_and(|x| *x == c);
    }
}
