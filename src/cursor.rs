pub struct Cursor {

    pub x: usize,
    pub y: usize,

}

pub fn move_left(cursor: &mut Cursor) {
    if cursor.x > 0 {
        cursor.x -= 1;
    }
}

pub fn move_right(cursor: &mut Cursor, lines: &[String]) {
    let current_line_len = lines[cursor.y].len();
    if cursor.x < current_line_len {
        cursor.x += 1;
    }
}

pub fn move_up(cursor: &mut Cursor, lines: &[String]) {
    if cursor.y > 0 {
        cursor.y -= 1;
        clamp_x(cursor, lines);
    }
}

pub fn move_down(cursor: &mut Cursor, lines: &[String]) {
    if cursor.y < lines.len() - 1 {
        cursor.y += 1;
        clamp_x(cursor, lines);
    }
}

fn clamp_x(cursor: &mut Cursor, lines: &[String]) {
    let current_line_len = lines[cursor.y].len();
    if cursor.x > current_line_len {
        cursor.x = current_line_len;
    }
}

