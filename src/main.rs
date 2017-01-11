extern crate cgmath;
extern crate bmp;
use cgmath::{Point3,Vector3,InnerSpace,EuclideanSpace,MetricSpace};
use bmp::{Image,Pixel};
use std::collections::LinkedList;
use std::f32;
use std::fmt::Debug;

#[derive(Debug)]
struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>
}

type Color = Pixel;

#[derive(Debug)]
struct Triangle {
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    difuse_color: Color
}

#[derive(Debug)]
struct Sphere {
    c: Point3<f32>,
    r: f32,
    difuse_color: Color
}

#[derive(Debug)]
struct Light {
    o: Point3<f32>,
    color: Color
}

#[derive(Debug)]
struct Scene {
    objects: LinkedList<Box<DrawableObject>>,
    lights: LinkedList<Light>
}

trait DrawableObject : Debug {
    // this functions return None when the Ray didn't hit the Object
    // and Some<Point3> when it hits, where Point3 is the hitpoint
    fn hit(&self, r: &Ray) -> Option<Point3<f32>>;
    fn get_difuse_color(&self) -> Color;
}

impl DrawableObject for Triangle {
    fn hit(&self, r: &Ray) -> Option<Point3<f32>> {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        let triangle_normal = edge1.cross(edge2);
        let epsilon: f32 = 0.0001;

        //verify if the tringle and ray are parallel
        let den = triangle_normal.dot(r.direction);
        if den.abs() < epsilon {
            return None;
        }
        //verify if the triangle is in front of the ray
        let d = -(triangle_normal.dot(self.p0.to_vec()));
        let num = triangle_normal.dot(r.origin.to_vec()) + d;
        let distance = -(num/den);
        if distance < epsilon {
            return None;
        }
        r.direction.normalize();
        let intersection_point = r.origin + r.direction * distance;

        //verify if the intersection_point is inside self
        let vp0 = intersection_point + (-self.p0.to_vec());
        let perpendicular_vp0 = (self.p1 - self.p0).cross(vp0.to_vec());
        if triangle_normal.dot(perpendicular_vp0) < 0f32 {
            return None;
        }
        let vp1 = intersection_point + (-self.p1.to_vec());
        let perpendicular_vp1 = (self.p2 - self.p1).cross(vp1.to_vec());
        if triangle_normal.dot(perpendicular_vp1) < 0f32 {
            return None;
        }
        let vp2 = intersection_point + -(self.p2.to_vec());
        let perpendicular_vp2 = (self.p0 - self.p2).cross(vp2.to_vec());
        if triangle_normal.dot(perpendicular_vp2) < 0f32 {
            return None;
        }

        return Some(intersection_point);
    }

    fn get_difuse_color(&self) -> Color {
        self.difuse_color
    }
}

impl DrawableObject for Sphere {
    fn hit(&self, r: &Ray) -> Option<Point3<f32>> {
        let d = r.direction.normalize();
        let dir = r.origin - self.c;
        let tmp = (r.origin - self.c).magnitude2();
        let delta = d.dot(dir).powf(2.0) - tmp + self.r.powf(2.0);
        
        if delta <= 0.0 {
            return None;
        } else if delta >= 0.1 {
            let d1 = -(d.dot(r.origin - self.c)) - delta.sqrt();
            let d2 = -(d.dot(r.origin - self.c)) + delta.sqrt();
            let dist = if d1 < d2 { d1 } else { d2 };
            let intersection_point = r.origin + d.normalize() * dist;
           // println!("intersection_point: {:?}", intersection_point);
            return Some(intersection_point);
        } else {
            let dist = -(d.dot(r.origin - self.c));
            let intersection_point = r.origin + d.normalize() * dist;
            //println!("intersection_point2: {:?}", intersection_point);            
            return Some(intersection_point);
        }
        None
    }

    fn get_difuse_color(&self) -> Color {
        self.difuse_color
    }
}

// this function return the color of the point (must receive a scene in future)
fn trace(r: &Ray, scene: &Scene) -> Pixel {
    let mut iter = scene.objects.iter();
    let mut min_distance = f32::INFINITY;
    let mut final_color = Pixel {r: 0, g:0, b:0};
    while let Some(t) = iter.next() {
        if let Some(point) = t.hit(r) {
            let distance = point.distance(r.origin);
            if distance < min_distance {
                min_distance = distance;
                final_color = t.get_difuse_color();
            }
        }
    }
    return final_color;
}

fn main() {
    let p0 = Point3{x:  0.0, y:  10.0, z: 10.0};
    let p0_1 = Point3{x:  0.0, y:  20.0, z: 11.0};
    let p1 = Point3{x: -10.0, y: -10.0, z: 10.0};
    let p2 = Point3{x:  10.0, y: -10.0, z: 10.0};
    let t = Triangle{p0: p0, p1: p1, p2: p2, difuse_color: Color{r:255, g:0, b:0}};
    let t2 = Triangle{p0: p0_1, p1: p1, p2: p2, difuse_color: Color{r:0, g:0, b:255}};

    let s = Sphere{c: Point3{x:-50.0, y: 0.0, z: 0.0}, r: 30.0, difuse_color: Color{r:0, g:255, b:0}};
    let mut objects = LinkedList::new();
    objects.push_back(Box::new(t) as Box<DrawableObject>);
    objects.push_back(Box::new(t2));
    objects.push_back(Box::new(s) as Box<DrawableObject>);
    let scene = Scene{objects: objects, lights: LinkedList::new()};
    let (w, h) = (512, 512);
    let mut img = Image::new(w, h);
    let o = Point3 {x: 0f32, y: 0f32, z: -50.0};
    for (x, y) in img.coordinates() {
        let d = Vector3 {x: (x as i32 - (w as i32/2)) as f32, y: ((h as i32/2) - y as i32) as f32, z: 50.0}.normalize();
        let r = Ray {origin: o, direction: d};
        let ret = trace(&r, &scene);
        img.set_pixel(x, y, ret);
    }

    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save("test.bmp");
}

#[cfg(test)]
mod tests {
    use cgmath::{Point3,Vector3};
    use super::Color;
    #[test]
    fn hit_should_hit() {
        let o = Point3{x: 0.0, y: 0.0, z: 0.0};
        let d = Vector3{x:1.0, y: 0.0, z: 0.0};
        let p0 = Point3{x: 10.0, y: 3.0, z: 0.0};
        let p1 = Point3{x: 10.0, y: -1.0, z: 3.0};
        let p2 = Point3{x: 10.0, y: -1.0, z: -3.0};
        let p_expected = Point3{x: 10.0, y: 0.0, z: 0.0};
        let r = super::Ray{origin: o, direction: d};
        let t = super::Triangle{p0: p0, p1: p1, p2: p2, difuse_color: Color{r:255, g:0, b:0}};
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
        let t = super::Triangle{p0: p0, p1: p1, p2: p2, difuse_color: Color{r:255, g:0, b:0}};
        let result = super::hit(&r,&t);
        assert_eq!(result, None);
    }
}
