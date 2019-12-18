
struct Start {
    start: Nested,
}

struct Start3 {
    start: Nested3,
}
enum Nested {
    With(Box<Value>),
    Without
}

enum Nested3 {
    With(Box<Value3>),
    Other(Box<Other>),
    Without
}

struct Value {
    data: usize,
    next: Nested,
}

struct Value3 {
    data: usize,
    next: Nested3,
}

struct Other {
    data: Value3,
    n: usize,
}

impl Start {
    fn nested(n: usize) -> Self {
        Self {
            start: Nested::nested(n),
        }
    }
}

impl Start3 {
    fn nested(n: usize) -> Self {
        Self {
            start: Nested3::nested(n),
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

impl Nested3 {
    fn nested(n: usize) -> Self {
        let mut inner = Self::Without;
        for i in 0..n {
            let v = Value3 {
                data: i,
                next: inner,
            };
            if i % 2 == 0 {
                inner = Self::With(Box::new(v));
            } else {
                let o = Other {
                    data: v,
                    n: i,
                };
                inner = Self::Other(Box::new(o));
            }
        }
        inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_deepest2() {
        // mbp i9 6 core 16gb ram
        // 18702 max
        for i in 0..std::usize::MAX {
            dbg!(i);
            {
                let _thing = Start::nested(i);
            }
        }
    }
    #[test]
    fn find_deepest3() {
        //16364
        for i in 0..std::usize::MAX {
            dbg!(i);
            {
                let _thing = Start3::nested(i);
            }
        }
    }
}
