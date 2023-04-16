mod vec3;
mod ray;
mod hittable_object;
mod hitable_list;
mod sphere;


use hittable_object::HitRecord;
use ray::Ray;
use vec3::Vec3;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;
use hittable_object::HittableObject;
use hitable_list::HitableList;
use sphere::Sphere;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(s: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(s: f64);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn ray_color<T: HittableObject>(ray: &Ray,  word: &HitableList<T>) ->Vec<u8>
{
    let mut record = HitRecord::default();
    if word.hit(&ray, 0.0, f64::INFINITY, &mut record)
    {
        let ir = ((0.5 * (record.normal.x + 1.0)) * 255.999) as u8;
        let ig = ((0.5 * (record.normal.y + 1.0)) * 255.999) as u8;
        let ib = ((0.5 * (record.normal.z + 1.0)) * 255.999) as u8;
        let ia = 255;

        return vec![ir, ig, ib, ia];
    }

    let unit_direction = Vec3::get_unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    let color = (1.0 - t) * Vec3{x: 1.0, y: 1.0, z: 1.0} + t * Vec3{x: 0.5, y: 0.7, z: 1.0};

    let ir = (color.x * 255.999) as u8;
    let ig = (color.y * 255.999) as u8;
    let ib = (color.z * 255.999) as u8;
    let ia = 255;

    return vec![ir, ig, ib, ia];
}



fn render_image<T: HittableObject>(image_width: u32, image_height: u32, camera: Camera, word: &HitableList<T>) -> Vec<u8>
{
    let mut image = Vec::new();

    for column in (0..image_height).rev()
    {
        for row in 0..image_width
        {
            let u = row as f64 / (image_width) as f64;
            let v = column as f64 / (image_height) as f64;

            let ray = Ray{origin: camera.origin, direction: camera.lower_left_corner + (u * camera.horizontal) + (v * camera.vertical) - camera.origin};

            image.extend(ray_color(&ray, &word));
        }
    }



    
    
    log("Done\n");
    return image;
}


fn get_image<T: HittableObject>(image_width: u32, image_height: u32, camera: Camera, word: &HitableList<T>) ->Vec<u8>
{
    return  render_image(image_width,image_height, camera, &word);
}




pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3
}


#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();


    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();



    //Image
    let aspect_radio = 16.0 / 9.0;
    let image_width = canvas.width();
    let image_height = canvas.height();

    log_u32(image_width);
    log_u32(image_height);

    //World
    let mut word = HitableList::new();
    word.add(Sphere { center: Vec3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5});
    word.add(Sphere { center: Vec3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0});

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_radio * viewport_height;
    let focal_lenght = 1.0;


    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vec3 { x: 0.0, y: 0.0, z: focal_lenght };

    let camera = Camera{origin: origin, horizontal:horizontal, vertical:vertical, lower_left_corner: lower_left_corner};

    //render
    let mut data = get_image(image_width, image_height, camera, &word);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), image_width, image_height).unwrap();
    context.put_image_data(&data, 0.0, 0.0).unwrap();


}