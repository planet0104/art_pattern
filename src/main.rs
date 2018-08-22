#[macro_use]
extern crate stdweb;
mod chapter1;
mod chapter2;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, IEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::html_element::InputElement;
use stdweb::web::INode;
use stdweb::web::{document, CanvasRenderingContext2d, Element};

#[derive(Debug, Copy, Clone)]
struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

fn random() -> f64 {
    js!(return Math.random(); ).try_into().unwrap()
}
fn rand_int(l: i32, b: i32) -> i32 {
    (random() * (b as f64 - l as f64 + 1.0) + l as f64).floor() as i32
}
fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("rgb({},{},{})", r, g, b)
}

fn get_param(i: i32) -> String {
    let param: InputElement = document()
        .query_selector(&format!("#param{}", i))
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    param.raw_value()
}

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    //创建菜单
    let ul = document().create_element("ul").unwrap();
    ul.append_child({ &elem_content("li", "第一章 绘图入门") });
    ul.append_child(&{
        let ch = document().create_element("ul").unwrap();
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample1::draw,
            "例1-1 画一条正弦曲线和余弦曲线",
            include_str!("chapter1/sample1.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample3::draw,
            "例1-3 在屏幕上每隔10点画一条竖线",
            include_str!("chapter1/sample3.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample4::draw,
            "例1-4 利用画线语句画一架飞机",
            include_str!("chapter1/sample4.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample5::draw,
            "例1-5 几何图案",
            include_str!("chapter1/sample5.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample6::draw,
            "例1-6 花形图案",
            include_str!("chapter1/sample6.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample7::draw,
            "例1-7 画一个由小到大变化的彩色矩形",
            include_str!("chapter1/sample7.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample8::draw,
            "例1-8 由小到大变化的着色矩形",
            include_str!("chapter1/sample8.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample14::draw,
            "例1-14 画同心圆图案",
            include_str!("chapter1/sample14.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample15::draw,
            "例1-15 画向右连续移动圆的图案",
            include_str!("chapter1/sample15.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample16::draw,
            "例1-16 画圆弧程序例",
            include_str!("chapter1/sample16.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample17::draw,
            "例1-17 使用arc画椭圆",
            include_str!("chapter1/sample17.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample18::draw,
            "例1-18 随机画圆",
            include_str!("chapter1/sample18.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample19::draw,
            "例1-19 五彩气泡",
            include_str!("chapter1/sample19.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample20::draw,
            "例1-20 紫红色三角形",
            include_str!("chapter1/sample20.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample21::draw,
            "例1-21 带字符图案的涂色立方体",
            include_str!("chapter1/sample21.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter1::sample27::draw,
            "例1-27 画正弦函数的字符图形",
            include_str!("chapter1/sample27.rs"),
        ));
        ch
    });

    ul.append_child({ &elem_content("li", "第二章 图案变换及操作") });
    ul.append_child(&{
        let ch = document().create_element("ul").unwrap();
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample2::draw,
            "例2-2 6x6个花型图案程序",
            include_str!("chapter2/sample2.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample3::draw,
            "例2-3 对三角形进行比例变换的程序",
            include_str!("chapter2/sample3.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample4::draw,
            "例2-4 将三角形进行旋转的程序",
            include_str!("chapter2/sample4.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample5::draw,
            "例2-5 将三角形以屏幕中心为旋转基准点的程序",
            include_str!("chapter2/sample5.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample6::draw,
            "例2-6 旋转小正方形程序-1",
            include_str!("chapter2/sample6.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample7::draw,
            "例2-7 旋转小正方形程序-2",
            include_str!("chapter2/sample7.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample8::draw,
            "例2-8 旋转小正方形程序-3",
            include_str!("chapter2/sample8.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample9::draw,
            "例2-9 矩阵图形变换",
            include_str!("chapter2/sample9.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample10::draw,
            "例2-10 抛物线到圆的渐变",
            include_str!("chapter2/sample10.rs"),
        ));
        // ch.append_child(&handle_click(
        //     context.clone(),
        //     chapter2::sample11::draw,
        //     "例2-11 由蝶形变化为苜蓿叶形",
        //     include_str!("chapter2/sample11.rs"),
        // ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample12::draw,
            "例2-12 由圆渐变为正方形",
            include_str!("chapter2/sample12.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample13::draw,
            "例2-13 由苜蓿叶渐变到圆",
            include_str!("chapter2/sample13.rs"),
        ));
        ch.append_child(&handle_click(
            context.clone(),
            chapter2::sample14::draw,
            "例2-14 渐变图形间隔按对数规律分布",
            include_str!("chapter2/sample14.rs"),
        ));
        ch
    });

    document().body().unwrap().append_child(&ul);

    document().body().unwrap().append_child(&canvas);

    canvas.set_width(800);
    canvas.set_height(600);
    chapter1::sample1::draw(&context);
    stdweb::event_loop();
}

fn handle_click(
    context: CanvasRenderingContext2d,
    draw: fn(&CanvasRenderingContext2d),
    title: &str,
    code: &str,
) -> Element {
    let li = document().create_element("li").unwrap();
    let button = elem_content("button", title);
    li.append_child(&button);
    let t = String::from(title);
    button.add_event_listener(move |event: ClickEvent| {
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
    close.add_event_listener(move |_event: ClickEvent| {
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
