struct Counter {}

pub trait Iterator<MEAOW = String> {
    type BLA;
    fn next(&mut self, cat: MEAOW) -> Option<Self::BLA>;
}

impl Iterator<u32> for Counter {
    type BLA = i32;
    fn next(&mut self, cat: u32) -> Option<Self::BLA> {
        println!("{cat}");
        Some(10)
    }
} // With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.

// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
