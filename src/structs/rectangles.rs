pub fn run() {
    no_struct();
    with_tuple();
    with_structs();
    mut_binding();
    with_traits();
    with_methods();
    more_method_params();
    associated_functions();
}

fn associated_functions() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {

        fn area(&self) -> u32 {
            self.width * self.height
        }

        // in other languages a class method
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let sq = Rectangle::square(3);

    println!("The area of the square is {} square pixels.", sq.area());

}

fn more_method_params() {

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn with_methods() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn width(&self) -> bool {
            self.width > 0
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

fn with_traits() {

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    // more structure
    println!("rect1 is {rect1:#?}");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30  * scale),
        height: 50,
    };
    dbg!(&rect2);
}

// this is not from the book, but I wanted to test mutable bindings
// intuitively I thought fn(arg & mut T) would mean the reference is
// mutable, but actually Rust uses (mut arg T) to mean the reference is
// mutable.
// Also, it confirms that fn(arg & mut T) means the type instance itself
// is mutable.
fn mut_binding() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn two_areas(mut rectangle: & Rectangle) -> u32 {
        let old_area = rectangle.width * rectangle.height;
        // changing the binding of the rectangle parameter
        rectangle = & Rectangle {width: 30, height: 50};
        rectangle.width * rectangle.height + old_area
    }

    let rect1  = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of two rectangles is {} square pixels.", two_areas(&rect1));
}

fn with_structs() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rect1  = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the rectangle is {} square pixels.", area(&rect1));
}

fn with_tuple() {
    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area(rect1));
}

fn no_struct() {

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}

