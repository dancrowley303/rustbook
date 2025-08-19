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
        Arkansas,
        California,
        Delaware,
        Florida,
        Georgia,
        Hawaii,
        Idaho,
        Illinois,
        Indiana,
        Iowa,
        Kansas,
        Kentucky,
        Louisiana,
        Maine,
        Maryland,
        Massachusetts,
        Michigan,
        Minnesota,
        Mississippi,
        Missouri,
        Montana,
        Nebraska,
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
                UsState::Arkansas => year >= 1965,
                UsState::California => year >= 1863,
                UsState::Delaware => year >= 1776,
                UsState::Florida => year >= 1787,
                UsState::Georgia => year >= 1789,
                UsState::Hawaii => year >= 1965,
                UsState::Idaho => year >= 1965,
                UsState::Illinois => year >= 1789,
                UsState::Indiana => year >= 1789,
                UsState::Iowa => year >= 1863,
                UsState::Kansas => year >= 1863,
                UsState::Kentucky => year >= 1789,
                UsState::Louisiana => year >= 1789,
                UsState::Maine => year >= 1789,
                UsState::Maryland => year >= 1789,
                UsState::Massachusetts => year >= 1789,
                UsState::Michigan => year >= 1863,
                UsState::Minnesota => year >= 1863,
                UsState::Mississippi => year >= 1863,
                UsState::Missouri => year >= 1863,
                UsState::Montana => year >= 1863,
                UsState::Nebraska => year >= 1863,
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
