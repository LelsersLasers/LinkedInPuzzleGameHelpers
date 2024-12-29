use macroquad::prelude as mq;

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
    let mut squares: u32 = 6;

    loop {
        // Clear background
        mq::clear_background(mq::GRAY);

        // Draw squares counter
        mq::draw_text(
            &format!("Squares:{}", squares),
            8.0,
            20.0,
            24.0,
            mq::BLACK,
        );

        // Draw increment button
        mq::draw_rectangle(110.0, 6.0, 16.0, 16.0, mq::BLACK);
        mq::draw_text("+", 113.0, 19.0, 24.0, mq::WHITE);

        // Draw decrement button
        mq::draw_rectangle(130.0, 6.0, 16.0, 16.0, mq::BLACK);
        mq::draw_text("-", 133.0, 19.0, 24.0, mq::WHITE);

        // Check if increment/decrement buttons are pressed
        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            let mouse_pos = mq::mouse_position();
            if mouse_pos.0 > 110.0 && mouse_pos.0 < 126.0 && mouse_pos.1 > 6.0 && mouse_pos.1 < 22.0 {
                squares += 2;
            }
            if mouse_pos.0 > 130.0 && mouse_pos.0 < 146.0 && mouse_pos.1 > 6.0 && mouse_pos.1 < 22.0 {
                squares = squares.saturating_sub(2);
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





        // Next frame
        mq::next_frame().await
    }
}
