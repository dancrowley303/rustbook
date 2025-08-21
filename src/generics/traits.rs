pub fn run() {
    implementing_trait_on_type();
    default_implementation();
    traits_as_parameters();
    trait_bound();
    conditional_implementation();
}

fn conditional_implementation() {
    use std::fmt::Display;
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let pi32 = Pair::new(3, 4);
    pi32.cmp_display();
    let pf64 = Pair::new(3.14, 4.2);
    pf64.cmp_display();

    struct Foo;

    let f = Foo;
    let f2 = Foo;

    let _foo_pair = Pair::new(f, f2);
    //below won't compile because Foo is not Comparable
    //_foo_pair.cmp_display();
}

fn trait_bound() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        #[allow(dead_code)]
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    //Enforces both args are of the same type
    pub fn notify<T: Summary>(item: &T, item2: &T) {
        println!("Breaking news! {}", item.summarize());
        println!("More Breaking news! {}", item2.summarize());
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    let article2 = NewsArticle {
        headline: String::from("Richmond win the AFL Premiership!"),
        location: String::from("Melbourne, Victoria, Australia"),
        author: String::from("Ralph"),
        content: String::from(
            "The Richmond Tigers are the best football team in the AFL.",
        ),
    };

    notify(&article, &article2);

}

fn traits_as_parameters() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        #[allow(dead_code)]
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    notify(&article);



}

fn default_implementation() {
    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    struct SocialPost {
        pub username: String,
        #[allow(dead_code)]
        pub content: String,
        #[allow(dead_code)]
        pub reply: bool,
        #[allow(dead_code)]
        pub repost: bool,
    }

    impl Summary for SocialPost {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize())
}

fn implementing_trait_on_type() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        #[allow(dead_code)]
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        #[allow(dead_code)]
        pub reply: bool,
        #[allow(dead_code)]
        pub repost: bool,
    }

    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("1 new article: {}", article.summarize());
    println!("1 new social post: {}", post.summarize());

    // note: we can also implement a trait on a type that already exists in the standard library
    // because Summary trait is local
    #[allow(non_local_definitions)]
    impl Summary for Vec<i32> {
        fn summarize(&self) -> String {
            format!("The vector has {} elements", self.len())
        }
    }

    let x = vec![3, 4, 5];
    println!("re: vector x: {}", x.summarize());
}
