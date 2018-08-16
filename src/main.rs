#[macro_use]
extern crate stdweb;
mod chapter1;

use stdweb::unstable::TryInto;
use stdweb::traits::*;
use stdweb::web::{Element, document, CanvasRenderingContext2d};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::INode;
use stdweb::web::event::{ ClickEvent, IEvent };

fn main(){
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    //创建菜单
    let ul = document().create_element("ul").unwrap();
    ul.append_child({&elem_content("li", "第一部分 BASIC语言绘图入门")});
    ul.append_child(&{
        let ch = document().create_element("ul").unwrap();
        ch.append_child(&handle_click(canvas.clone(), context.clone(), "例1-1 画一条正弦曲线和余弦曲线", 1, 1));
        ch.append_child(&handle_click(canvas.clone(), context.clone(), "例1-3 在屏幕上每隔10点画一条竖线", 1, 3));
        ch.append_child(&handle_click(canvas.clone(), context.clone(), "例1-4 利用画线语句画一架飞机", 1, 4));
        ch.append_child(&handle_click(canvas.clone(), context.clone(), "例1-5 几何图案", 1, 5));
        ch.append_child(&handle_click(canvas.clone(), context.clone(), "例1-6 花形图案", 1, 6));
        ch
    });

    ul.append_child({&elem_content("li", "第二部分 平面图形程序")});

    document().body().unwrap().append_child(&ul);

    document().body().unwrap().append_child(&canvas);

    canvas.set_width(800);
    canvas.set_height(600);
    chapter1::sample1::draw(&context);
    stdweb::event_loop();
}

fn handle_click(_canvas: CanvasElement, context: CanvasRenderingContext2d, title: &str, chapter: i32, sample:i32) -> Element{
    let li = document().create_element("li").unwrap();
    let button = elem_content("button", title);
    li.append_child(&button);
    button.add_event_listener(move |event: ClickEvent|{
        js!{ console.log(@{event.event_type()}, @{chapter}, @{sample}); }
        context.set_fill_style_color("#ffffff");
        context.clear_rect(0.0, 0.0, 800.0, 600.0);
        match chapter{
            1 => {
                match sample{
                    1 => chapter1::sample1::draw(&context),
                    3 => chapter1::sample3::draw(&context),
                    4 => chapter1::sample4::draw(&context),
                    5 => chapter1::sample5::draw(&context),
                    6 => chapter1::sample6::draw(&context),
                    _ => println!("未定义"),
                }
            },
            _ => {
                println!("未定义");
            }
        }
    });

    li
}

/// Create an element and set its content.
fn elem_content(elem_type: &str, content: &str) -> Element {
    let elem = document().create_element(elem_type).unwrap();
    elem.set_text_content(content);
    elem
}