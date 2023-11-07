use std::{thread, time::Duration};

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

    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

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

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;

    // NEXT

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    //NEXT
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // If you want to force the closure to take ownership of the values it uses in the environment
    // even though the body of the closure doesn’t strictly need ownership, you can use the move
    // keyword before the parameter list.
    // let mut borrows_mutably = move || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    //NEXT
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // NEXT closure with fnMut

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // NEXT
    // Closures Must Name Captured Lifetimes

    // fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    //     move || s_ref.to_string()
    // }

    // we need to tell Rust that the closure returned from make_a_cloner must not live longer than s_ref
    // fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    //     move || s_ref.to_string()
    // }

    //we can use the lifetime elision rules to make the function type more concise.
    //We can remove the <'a> generic so long as we keep an indicator that the returned closure depends on some lifetime, like this:
    fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {
        move || s_ref.to_string()
    }

    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    // drop(s_own);
    cloner();

    fn for_each_mut<T, F: FnMut(&mut T)>(v: &mut Vec<T>, mut f: F) {
        for x in v.iter_mut() {
            f(x);
        }
    }
}
