use stdweb::web::CanvasRenderingContext2d;

pub fn draw(context: &CanvasRenderingContext2d) {
    context.set_fill_style_color("#000");

    let r = 150.0;

    for mut x in (-180..=180).step_by(5).map(|x| x as f64) {
        for y in (-180..=180).step_by(5).map(|y| y as f64) {
            let (xc, yc) = if x * x + y * y < r * r {
                /*球面变换 */
                let s = if x < 0.0 { -1.0 } else { 1.0 };
                if x == 0.0 {
                    x = 0.1;
                }
                let l = (x * x + y * y).sqrt();
                let bt = 2.0 * (l / r).atan();
                let th = (y / x).atan();
                let m = r * bt.sin();
                (s * m * th.cos(), s * m * th.sin())
            } else {
                (x, y)
            };
            context.fill_rect(xc + 320.0, yc + 200.0, 1.0, 1.0);
        }
    }
}
