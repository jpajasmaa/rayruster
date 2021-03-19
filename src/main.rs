mod color;
mod vec3;
mod ray;

use std::io::{self, Write};
use color::write_color;
use vec3::{Color, Point3, Vec3};
use ray::Ray;


fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new_vec(0.0,0.0,-1.0), 0.5, &ray); 
    if t > 0.0 {
        let normal = Color::unit_vector(ray.at(t)+Vec3::new_vec(1.0, 1.0, 0.0)); // täälä näitten kaa jotain kikkailua tarvii, tai puuttuu
        return 0.5*normal;
    }
    let unit_dir: Vec3 = Vec3::unit_vector(ray.direction);
    let t = 0.5*(unit_dir.y + 1.0);
    (1.0-t) * Color::new_vec(1.0,1.0,1.0)+ t * Color::new_vec(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, rad: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = Vec3::dot(&r.direction, &r.direction);
    let b = 2.0 * Vec3::dot(&oc, &r.direction);
    let c = Vec3::dot(&oc, &oc) - rad*rad;
    let disc = b*b - 4.0*a*c;
    if disc < 0.0 {
        -1.0
    }
    else {
        (-b - disc.sqrt()) / 2.0*a
    }
}


fn main() {
   
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // image
    let image_w = 256;
    let image_h = 256;
    let aspect_ratio: f64 = 4.0/3.0;

    // Camera
    let viewport_h: f64 = 2.0;
    let viewport_w: f64 = aspect_ratio * viewport_h;
    let focal_length: f64 = 1.0;

    let origin = Point3::new();
    let hor = Vec3::new_vec(viewport_w,0.0,0.0);
    let ver = Vec3::new_vec(0.0,viewport_h,0.0);
    let low_left_corner = origin - hor/2.0 - ver/2.0 - Vec3::new_vec(0.0,0.0, focal_length);


    println!("P3\n{} {}\n255", image_h, image_w);
    for j in (0..image_h).rev() {
        for i in 0..image_w {
            let u = i as f64 /(image_w - 1) as f64;
            let v = j as f64 /(image_h - 1) as f64;

            let ray = Ray::new_ray(origin, low_left_corner + u*hor + v*ver - origin); 

            let pixel_color = ray_color(&ray);

            match write_color(&mut handle, pixel_color) {
                Ok(_) => continue,
                Err(e) => eprint!(
                    "Oops, error {} saving pixel {} for indices i {} j {}",
                    e, pixel_color, i, j
                ),
            }           
        }
    }
}
