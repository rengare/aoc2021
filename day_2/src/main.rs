#[derive(Debug)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn main() {
    let input = include_str!("../input.txt");
    let sample = include_str!("../sample.txt");

    let commands: Vec<Command> = input
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split(" ").collect();
            let line_value = line[1].parse::<usize>().unwrap();
            return match line[0] {
                "up" => Command::Up(line_value),
                "down" => Command::Down(line_value),
                "forward" => Command::Forward(line_value),
                _ => panic!("Unknown direction"),
            };
        })
        .collect();

    part_1(&commands);
    part_2(&commands);
}

fn part_2(commands: &Vec<Command>) {
    let result: (usize, usize, usize) = commands.iter().fold(
        (0, 0, 0),
        |(mut horizontal, mut depth, mut aim), direction| {
            match direction {
                Command::Up(value) => {
                    aim -= value;
                }
                Command::Down(value) => {
                    aim += value;
                }
                Command::Forward(value) => {
                    horizontal += value;
                    depth += aim * value;
                }
            }
            (horizontal, depth, aim)
        },
    );

    println!(
        "horizontal:{}, depth:{}, aim:{}",
        result.0, result.1, result.2
    );
    println!("horizontal*depth:{}", result.0 * result.1);
}

fn part_1(commands: &Vec<Command>) {
    let result: (usize, usize) =
        commands
            .iter()
            .fold((0, 0), |(mut horizontal, mut depth), direction| {
                match direction {
                    Command::Up(value) => {
                        depth -= value;
                    }
                    Command::Down(value) => {
                        depth += value;
                    }
                    Command::Forward(value) => {
                        horizontal += value;
                    }
                }
                (horizontal, depth)
            });

    println!("horizontal:{}, depth:{}", result.0, result.1);
    println!("horizontal*depth:{}", result.0 * result.1);
}
