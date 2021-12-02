#[derive(Debug)]
enum Command {
    Up(usize),
    Down(usize),
    Forward(usize),
}

fn main() {
    // let sample = include_str!("../sample.txt");
    let sample = include_str!("../input.txt");

    let commands: Vec<Command> = sample
        .lines()
        .map(|line| {
            let command = line.split_whitespace().collect::<Vec<_>>();
            match command[0] {
                "up" => Command::Up(command[1].parse().unwrap()),
                "down" => Command::Down(command[1].parse().unwrap()),
                "forward" => Command::Forward(command[1].parse().unwrap()),
                _ => panic!("Unknown command"),
            }
        })
        .collect();

    let result: (usize, usize) =
        commands
            .iter()
            .fold((0, 0), |(mut horizontal, mut depth), command| {
                match command {
                    Command::Up(value) => depth -= value,
                    Command::Down(value) => depth += value,
                    Command::Forward(value) => horizontal += value,
                }

                (horizontal, depth)
            });

    println!("{:?}", result);
    println!("horizontal*depth = {:?}", result.0 * result.1);

    println!("part2");

    let result: (usize, usize, usize) = commands.iter().fold(
        (0, 0, 0),
        |(mut horizontal, mut depth, mut aim), command| {
            match command {
                Command::Down(x) => aim += x,
                Command::Up(x) => aim -= x,
                Command::Forward(x) => {
                    horizontal += x;
                    depth += aim * x
                }
            }

            (horizontal, depth, aim)
        },
    );

    println!("horizontal*depth = {:?}", result.0 * result.1);
}
