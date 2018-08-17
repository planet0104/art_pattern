use stdweb::web::CanvasRenderingContext2d;
use rgb;

pub fn draw(context: &CanvasRenderingContext2d){
    context.set_stroke_style_color(&rgb(0, 0, 0));
    let mut i = 1.0;
    while i<=640.0{
        context.begin_path();
        context.arc(i, 100.0, 50.0, 0.0, ::std::f64::consts::PI*2.0, true);
        context.stroke();
        i += 20.0;
    }
}