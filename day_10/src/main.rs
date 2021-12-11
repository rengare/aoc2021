const SAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

fn main() {
    let inputs = include_str!("../input.txt");

    println!("{}", part_1_answer(inputs));
    println!("{}", part_2_answer(inputs));
}

fn char_to_score(stack_char: char, current_char: char) -> Option<i32> {
    match (stack_char, current_char) {
        ('(', ')') => {}
        ('[', ']') => {}
        ('{', '}') => {}
        ('<', '>') => {}
        (_, ')') => return Some(3),
        (_, ']') => return Some(57),
        (_, '}') => return Some(1197),
        (_, '>') => return Some(25137),
        _ => unreachable!(),
    };
    None
}

fn handle_stack(stack: &mut Vec<char>, c: char) -> Option<i32> {
    match c {
        '{' | '[' | '(' | '<' => stack.push(c),
        _ => return char_to_score(stack.pop().unwrap(), c),
    }

    None
}

fn part_1(inputs: &str) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    inputs
        .chars()
        .map(|c| handle_stack(&mut stack, c).unwrap_or(0))
        .sum()
}

fn part_2(inputs: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();

    if inputs
        .chars()
        .map(|c| handle_stack(&mut stack, c))
        .any(|x| x.is_some())
    {
        return 0;
    }

    stack.iter().rev().fold(0, |score, c| match c {
        '(' => score * 5 + 1,
        '[' => score * 5 + 2,
        '{' => score * 5 + 3,
        '<' => score * 5 + 4,
        _ => 0,
    })
}

fn part_1_answer(inputs: &str) -> i32 {
    inputs.lines().map(part_1).sum::<i32>()
}

fn part_2_answer(inputs: &str) -> u64 {
    let mut line_scores = inputs
        .lines()
        .map(part_2)
        .filter(|x| *x > 0)
        .collect::<Vec<_>>();

    line_scores.sort();

    line_scores[line_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_answer(SAMPLE), 26397);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_answer(SAMPLE), 288957);
    }
}
