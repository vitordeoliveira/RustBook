use std::slice;

fn main() {
    // Unsafe Superpowers
    // Dereferencing a Raw Pointer

    // Raw Pointers
    // Different from references and smart pointers, raw pointers:
    // Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // Aren’t guaranteed to point to valid memory
    // Are allowed to be null
    // Don’t implement any automatic cleanup

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Notice that we don’t include the unsafe keyword in this code. We can create raw pointers in
    // safe code; we just can’t dereference raw pointers outside an unsafe block

    let address = 0x012345usize;
    let r = address as *const i32;

    // Note we create a mut from a non-mut and we can change his value. That is dangerous (unsafe)
    unsafe {
        *r2 = 10;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("r is: {}", *r);
        println!("1: num is: {}", num);
    }

    println!("2: num is: {}", num);

    // Calling an Unsafe Function or Method
    unsafe fn dangerous() -> i32 {
        5
    }

    let mut num = 10;
    unsafe {
        num = dangerous();
    }

    println!("3: num is: {}", num);

    // Creating a Safe Abstraction over Unsafe Code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // it is possible to create a safe function with unsafe return (we know that function is safe
    // so we garantee that will work)
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // Using extern Functions to Call External Code
    // Sometimes, your Rust code might need to interact with code written in another language. For
    // this, Rust has the keyword extern that facilitates the creation and use of a Foreign
    // Function Interface (FFI).
    // Functions declared within extern blocks are always unsafe to call from Rust code. The reason
    // is that other languages don’t enforce Rust’s rules and guarantees, and Rust can’t check them,
    // so responsibility falls on the programmer to ensure safety.

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // This is the way to call rust from another c program
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // Accessing or Modifying a Mutable Static Variable
    // In this book, we’ve not yet talked about global variables, which Rust does support but can be
    // problematic with Rust’s ownership rules. If two threads are accessing the same mutable
    // global variable, it can cause a data race.
    //
    // In Rust, global variables are called static variables
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    // Accessing and modifying mutable static variables is unsafe.
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(10);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // Implementing an Unsafe Trait
    // If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and
    // we want to mark that type as Send or Sync, we must use unsafe.
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // Accessing Fields of a Union
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 };
    println!("{f}")
}
