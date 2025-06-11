use macroquad::prelude::*;
use std::{fs::File, io::Write};

struct DrawPoint {
    x: f32,
    y: f32,
}

struct Draw {
    draw_points: Vec<DrawPoint>,
    color: Color,
    width: f32,
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mut drawings: Vec<Draw> = Vec::new();
    let mut drawing_width: f32 = 1.0;
    let colors: [Color; 8] = [WHITE, GRAY, BLUE, GREEN, GOLD, PURPLE, RED, BROWN];
    let colors_squares_width = 20;
    let colors_squares_dist = 10;
    let colors_squares_y = 15;
    let mut actual_color = WHITE;
    let mut prev_mouse_pos: (f32, f32) = (0.0, 0.0);
    let mut min_x: f32 = screen_width();
    let mut max_x: f32 = 0.0;
    let mut min_y: f32 = screen_height();
    let mut max_y: f32 = 0.0;
    const SEPARATION: f32 = 30.0;

    loop {
        clear_background(BLACK);
        // println!("{}, {}", min_x, min_y);

        if is_key_down(KeyCode::Escape) {
            break;
        }
        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
            && is_key_pressed(KeyCode::Z)
        {
            if drawings.len() > 0 {
                drawings.truncate(drawings.len() - 1);

                min_x = screen_width();
                max_x = 0.0;
                min_y = screen_height();
                max_y = 0.0;

                for drawing in &drawings {
                    for draw_point in &drawing.draw_points {
                        let (x, y) = (draw_point.x, draw_point.y);
                        if x < min_x {
                            min_x = x
                        }
                        if x > max_x {
                            max_x = x
                        }
                        if y < min_y {
                            min_y = y
                        }
                        if y > max_y {
                            max_y = y
                        }
                    }
                }
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            drawings.push(Draw {
                draw_points: Vec::new(),
                color: actual_color,
                width: drawing_width,
            });

            let (mouse_x, mouse_y) = mouse_position();
            for i in 0..colors.len() {
                let color = colors[i];

                let min_color_x = (colors_squares_dist * (i + 1) + colors_squares_width * i) as f32;
                let max_color_x = (colors_squares_dist * (i + 1)
                    + colors_squares_width * i
                    + colors_squares_width) as f32;
                let min_color_y = colors_squares_y as f32;
                let max_color_y = (colors_squares_y + colors_squares_width) as f32;

                if (min_color_x <= mouse_x && mouse_x <= max_color_x)
                    && (min_color_y <= mouse_y && mouse_y <= max_color_y)
                {
                    actual_color = color;
                }
            }
        }
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            if mouse_x != prev_mouse_pos.0 || mouse_y != prev_mouse_pos.1 {
                let last: &mut Draw = drawings.last_mut().expect("There is no drawing");
                last.draw_points.push(DrawPoint {
                    x: mouse_x,
                    y: mouse_y,
                });

                if last.draw_points.len() >= 2 {
                    if mouse_x < min_x {
                        min_x = mouse_x
                    }
                    if mouse_x > max_x {
                        max_x = mouse_x
                    }
                    if mouse_y < min_y {
                        min_y = mouse_y
                    }
                    if mouse_y > max_y {
                        max_y = mouse_y
                    }
                }
                prev_mouse_pos = (mouse_x, mouse_y);
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            let last: &Draw = drawings.last().expect("There is no drawing");
            if last.draw_points.len() < 2 {
                drawings.truncate(drawings.len() - 1);
            }
        }
        let button_wheel = mouse_wheel().1;
        if button_wheel > 0.0 {
            drawing_width += 0.2;
        }
        if button_wheel < 0.0 && drawing_width > 1.0 {
            drawing_width -= 0.2;
        }

        for drawing in &drawings {
            if drawing.draw_points.len() >= 2 {
                for i in 0..(drawing.draw_points.len() - 1) {
                    let DrawPoint { x: x1, y: y1 } = &drawing.draw_points[i];
                    let DrawPoint { x: x2, y: y2 } = &drawing.draw_points[i + 1];

                    draw_line(*x1, *y1, *x2, *y2, drawing.width, drawing.color);
                }
            }
        }

        for i in 0..colors.len() {
            let color = colors[i];
            draw_rectangle(
                (colors_squares_dist * (i + 1) + colors_squares_width * i) as f32,
                colors_squares_y as f32,
                colors_squares_width as f32,
                colors_squares_width as f32,
                color,
            );
        }

        next_frame().await
    }

    let mid_x: f32 = (min_x + max_x) / 2.0;
    let mid_y: f32 = (min_y + max_y) / 2.0;
    let mut paths_strs: String = String::new();
    for drawing in &drawings {
        let color = drawing.color;
        let (x, y) = (drawing.draw_points[0].x, drawing.draw_points[0].y);
        let mut path_str = format!("M{} {}", x - mid_x, y - mid_y);

        for i in 1..(drawing.draw_points.len() - 1) {
            let (mut prev_x, mut prev_y) =
                (drawing.draw_points[i - 1].x, drawing.draw_points[i - 1].y);
            let (mut x, mut y) = (drawing.draw_points[i].x, drawing.draw_points[i].y);

            prev_x = prev_x - mid_x;
            prev_y = prev_y - mid_y;
            x = x - mid_x;
            y = y - mid_y;

            let control_x: f32 = (prev_x + x) / 2.0;
            let control_y: f32 = (prev_y + y) / 2.0;

            path_str.push_str(format!(" Q{} {} {} {}", control_x, control_y, x, y).as_str());
        }

        let path = format!(
            "<path d='{}' style='fill:none;stroke:rgba({},{},{},{});stroke-width:{}' />",
            path_str,
            (color.r * 255.0) as i32,
            (color.g * 255.0) as i32,
            (color.b * 255.0) as i32,
            (color.a * 255.0) as i32,
            drawing.width as i32
        );
        paths_strs.push_str(&format!("{}\n", path));
    }
    let svg_height: f32 = max_y - min_y;
    let svg_width: f32 = max_x - min_x;
    let mut prefijo: String = format!(
        "<svg width='{}' height='{}' viewBox='{} {} {} {}' xmlns='http://www.w3.org/2000/svg'>\n",
        svg_width,
        svg_height,
        -svg_width / 2.0,
        -svg_height / 2.0,
        svg_width,
        svg_height
    );
    let sufijo: &str = "</svg>";

    prefijo.push_str(&paths_strs);
    prefijo.push_str(sufijo);

    // println!("{min_y}, {max_y}, {min_x}, {max_x}");
    let mut file = File::create("drawings.svg").expect("Problem with the file");
    File::write(&mut file, prefijo.as_bytes());
}
