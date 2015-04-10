trait Reducible {
    fn reducible() -> bool;
    fn reduce(&self) -> i64;
}

struct PolishInt {
    value: i64
}

impl Reducible for PolishInt {
    fn reducible() -> bool {
        false
    }

    fn reduce(&self) -> i64 {
        self.value
    }
}

struct PolishOperation<T:Reducible, U:Reducible> {
    left: Box<T>,
    right: Box<U>
}

impl Reducible for PolishOperation {
    fn reducible() -> bool {
        true
    }

    fn reduce(&self) -> i64 {
        self.left.reduce() + self.right.reduce()
    }
}

fn main() {
}
