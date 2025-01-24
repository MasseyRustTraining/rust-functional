fn accumulate<I>(
    range: std::ops::RangeInclusive<u64>,
    range_filter: impl Fn(&u64) -> bool,
    accumulator: impl Fn(I) -> u64,
) -> u64
{
    accumulator(range.filter(range_filter))
}

fn main() {
    let numbers: Vec<u64> = (1..=100).collect();

    let mut sum: u64 = 0;
    for &n in &numbers {
        sum += n as u64;
    }
    println!("{}", sum);

    let num_iterator = numbers.iter();
    let sum: u64 = num_iterator.fold(0u64, |a, &n| a + n);
    println!("{}", sum);

    let sum: u64 = numbers.iter().sum();
    println!("{}", sum);

    fn summer(iter: impl Iterator<Item = u64>) -> u64 {
        iter.sum()
    }
    let filter = |&n| n % 2 == 0;
    let sum = accumulate(1..=100, filter, summer);
    println!("{}", sum);
}
