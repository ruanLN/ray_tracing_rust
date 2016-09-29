extern crate cgmath;
use cgmath::{Point3,Vector3};

#[derive(Debug)]
struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>
}
fn main() {
    let o = Point3{x: 0.0, y: 0.0, z: 0.0};
    let d = Vector3{x:1.0, y: 0.0, z: 0.0};
    let r = Ray{origin: o, direction: d};
    println!("Hello, world! {:?} {:?}\n{:?}", o, d, r);
}
