extern crate cgmath;
extern crate image;
use cgmath::{Point3,Vector3,InnerSpace};
use std::fs::File;
use std::path::Path;
use image::GenericImage;
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
    //Construct a new by repeated calls to the supplied closure.
    let img = ImageBuffer::from_fn(512, 512, |x, y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let ref mut fout = File::create(&Path::new("test.png")).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save(fout, image::PNG).unwrap();
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
