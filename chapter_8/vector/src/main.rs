fn main() {
    // Create a Vector
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    // Updating a Vector

    //  The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.
    let mut v = Vec::new();
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    let third: &i32 = v.get(2).unwrap();

    // next
    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // next
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    v.push(6);

    // In that case, the reference to the first element would be pointing to deallocated memory.
    // The borrowing rules prevent programs from ending up in that situation.
    // println!("The first element is: {first}");

    // Iterating over the Values in a Vector

    let v: Vec<i32> = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }
    println!("{:?}", v);

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }
    println!("{:?}", v);

    // Safely Using Iterators
    // We can see how iterators work by desugaring a for-loop into the corresponding method calls of Vec::iter and Iterator::next:
    use core::slice::Iter;
    use std::ops::Range;
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();
    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next();

    // As we discussed in Chapter 4, the safety issue beneath this error is reading deallocated memory.
    // As soon as v.push(1) happens, the vector will reallocate its contents and invalidate the iterator's pointer.
    // So to use iterators safely, Rust does not allow you to add or remove elements from the vector during iteration.
    // One way to iterate over a vector without using a pointer is with a range, like we used for string slices in Chapter 4.4.
    // For example, the range 0 .. v.len() is an iterator over all indices of a vector v, as seen here:
    fn dup_in_place(v: &mut Vec<i32>) {
        for n_ref in v.iter() {
            // Notice that v.iter() removes the W permission from *v. Therefore the v.push(..) operation is missing the expected W permission
            // v.push(*n_ref);
        }
    }

    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Range<usize> = 0..v.len();
    let i1: usize = iter.next().unwrap();
    let n1: &i32 = &v[i1];
}
