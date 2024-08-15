fn if_statement(x: i32) {
    if x == 42 {
        println!("x is 42");
    } else {
        println!("x is not 42");
    }
}

fn iterate() {
    for x in 0..100 {
        println!("{}", x);
    }
}

fn endless_loop() {
    loop {
        println!("running loop");

        break;
    }

    println!("used break to stop endless loop");
}

fn while_loop() {
    let mut running = true;

    while running {
        println!("running loop.. setting running to false");
        running = false;
    }

    println!("loop stopped");
}

fn main() {
    if_statement(42);
    if_statement(420);

    iterate();

    endless_loop();
    while_loop();
}
