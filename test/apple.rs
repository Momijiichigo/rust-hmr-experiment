pub struct A<S> {
    s: S,
}

impl<S> A<S> {
    pub fn new(s: S) -> Self {
        Self { s }
    }
}

pub fn unko<S>(s: S) -> usize {
    std::mem::size_of::<A<S>>()
}
