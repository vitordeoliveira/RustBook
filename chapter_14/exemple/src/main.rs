// use art::kinds::PrimaryColor;
// use art::mix;
// In cases where there are many nested modules, re-exporting the types at the top level with pub
// use can make a significant difference in the experience of people who use the crate.

use art::utils::mix;
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

