use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context
        .arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI)
        .unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();

    // let context = Rc::new(context);
    // let pressed = Rc::new(Cell::new(false));
    // {
    //     let context = context.clone();
    //     let pressed = pressed.clone();
    //     let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
    //         context.begin_path();
    //         context.move_to(event.offset_x() as f64, event.offset_y() as f64);
    //         pressed.set(true);
    //     });
    //     canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }
    // {
    //     let context = context.clone();
    //     let pressed = pressed.clone();
    //     let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
    //         if pressed.get() {
    //             context.line_to(event.offset_x() as f64, event.offset_y() as f64);
    //             context.stroke();
    //             context.begin_path();
    //             context.move_to(event.offset_x() as f64, event.offset_y() as f64);
    //         }
    //     });
    //     canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }
    // {
    //     let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
    //         pressed.set(false);
    //         context.line_to(event.offset_x() as f64, event.offset_y() as f64);
    //         context.stroke();
    //     });
    //     canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }

    Ok(())
}
