fn main() {
    let sample = include_str!("../sample.txt");
    let input = include_str!("../input.txt");

    let sample2 = include_str!("../sample2.txt");

    println!("sample count: {}", positive_change_count(sample));
    println!("input count: {}", positive_change_count(input));

    println!("part2 sample count: {}", positive_change_count_2(sample2));
    println!("part2 input count: {}", positive_change_count_2(input));
}

fn positive_change_count(file: &str) -> usize {
    file.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn positive_change_count_2(file: &str) -> usize {
    file.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .windows(3)
        .map(|triple| triple.iter().sum())
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}
