pub fn run() {
    ownership();
    ret_values();
    ret_multiple();
    borrow();
    borrow_cant_modify();
    borrow_mut_can_modify();
    notes_on_data_races();
    dangling_references();
}

fn dangling_references() {
    // note that dangle won't compile so can't do this
    // let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
    print!("{reference_to_something}");
}

fn no_dangle() -> String {
    let s = String::from("hello from no_dangle");
    s
} // ownership is moved out

// fn dangle() -> &String {
//     let s = String::from("hello from dangle");
//     &s // we returned a reference to the String, s
// } // but here, s goes out of scope and is dropped!

fn notes_on_data_races() {
    //note that this would not compile
    let mut s = String::from("hello from notes_on_data_races");
    let r1 = &mut s;
    // the compiler stops the line below
    //let r2 = &mut s;

    // an exclusive mutable borrow ensures data races can't occur

    //println!("{r1}, {r2}");

    println!("{r1}");
}

fn borrow_mut_can_modify() {
    let mut s = String::from("hello from borrow_mut_can_modify");
    change_mut(&mut s);
    println!("{s}");
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

fn borrow_cant_modify() {
    let s = String::from("hello from borrow_cant_modify");
    change(&s);
}

fn change(some_string: &String) {
    // below causes a compiler error because can't borrow immutable
    //some_string.push_str(", world");
    println!("{some_string}");
}

fn borrow() {
    let s1 = String::from("hello from borrow");

    let len = calculate_length2(&s1);        //&s1 is the reference only to s1
    //it is different from the contents of s1 - which would be a pointer, a len, and a capacity
    // so after returning, s1 remains
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length2(s: &String) -> usize { // s is a reference to a string
    s.len()
} // here, s goes out of scope. But because s does not have ownership of what it refers to,
// the String is not dropped (e.g - this is BORROWING)

fn ret_multiple() {
    let s1 = String::from("hello from ret_multiple");

    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a string
    (s, length)
}

fn ret_values() {
    let s1 = gives_ownership();     //fn moves its return value into s1
    println!("{s1}");
    let s2 = String::from("hello from ret_values"); //s2 comes into scope
    println!("{s2}");

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    // from here, s2 was moved
    //so you could do
    println!("{s3}");
    // but you can't do
    // println!("{s2}");

    // NOTE s2 was not 'dropped'. Because of the move, the original value on the heap is preserved. This is
    // efficient because the heap is reused by the return value from the function. So drop on s3 only needs to
    // be done once.
} // here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 also goes out of scope and is dropped


// This function takes a String and returns a string
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn gives_ownership() -> String {    // gives_ownership will move its return value
    // into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                                     // some_string is returned and MOVED out to calling function
}

fn ownership() {
    let s = String::from("hello from ownership");       // s comes into scope
    takes_ownership(s);                         // s's value moves into the function
    // and is no longer valid here

    let x = 5;                             // x comes into scope
    makes_copy(x);                              // x's value is copied not MOVED into the function
    // so it's ok to use after
    //println!("{x}"); would work
    //println!("{s}"); would NOT work
}

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{some_integer}")
}   // here, some_integer goes out of scope. nothing special happens (eg drop)

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}")
} // some_string goes out of scope and 'drop' is called