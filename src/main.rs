use std::fmt;

#[derive(Debug)]
struct Coordinate(f32, f32, f32);
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[x={}, y={}, z={}]", self.0, self.1, self.2)
    }
}

#[derive(Debug)]
struct Vector(f32, f32);
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[velocity={}, direction={}]", self.0, self.1)
    }
}

fn transpose_coord(input: Coordinate) -> Coordinate {
    Coordinate(input.2, input.1, input.0)
}


fn format_fun() {
    println!("Hello, formatting!");

    let vector = Vector(100.1, 200.1);
    println!("vector: {}", vector);
    println!("vector Debug: {:?}", vector);

    let coord = Coordinate(1.1, 1.2, 2.3);
    println!("Coordinate FMT: {}", coord);
    println!("Coordinate Debug: {:?}", coord);
    println!("Transposed: {:?}", transpose_coord(coord));
}


fn main() {
    format_fun();    
}
