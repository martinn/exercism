/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|c| !c.is_digit(10) && !c.is_whitespace()) {
        return false;
    }
    let sum = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .fold((0, 0), |(sum, count), c| match c.to_digit(10) {
            Some(d) => {
                if count % 2 == 0 {
                    (sum + d, count + 1)
                } else {
                    (sum + if d * 2 > 9 { d * 2 - 9 } else { d * 2 }, count + 1)
                }
            }
            None => (0, count),
        });

    sum.1 > 1 && sum.0 % 10 == 0
}
