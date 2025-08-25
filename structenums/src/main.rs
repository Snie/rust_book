use std::fmt;

mod option_utils;

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddrOld {
    kind: IpAddrKind,
    address: String,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("Hello {}", user1.username);

    // to change values all the struct must ofc be mut

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let mut u2 = build_user(String::from("john"), String::from("doe"));
    println!(
        "Hello {}, nice to see that you use field init shorthand",
        u2.username
    );
    u2.email = String::from("example2@test.ch");

    let mut i = 10;
    while i > 0 {
        u2.sign_in_count += 1;
        i -= 1;
    }
    login(&mut u2);

    let mut u3 = User {
        email: u2.email,
        ..user1.clone() // here we create u3 with a new email, .. fill all the remaining with values from u1
    };

    user1.sign_in_count += 1;

    println!("{}", user1);
    println!("{}", u3);

    // lets play with toupled structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let top = Point(0, 1, 0);
    let topright = Point(1, 1, 0);

    // and related associated functions
    println!("origin: {}", origin);
    println!("origin len is ofc 0: {}", origin.length());
    println!("top len is: {} and also {}", top.length(), plength(&top));
    println!(
        "distance between origin {}\n    and {}\n    is {}",
        origin,
        topright,
        origin.distance(&topright)
    );

    enum IpAddrKind {
        V4,
        V6,
    }

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    struct IpAddrOld {
        kind: IpAddrKind,
        address: String,
    }

    // now lets see enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrOld {
        kind: four,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddrOld {
        kind: six,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("calling {}", 42)
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // Example usage of option_utils
    println!("Some number: {:?}", option_utils::SOME_NUMBER);
    println!("Some char: {:?}", option_utils::SOME_CHAR);
    println!("Absent number: {:?}", option_utils::ABSENT_NUMBER);

    let sum = option_utils::sum_options(option_utils::SOME_NUMBER, Some(10));
    println!("Sum of options: {:?}", sum);

    //### The match control flow construct
    enum OldCoin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn old_value_in_cents(coin: OldCoin) -> u8 {
        match coin {
            OldCoin::Penny => {
                println!("Lucky Penny");
                1
            }
            OldCoin::Nickel => 5,
            OldCoin::Dime => 10,
            OldCoin::Quarter => 25,
        }
    }

    let onecent = old_value_in_cents(OldCoin::Penny);

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
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Penny);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let six = plus_one(five);
    let none = plus_one(Some(5));

    let dice_roll = 9;

    //### match Catch-all Patterns and the _ Placeholder
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), // this matches everything except the others, other can be used too!
        _ => reroll(), // _ has the same functionality of all-cathing, without using what is catched, nice!
    }

    fn add_fancy_hat() {
        println!("got the hat!");
    }
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        use rand::seq::SliceRandom;
        let locations = ["Mordor", "Moria", "Naboo", "Tatooine"];
        let mut rng = rand::thread_rng();
        let location = locations.choose(&mut rng).unwrap();
        println!("Player moved to {}", location);
    }
    fn reroll() {
        println!("rerolled");
    }

    //### if let
    // see that we can use Optional matching, however we need _ to catch the case it is None
    // however this is quite verbose and can be written better
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // luckily this code that does nothing when max is null can be written shorter using if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
// ##################### END OF MAIN #####################

fn login(user: &mut User) {
    user.sign_in_count += 1;
    println!(
        "User {}, logged in {} times",
        user.username, user.sign_in_count
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn plength(point: &Point) -> f64 {
    ((point.0.pow(2) + point.1.pow(2) + point.2.pow(2)) as f64).sqrt()
}

// Impl are associated functions, in this case to Point
impl Point {
    fn length(&self) -> f64 {
        plength(self)
    }

    fn distance(&self, other: &Point) -> f64 {
        plength(&Point(self.0 - other.0, self.1 - other.1, self.2 - other.2))
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{x={}, y={}, z={}}}", self.0, self.1, self.2)
    }
}

// Implement Display for User
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User {{ username: {}, email: {}, active: {}, sign_in_count: {} }}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            active: self.active,
            username: self.username.clone(),
            email: self.email.clone(),
            sign_in_count: self.sign_in_count,
        }
    }
}
