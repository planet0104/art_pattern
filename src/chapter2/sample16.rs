use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use Point;

pub fn draw(context: &CanvasRenderingContext2d) {
    let text = r#"
        带状图形上任意点P(X1,Y1)变换为环状图上点P(X2,Y2)，其坐标之间关系为:
        X2 = [L + Y1] * COS[TH] + PX
        Y2 = -[L + Y1] * SIN[TH] + PY
        式中
        TH = 2π * [X - X1] / X
        L为环形内径的1/2，L的值越小，变换后的图案变形也越大。
    "#;

    context.set_fill_style_color("#aaa");
    context.set_font("12pt Arial");
    let mut y = 400.0;
    for line in text.lines() {
        context.fill_text(line, 10.0, y, None);
        y += 22.0;
    }
    context.set_stroke_style_color("#000");

    /*高w，宽x，半径l */
    let (w, x) = (150.0, 1200.0);
    let l = 50.0;

    context.begin_path();

    let mut px = w / 2.0;
    let mut start = Point::new();
    while px <= 7.0 * w + w / 2.0 {
        let mut a = 0.0;
        while a < 2.0 * PI {
            let d = w / 2.0 * (4.0 / 5.0 + 1.0 / 5.0 * (18.0 * a).sin()) * 1.2;
            let t = d * (1.0 / 2.0 + 1.0 / 2.0 * (6.0 * a).sin());
            let x1 = px + t * a.cos();
            let y1 = w / 2.0 - t * a.sin();

            /*变换 */
            let th = 2.0 * PI * (x - x1) / x;
            let x2 = (l + y1) * th.cos() + 320.0;
            let y2 = (l + y1) * th.sin() + 200.0;

            /*画线 */
            if a == 0.0 {
                context.move_to(x2, y2);
                start.x = x2;
                start.y = y2;
            } else {
                context.line_to(x2, y2);
            }
            a += PI / 84.0; /*步进越画的图越精细 */
        }
        context.line_to(start.x, start.y);
        px += w;
    }

    context.stroke();
}
