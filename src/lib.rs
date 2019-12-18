
#[derive(Debug)]
struct Start {
    start: Nested,
}

#[derive(Debug)]
enum Nested {
    With(Box<Value>),
    Without,
}

#[derive(Debug)]
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

#[test]
fn find_deepest2() {
    let mut depth = 2;
    loop {
        println!("{}", depth);
        {
            let _item = dbg!(Start::nested(depth));
        }
        depth += 1;
    }
}

#[derive(Debug)]
struct Start3 {
    start: Nested3,
}

impl Start3 {
    fn nested(n: usize) -> Self {
        Self {
            start: Nested3::nested(n),
        }
    }
}

#[derive(Debug)]
enum Nested3 {
    With(Box<Value3>),
    Other(Box<Other>),
    Without,
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
                let o = Other { data: v, n: i };
                inner = Self::Other(Box::new(o));
            }
        }
        inner
    }
}

#[derive(Debug)]
struct Value3 {
    data: usize,
    next: Nested3,
}

#[derive(Debug)]
struct Other {
    data: Value3,
    n: usize,
}

#[test]
fn find_deepest3() {
    let mut depth = 2;
    loop {
        println!("{}", depth);
        {
            let _item = dbg!(Start3::nested(depth));
        }
        depth += 1;
    }
}

#[derive(Debug)]
struct StartLarge {
    start: NestedLarge,
}

impl StartLarge {
    fn nested(n: usize) -> Self {
        Self {
            start: NestedLarge::nested(n),
        }
    }
}

#[derive(Debug)]
enum NestedLarge {
    With(Box<ValueLarge>),
    Other(Box<OtherLarge>),
    Without,
}

impl NestedLarge {
    fn nested(n: usize) -> Self {
        let mut inner = Self::Without;
        for i in 0..n {
            let v = ValueLarge {
                data: i,
                next: inner,
            };
            if i % 2 == 0 {
                inner = Self::With(Box::new(v));
            } else {
                let o = OtherLarge { data: v, n: i };
                inner = Self::Other(Box::new(o));
            }
        }
        inner
    }
}

#[derive(Debug)]
struct ValueLarge {
    data: usize,
    next: NestedLarge,
}

#[derive(Debug)]
struct OtherLarge {
    data: ValueLarge,
    n: usize,
}

#[test]
fn find_deepest_large() {
    let mut depth = 2;
    loop {
        println!("{}", depth);
        {
            let _item = dbg!(StartLarge::nested(depth));
        }
        depth += 1;
    }
}

#[derive(Debug)]
struct StartAgain {
    start: NestedAgain,
}

impl StartAgain {
    fn nested(n: usize) -> Self {
        Self {
            start: NestedAgain::nested(n),
        }
    }
}

#[derive(Debug)]
enum NestedAgain {
    With(Box<ValueAgain>),
    Other(Box<OtherAgain>),
    Without,
}

impl NestedAgain {
    fn nested(n: usize) -> Self {
        let mut inner = Self::Without;
        for i in 0..n {
            let v = ValueAgain {
                data: std::usize::MAX - i,
                next: inner,
            };
            if i % 2 == 0 {
                let o = OtherAgain {
                    data: v,
                    next: Self::nested(i),
                };
                inner = Self::Other(Box::new(o));
            } else {
                inner = Self::With(Box::new(v));
            }
        }
        inner
    }
}

#[derive(Debug)]
struct ValueAgain {
    data: usize,
    next: NestedAgain,
}

#[derive(Debug)]
struct OtherAgain {
    data: ValueAgain,
    next: NestedAgain,
}


#[test]
fn find_deepest_again() {
    let mut depth = 2;
    loop {
        println!("{}", depth);
        {
            let _item = dbg!(StartAgain::nested(depth));
        }
        depth += 1;
    }
}