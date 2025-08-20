pub fn run() {
    create();
    indexing();
    holding_references();
    iterating();
    enum_vectors();
}

fn enum_vectors() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for c in &row {
        match c {
            SpreadsheetCell::Int(value) => println!("{} is an int", value),
            SpreadsheetCell::Float(value) => println!("{} is a float", value),
            SpreadsheetCell::Text(value) => println!("{} is a text", value),
        }
    }
}

fn iterating() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn holding_references() {
    //note that this has an error because can't borrow v as mutable because it is already borrowed
    // as immutable
    #[allow(unused_mut)]
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //below causes a panic
    //v.push(6);
    println!("The first element is {}", first);

    //but this does not cause a panic because first is not used again
    let mut v = vec![1, 2, 3, 4, 5];
    #[allow(unused)]
    let first = &v[0];
    v.push(6);

}

fn indexing() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let sixth: Option<&i32> = v.get(6);
    match sixth {
        Some(third) => println!("The sixth element is {}", third),
        None => println!("There is no sixth element."),
    }

    //below causes a panic - only use when an error is acceptable
    //let does_not_exist = &v[100];

    //get access is safer because it returns an Option
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(element) => println!("The 100th element is {}", element),
        None => println!("There is no 100th element."),
    }
}

fn create() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    //inferred type annotation
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);
}