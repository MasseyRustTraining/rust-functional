fn produce<F, G>(
    range: std::ops::RangeInclusive<u64>,
    range_filter: F,
    (fold_start, folder): (u64, G),
) -> u64 where
    F: FnMut(&u64) -> bool,
    G: FnMut(u64, u64) -> u64,
{
    range.filter(range_filter).fold(fold_start, folder)
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

    let filter = |&n: &u64| {
        n % 2 == 0
    };
    let summer = |a: u64, n: u64| {
        a + n
    };
    let sum = produce(1..=100, filter, (0, summer));
    println!("{}", sum);
}
