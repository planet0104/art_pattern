#[macro_use]
extern crate stdweb;
mod chapter1;

use stdweb::unstable::TryInto;
use stdweb::traits::*;
use stdweb::web::{Element, document, CanvasRenderingContext2d};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::INode;
use stdweb::web::event::{ ClickEvent, IEvent };

fn random() -> f64{
    js!(return Math.random(); ).try_into().unwrap()
}
fn rand_int(l: i32, b: i32) -> i32{
	(random()*(b as f64 - l as f64 + 1.0) + l as f64).floor() as i32
}
fn rgb(r:i32, g:i32, b:i32) -> String{
    format!("rgb({},{},{})", r, g, b)
}

fn main(){
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    //创建菜单
    let ul = document().create_element("ul").unwrap();
    ul.append_child({&elem_content("li", "第一部分 BASIC语言绘图入门")});
    ul.append_child(&{
        let ch = document().create_element("ul").unwrap();
        ch.append_child(&handle_click(context.clone(), chapter1::sample1::draw, "例1-1 画一条正弦曲线和余弦曲线", include_str!("chapter1/sample1.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample3::draw, "例1-3 在屏幕上每隔10点画一条竖线", include_str!("chapter1/sample3.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample4::draw, "例1-4 利用画线语句画一架飞机", include_str!("chapter1/sample4.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample5::draw, "例1-5 几何图案", include_str!("chapter1/sample5.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample6::draw, "例1-6 花形图案", include_str!("chapter1/sample6.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample7::draw, "例1-7 画一个由小到大变化的彩色矩形", include_str!("chapter1/sample7.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample8::draw, "例1-8 由小到大变化的着色矩形", include_str!("chapter1/sample8.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample14::draw, "例1-14 画同心圆图案", include_str!("chapter1/sample14.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample15::draw, "例1-15 画向右连续移动圆的图案", include_str!("chapter1/sample15.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample16::draw, "例1-16 画圆弧程序例", include_str!("chapter1/sample16.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample17::draw, "例1-17 使用arc画椭圆", include_str!("chapter1/sample17.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample18::draw, "例1-18 随机画圆", include_str!("chapter1/sample18.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample19::draw, "例1-19 五彩气泡", include_str!("chapter1/sample19.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample20::draw, "例1-20 紫红色三角形", include_str!("chapter1/sample20.rs")));
        ch.append_child(&handle_click(context.clone(), chapter1::sample21::draw, "例1-21 带字符图案的涂色立方体", include_str!("chapter1/sample21.rs")));
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

fn handle_click(context: CanvasRenderingContext2d, draw: fn(&CanvasRenderingContext2d), title: &str, code:&str) -> Element{
    let li = document().create_element("li").unwrap();
    let button = elem_content("button", title);
    li.append_child(&button);
    let t = String::from(title);
    button.add_event_listener(move |event: ClickEvent|{
        js!{ console.log(@{event.event_type()}, @{&t}); }
        context.set_fill_style_color("#ffffff");
        context.clear_rect(0.0, 0.0, 800.0, 600.0);
        js!(close_code());
        draw(&context);
    });

    let close = elem_content("button", "代码");
    li.append_child(&close);
    let code = String::from(code);
    let t = String::from(title);
    close.add_event_listener(move |_event: ClickEvent|{
        js!(show_code(@{&t}, @{&code}));
    });

    li
}

/// Create an element and set its content.
fn elem_content(elem_type: &str, content: &str) -> Element {
    let elem = document().create_element(elem_type).unwrap();
    elem.set_text_content(content);
    elem
}