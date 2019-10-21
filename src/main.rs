use std::fmt;
use std::mem;

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

// Lifetime struct
struct Person<'a> {
    name: &'a str,
    coord: Coordinate,
    vec: Vector,
}
impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Person {} is at coordinate {} with vector {}.",
            self.name,
            self.coord,
            self.vec)
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

fn analyze_slice(slice: &[i32]) {
    println!("  Slice[0]: {}", slice[0]);
    println!("  Slice has {} elements", slice.len());
}

fn slice_magic() {
    let xs: [i32; 5] = [2, 4, 6, 8, 10];
    println!("xs first: {}", xs[0]);
    println!("xs last:  {}", xs[4]);
    println!("xs occupies {} bytes", mem::size_of_val(&xs));
    println!("Big slice:");
    analyze_slice(&xs);
    println!("Small slice:");
    analyze_slice(&xs[1..3]);
}

fn game() {
    let paul = Person{
        name: "Paul",
        coord: Coordinate(0.0, 0.0, 0.0),
        vec: Vector(0.0, 0.0)};
    println!("{}", paul);    
}


fn main() {
    format_fun();
    slice_magic();
    game();
}
