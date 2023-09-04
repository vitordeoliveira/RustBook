use std::vec;

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn read_box_from_borrowing(y: &Box<[i32; 1_000_000]>) {
    println!("{}", y[0])
}

fn _read_box_from_moving(y: Box<[i32; 1_000_000]>) {
    println!("{}", y[0])
}

fn _read_box_from_unboxing(y: [i32; 1_000_000]) {
    println!("{}", y[0])
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn _move_a_box(b: Box<i32>) {
    // This space intentionally left blank
}

fn main() {

    println!("\n\nReferences and Borrowing 4.2\n\n");
    let x = true;
    read(x);

    // Next exemple

    let a = Box::new([0; 1_000_000]);

    let _b = a;

    // _read_box_from_moving(_b);
    read_box_from_borrowing(&_b);
    _read_box_from_unboxing(*_b);

    // free(_b);

    println!("{}", _b[0]);

    // This is not valid
    // println!("{}", a[0]);

    // Next exemple
    // Collections Use Boxes

    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");

    // let b = Box::new(0);
    // let b2 = b;
    // println!("{}", b);
    // move_a_box(b2);

    // Next exemple
    // References and Borrowing
    println!("References and Borrowing");

    fn greet(g1: String, g2: String) -> (String, String) {
        println!("{} {}!", g1, g2);
        (g1, g2)
    }

    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);

    println!("{s}");

    // References Are Non-Owning Pointers
    println!("References Are Non-Owning Pointers");
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet_2(&m1, &m2); // note the ampersands
    let s = format!("{} {}!", m1, m2);

    fn greet_2(g1: &String, g2: &String) {
        // note the ampersands
        println!("{} {}!", g1, g2);
    }
    println!("{s}");

    // Next exemple
    // Dereferencing a Pointer Accesses Its Data
    println!("Dereferencing a Pointer Accesses Its Data");

    let mut x: Box<i32> = Box::new(1);
    let _a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let _b: i32 = **r1; // two dereferences get us to the heap value

    let r2: &i32 = &*x; // r2 points to the heap value directly
                        // let r2: Box<i32> = x; // r2 points to the heap value directly

    let _c: i32 = *r2; // so only one dereference is needed to read it

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);

    // Next Exemple
    // Rust Avoids Simultaneous Aliasing and Mutation
    println!("Rust Avoids Simultaneous Aliasing and Mutation");

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);
    v.push(4);
    println!("Third element is {}", v[3]);
    // Pointer Safety Principle: data should never be aliased and mutated at the same time.

    // Next exemple
    // Mutable References Provide Unique and Non-Owning Access to Data
    println!("Mutable References Provide Unique and Non-Owning Access to Data");

    // The mechanism for this is mutable references (also called unique references).]
    // Here's a simple example of a mutable reference with the accompanying permissions changes:

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", num);
    println!("Vector is now {:?}", v);

    // Here we can access the value of v[2] but not changing v itself
    let v: Vec<i32> = vec![1, 2, 3];
    let mut num: i32 = *&v[2];
    num += 1;
    println!("Third element is {}", num);
    println!("Vector is now {:?}", v);

    // Mutable references can also be temporarily "downgraded" to read-only references. For example:
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);

    // Permissions Are Returned At The End of a Reference's Lifetime
    let mut x = 1;
    let y = &x;
    let z = *y;
    x += z;

    fn ascii_capitalize(v: &mut Vec<char>) {
        let c = &v[0];
        if c.is_ascii_lowercase() {
            let up = c.to_ascii_uppercase();
            v[0] = up;
        } else {
            println!("Already capitalized: {:?}", v);
        }
    }

    let mut a: Vec<char> = vec!['D', 'i'];
    ascii_capitalize(&mut a);

    println!("{:?}", a);

    // Context: The mutable borrow t = &mut s removes all permissions on s while t is live.
    let mut s = String::from("Hello");
    let t = &mut s;
    /* here */
    // println!("{}", s);}
    t.push_str(" world");
    println!("{}", s);

    // Next exemple
    let mut s = String::from("hello");
    let s2 = &s;
    let s3 = &mut s;
    s3.push_str(" world");

    // If this like exist the code does not compile because the lifecicle will not free the permissions of s2
    // println!("{s2}");

    // All variables can read, own, and (optionally) write their data.
    // Creating a reference will transfer permissions from the borrowed path to the reference.
    // Permissions are returned once the reference's lifetime has ended.
    // Data must outlive all references that point to it.

    println!("\n\nFixing Ownership Errors 4.3\n\n");

}
