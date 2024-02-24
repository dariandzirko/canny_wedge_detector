use image::ImageBuffer;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::*;

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

    pub const image_bytes: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/",
        "happy-tree.png"
    ));

    let image = image::load_from_memory_with_format(&image_bytes, image::ImageFormat::Png).unwrap();
    let rgba_image = image.as_rgba8().unwrap();

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(rgba_image.as_raw()),
        image.width(),
        image.height(),
    )
    .unwrap();

    context.put_image_data(&image_data, 0.0, 0.0)?;

    Ok(())
}
