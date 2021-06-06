mod cameras;
mod hitables;
mod materials;
mod structs;

use cameras::*;
use hitables::*;
use materials::*;

use rand::Rng;
use std::ops::Div;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

struct Sky {
    pub color1: Vec3,
    pub color2: Vec3,
}

impl Material for Sky {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emit(&self, _u: f64, _v: f64, direction: &Vec3) -> Vec3 {
        let unit_direction = unit_vector(*direction);
        let t: f64 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * self.color1 + t * self.color2
    }
}

fn make_sky_material(color1: &Vec3, color2: &Vec3) -> Arc<dyn Material> {
    Arc::new(Sky {
        color1: color1.clone(),
        color2: color2.clone(),
    })
}

/// Generate a random scene with 484 little random spheres,
/// 3 bigger spheres in center, and a spheric ground.
fn random_scene(background: &Vec3) -> HitableList {
    let mut rng = rand::thread_rng();
    let background = make_sky_material(&Vec3::new(1.0, 1.0, 1.0), background);

    let mut world = HitableList::new(background);

    let mut texture_manager = ResourceManager::new();

    let odd_color = TextureConfig {
        kind: TextureKind::Constant(Color::new(51, 77, 26)),
    };
    let even_color = TextureConfig {
        kind: TextureKind::Constant(Color::new(230, 230, 230)),
    };
    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Checker(&odd_color, &even_color),
            }),
        }),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = rng.gen::<Vec3>() * rng.gen::<Vec3>();
                    let texture = texture_manager.get_resource(&TextureConfig {
                        kind: TextureKind::Constant(Color::new(
                            (albedo.x * 255.99) as u8,
                            (albedo.y * 255.99) as u8,
                            (albedo.z * 255.99) as u8,
                        )),
                    });
                    world.push(Arc::new(MovingSphere {
                        center0: center,
                        center1: center + Vec3::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian { albedo: texture }),
                    }));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(albedo, fuzz)),
                    }));
                } else {
                    // Glass
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric { ref_idx: 1.5 }),
                    }));
                }
            }
        }
    }

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    }));

    world
}

fn two_spheres(background: &Vec3) -> HitableList {
    let background = make_sky_material(&Vec3::new(1.0, 1.0, 1.0), background);

    let mut world = HitableList::new(background);

    let mut texture_manager = ResourceManager::new();

    let odd_color = TextureConfig {
        kind: TextureKind::Constant(Color::new(51, 77, 26)),
    };
    let even_color = TextureConfig {
        kind: TextureKind::Constant(Color::new(230, 230, 230)),
    };

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Checker(&odd_color, &even_color),
            }),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Checker(&odd_color, &even_color),
            }),
        }),
    }));

    world
}

fn two_perlin_spheres(background: &Vec3) -> HitableList {
    let background = make_sky_material(&Vec3::new(1.0, 1.0, 1.0), background);

    let mut world = HitableList::new(background);

    let mut texture_manager = ResourceManager::new();

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Perlin(256, 4),
            }),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Perlin(256, 4),
            }),
        }),
    }));

    world
}

fn earth(background: &Vec3) -> HitableList {
    let mut texture_manager = ResourceManager::new();
    let earth_texture = texture_manager.get_resource(&TextureConfig {
        kind: TextureKind::FromFile(std::path::Path::new("assets/images/earthmap.jpg")),
    });
    let earth_surface = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    let globe = Arc::new(Sphere {
        center: Vec3::default(),
        radius: 2.0,
        material: earth_surface,
    });

    let background = make_sky_material(&Vec3::new(1.0, 1.0, 1.0), background);

    let mut world = HitableList::new(background);

    world.push(globe);

    world
}

struct BlackMaterial;

impl Material for BlackMaterial {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }
}

fn simple_light() -> HitableList {
    let background = Arc::new(BlackMaterial {});
    let mut world = HitableList::new(background);

    let mut texture_manager = ResourceManager::new();
    let perlin_texture = texture_manager.get_resource(&TextureConfig {
        kind: TextureKind::Perlin(256, 4),
    });
    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: perlin_texture.clone(),
        }),
    }));
    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Arc::new(Lambertian {
            albedo: perlin_texture.clone(),
        }),
    }));

    let light = Arc::new(Emissive {
        emit: texture_manager.get_resource(&TextureConfig {
            kind: TextureKind::Constant(Color {
                r: 255,
                g: 255,
                b: 255,
            }),
        }),
        multiplier: 4.0,
    });
    world.push(Arc::new(XYRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        material: light,
    }));

    world
}

/// Compute the color of the current ray
/// in the world of hitables.
/// This function run recursively until maximum number of recursions
/// (depth parameter) is reached or no hitable is hit.
fn color(ray: &Ray, background: &Arc<dyn Material>, world: &dyn Hitable, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::default();
    }

    let record = world.hit(&ray, 0.001, std::f64::MAX);
    match record {
        Some(rec) => {
            let res = rec.material.scatter(&ray, &rec);
            let emitted = rec.material.emit(rec.u, rec.v, &rec.p);
            if let Some((attenuation, scattered)) = res {
                return emitted + attenuation * color(&scattered, background, world, depth - 1);
            }
            emitted
        }
        None => {
            let emitted = background.emit(0.0, 0.0, &ray.direction());
            emitted
        }
    }
}

fn gamma(color: Vec3) -> Vec3 {
    Vec3 {
        x: color.x.sqrt(),
        y: color.y.sqrt(),
        z: color.z.sqrt(),
    }
}

/// Dispatch ray-tracing algorithm on several threads to create an image of the current scene
fn render(
    image_width: u32,
    image_height: u32,
    sample_per_pixel: u32,
    world: HitableList,
    camera: Arc<dyn Camera>,
) -> Image {
    let max_depth: u32 = 10;
    let thread_count = debug_limiter(16, 16);

    let mut handles = vec![];
    let lines: Arc<Vec<u32>> = Arc::new((0..image_height).rev().collect());

    let lines_per_thread = lines.len() / thread_count;
    let tougher_threads = lines.len() % thread_count;
    let mut offset = 0;

    let (tx, rx) = mpsc::channel();

    // Compute a BVH of the scene
    let bvh = BVHNode::new(&world, 0.0, 0.1);

    for id in 0..thread_count {
        let chunksize = if id < tougher_threads {
            lines_per_thread + 1
        } else {
            lines_per_thread
        };

        let camera = camera.clone();
        let lines = lines.clone();
        let tx = tx.clone();
        let bvh = bvh.clone();
        let background = world.background.clone();

        handles.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut pixels = vec![];
            let end = offset + chunksize;
            for j in &lines[offset..end] {
                for i in 0..image_width {
                    let mut col = Vec3::default();
                    for _s in 0..sample_per_pixel {
                        let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(image_width);
                        let v = (f64::from(*j) + rng.gen::<f64>()) / f64::from(image_height);
                        let r = camera.get_ray(u, v);

                        let _p = r.point_at(2.0);
                        col += color(&r, &background, &bvh, max_depth);
                    }

                    col /= f64::from(sample_per_pixel);
                    let col = gamma(col);

                    pixels.push(col.x);
                    pixels.push(col.y);
                    pixels.push(col.z);
                }
                tx.send(1).unwrap();
            }
            pixels
        }));
        offset += chunksize;
    }

    // print status
    // lines is a reversed list of number, so it work well to count
    for i in lines.iter() {
        rx.recv().unwrap();
        eprint!("Scanlines remaining: {}    \r", i);
    }

    let mut buffer = Vec::new();

    for handle in handles {
        let mut pixels = handle.join().unwrap();
        buffer.append(&mut pixels);
    }

    Image::from(image_width, image_height, PixelFormat::RGBU8, &buffer)
}

/// divise a number by a divisor when debug is the active target
fn debug_limiter<T: Div<Output = T> + Copy>(number: T, divisor: T) -> T {
    if cfg!(debug_assertions) {
        number / divisor
    } else {
        number
    }
}

fn main() {
    let image_width: u32 = debug_limiter(1920, 4);
    let image_height: u32 = debug_limiter(1080, 4);
    let sample_per_pixel: u32 = debug_limiter(100, 50);

    // Create a scene
    let scene = 4;
    let (world, lookfrom, lookat, vfov, aperture) = match scene {
        0 => (
            random_scene(&Vec3::new(0.5, 0.7, 1.0)),
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::default(),
            20.0,
            0.1,
        ),
        1 => (
            two_spheres(&Vec3::new(0.5, 0.7, 1.0)),
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::default(),
            20.0,
            0.0,
        ),
        2 => (
            two_perlin_spheres(&Vec3::new(0.5, 0.7, 1.0)),
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::default(),
            20.0,
            0.0,
        ),
        3 => (
            earth(&Vec3::new(0.5, 0.7, 1.0)),
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::default(),
            20.0,
            0.0,
        ),
        4 | _ => (
            simple_light(),
            Vec3::new(26.0, 3.0, 6.0),
            Vec3::new(0.0, 2.0, 0.0),
            20.0,
            0.0,
        ),
    };

    let aspect_ratio = f64::from(image_width) / f64::from(image_height);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = ThinLensCamera::new_look_at(
        lookfrom,
        lookat,
        up,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let before = Instant::now();

    let image = render(
        image_width,
        image_height,
        sample_per_pixel,
        world,
        Arc::from(cam),
    );

    eprintln!("Done in {}secs!           ", before.elapsed().as_secs());

    println!("{}", image.encode(ImageFormat::PPM));
}
