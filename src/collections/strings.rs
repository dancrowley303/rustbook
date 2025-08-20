pub fn run() {
    new_string();
    update_strings();
    concatenation();
}

fn concatenation() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{}", s3);

    let s4 = String::from("hello, ");
    let mut s5 = String::from("world!");
    let s6 = s4 + &s5;
    println!("{}", s6);
    s5.push_str(", again!");
    //note that s6 is not affected by this - deref coercion on s5 in action
    println!("{}", s6);
}

fn update_strings() {
    let mut s = String::new();
    s.push('a');
    s.push('b');
    s.push('c');
    println!("{}", s);

    let mut s2 = String::from("foo");
    s2.push_str("bar");
    println!("{}", s2);

    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    println!("{}", s4);
}

fn new_string() {
    //longwinded but correct
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    //use a literal
    let s = "initial contents".to_string();
    println!("{}", s);

    //equivalent to the above
    let s = String::from("initial contents");
    println!("{}", s);
}