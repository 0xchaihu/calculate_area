trait Area {
    fn cal_area(&self) -> u32;
}

struct Rectangle {
    length: u32,
    width: u32,
}


struct Square {
    width: u32,
}

impl Area for Rectangle {
    fn cal_area(&self) -> u32 {
        self.length * self.width
    }
}
impl Area for Square {
    fn cal_area(&self) -> u32 {
        self.width * self.width
    }
}

fn print_area<T: Area>(shape: T) {
    let area = shape.cal_area();
    println!("The area is: {:?}", area);
}
fn main() {
    let rect = Rectangle {
        length: 941,
        width: 1,
    };
    let square = Square { width: 941 };

    println!("calculate the area of Rectangle...");
    print_area(rect);

    println!("calculate the area of Square...");
    print_area(square);
}
