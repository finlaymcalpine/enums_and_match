// unlike the multiple fields of a struct, an enum has multiple variants (although they can have values)
// one and only one of the possible variants applies to the instance

// an example with IP addresses: they can be one of two types, IPV4 and IPV6
enum IpAddr {
    V4(u8, u8, u8, u8), // different variants in the enum can have differing types/numbers of associated values
    V6(String), // we could also put structs inside the enum, as is the case with the std library IpAddr
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

// the Option enum is a std library defined type for use when a value could be something or nothing. E.g. the first item in a non-empty
// vs an empty list. this enum allows for handling of both cases. Since Rust doesn't have the Null concept found in other languages.
// Thus, the Option<T> enum is used in place to handle scenarios where a value could be nothing (i.e. None).
// MORE TO BE ADDED...

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    More,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // we're holding data inside the Coin enum of Quarter variant, as only quarters had state designs on them
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        }
    }
}


fn main() {
    
    // some instances of enum variants
    let home = IpAddr::V4(127, 0, 0, 1);
    let work = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();

    let coin_value = value_in_cents(Coin::Quarter(UsState::Alaska)); // No string here, because the Alaska data is an enum, not a string
    println!("coin_value is {} cents.", coin_value);
}
