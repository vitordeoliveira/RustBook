fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // iter_mut
    {
        let mut v2 = vec![1, 2, 3];
        let v2_iter_mut = v2.iter_mut();

        for val in v2_iter_mut {
            *val = 3;
        }

        println!("{:?}", v2);
    }
    // into_iter
    {
        let v3 = vec![1, 2, 3];
        let v3_iter_mut = v3.into_iter();
        for mut val in v3_iter_mut {
            val = 3;
            println!("{val}");
        }
    }

    // Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
    // Instead, they produce different iterators by changing some aspect of the original iterator.
    let v2: Vec<i32> = vec![1, 2, 3];

    let val = v2.iter().map(|x| x + 1);
    // The code in Listing 13-14 doesn’t do anything; the closure we’ve specified never gets called.
    // The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here.
    // To fix this warning and consume the iterator, we’ll use the collect method, which we used in Chapter 12 with env::args in Listing 12-1.
    // This method consumes the iterator and collects the resulting values into a collection data type.
    println!("{:?}", val);
    println!("{:?}", v2);

    let v3: Vec<i32> = vec![1, 2, 3];

    let val_2: Vec<_> = v3.iter().map(|x| x + 1).collect();

    println!("{:?}", val);
    println!("{:?}", val_2);

    // NEXT in lib.rs
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // Methods that call next are called consuming adaptors, because calling them uses up the iterator.
    // One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator.
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterator_map_chaining() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).map(|y| y + 1).collect();

    assert_eq!(v2, vec![3, 4, 5]);
}
