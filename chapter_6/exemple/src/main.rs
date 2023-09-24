use std::ops::Add;

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant
    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    // NEXT
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    // NEXT
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr4 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // NEXT
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
            println!("{:?}", self)
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // NEXT
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // #[derive(Debug, PartialEq)]
    struct MyOptionI8(Option<i8>);

    impl Add<MyOptionI8> for i8 {
        type Output = Option<i8>;

        fn add(self, rhs: MyOptionI8) -> Option<i8> {
            match rhs.0 {
                Some(x) => Some(self + x),
                None => None,
            }
        }
    }

    let z = MyOptionI8(Some(5));
    let sum = x + z;

    // NEXT
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    // NEXT

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", six.unwrap());

    // NEXT
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        // _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
    fn move_player(num_spaces: u8) {}

    // NEXT
    println!("\n\n How Matches Interact with Ownership \n\n");

    let opt: Option<String> = Some(String::from("Hello world"));

    match opt {
        Some(_) => println!("Some!"),
        // This does not compile
        // Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);

    let opt: Option<String> = Some(String::from("Hello world"));

    // opt became &opt
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);

    // Concise Control Flow with if let
    println!("\n\nConcise Control Flow with if let \n\n");


    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Using if let means less typing, less indentation, and less boilerplate code. 
    // However, you lose the exhaustive checking that match enforces. 
    // Choosing between match and if let depends on what you’re doing 
    // in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Next
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
