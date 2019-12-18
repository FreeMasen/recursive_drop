
struct Start {
    start: Nested
}
enum Nested {
    With(Box<Value>),
    Without
}

struct Value {
    data: usize,
    next: Nested,
}

impl Start {
    fn nested(n: usize) -> Self {
        Self {
            start: Nested::nested(n),
        }
    }
}

impl Nested {
    fn nested(n: usize) -> Self {
        let mut inner = Self::Without;
        for i in 0..n {
            let v = Value {
                data: i,
                next: inner,
            };
            inner = Self::With(Box::new(v));
        }
        inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_deepest() {
        for i in 0..std::usize::MAX {
            dbg!(i);
            {
                let _thing = Start::nested(i);
            }
        }
    }
}
