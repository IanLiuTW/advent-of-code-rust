advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<i32> {
    let mut computer = Computer::new();
    computer.run(input);

    Some(computer.total_signal_strength)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut computer = Computer::new();
    computer.run(input);
    computer.draw_image();

    None
}

struct Computer {
    cycle: i32,
    reg_x: i32,
    total_signal_strength: i32,
    image: Vec<char>,
}

impl Computer {
    fn new() -> Self {
        Self {
            cycle: 1,
            reg_x: 1,
            total_signal_strength: 0,
            image: vec![],
        }
    }

    fn run(&mut self, input: &str) {
        input.lines().for_each(|line| match line {
            "noop" => self.noop(),
            line if line.starts_with("addx") => {
                self.addx(line.trim_start_matches("addx ").parse().unwrap());
            }
            _ => unreachable!(),
        });
    }

    fn advance_cycle(&mut self) {
        if self.cycle >= 20 && (self.cycle - 20) % 40 == 0 {
            self.total_signal_strength += self.reg_x * self.cycle;
        }

        self.update_image();

        self.cycle += 1;
    }

    fn update_image(&mut self) {
        let cur = (self.cycle - 1) % 40;
        if self.reg_x - 1 <= cur && cur <= self.reg_x + 1 {
            self.image.push('#');
        } else {
            self.image.push('.');
        }
    }

    fn draw_image(&mut self) {
        self.image
            .chunks(40)
            .for_each(|line| println!("{}", line.iter().collect::<String>()));
    }

    fn noop(&mut self) {
        self.advance_cycle();
    }

    fn addx(&mut self, val: i32) {
        self.advance_cycle();
        self.advance_cycle();

        self.reg_x += val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
