extern crate cgmath;
extern crate bmp;
use cgmath::{Point3,Vector3,InnerSpace,EuclideanSpace};
use bmp::{Image,Pixel};
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

    //verify if the tringle and ray are parallel
    let den = triangle_normal.dot(r.direction);
    if den.abs() <= epsilon {
        return None;
    }
    //verify if the triangle is in front of the ray
    let d = -(triangle_normal.dot(t.p0.to_vec()));
    let num = triangle_normal.dot(r.origin.to_vec()) + d;
    let distance = -(num/den);
    if distance < epsilon {
        return None;
    }
    r.direction.normalize();
    let intersection_point = r.origin + r.direction * distance;

    //verify if the intersection_point is inside t
    let vp0 = intersection_point + (-t.p0.to_vec());
    let perpendicular_vp0 = (t.p1 - t.p0).cross(vp0.to_vec());
    if triangle_normal.dot(perpendicular_vp0) < 0f32 {
        return None;
    }
    let vp1 = intersection_point + (-t.p1.to_vec());
    let perpendicular_vp1 = (t.p2 - t.p1).cross(vp1.to_vec());
    if triangle_normal.dot(perpendicular_vp1) < 0f32 {
        return None;
    }
    let vp2 = intersection_point + -(t.p2.to_vec());
    let perpendicular_vp2 = (t.p0 - t.p2).cross(vp2.to_vec());
    if triangle_normal.dot(perpendicular_vp2) < 0f32 {
        return None;
    }

    return Some(intersection_point);
}

fn main() {
    let p0 = Point3{x:  0.0, y:  10.0, z: 10.0};
    let p1 = Point3{x: -10.0, y: -10.0, z: 10.0};
    let p2 = Point3{x:  10.0, y: -10.0, z: 10.0};
    let t = Triangle{p0: p0, p1: p1, p2: p2};

    let (w, h) = (512, 512);
    let mut img = Image::new(w, h);
    for (x, y) in img.coordinates() {
        let o = Point3 {x: (((w/2) as f32)-(x as f32)) as f32, y: (((h/2) as f32)-(y as f32)) as f32, z: -1000.0};
        let d = Vector3 {x: 0.0, y:0.0, z: 11.0};
        let r = Ray {origin: o, direction: d};
        let ret = hit(&r, &t);
        if ret == None {
            img.set_pixel(x, y, Pixel { r: 255, g: 255, b: 255 });
        } else {
            img.set_pixel(x, y, Pixel { r: 0, g: 0, b: 0 });
        }
    }

    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save("test.bmp");
}

#[cfg(test)]
mod tests {
    use cgmath::{Point3,Vector3};
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
        let r = super::Ray{origin: o, direction: d};
        let t = super::Triangle{p0: p0, p1: p1, p2: p2};
        let result = super::hit(&r,&t);
        assert_eq!(result, None);
    }
}
