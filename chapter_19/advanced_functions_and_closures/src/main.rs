fn main() {
    // Function Pointers
    // Functions coerce to the type fn (with a lowercase f), not to be confused with the Fn closure trait. The fn type is called a function pointer.
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // NEXT
    //
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{:?}", list_of_statuses);
    // Here we create Status::Value instances using each u32 value in the range that map is called on
    // by using the initializer function of Status::Value. Some people prefer this style, and some
    // people prefer to use closures. They compile to the same code, so use whichever style is
    // clearer to you.
    //
    //
    //
    //
    // Returning Closures
    //
    //
    //

    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    // The error references the Sized trait again! Rust doesn’t know how much space it will need to
    // store the closure. We saw a solution to this problem earlier. We can use a trait object:

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    println!("{}", returns_closure()(10))
}
