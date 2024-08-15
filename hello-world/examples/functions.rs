fn hello() {
    println!("Hello!");
}

fn sum(a: usize, b: usize) -> usize {
    a + b
}

// public function
pub fn goodbye() {
    println!("Goodbye!");
}

// public function in crate scope
pub(crate) fn foo() -> &'static str {
    "bar"
}

fn a_number() -> usize {
    69
}

fn another_number() -> usize {
    return 420;
}

fn main() {
    hello();

    assert_eq!(sum(69, 420), 69 + 420);

    goodbye();

    println!("foo() returns: {}", foo());

    println!("a_number() returns: {}", a_number());
    println!("another_number() returns: {}", another_number());
}
