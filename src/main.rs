use macroquad::prelude::*;

struct DrawPoint {
    x: f32,
    y: f32,
    width: f32,
}

struct Draw {
    draw_points: Vec<DrawPoint>,
    color: Color,
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mut drawings: Vec<Draw> = Vec::new();
    let mut drawings_width: f32 = 1.0;
    let colors: [Color; 8] = [WHITE, GRAY, BLUE, GREEN, GOLD, PURPLE, RED, BROWN];
    let colors_squares_width = 20;
    let colors_squares_dist = 10;
    let colors_squares_y = 15;
    let mut actual_color = WHITE;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Escape) {
            break;
        }
        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
            && is_key_pressed(KeyCode::Z)
        {
            if drawings.len() > 0 {
                drawings.truncate(drawings.len() - 1);
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            drawings.push(Draw {
                draw_points: Vec::new(),
                color: actual_color,
            });

            let (mouse_x, mouse_y) = mouse_position();
            for i in 0..colors.len() {
                let color = colors[i];

                let min_x = (colors_squares_dist * (i + 1) + colors_squares_width * i) as f32;
                let max_x = (colors_squares_dist * (i + 1)
                    + colors_squares_width * i
                    + colors_squares_width) as f32;
                let min_y = colors_squares_y as f32;
                let max_y = (colors_squares_y + colors_squares_width) as f32;

                if (min_x <= mouse_x && mouse_x <= max_x) && (min_y <= mouse_y && mouse_y <= max_y)
                {
                    actual_color = color;
                }
            }
        }
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let last: &mut Draw = drawings.last_mut().expect("There is no drawing");
            last.draw_points.push(DrawPoint {
                x: mouse_x,
                y: mouse_y,
                width: drawings_width,
            });
        }
        if is_mouse_button_released(MouseButton::Left) {
            let last: &Draw = drawings.last().expect("There is no drawing");
            if last.draw_points.len() < 2 {
                drawings.truncate(drawings.len() - 1);
            }
        }
        let button_wheel = mouse_wheel().1;
        if button_wheel > 0.0 {
            drawings_width += 0.2;
        }
        if button_wheel < 0.0 && drawings_width > 1.0 {
            drawings_width -= 0.2;
        }

        for drawing in &drawings {
            if drawing.draw_points.len() >= 2 {
                for i in 0..(drawing.draw_points.len() - 1) {
                    let DrawPoint {
                        x: x1,
                        y: y1,
                        width,
                    } = &drawing.draw_points[i];
                    let DrawPoint {
                        x: x2,
                        y: y2,
                        width: _,
                    } = &drawing.draw_points[i + 1];

                    draw_line(*x1, *y1, *x2, *y2, *width, drawing.color);
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
}
