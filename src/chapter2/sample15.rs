use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    

    let text =r#"
    原图中任意点的P(X1,Y1)经过扇形变换后，点P2(X2,Y2)的坐标为:
    X2 = [L + Y1] * COS[TH] + X0
    Y2 = -[L + Y1] * SIN[TH] + X0
    式中
    TH = C * [X - X1] / X + B
    X0, Y0为圆心坐标，L为扇形半径，B为图形在扇形中的角度。
    "#;

    context.set_fill_style_color("#aaa");
    context.set_font("12pt Arial");
    let mut y = 400.0;
    for line in text.lines(){
        context.fill_text(line, 10.0, y, None);
        y += 22.0;
    }
    context.set_stroke_style_color("#000");

    let x = 640.0;
    let y = 300.0;

    /*扇形半径 */
    let l = 280.0;

    /*扇形圆心坐标 */
    let (x0,y0) = (320.0,700.0);

    let c = PI / 3.0;
    let r = x / 9.0;

    context.begin_path();

    let mut px = x / 16.0;

    while px <= 15.0 * x / 16.0 + x / 16.0 {
        let mut py = y / 8.0;
        while py <= 7.0 * y / 8.0 {
            let mut a = 0.0;
            while a < 2.0 * PI + PI / 60.0 {
                let d = r * (4.0 / 5.0 + 1.0 / 5.0 * (12.0 * a).sin());
                let t = d * (0.5 + 0.5 * (4.0 * a).sin());
                let x1 = t * a.cos() + px;
                let y1 = t * a.sin() + py;

                /*变换 */
                let th = c * (x - x1) / x + 1.0; /*+b */
                let x2 = (l + y1) * th.cos() + x0;
                let y2 = -(l + y1) * th.sin() + y0;

                /*画线 */
                if a == 0.0 {
                    context.move_to(x2, y2 / 1.5);
                } else {
                    context.line_to(x2, y2 / 1.5);
                }
                a += PI / 64.0;
            }

            py += y / 4.0;
        }

        px += x / 8.0;
    }

    context.stroke();
}
