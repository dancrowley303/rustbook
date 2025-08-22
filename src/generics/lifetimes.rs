pub fn run() {
    dangling_reference();
    no_dangling_reference();
    generic_lifetimes_in_functions();
    return_owned_value();
}

fn return_owned_value() {
    fn longest<'a>(_x: &str, _y: &str) -> String {
        let result = String::from("really long string");
        result
    }
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), &string2);
    // note this scope takes ownership of the result
    println!("The longest string is {}", result);

}

fn generic_lifetimes_in_functions() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), &string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    // the below does not work because the borrowed value of string6 does not live long enough
    // let string5 = String::from("long string is long");
    // let result;
    // {
    //     let string6 = String::from("xyz");
    //     result = longest(string5.as_str(), string6.as_str());
    // }
    // println!("The longest string is {}", result);
}

fn no_dangling_reference() {
    let x = 5;
    let r = &x;
    println!("{}", r);
}

fn dangling_reference() {
    // below does not work because the borrowed value does not live long enough
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);
}