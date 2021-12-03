fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    let half_size = input.len() / 2;
    let bit_count: usize = input[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit_position in 0..bit_count {
        gamma <<= 1;
        epsilon <<= 1;

        let count: u32 = input
            .iter()
            .filter(|x| x.chars().nth(bit_position as usize).unwrap() == '1')
            .count() as u32;

        if count as usize > half_size {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("gamma * epsilon {}", epsilon * gamma);

    println!("part 2");
    let is_zero = |x, bit_position| is_char(x, bit_position, '0');
    let is_one = |x, bit_position| is_char(x, bit_position, '1');

    let mut sample = input.to_vec();

    for bit_position in 0..bit_count {
        let ones: u32 = sample.iter().filter(|x| is_one(x, bit_position)).count() as u32;
        let zeros: u32 = sample.iter().filter(|x| is_zero(x, bit_position)).count() as u32;

        if ones >= zeros {
            sample.retain(|x| is_one(x, bit_position));
        } else {
            sample.retain(|x| is_zero(x, bit_position));
        }

        if sample.len() == 1 {
            break;
        }
    }

    let oxygen_gen_rating = u32::from_str_radix(sample[0], 2).unwrap();

    let mut sample = input.to_vec();

    for bit_position in 0..bit_count {
        let ones: u32 = sample.iter().filter(|x| is_one(x, bit_position)).count() as u32;
        let zeros: u32 = sample.iter().filter(|x| is_zero(x, bit_position)).count() as u32;

        if ones >= zeros {
            sample.retain(|x| is_zero(x, bit_position));
        } else {
            sample.retain(|x| is_one(x, bit_position));
        }

        if sample.len() == 1 {
            break;
        }
    }

    let scruber_rating = u32::from_str_radix(sample[0], 2).unwrap();

    println!("==========");

    println!("{:?}", oxygen_gen_rating);
    println!("{:?}", scruber_rating);
    println!("result {}", scruber_rating * &oxygen_gen_rating);
}

fn is_char(input: &str, bit_position: usize, char: char) -> bool {
    input.chars().nth(bit_position as usize).unwrap() == char
}
