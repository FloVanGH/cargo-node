extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, CanvasRenderingContext2d};

use stdweb::web::html_element::CanvasElement;

pub fn say_hello() {
    stdweb::initialize();

    let canvas: CanvasElement = document()
        .create_element("canvas")
        .unwrap()
        .try_into()
        .unwrap();

    canvas.set_width(200);
    canvas.set_height(100);

    document().body().unwrap().append_child(&canvas);

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    context.set_font("bold 16px serif");
    context.fill_text("Hello cargo-node", 20.0, 20.0, None);
}
