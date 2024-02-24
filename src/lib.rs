use image::{ImageBuffer, Rgba};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::*;

use crate::canny_edge::canny_edge_detector;

mod canny_edge;
mod conv_2d;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(800);
    canvas.set_height(800);
    canvas.style().set_property("border", "solid")?;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    pub const IMAGE_BYTES: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/",
        "happy-tree.png"
    ));

    let image = image::load_from_memory_with_format(&IMAGE_BYTES, image::ImageFormat::Png).unwrap();
    let rgba_image = image.as_rgba8().unwrap();
    let rgba_image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(rgba_image.as_raw()),
        image.width(),
        image.height(),
    )
    .unwrap();

    context.put_image_data(&rgba_image_data, 0.0, 0.0)?;
    context.put_image_data(&rgba_image_data, image.width() as f64, 0.0)?;
    context.put_image_data(&rgba_image_data, 2.0 * image.width() as f64, 0.0)?;

    let binding = image.to_luma8();
    let luma_image = binding.as_raw();
    // let canny_edge_image = canny_edge_detector(&luma_image);

    let rgba_luma_image: Vec<u8> = luma_image
        .into_iter()
        .map(|pix| [*pix, *pix, *pix, 255])
        .flatten()
        .collect();

    let luma_image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(rgba_luma_image.as_slice()),
        image.width(),
        image.height(),
    )
    .unwrap();

    context.put_image_data(&luma_image_data, 0.0, image.height() as f64)?;
    context.put_image_data(
        &rgba_image_data,
        image.width() as f64,
        image.height() as f64,
    )?;

    Ok(())
}
