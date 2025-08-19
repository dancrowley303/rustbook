struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

pub fn structs() -> () {
    users();
    tuple_structs();
}

fn tuple_structs() {
    let black = Color(0, 0, 0);
    // access values by index
    println!("{} {} {}", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    // black != origin

    // for de-referencing
    let Point(x, y, z) = origin;
    println!("{} {} {}", x, y, z);



}

fn users() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 2
    };
    // can't modify immutable instance
    //user1.active = false;
    println!("{}", &user1.email);

    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 2
    };

    user2.email = String::from("anotheremail@example.com");
    println!("{}", user2.email);

    let user3 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"));
    println!("{}", &user3.email);


    let user4 = User {
        email: String::from("anotheremail@example.com"),
        ..user3 // struct update syntax
    }; // user3 is PARTIALLY moved
    println!("{}", &user4.email);

    // because it is paritally moved you can still access the email value
    println!("{}", &user3.email);

    // but this wouldn't work
    //println!("{}", &user3.username);

    // and the struct can't be used either
    // let user5 = User {
    //     ..user3
    // };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email, // field init shorthand
        sign_in_count: 1
    }
}
