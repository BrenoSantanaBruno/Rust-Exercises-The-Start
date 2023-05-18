#![allow(unused)]

fn main () {
    //Enumeration or enum is a custom data type that allows you to define a type by enumerating its possible values
    //Enums are useful when you have a small set of values that you know at compile time
    //Enums are useful when you want to encode some meaning to a value
    //Enums are useful when you want to define a type that can only have a certain set of values
    //Enums are useful when you want to define a type that can have different values at different times
    //Enums are defined using the enum keyword
    //Enums are similar to structs
    //Enums can have named fields like structs
    //Enums can have unnamed fields like tuples
    //Enums can have different types of fields
    //Enums can have methods
    //Enums can have associated functions
    //Enums can have different implementations
    //Enums can have different traits
    //Enums can have different visibility
    //Enums can have different lifetimes
    //Enums can have different types
    //Enums can have different values
    //Enums can have different sizes
    //Enums can have different memory layouts
    //Enums can have different alignment
    //Enums can have different padding
    //Enums can have different representations
    //Enums can have different variants
    //Enums can have different discriminants

    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8),
        CMYK{cyan: u8, magenta: u8, yellow: u8, black: u8},
    }

    let color: Color = Color::RGB(255, 0, 0);
    impl Color {
        fn rgb(&self) -> String {
            match self {
                Color::Red => String::from("FF0000"),
                Color::Green => String::from("00FF00"),
                Color::Blue => String::from("0000FF"),
                Color::RGB(r, g, b) => format!("{:02X}{:02X}{:02X}", r, g, b),
                Color::CMYK{cyan: c, magenta: m, yellow: y, black: k} => format!("{:02X}{:02X}{:02X}{:02X}", c, m, y, k),
            }
        }
    }

    println!("Color: {}", color.rgb());
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    println!("{} {}", Direction::Up, Direction::Down);

    let up: Direction = Direction::Up;
    #[derive(Debug)]
    println!("{:?}", up);

    enum Months{
        January,
        February,

    }




    enum Shape {
        Circle(f32),
        Rectangle(f32, f32),
        Square(f32),
    }

    let circle = Shape::Cirle(10.0);
    let rectangle = Shape::Rectangle(10.0, 5.0);
    let square = Shape::Square(10.0);

    println!("{:?}", circle);
}