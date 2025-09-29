#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, input_rectangle: &Rectangle) -> bool {
        self.area() >= input_rectangle.area() &&
        self.width >= input_rectangle.width && // AND operator
        self.height >= input_rectangle.height
    }

    fn square(size: u32) -> Self {
        // this is an associated function. It Returns a 'square' from type Rectangle        
        Self { width: size, height: size }
    }

    // set_width needs a mutable reference to self as it will change the value of the instance when called.
    fn set_width(&mut self, width: u32){
        self.width = width;
    }

    // the method max moves 'self' from the instance itself to the caller when it is returned, so downstream references of the initial instance becomes invalid
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width),
            height: self.height.max(other.height) 
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2= Rectangle{
        width: 10,
        height: 40
    };

    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    println!("The area of the rect1 is {} sq pixels", {rect1.area()});
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);

    println!("The area of the square is {}", &square.area());

    let mut rect4 = Rectangle {
        width: 20,
        height: 40
    };

    println!("rect4 width was {}", rect4.width);

    rect4.set_width(40);

    println!("but now rect4 width is {}", rect4.width);

    dbg!(&rect1);
    let rect1 = rect1.max(rect3);
    dbg!(&rect1);

}
