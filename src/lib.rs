mod vec3;
mod ray;

use ray::Ray;
use vec3::Vec3;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

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


fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> bool
{
    let oc = ray.origin() - center;
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * Vec3::dot(oc, ray.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    
    return discriminant > 0.0;
}


fn ray_color(ray: Ray) ->Vec<u8>
{
    let center = Vec3{x: 0.0, y: 0.0, z: -1.0};
    if hit_sphere(center, 0.5, ray.clone())
    {
        return vec![255, 0, 0, 255];
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


fn render_image(image_width: u32, image_height: u32, camera: Camera) -> Vec<u8>
{
    let mut image = Vec::new();

    for column in (0..image_height).rev()
    {
        for row in 0..image_width
        {
            let u = row as f64 / (image_width) as f64;
            let v = column as f64 / (image_height) as f64;

            let ray = Ray{origin: camera.origin, direction: camera.lower_left_corner + (u * camera.horizontal) + (v * camera.vertical) - camera.origin};

            image.extend(ray_color(ray));
        }
    }



    
    
    log("Done\n");
    return image;
}


fn get_image(image_width: u32, image_height: u32, camera: Camera) ->Vec<u8>
{
    return  render_image(image_width,image_height, camera );
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


    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_radio * viewport_height;
    let focal_lenght = 1.0;

    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vec3 { x: 0.0, y: 0.0, z: focal_lenght };

    let camera = Camera{origin: origin, horizontal:horizontal, vertical:vertical, lower_left_corner: lower_left_corner};


    let mut data = get_image(image_width, image_height, camera);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), image_width, image_height).unwrap();
    context.put_image_data(&data, 0.0, 0.0).unwrap();


}