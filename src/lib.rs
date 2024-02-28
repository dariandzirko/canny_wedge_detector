use image::{imageops::blur, ImageBuffer, Rgba};
use num::traits::float;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::*;

use oxidized_image_processing::{self, float_image, helper_ops::conv_2d, kernel::Kernel};

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

    let binding = image.to_luma8();

    //Before blur
    let luma_img = binding.as_raw();

    let rgba_luma_image: Vec<u8> = luma_img
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

    context.put_image_data(&luma_image_data, 0.0, 0.0)?;

    //After blur
    let mut blurred_float_image = float_image::FloatImage::from_luma8(binding);
    blurred_float_image.matrix = conv_2d(
        &mut Kernel::gaussian_2d(5.0).matrix,
        &blurred_float_image.matrix,
        true,
    );

    let blurred_binding = blurred_float_image.to_luma8();
    let blurred_luma_image = blurred_binding.as_raw();

    let blurred_rgba_luma_image: Vec<u8> = blurred_luma_image
        .into_iter()
        .map(|pix| [*pix, *pix, *pix, 255])
        .flatten()
        .collect();

    let blurred_luma_image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(blurred_rgba_luma_image.as_slice()),
        image.width(),
        image.height(),
    )
    .unwrap();

    context.put_image_data(&blurred_luma_image_data, 0.0, image.height() as f64)?;

    Ok(())
}
