use macroquad::prelude as mq;




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


fn xy_to_index(x: u32, y: u32, width: u32) -> usize {
    (y * width + x) as usize
}







fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Tango Solver".to_owned(),
        window_width: 600,
        window_height: 500,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut squares_count: u32 = 6;
    let mut squares = vec![Square::Empty; (squares_count * squares_count) as usize];

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
                let color = squares[index].to_color();
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
                    3.0,
                    mq::BLACK,
                );
            }
        }

        // Check if a square is clicked
        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            let mouse_pos = mq::mouse_position();
            let x = ((mouse_pos.0 - x_padding) / square_size) as u32;
            let y = ((mouse_pos.1 - y_padding) / square_size) as u32;
            if x < squares_count && y < squares_count {
                let index = xy_to_index(x, y, squares_count);
                squares[index] = squares[index].next();
            }
        }

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
