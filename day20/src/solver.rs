use std::collections::VecDeque;

pub fn decrypt(input: &str, key: i64, rounds: usize) -> i64 {
    let original_numbers = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut indices = (0..original_numbers.len()).collect::<VecDeque<_>>();

    for _round in 0..rounds {
        for i in 0..original_numbers.len() {
            let pos = indices.iter().position(|&p| p == i).unwrap();

            let num = original_numbers[i] * key;

            let ir = indices.remove(pos);
            assert_eq!(ir, Some(i));

            let new_index = (pos as i64 + num).rem_euclid(original_numbers.len() as i64 - 1);
            indices.insert(new_index as usize, i);
        }
    }

    let numbers = indices
        .iter()
        .map(|&i| original_numbers[i])
        .collect::<Vec<_>>();

    let zero = numbers.iter().position(|&n| n == 0).unwrap();
    let indices = [1000, 2000, 3000];

    indices
        .iter()
        .map(|offset| numbers[(zero + offset) % numbers.len()] * key)
        .sum()
}