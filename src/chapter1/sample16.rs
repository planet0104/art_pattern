use stdweb::web::CanvasRenderingContext2d;
use rgb;
use std::f64::consts::PI;

pub fn draw(context: &CanvasRenderingContext2d){
    context.set_stroke_style_color(&rgb(0, 0, 0));
    let mut i = 30.0;
    while i<=180.0{
        context.begin_path();

        let (x, y, r, start_angle, end_angle) = (i/30.0*110.0, 55.0, 50.0, -0.0001, -i*PI/180.0);
        /*画圆弧*/
        context.arc(x, y, r, start_angle, end_angle, true);
        /*圆弧终点到圆心的线*/
        context.line_to(x, y);
        /*画圆心到圆弧起点的线*/
        context.close_path();

        context.stroke();

        i += 30.0;
    }

    i = 210.0;
    while i<=330.0{
        let (x, y, r, start_angle, end_angle) = (i/30.0*110.0-660.0, 110.0, 50.0, -0.4, -i*PI/180.0);

        context.begin_path();
        context.arc(x, y, r, start_angle, end_angle, true);
        /*圆弧终点到圆心的线*/
        context.line_to(x, y);
        /*画圆心到圆弧起点的线*/
        context.close_path();

        context.stroke();

        i += 30.0;
    }
}