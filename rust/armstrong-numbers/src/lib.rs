pub fn is_armstrong_number(num: u32) -> bool {
    let count = num.to_string().len();
    let sum = num
        .to_string()
        .chars()
        .map(|d| (d.to_digit(10).unwrap() as u64).pow(count as u32))
        .sum::<u64>();

    sum == num as u64
}
