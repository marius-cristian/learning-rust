#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self, l: u32) -> u32 {
        self.width * self.height * l
    }

    fn modify(&mut self, l: u32) {
        self.width = l
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
            || self.height >= other.width && self.width >= other.height
    }
}

// multiple impl blocks
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle {
        width: 50,
        height: 30,
    };
    // call of square is done with resolution operator ::
    let sq = Rectangle::square(3);
    println!("square is: {:#?}", sq);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(2)
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "Can rect1 hold rect4 (rect4 isrect1 fliped)? {}",
        rect1.can_hold(&rect4)
    );

    rect1.modify(100);
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("{:#?}", rect1);
}
