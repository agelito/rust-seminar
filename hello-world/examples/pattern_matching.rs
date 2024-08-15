fn match_optional(optional: Option<i32>) {
    match optional {
        Some(42) => {
            println!("matched 42");
        }
        Some(_) => {
            println!("matched another number");
        }
        None => {
            println!("there is no number");
        }
    }
}

fn match_skip(n: i32) {
    match n {
        69 => {
            println!("niiceee");
        }
        _ => {
            println!("meh")
        }
    }
}

fn main() {
    match_optional(Some(42));
    match_optional(Some(69));
    match_optional(None);

    match_skip(69);
    match_skip(1);
}
