fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let _plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();

    let _new_numbers: Vec<_> = numbers.iter().filter(|&num| *num <= 3).collect();

    let _find_me: Option<i32> = numbers.iter().find(|num| **num == 3).copied();

    let _count = numbers.iter().count();

    let _last: Option<i32> = numbers.iter().last().copied();

    let _min: Option<i32> = numbers.iter().min().copied();

    let _max: Option<i32> = numbers.iter().max().copied();

    let _take: Vec<i32> = numbers.iter().take(3).cloned().collect();
}
