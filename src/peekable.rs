

pub struct Peekable<T> {
    inner: Vec<T>,
    index: usize
}



impl<T> Peekable<T> {
    pub fn new() -> Self {
        return Self {
            inner: Vec::new(),
            index: 0
        };
    }

    pub fn from_string(s: String) -> Self where T: From<char> {
        return Self {
            inner: s.chars().map(T::from).collect(),
            index: 0,
        };
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        return Self {
            inner: v,
            index: 0,
        };
    }


    pub fn consume(&mut self, n: usize) {
        self.index += n;
    }

    pub fn peek(&self) -> Option<&T> {
        return self.inner.get(self.index);
    }

    pub fn peek_n(&self, n: usize) -> Option<&[T]> {
        return self.inner.get(self.index..self.index+n);
    }
}
