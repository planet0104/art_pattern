use ::stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d){
    context.set_stroke_style_color("#663366");
    context.set_line_width(1.0);
    context.begin_path();

    let mut x = 0.0;
    while x<320.0{
        context.move_to(x, 0.0);
        context.line_to(x, 200.0);
        x += 10.0;
    }
    context.close_path();
    context.stroke();

    context.stroke_text("例1-3 在屏幕上每隔10点画一条竖线", 10.0, 260.0, None);
}