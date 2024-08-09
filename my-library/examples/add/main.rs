use my_library;

fn main() {
    let result = my_library::add(2, 5);

    assert_eq!(result, 7);

    println!("{}", result);
}
