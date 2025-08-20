pub fn run() {
    coin_match();
    if_let();
    let_else();
}

fn let_else() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1863,
                UsState::Alaska => year >= 1965,
            }
        }
    }
    fn describe_state_quarter(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new"))
        }
    }

    println!("{:?}", describe_state_quarter(Coin::Quarter(UsState::Alabama)));
    println!("{:?}", describe_state_quarter(Coin::Quarter(UsState::Alaska)));
    println!("{:?}", describe_state_quarter(Coin::Nickel));
    println!("{:?}", describe_state_quarter(Coin::Penny));
    println!("{:?}", describe_state_quarter(Coin::Dime));
}

fn if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max value is {}", max);
    }

    let config_none: Option<u8> = None;
    if let Some(max) = config_none {
        println!("The max value is {}", max);
    } else {
        println!("No max value");
    }

}

fn coin_match() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let sum = value_in_cents(Coin::Quarter) + value_in_cents(Coin::Nickel) + value_in_cents(Coin::Dime) + value_in_cents(Coin::Penny);
    println!("The sum of the coins (Quarter, Nickel, Dime, Penny) value is {}", sum);
}
