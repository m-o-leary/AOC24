advent_of_code::solution!(4);

fn is_valid(x: i32, y: i32, max_x: i32, max_y: i32) -> bool {
    let x_valid = x >= 0 && x <= max_x;
    let y_valid = y >= 0 && y <= max_y;
    return x_valid && y_valid;
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let len = lines.len();
    let mut visited = vec![vec![0;len];len];
    let directions = vec![
        (0, 1), 
        (0, -1), 
        (1, 0), 
        (-1, 0), 
        (1, 1), 
        (1, -1), 
        (-1, 1), 
        (-1, -1)];
    for line_ind in 0..len {
        let line = lines[line_ind];
        for char_ind in 0..len {
            visited[line_ind][char_ind] = 1;
            let char = line.chars().nth(char_ind);
            if  == 'X' {
                // search in 8 directions
                if lines line_ind
            } 
        }
    }
    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
