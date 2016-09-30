extern crate cgmath;
use cgmath::{Point3,Vector3};

#[derive(Debug)]
struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>
}

#[derive(Debug)]
struct Triangle {
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>
}

// this functions return None when the Ray didn't hit the Triangle
// and Some<Point3> when it hits, where Point3 is the hitpoint
fn hit(r: Ray, t: Triangle) -> Option<Point3<f32>> {
    return Some(t.p0);
}

fn main() {
    let o = Point3{x: 0.0, y: 0.0, z: 0.0};
    let d = Vector3{x:1.0, y: 0.0, z: 0.0};
    let p0 = Point3{x: 10.0, y: 3.0, z: 0.0};
    let p1 = Point3{x: 10.0, y: -1.0, z: 3.0};
    let p2 = Point3{x: 10.0, y: -1.0, z: 3.0};
    let r = Ray{origin: o, direction: d};
    let t = Triangle{p0: p0, p1: p1, p2: p2};
    println!("Hello, world! {:?} {:?}\n{:?}\n{:?}", o, d, r, t);
}
