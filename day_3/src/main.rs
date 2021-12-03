fn main() {
    let sample: Vec<&str> = include_str!("../input.txt").lines().collect();

    let half_size = sample.len() / 2;
    let bit_count: usize = sample[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit_position in 0..bit_count {
        gamma <<= 1;
        epsilon <<= 1;
        println!("===========");
        println!("{:0b}", gamma);
        println!("{:0b}", epsilon);

        let count: u32 = sample
            .iter()
            .filter(|x| x.chars().nth(bit_position).unwrap() == '1')
            .count() as u32;

        if count as usize >= half_size {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("{:?}", gamma);
    println!("{:?}", epsilon);
    println!("gamma * epsilon {}", epsilon * gamma);
}
