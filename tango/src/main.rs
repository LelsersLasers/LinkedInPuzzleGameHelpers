use macroquad::prelude as mq;
use std::collections::HashMap;

mod permute;




#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn to_u8(self) -> u8 {
        match self {
            Square::Empty => 0,
            Square::Sun => 1,
            Square::Moon => 2,
        }
    }
    fn from_u8(value: u8) -> Square {
        match value {
            0 => Square::Empty,
            1 => Square::Sun,
            2 => Square::Moon,
            _ => panic!("Invalid value"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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


fn solve(
    squares: &[Square],
    edges: &HashMap<(usize, usize), Edge>,
    squares_count: u32,
) -> bool {
    let total = squares_count * squares_count;
    println!("Generating permutations...");
    let permutations = permute::permutations_with_equal_ones_and_twos(total as usize);
    println!("Permutations generated");

    let mut i = 0;

    for permutation in permutations {
        if is_valid_permutation(&permutation, squares, edges, squares_count) {
            return true;
        }
        i += 1;
        println!("Permutations: {}", i);
    }


    false
}

fn is_valid_permutation(
    permutation: &[u8],
    squares: &[Square],
    edges: &HashMap<(usize, usize), Edge>,
    squares_count: u32,
) -> bool {
    // Matches squares with permutation
    for (i, square) in squares.iter().enumerate() {
        if square != &Square::Empty && square.to_u8() != permutation[i] {
            return false;
        }
    }

    // Checks edges
    for (key, edge) in edges {
        let (i, j) = key;
        let square1 = &permutation[*i];
        let square2 = &permutation[*j];

        match edge {
            Edge::X => {
                if square1 == square2 {
                    return false;
                }
            }
            Edge::Equals => {
                if square1 != square2 {
                    return false;
                }
            }
        }
    }

    // Checks rows (equal number of suns and moons)
    for y in 0..squares_count {
        let mut suns = 0;
        let mut moons = 0;
        for x in 0..squares_count {
            let index = xy_to_index(x, y, squares_count);
            let square = Square::from_u8(permutation[index]);
            match square {
                Square::Sun => suns += 1,
                Square::Moon => moons += 1,
                Square::Empty => panic!("Empty square in permutation"),
            };
        }
        if suns != moons {
            return false;
        }
    }

    // Checks columns (equal number of suns and moons)
    for x in 0..squares_count {
        let mut suns = 0;
        let mut moons = 0;
        for y in 0..squares_count {
            let index = xy_to_index(x, y, squares_count);
            let square = Square::from_u8(permutation[index]);
            match square {
                Square::Sun => suns += 1,
                Square::Moon => moons += 1,
                Square::Empty => panic!("Empty square in permutation"),
            };
        }
        if suns != moons {
            return false;
        }
    }

    // Check rows (no more than 2 suns or moons in a row)
    for y in 0..squares_count {
        let mut suns = 0;
        let mut moons = 0;
        for x in 0..squares_count {
            let index = xy_to_index(x, y, squares_count);
            let square = Square::from_u8(permutation[index]);
            match square {
                Square::Sun => {
                    suns += 1;
                    moons = 0;
                }
                Square::Moon => {
                    moons += 1;
                    suns = 0;
                }
                Square::Empty => panic!("Empty square in permutation"),
            }
            if suns > 2 || moons > 2 {
                return false;
            }
        }
    }

    // Check columns (no more than 2 suns or moons in a row)
    for x in 0..squares_count {
        let mut suns = 0;
        let mut moons = 0;
        for y in 0..squares_count {
            let index = xy_to_index(x, y, squares_count);
            let square = Square::from_u8(permutation[index]);
            match square {
                Square::Sun => {
                    suns += 1;
                    moons = 0;
                }
                Square::Moon => {
                    moons += 1;
                    suns = 0;
                }
                Square::Empty => panic!("Empty square in permutation"),
            }
            if suns > 2 || moons > 2 {
                return false;
            }
        }
    }

    true
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
                edges.clear();
            }
            if mouse_pos.0 > 136.0 && mouse_pos.0 < 152.0 && mouse_pos.1 > 6.0 && mouse_pos.1 < 22.0 {
                squares_count = (squares_count - 2).max(4);
                squares = vec![Square::Empty; (squares_count * squares_count) as usize];
                edges.clear();
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
        for (key, edge) in &edges {
            let (x1, y1) = index_to_xy(key.0, squares_count);
            let (x2, y2) = index_to_xy(key.1, squares_count);

            let square1_center = (
                x_padding + x1 as f32 * square_size + square_size / 2.0,
                y_padding + y1 as f32 * square_size + square_size / 2.0,
            );
            let square2_center = (
                x_padding + x2 as f32 * square_size + square_size / 2.0,
                y_padding + y2 as f32 * square_size + square_size / 2.0,
            );
            let center = (
                (square1_center.0 + square2_center.0) / 2.0,
                (square1_center.1 + square2_center.1) / 2.0,
            );

            let text_dims = mq::measure_text(edge.to_text(), None, 25, 1.0);
            mq::draw_text(
                edge.to_text(),
                center.0 - text_dims.width / 2.0,
                center.1 - text_dims.height / 2.0 + text_dims.offset_y,
                25.0,
                mq::BLACK,
            );
        }

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
                                let key1 = (idx, index);
                                let edge1 = edges.get(&key1);

                                let key2 = (index, idx);
                                let edge2 = edges.get(&key2);

                                if edge1.is_none() && edge2.is_none() {
                                    edges.insert(key1, Edge::X);
                                } else if edge1.is_some() && edge2.is_none() {
                                    let edge = edge1.unwrap();
                                    if edge == &Edge::X {
                                        edges.remove(&key1);
                                        edges.insert(key1, Edge::Equals);
                                    } else {
                                        edges.remove(&key1);
                                    }
                                } else if edge1.is_none() && edge2.is_some() {
                                    let edge = edge2.unwrap();
                                    if edge == &Edge::X {
                                        edges.remove(&key2);
                                        edges.insert(key2, Edge::Equals);
                                    } else {
                                        edges.remove(&key2);
                                    }
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

        if mq::is_key_pressed(mq::KeyCode::Space) {
            let solved = solve(&squares, &edges, squares_count);
            println!("Solved: {}", solved);
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
