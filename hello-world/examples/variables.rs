fn main() {
    let x = 42;
    assert_eq!(x, 42);

    let x: usize = 42;
    assert_eq!(x, 42);

    let mut x = 42;

    assert_eq!(x, 42);

    x = 69;

    assert_eq!(x, 69);
}
