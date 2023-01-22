use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;


fn get_image() ->Vec<u8>
{

    let mut image = Vec::new();

    for x in 0..800
    {
        for y in 0..800
        {
            let r = (x as f64) / (800.0 - 1.0);
            let g = (y as f64) / (800.0 - 1.0);
            let b = 0.25;


            let ir = (r * 255.999) as u8;
            let ig = (g * 255.999) as u8;
            let ib = (b * 255.999) as u8;

            image.push(ir);
            image.push(ig);
            image.push(ib);
            image.push(255);
        }
    }

    return image;
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



    let mut data = get_image();

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), 800, 800).unwrap();
    context.put_image_data(&data, 0.0, 0.0).unwrap();


}