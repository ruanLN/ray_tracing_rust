extern crate cgmath;
use cgmath::{Point3,Vector3,InnerSpace};
use std::f32;

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
fn hit(r: &Ray, t: &Triangle) -> Option<Point3<f32>> {
    let edge1 = t.p1 - t.p0;
    let edge2 = t.p2 - t.p0;
    let triangle_normal = edge1.cross(edge2);
    let epsilon: f32 = 0.0001;
    let den = triangle_normal.dot(r.direction);
    if(den.abs() <= epsilon) {
        return None;
    }
    return Some(t.p0);
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::{Point3,Vector3,InnerSpace};
    #[test]
    fn hit_should_hit() {
        let o = Point3{x: 0.0, y: 0.0, z: 0.0};
        let d = Vector3{x:1.0, y: 0.0, z: 0.0};
        let p0 = Point3{x: 10.0, y: 3.0, z: 0.0};
        let p1 = Point3{x: 10.0, y: -1.0, z: 3.0};
        let p2 = Point3{x: 10.0, y: -1.0, z: -3.0};
        let p_expected = Point3{x: 10.0, y: 0.0, z: 0.0};
        let r = super::Ray{origin: o, direction: d};
        let t = super::Triangle{p0: p0, p1: p1, p2: p2};
        let result = super::hit(&r,&t);
        assert_eq!(result, Some(p_expected));
    }
    #[test]
    fn should_not_hit() {
        let o = Point3{x: 0.0, y: 0.0, z: 0.0};
        let d = Vector3{x:1.0, y: 0.0, z: 0.0};
        let p0 = Point3{x: 10.0, y: 3.0, z: 0.0};
        let p1 = Point3{x: 13.0, y: -1.0, z: 0.0};
        let p2 = Point3{x: 7.0, y: -1.0, z: 0.0};
        let p_expected = Point3{x: 10.0, y: 0.0, z: 0.0};
        let r = super::Ray{origin: o, direction: d};
        let t = super::Triangle{p0: p0, p1: p1, p2: p2};
        let result = super::hit(&r,&t);
        assert_eq!(result, None);
    }
}
