//#![no_main]
extern crate piston_window;
mod canvas;
mod chapter1;
use canvas::Canvas;

use piston_window::*;


struct PistonCanvas<'a>{
    
}



impl <'a> Canvas for PistonCanvas<'a>{
    fn point(){

    }
}

fn main(){
    let mut window: PistonWindow = 
            WindowSettings::new("Rust艺术图案程序设计", [640, 480])
            .exit_on_esc(true).build().unwrap();
    let canvas = PistonCanvas{window: &window};
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);

            let mut x = 0.0;
            while x<2.0*std::f64::consts::PI*2.0{
                let ysin = 1.0-x.sin();
                let ycos = 1.0-x.cos();
                rectangle([1.0, 1.0, 1.0, 1.0],
                    [50.0*x, 99.0*ysin, 1.0, 1.0],
                    c.transform, g);
                rectangle([1.0, 0.0, 0.0, 1.0],
                    [50.0*x, 99.0*ycos, 1.0, 1.0],
                    c.transform, g);
                x += 0.01;
            }

        /*
            //画点
            rectangle([1.0, 1.0, 1.0, 1.0],
                    [10.0, 10.0, 1.0, 1.0],
                    c.transform, g);
            
            //画线
            line([1.0, 1.0, 1.0, 1.0],
                    0.5,
                    [50.0, 50.0, 180.0, 150.0],
                    c.transform, g);

            //正方形
            let border = Rectangle::new_border([1.0, 1.1, 1.0, 1.0], 0.5);
            border.draw([190.0, 190.0, 70.0, 70.0],
                                &DrawState::new_outside(),
                                c.transform,
                                g);

            //方块
            rectangle([1.0, 0.0, 0.0, 1.0],
                    [150.0, 150.0, 30.0, 30.0],
                    c.transform, g);

            //圆圈
            circle_arc([1.0, 1.0, 1.0, 1.0],//color
                    0.5,//线宽
                    0.0001,//起始
                    std::f64::consts::PI*2.0,//结束
                    [200.0, 200.0, 50.0, 50.0],
                    c.transform, g);

            //圆形/椭圆
            ellipse([1.0, 1.0, 0.0, 1.0],
                    [300.0, 300.0, 60.0, 60.0],
                    c.transform, g);
            
            //多边形
            let polygon = Polygon::new([1.0; 4]);
            polygon.draw(
                    &vec![[100.0, 100.0],[120.0, 100.0],[120.0, 120.0], [110.0, 120.0]],
                    &DrawState::new_alpha(),
                    c.transform,
                    g);
                    */
        });
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    main();
    0
}