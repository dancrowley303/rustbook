pub fn run() {
    message();
    option();
}

fn option() {
    let x = 5;
    let y: Option<i32> = Some(5);

    // can't do this because i32 and Option<i32> are different types
    // let sum = x + y;

    let sum = x + match y {
        Some(num) => num,
        None => 0, // explicitly stating the numerical value if None
    };

    println!("The sum is {}", sum);

    // quicker version of above
    let sum2 = x + y.unwrap_or_else(|| 0);
    println!("The sum is {}", sum2);


}

fn message() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Quit"),
                Message::Move { x, y } => println!("Move to ({}, {})", x, y),
                Message::Write(text) => println!("Write: {}", text),
                Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    let m2 = Message::ChangeColor(0, 160, 255);
    let m3 = Message::Move { x: 3, y: 5 };
    let m4 = Message::Quit;

    m.call();
    m2.call();
    m3.call();
    m4.call();

}