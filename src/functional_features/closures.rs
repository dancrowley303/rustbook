use std::thread;

pub fn run() {
    shirts();
    inference();
    reference_capture();
    reference_capture_scope();
    borrows_mutably();
    move_on_thread();
    invoke_fnmut();
}

fn invoke_fnmut() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        #[allow(dead_code)]
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:?}", list);

}

fn move_on_thread() {
    let list = vec![1, 2, 3];
    println!("before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    //moved, so the below is invalid
    // println!("after calling closure: {:?}", list);
}

fn borrows_mutably() {
    let mut list = vec![1, 2, 3];
    println!("before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("after calling closure: {:?}", list);
}

fn reference_capture_scope() {
    let only_borrows;
    let list;
    {
        //couldn't define (with let) list here because its lifetime would end before the closure
        list = vec![4, 5, 6];
        only_borrows = || println!("From closure: {:?}", list);
    }
    only_borrows();
    println!("after calling closure: {:?}", list);
}

fn reference_capture() {
    let list = vec![1, 2, 3];
    println!("before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    println!("before calling closure: {:?}", list);
    only_borrows();
    println!("after calling closure: {:?}", list);
}

fn inference() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // the compiler then infers the closure uses the String type, so the following is not valid
    //let n = example_closure(5);
    println!("s is {}", s);
}

fn shirts() {
    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
}