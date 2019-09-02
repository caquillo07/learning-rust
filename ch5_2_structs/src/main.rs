
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "the area of the rectangle is {} square pixels",
        area(width, height)
    );
    println!(
        "the area_with_tuples of the rectangle is {} square pixels",
        area_with_tuples((width, height))
    );
    println!(
        "the area_with_struct of the rectangle is {} square pixels",
        area_with_struct(&Rectangle{width: 30, height: 50})
    );

    let rect1 = Rectangle{width:30, height: 50};

    // regular print
    println!("rect1 is {:?}", rect1);

    // pretty print
    println!("rect1 is {:#?}", rect1);

    let rect1 = Rectangle {width: 14, height: 40};
    println!("rect1 area is {:#?}", rect1.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}