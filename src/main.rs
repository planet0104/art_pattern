#[macro_use]
extern crate stdweb;
mod chapter1;

use stdweb::unstable::TryInto;
use stdweb::traits::*;
use stdweb::web::{document, window, CanvasRenderingContext2d};
use stdweb::web::html_element::CanvasElement;
use chapter1::*;

fn main(){
    stdweb::initialize();
    
    js!{
        var c = document.createElement("canvas");
        c.id = "canvas";
        document.body.appendChild(c);
    };
    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    let window = window();

    canvas.set_width(window.inner_width() as u32);
    canvas.set_height(window.inner_height() as u32);

    sample6::draw(&context);

    stdweb::event_loop();
}