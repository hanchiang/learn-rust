pub fn main() {
    gui();
}

fn gui() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK")
            })
        ]
    };

    screen.run();
}

trait Draw {
    fn draw(&self);
}

struct Screen {
    // Box<dyn Draw> is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
    // This works differently from defining a struct that uses a generic type parameter with trait bounds.
    // A generic type parameter can only be substituted with one concrete type at a time,
    // whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
    // On the other hand, with the method using trait objects,
    // one Screen instance can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // run doesn’t need to know what the concrete type of each component is.
    // It doesn’t check whether a component is an instance of a Button or a SelectBox, it just calls the draw method on the component.
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Define the GUI components!
// Each of the types we want to draw on the screen will implement the Draw trait but will use
// different code in the draw method to define how to draw that particular type.
struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}