pub fn get_rotations(number: i32, rotation: i32, left: i32, right: i32) -> i32 {
    // expects left to be 0 and the counting number to be 0 as well
    let interval = right - left + 1;
    let mut rotations = rotation.abs() / interval;
    let rotation = rotation % interval;
    let position = number + rotation;
    let number_was_zero = (number == 0) as i32;
    if position < 0 || position > (right + 1) {
        rotations += 1 - number_was_zero;
    }
    rotations
}

pub fn get_rotations_number(number: i32, rotation: i32, left: i32, right: i32) -> (i32, i32) {
    let mut position = number;
    let mut number_was_zero = 0;
    let mut direction = 1;
    if rotation < 0 {
        direction = -1
    }
    if rotation == 0 {
        panic!("Rotation is 0")
    }
    for _ in 0..rotation.abs() {
        position += direction;
        if position == left || position == (right + 1) {
            number_was_zero += 1;
        }
        if position < left {
            position = right
        } else if position > right {
            position = left
        }
    }
    (position, number_was_zero)
}

pub fn map_number(number: i32, left: i32, right: i32) -> i32 {
    let mut position = number;
    let interval = right - left + 1;
    position %= interval;
    if position < left {
        position += interval;
    }
    position
}

pub fn filter_non_relevant_chars(s: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut current_s: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '\n' => {
                if !current_s.is_empty() {
                    result.push(current_s.into_iter().collect());
                    current_s = Vec::new();
                }
            }
            '\r' => {
                if !current_s.is_empty() {
                    result.push(current_s.into_iter().collect());
                    current_s = Vec::new();
                }
            }
            ' ' => {
                if !current_s.is_empty() {
                    result.push(current_s.into_iter().collect());
                    current_s = Vec::new();
                }
            }
            _ => {
                current_s.push(c);
            }
        };
    }
    if !current_s.is_empty() {
        result.push(current_s.into_iter().collect());
    }
    result
}
