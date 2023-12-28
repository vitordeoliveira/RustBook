pub trait Draw {
    fn draw(&self);
}

// Trait object
pub struct Screen {
    // Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw button
    }
}

impl Draw for String {
    fn draw(&self) {
        // code
    }
}

// A generic type parameter can only be substituted with one concrete type at a time, whereas trait
// objects allow for multiple concrete types to fill in for the trait object at runtime.
//
// This restricts us to a Screen instance that has a list of components all of type Button or all
// of type TextField. If you’ll only ever have homogeneous collections, using generics and trait
// bounds is preferable because the definitions will be monomorphized at compile time to use the
// concrete types.
//
// On the other hand, with the method using trait objects, one Screen instance can hold a Vec<T>
// that contains a Box<Button> as well as a Box<TextField>
//
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
