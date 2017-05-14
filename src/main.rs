extern crate cgmath;
extern crate bmp;
use cgmath::{Point3, Vector3, InnerSpace, EuclideanSpace, MetricSpace};
use bmp::{Image, Pixel};
use std::collections::LinkedList;
use std::f32;
use std::fmt::Debug;


#[derive(Debug)]
struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    fn point_at_parameter(&self, t: f32) -> Point3<f32> {
        self.origin + t * self.direction
    }

    fn trace(&self, scene: &Scene) -> Color {
        let mut iter = scene.objects.iter();
        let mut min_distance = f32::INFINITY;
        let mut final_color = Color { r: 0, g: 0, b: 0 };
        while let Some(t) = iter.next() {
            if let Some(point) = t.hit(self) {
                let distance = point.distance(self.origin);
                if distance < min_distance {
                    min_distance = distance;
                    final_color = t.get_difuse_color();
                }
            }
        }
        return final_color;
    }
}

type Color = Pixel;

#[derive(Debug)]
#[allow(dead_code)]
struct Triangle {
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    //n0: Vector3<f32>,
    //n1: Vector3<f32>,
    //n2: Vector3<f32>,
    difuse_color: Color,
}

#[derive(Debug)]
struct Sphere {
    c: Point3<f32>,
    r: f32,
    difuse_color: Color,
}

#[derive(Debug)]
struct Plane {
    p: Point3<f32>,
    n: Vector3<f32>,
}

#[derive(Debug)]
struct Light {
    o: Point3<f32>,
    color: Color,
}

#[derive(Debug)]
struct Scene {
    objects: LinkedList<Box<DrawableObject>>,
    lights: LinkedList<Light>,
}

trait DrawableObject: Debug {
    // this functions return None when the Ray didn't hit the Object
    // and Some<Point3> when it hits, where Point3 is the hitpoint
    fn hit(&self, r: &Ray) -> Option<Point3<f32>>;
    fn get_difuse_color(&self) -> Color;
    fn get_normal(&self, p: &Point3<f32>) -> Option<Vector3<f32>>;
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
        let distance = -(num / den);
        if distance < epsilon {
            return None;
        }
        r.direction.normalize();
        let intersection_point = r.point_at_parameter(distance);

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
    #[allow(unused_variables)]
    fn get_normal(&self, p: &Point3<f32>) -> Option<Vector3<f32>> {
        /*in future will use a normal for each vertex,
        and those normals will be interpolated using barycentric coordinates
        this will avoid useless computation and achieve normal mapping*/
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        let triangle_normal = edge1.cross(edge2);
        triangle_normal.normalize();
        Some(triangle_normal)
    }
}

impl DrawableObject for Sphere {
    fn hit(&self, r: &Ray) -> Option<Point3<f32>> {
        let d = r.direction.normalize();
        let dir = r.origin - self.c;
        let tmp = (r.origin - self.c).magnitude2();
        let delta = d.dot(dir).powf(2.0) - tmp + self.r.powf(2.0);
        if delta < 0.0 {
            None
        } else {
            let d1 = -(d.dot(r.origin - self.c)) - delta.sqrt();
            let d2 = -(d.dot(r.origin - self.c)) + delta.sqrt();
            let dist = if d1 < d2 { d1 } else { d2 };
            let intersection_point = r.point_at_parameter(dist);
            Some(intersection_point)
        }
    }

    fn get_difuse_color(&self) -> Color {
        self.difuse_color
    }

    fn get_normal(&self, p: &Point3<f32>) -> Option<Vector3<f32>> {
        Some((p - self.c).normalize())
    }
}


fn main() {
    let s = Sphere {
        c: Point3 {
            x: 512.0,
            y: 0.0,
            z: 0.0,
        },
        r: 50.0,
        difuse_color: Color {
            r: 0,
            g: 0,
            b: (0.7 * 256.0) as u8,
        },
    };
    let s1 = Sphere {
        c: Point3 {
            x: 512.0,
            y: 150.0,
            z: 0.0,
        },
        r: 50.0,
        difuse_color: Color {
            r: 0,
            g: 255,
            b: 0,
        },
    };
    let s2 = Sphere {
        c: Point3 {
            x: 512.0,
            y: -150.0,
            z: 0.0,
        },
        r: 50.0,
        difuse_color: Color {
            r: (0.8 * 256.0) as u8,
            g: 0,
            b: (0.4 * 256.0) as u8,
        },
    };
    let s3 = Sphere {
        c: Point3 {
            x: 350.0,
            y: 0.0,
            z: 0.0,
        },
        r: 20.0,
        difuse_color: Color {
            r: (0.4 * 256.0) as u8,
            g: (0.6 * 256.0) as u8,
            b: 0,
        },
    };
    let s4 = Sphere {
        c: Point3 {
            x: 1024.0,
            y: 0.0,
            z: 0.0,
        },
        r: 150.0,
        difuse_color: Color {
            r: (0.5 * 256.0) as u8,
            g: (0.5 * 256.0) as u8,
            b: 0,
        },
    };
    let mut objects = LinkedList::new();
    objects.push_back(Box::new(s) as Box<DrawableObject>);
    objects.push_back(Box::new(s1));
    objects.push_back(Box::new(s2));
    objects.push_back(Box::new(s3));
    objects.push_back(Box::new(s4));
    let scene = Scene {
        objects: objects,
        lights: LinkedList::new(),
    };
    let (w, h) = (1280, 720);
    let mut img = Image::new(w, h);
    let o = Point3 {
        x: -512f32,
        y: 0f32,
        z: 0f32,
    };
    for (x, y) in img.coordinates() {
        let d = Vector3 {
                x: 512.0,
                y: (x as i32 - (w as i32 / 2)) as f32,
                z: ((h as i32 / 2) - y as i32) as f32,
            }
            .normalize();
        let r = Ray {
            origin: o,
            direction: d,
        };
        let ret = r.trace(&scene);
        img.set_pixel(x, y, ret);
    }

    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save("test.bmp");
}

#[cfg(test)]
mod tests {
    use cgmath::{Point3, Vector3};
    use super::{Color, DrawableObject};
    #[test]
    fn hit_should_hit() {
        let o = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let d = Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let p0 = Point3 {
            x: 10.0,
            y: 3.0,
            z: 0.0,
        };
        let p1 = Point3 {
            x: 10.0,
            y: -1.0,
            z: 3.0,
        };
        let p2 = Point3 {
            x: 10.0,
            y: -1.0,
            z: -3.0,
        };
        let p_expected = Point3 {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        };
        let r = super::Ray {
            origin: o,
            direction: d,
        };
        let t = super::Triangle {
            p0: p0,
            p1: p1,
            p2: p2,
            difuse_color: Color {
                r: 255,
                g: 0,
                b: 0,
            },
        };
        let result = t.hit(&r);
        assert_eq!(result, Some(p_expected));
    }
    #[test]
    fn should_not_hit() {
        let o = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let d = Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let p0 = Point3 {
            x: 10.0,
            y: 3.0,
            z: 0.0,
        };
        let p1 = Point3 {
            x: 13.0,
            y: -1.0,
            z: 0.0,
        };
        let p2 = Point3 {
            x: 7.0,
            y: -1.0,
            z: 0.0,
        };
        let r = super::Ray {
            origin: o,
            direction: d,
        };
        let t = super::Triangle {
            p0: p0,
            p1: p1,
            p2: p2,
            difuse_color: Color {
                r: 255,
                g: 0,
                b: 0,
            },
        };
        let result = t.hit(&r);
        assert_eq!(result, None);
    }
}

