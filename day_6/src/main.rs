fn main() {
    let inputs = include_str!("../input.txt")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!(" {:?}", calculate_population(inputs.iter(), 80));
    println!(" {:?}", calculate_population(inputs.iter(), 256));
}

fn calculate_population<'a>(inputs: impl Iterator<Item = &'a u32>, days: u32) -> u64 {
    let mut fishes = [0; 9];
    for i in inputs {
        fishes[*i as usize] += 1;
    }

    for _ in 0..days {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_population() {
        let input = vec![3, 4, 3, 1, 2];
        let days = 18;
        let days_to_conquer = 256;

        assert_eq!(26, calculate_population(input.iter(), days));

        assert_eq!(
            26984457539,
            calculate_population(input.iter(), days_to_conquer)
        );
    }
}
