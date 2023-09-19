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

fn four_two() {
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
}

fn four_three() {
    println!("Fixing an Unsafe Program: Returning a Reference to the Stack");
    // fn return_a_string() -> &String {
    //     let s = String::from("Hello world");
    //     &s
    // }

    fn _return_a_string_2() -> String {
        let s = String::from("Hello world");
        s
    }

    fn _return_a_string_3() -> &'static str {
        "Hello world"
    }

    use std::rc::Rc;
    fn _return_a_string_4() -> Rc<String> {
        let s = Rc::new(String::from("Hello world"));
        Rc::clone(&s)
    }

    fn _return_a_string_5(output: &mut String) {
        output.replace_range(.., "Hello world");
    }

    // Next exemple
    println!("\n\nFixing an Unsafe Program: Not Enough Permissions\n\n");

    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title4(&name);
    println!("{}", first);
    println!("{:?}", name);

    // Next exemple
    println!("\n\nFixing an Unsafe Program: Copying vs. Moving Out of a Collection\n\n");

    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;

    // if we change the type of elements in the vector from i32 to String? Then it turns out we no longer have the necessary permissions:
    // What happens here is a double-free. After executing let s = *s_ref, both v and s think they own "Hello world".
    // After s is dropped, "Hello world" is deallocated.
    // Then v is dropped, and undefined behavior happens when the string is freed a second time.
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    // let s: String = *s_ref;

    // However, this undefined behavior does not happen when the vector contains i32 elements.
    // The difference is that copying a String copies a pointer to heap data. Copying an i32 does not. In technical terms,
    // Rust says that the type i32 implements the Copy trait, while String does not implement Copy (we will discuss traits in a later chapter).
    // In sum, if a value does not own heap data, then it can be copied without a move. For example:
    // An i32 does not own heap data, so it can be copied without a move.
    // A String does own heap data, so it can not be copied without a move.
    // An &String does not own heap data, so it can be copied without a move.

    // Note: One exception to this rule is mutable references. For example, &mut i32 is not a copyable type. So if you do something like:
    let mut n = 0;
    let a = &mut n;
    let b = a;
    // Then a cannot be used after being assigned to b. That prevents two mutable references to the same data from being used at the same time.

    // So if we have a vector of non-Copy types like String, then how do we safely get access to an element of the vector?
    // Here's a few different ways to safely do so. First, you can avoid taking ownership of the string and just use an immutable reference:
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");
    // Second, you can clone the data if you want to get ownership of the string while leaving the vector alone:

    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");
    // Finally, you can use a method like Vec::remove to move the string out of the vector:

    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);

    // Fixing a Safe Program: Mutating Different Array Elements
    // let mut a = [0, 1, 2, 3];
    // let x = &mut a[0];
    // let y = &a[1];
    // *x += *y;

    let mut a = [0, 1, 2, 3];
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x += *y;

    let mut a = [0, 1, 2, 3];
    let x = &mut a[0] as *mut i32;
    let y = &a[1] as *const i32;
    unsafe {
        *x += *y;
    } // DO NOT DO THIS unless you know what you're doing!
}

fn four_four() {
    println!("\n\nThe Slice Type\n\n");
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();

    // let s = String::from("hello world");
    // let hello: &str = &s[0..5];
    // let world: &str = &s[6..11];
    // let s2: &String = &s;

    fn first_word_rewrite(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let s = String::from("hello world");
    let word = first_word_rewrite(&s);
    // s.clear();
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_rewrite(&my_string[0..6]);
    let word = first_word_rewrite(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    // my_string.clear();
    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_rewrite(&my_string_literal[0..6]);
    let word = first_word_rewrite(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_rewrite(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    // Summary
    // Slices are a special kind of reference that refer to sub-ranges of a sequence, like a string or a vector.
    // At runtime, a slice is represented as a "fat pointer" which contains a pointer to the beginning of the range and a length of the range.
    // One advantage of slices over index-based ranges is that the slice cannot be invalidated while it's being used.
}

fn four_five() {
    type Document = Vec<String>;

    fn new_document(words: Vec<String>) -> Document {
        words
    }

    fn add_word(this: &mut Document, word: String) {
        this.push(word);
    }

    fn get_words(this: &Document) -> &[String] {
        this.as_slice()
    }

    let words = vec!["hello".to_string()];
    let d: Document = new_document(words);
    // .to_vec() converts &[String] to Vec<String> by cloning each string
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    // The modification to `d2` does not affect `d`
    assert!(!get_words(&d).contains(&"world".into()));

    // NEXT EXEMPLE
    // This diagram illustrates how each concept looks at runtime:
    let mut a_num = 0;
    inner(&mut a_num);

    fn inner(x: &mut i32) {
        let another_num = 1;
        let a_stack_ref = &another_num;

        let a_box = Box::new(2);
        let a_box_stack_ref = &a_box;
        let a_box_heap_ref = &*a_box;

        *x += 5;
    }

    // Slices are a special kind of reference that refer to a contiguous sequence of data in memory.
    // This diagram illustrates how a slice refers to a subsequence of characters in a string:

    let mut s = String::from("Hello");
    let s_ref = &s;
    let s2 = &*s.clone();
    let s3 = &[s];
    println!("{} - {:?}", s2, s3);

    let mut v = vec![1, 2, 3];
    let n = &v[0];
    v.push(4);
    let n = &v[0];
    println!("{n}");
    
}

fn main() {
    println!("References and Borrowing 4.2\n\n");
    four_two();
    
    println!("\n\nFixing Ownership Errors 4.3\n\n");
    four_three();

    println!("\n\nThe Slice Type 4.3\n\n");
    four_four();

    four_five();
}

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

/// But this is also not a good solution!
/// It is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String.
/// This version of stringify_name_with_title would make the input name unusable, which is very annoying to a
/// caller as we discussed at the beginning of "References and Borrowing".
fn _stringify_name_with_title1(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

/// But this is not a good solution! Functions should not mutate their inputs if the caller would not expect it.
/// A person calling stringify_name_with_title probably does not expect their vector to be modified by this function.
/// Another function like add_title_to_name might be expected to mutate its input, but not our function.
fn _stringify_name_with_title2(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

/// So the choice of &Vec is actually a good one, which we do not want to change. Instead, we can change the body of the function.
/// There are many possible fixes which vary in how much memory they use. One possibility is to clone the input name:
fn _stringify_name_with_title3(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

/// By cloning name, we are allowed to mutate the local copy of the vector.
/// However, the clone copies every string in the input. We can avoid unnecessary copies by adding the suffix later:
fn stringify_name_with_title4(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

/// This program is rejected by the borrow checker because let largest =
/// .. removes the W permissions on dst. However, dst.push(..) requires the W permission. Again, we should ask: why is this program unsafe?
/// Because dst.push(..) could deallocate the contents of dst, invalidating the reference largest.
fn _add_big_strings_0(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            // dst.push(s.clone());
        }
    }
}

/// A final possibility is to copy out the length of largest,
/// since we don't actually need the contents of largest, just its length.
/// This solution is arguably the most idiomatic and the most performant:
fn _add_big_strings_1(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
