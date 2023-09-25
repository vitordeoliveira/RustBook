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


}
