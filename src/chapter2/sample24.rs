use std::f64::consts::PI;
use stdweb::web::CanvasRenderingContext2d;
use {get_param, show_params, Point};

pub fn draw(context: &CanvasRenderingContext2d) {
    let mut points = vec![];

    show_params();
    /*读取参数*/
    let un = get_param(0).parse::<f64>().unwrap_or(12.0);

    let uv = 360.0 / un;
    let k = uv / 2.0;
    let sc = uv / 100.0;

    let mut a = 0.0;
    while a <= 2.0 * PI {
        let d = 90.0 * (4.0 / 5.0 + 1.0 / 5.0 * (12.0 * a).sin());
        let dd = d * (1.0 / 2.0 + 1.0 / 2.0 * (4.0/*花瓣数*/* a).sin());
        points.push(Point {
            x: dd * a.cos(),
            y: dd * a.sin(),
        });
        a += PI / 60.0;
    }

    let r = 150.0;
    context.set_stroke_style_color("#000");
    context.begin_path();

    for px in (-180 + k as i32..=180 - k as i32)
        .step_by(uv as usize)
        .map(|x| x as f64)
    {
        for py in (-180 + k as i32..=180 - k as i32)
            .step_by(uv as usize)
            .map(|y| y as f64)
        {
            let (mut bx, mut by) = (0.0, 0.0);

            for i in 0..points.len() {
                let point = &points[i];
                let (mut x1, y1) = (point.x * sc + px, point.y * sc + py);
                let sq = x1 * x1 + y1 * y1;
                let (x, y) = if sq < r * r {
                    let s = if x1 < 0.0 { -1.0 } else { 1.0 };
                    if x1 == 0.0 {
                        x1 = 0.1;
                    }
                    let l = sq.sqrt();
                    let bt = 2.0 * (l / r).atan();
                    let th = (y1 / x1).atan();
                    let m = r * bt.sin();
                    (s * m * th.cos(), s * m * th.sin())
                } else {
                    (x1, y1)
                };

                let (x, y) = (x + 320.0, y + 200.0);
                if i == 0 {
                    context.move_to(x, y);
                    bx = x;
                    by = y;
                } else {
                    context.line_to(x, y);
                }
            }
            context.line_to(bx, by);
        }
    }

    context.stroke();
}
