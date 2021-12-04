fn main() {
    let input: Vec<&str> = include_str!("../sample.txt").lines().collect();

    let half_size = input.len() / 2;
    let bit_count: usize = input[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit_position in 0..bit_count {
        gamma <<= 1;
        epsilon <<= 1;

        let count: u32 = input.iter().filter(|x| is_one(*x, bit_position)).count() as u32;

        if count as usize > half_size {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("gamma * epsilon {}", epsilon * gamma);

    println!("part 2");

    let oxygen_gen_rating = retain(is_one, is_zero, input.clone(), bit_count);
    let scruber_rating = retain(is_zero, is_one, input.clone(), bit_count);

    println!("{:?}", oxygen_gen_rating);
    println!("{:?}", scruber_rating);
    println!("result {}", scruber_rating * &oxygen_gen_rating);
}

fn is_zero(x: &str, bit_position: usize) -> bool {
    is_char(x, bit_position, '0')
}

fn is_one(x: &str, bit_position: usize) -> bool {
    is_char(x, bit_position, '1')
}

fn retain(
    when_greater_or_equal: fn(&str, usize) -> bool,
    when_less: fn(&str, usize) -> bool,
    mut sample: Vec<&str>,
    bit_count: usize,
) -> u32 {
    for bit_position in 0..bit_count {
        let ones: u32 = sample.iter().filter(|x| is_one(x, bit_position)).count() as u32;
        let zeros: u32 = sample.iter().filter(|x| is_zero(x, bit_position)).count() as u32;

        if ones >= zeros {
            sample.retain(|x| when_greater_or_equal(x, bit_position));
        } else {
            sample.retain(|x| when_less(x, bit_position));
        }

        if sample.len() == 1 {
            break;
        }
    }
    u32::from_str_radix(sample[0], 2).unwrap()
}

fn is_char(input: &str, bit_position: usize, char: char) -> bool {
    input.chars().nth(bit_position as usize).unwrap() == char
}
