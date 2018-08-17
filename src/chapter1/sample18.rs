use stdweb::web::CanvasRenderingContext2d;
use {random, rgb};
use std::f64::consts::PI;

pub fn draw(context: &CanvasRenderingContext2d){
    context.set_stroke_style_color(&rgb(237, 90, 101));

    for _ in 0..50{
        context.begin_path();
        let x = random() * 400.0;
        let y = random() * 400.0;
        let radius = random() * 40.0;
        context.arc(x, y, radius, 0.00001, PI*2.0, false);
        context.stroke();   
    }
}