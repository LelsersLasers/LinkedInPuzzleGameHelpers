use macroquad::prelude as mq;
use std::collections::HashMap;




#[derive(Clone, Copy)]
enum Square {
    Empty,
    Sun,
    Moon,
}
impl Square {
    fn to_color(self) -> mq::Color {
        match self {
            Square::Empty => mq::WHITE,
            Square::Sun => mq::YELLOW,
            Square::Moon => mq::BLUE,
        }
    }
    fn next(self) -> Square {
        match self {
            Square::Empty => Square::Sun,
            Square::Sun => Square::Moon,
            Square::Moon => Square::Empty,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Edge {
    X,
    Equals,
}
impl Edge {
    fn to_text(self) -> &'static str {
        match self {
            Edge::X => "X",
            Edge::Equals => "=",
        }
    }
}


fn xy_to_index(x: u32, y: u32, width: u32) -> usize {
    (y * width + x) as usize
}
fn index_to_xy(index: usize, width: u32) -> (u32, u32) {
    let x = index as u32 % width;
    let y = index as u32 / width;
    (x, y)
}







fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Tango Solver".to_owned(),
        window_width: 600,
        window_height: 500,
        window_resizable: false,
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut squares_count: u32 = 6;
    let mut squares = vec![Square::Empty; (squares_count * squares_count) as usize];
    let mut clicked_idx: Option<usize> = None;
    let mut edges: HashMap<(usize, usize), Edge> = HashMap::new();

    loop {
        // Clear background
        mq::clear_background(mq::GRAY);

        // Draw squares counter
        mq::draw_text(
            &format!("Squares:{}", squares_count),
            8.0,
            20.0,
            24.0,
            mq::BLACK,
        );

        // Draw increment button
        mq::draw_rectangle(116.0, 6.0, 16.0, 16.0, mq::BLACK);
        mq::draw_text("+", 119.0, 19.0, 24.0, mq::WHITE);

        // Draw decrement button
        mq::draw_rectangle(136.0, 6.0, 16.0, 16.0, mq::BLACK);
        mq::draw_text("-", 139.0, 19.0, 24.0, mq::WHITE);

        // Check if increment/decrement buttons are pressed
        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            let mouse_pos = mq::mouse_position();
            if mouse_pos.0 > 116.0 && mouse_pos.0 < 132.0 && mouse_pos.1 > 6.0 && mouse_pos.1 < 22.0 {
                squares_count = (squares_count + 2).min(8);
                squares = vec![Square::Empty; (squares_count * squares_count) as usize];
            }
            if mouse_pos.0 > 136.0 && mouse_pos.0 < 152.0 && mouse_pos.1 > 6.0 && mouse_pos.1 < 22.0 {
                squares_count = (squares_count - 2).max(4);
                squares = vec![Square::Empty; (squares_count * squares_count) as usize];
            }
        }

        // Draw squares
        let square_size = (mq::screen_height() - 100.0) / squares_count as f32;
        let x_padding = (mq::screen_width() - square_size * squares_count as f32) / 2.0;
        let y_padding = (mq::screen_height() - square_size * squares_count as f32) * (2.0 / 3.0);

        // Draw filled squares
        for y in 0..squares_count {
            for x in 0..squares_count {
                let index = xy_to_index(x, y, squares_count);
                let color = match clicked_idx {
                    Some(idx) if idx == index => mq::RED,
                    _ => squares[index].to_color()
                };
                mq::draw_rectangle(
                    x_padding + x as f32 * square_size,
                    y_padding + y as f32 * square_size,
                    square_size,
                    square_size,
                    color,
                );
            }
        }

        // Draw grid
        mq::draw_rectangle_lines(
            x_padding,
            y_padding,
            square_size * squares_count as f32,
            square_size * squares_count as f32,
            5.0,
            mq::BLACK,
        );

        for y in 0..squares_count {
            for x in 0..squares_count {
                mq::draw_rectangle_lines(
                    x_padding + x as f32 * square_size,
                    y_padding + y as f32 * square_size,
                    square_size,
                    square_size,
                    2.0,
                    mq::BLACK,
                );
            }
        }

        // Draw edges
        // TODO!

        // Check if a square is clicked
        let mouse_pos = mq::mouse_position();
        let grid_x = (mouse_pos.0 - x_padding) / square_size;
        let grid_y = (mouse_pos.1 - y_padding) / square_size;

        if grid_x > 0.0 && grid_y > 0.0 {
            let x_index = grid_x as u32;
            let y_index = grid_y as u32;

            if x_index < squares_count && y_index < squares_count {
                let index = xy_to_index(x_index, y_index, squares_count);
                if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
                    squares[index] = squares[index].next();
                    clicked_idx = None;
                } else if mq::is_mouse_button_pressed(mq::MouseButton::Right) {
                    if let Some(idx) = clicked_idx {
                        if idx == index {
                            clicked_idx = None;
                        } else {
                            let (x1, y1) = index_to_xy(idx, squares_count);
                            let (x2, y2) = index_to_xy(index, squares_count);

                            let x_diff = (x1 as i32 - x2 as i32).abs();
                            let y_diff = (y1 as i32 - y2 as i32).abs();
                            
                            if (x_diff + y_diff) == 1 {
                                let key = (idx, index);
                                let edge = edges.get(&key);

                                if let Some(edge) = edge {
                                    if edge == &Edge::X {
                                        edges.remove(&key);
                                        edges.insert(key, Edge::Equals);
                                    } else {
                                        edges.remove(&key);
                                    }
                                } else {
                                    edges.insert(key, Edge::X);
                                }
                            }
                            clicked_idx = None;
                        }
                    } else {
                        clicked_idx = Some(index);
                    }
                }
            }
        }

        println!("{:?}", edges);

        // Debug - Draw mouse position
        let mouse_pos = mq::mouse_position();
        mq::draw_text(
            &format!("Mouse: ({:.0}, {:.0})", mouse_pos.0, mouse_pos.1),
            10.0,
            40.0,
            16.0,
            mq::BLACK,
        );

        // Debug - Draw fps
        mq::draw_text(
            &format!("FPS: {:.0}", mq::get_fps()),
            10.0,
            60.0,
            16.0,
            mq::BLACK,
        );

        // Next frame
        mq::next_frame().await
    }
}
