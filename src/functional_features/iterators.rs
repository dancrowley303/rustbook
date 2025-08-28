pub fn run() {
    creating_iterator();
    calling_next_on_iterator();
    iterating_consumers();
    iterating_producers();
    closure_capturing_environment();
}

fn closure_capturing_environment() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker") },
        Shoe {size: 13, style: String::from("sandal") },
        Shoe {size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    assert_eq!(in_my_size, vec![
        Shoe {size: 10, style: String::from("sneaker") },
        Shoe {size: 10, style: String::from("boot") }
    ]);
}

fn iterating_producers() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    //note v1_iter is still moved
    let total: i32 = v1.iter().map(|x| x + 2).sum();
    assert_eq!(total, 12);
}

fn iterating_consumers() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    //v1_iter has been moved

}

fn calling_next_on_iterator() {
    let v1 = vec![1, 2, 3];
    // note that next consumes the iterator so the variable must be mutable when calling next
    // directly - although the for loop handles this automatically
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn creating_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter  = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // iterator was moved so the following line will not work
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    let v2 = vec![4, 5, 6];
    for val in v2 {
        println!("Got: {}", val);
    }

    //still would not work since v2 was moved
    // for val in v2 {
    //     println!("Again Got: {}", val);
    // }

    // explicitly borrowing the iterator works multiple times
    let v3 = vec![7, 8, 9];
    for val in &v3 {
        println!("Got: {}", val);
    }
    for val in &v3 {
        println!("Again Got: {}", val);
    }

}