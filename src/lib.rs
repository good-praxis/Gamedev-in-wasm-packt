use rand::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    sierpinski(
        &context,
        [
            &Point { x: 300.0, y: 0.0 },
            &Point { x: 0.0, y: 600.0 },
            &Point { x: 600.0, y: 600.0 },
        ],
        &Color { r: 0, b: 255, g: 0 },
        5,
    );

    Ok(())
}

type Triangle<'a> = [&'a Point; 3];
struct Point {
    x: f64,
    y: f64,
}
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: Triangle, color: &Color) {
    let [top, left, right] = points;
    let color_str = format!("rgb({}, {}, {})", color.r, color.g, color.b);

    context.set_fill_style(&JsValue::from_str(&color_str));
    context.move_to(top.x, top.y);
    context.begin_path();
    context.line_to(left.x, left.y);
    context.line_to(right.x, right.y);
    context.line_to(top.x, top.y);
    context.close_path();
    context.stroke();
    context.fill();
}

fn sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    points: Triangle,
    color: &Color,
    depth: u8,
) {
    draw_triangle(context, points, color);

    let depth = depth - 1;

    if depth > 0 {
        let mut rng = thread_rng();

        let next_color = Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
        };

        let [top, left, right] = points;
        let left_middle = &midpoint(top, left);
        let right_middle = &midpoint(top, right);
        let bottom_middle = &midpoint(left, right);

        sierpinski(
            context,
            [top, left_middle, right_middle],
            &next_color,
            depth,
        );
        sierpinski(
            context,
            [left_middle, left, bottom_middle],
            &next_color,
            depth,
        );
        sierpinski(
            context,
            [right_middle, bottom_middle, right],
            &next_color,
            depth,
        );
    }
}

fn midpoint(point_1: &Point, point_2: &Point) -> Point {
    Point {
        x: (point_1.x + point_2.x) / 2.0,
        y: (point_1.y + point_2.y) / 2.0,
    }
}
