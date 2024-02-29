use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::*;

use oxidized_image_processing::{self, float_image, helper_ops::conv_2d, kernel::Kernel};

pub fn put_image_to_canvas(
    image: &DynamicImage,
    context: &CanvasRenderingContext2d,
    dx: f64,
    dy: f64,
    rgba: bool,
) {
    let image_data = if rgba {
        let image_buffer = image.to_rgba8();
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(image_buffer.as_raw()),
            image.width(),
            image.height(),
        )
        .unwrap()
    } else {
        let image_buffer = image.to_luma8();
        let image_buffer_raw = image_buffer.as_raw();
        let rgba_image: Vec<u8> = image_buffer_raw
            .into_iter()
            .map(|pix| [*pix, *pix, *pix, 255])
            .flatten()
            .collect();
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(rgba_image.as_slice()),
            image.width(),
            image.height(),
        )
        .unwrap()
    };

    context.put_image_data(&image_data, dx, dy).unwrap();
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    // let canvas = document
    //     .create_element("canvas")?
    //     .dyn_into::<web_sys::HtmlCanvasElement>()?;
    // document.body().unwrap().append_child(&canvas)?;
    // canvas.set_width(800);
    // canvas.set_height(800);
    // canvas.style().set_property("border", "solid")?;

    // let context = canvas
    //     .get_context("2d")
    //     .unwrap()
    //     .unwrap()
    //     .dyn_into::<web_sys::CanvasRenderingContext2d>()
    //     .unwrap();

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

    let image = context.get_image_data(0.0, 0.0, 256.0, 256.0).unwrap();

    let clamped_data = image.data();
    let data = clamped_data.0;
    let image = RgbaImage::from_vec(256, 256, data).unwrap();
    let dyn_image = DynamicImage::from(image);

    // //Before blur
    put_image_to_canvas(&dyn_image, &context, 200.0, 200.0, true);

    //After blur
    let binding = dyn_image.to_luma8();
    let mut blurred_float_image = float_image::FloatImage::from_luma8(binding);
    blurred_float_image.matrix = conv_2d(
        &mut Kernel::gaussian_2d(5.0).matrix,
        &blurred_float_image.matrix,
        true,
    );

    let blurred_binding = blurred_float_image.to_luma8();
    let dyn_image = DynamicImage::from(blurred_binding);
    put_image_to_canvas(&dyn_image, &context, 250.0, 250.0, true);

    Ok(())
}
