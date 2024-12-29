use macroquad::prelude as mq;

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Tango Solver".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        mq::clear_background(mq::WHITE);
        mq::draw_text("Hello, world!", 20.0, 20.0, 30.0, mq::BLACK);
        mq::next_frame().await
    }
}
