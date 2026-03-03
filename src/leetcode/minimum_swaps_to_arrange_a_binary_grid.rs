pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();

    let mut zeroes: Vec<usize> = grid
        .into_iter()
        .map(|row| row.iter().rev().take_while(|&&c| c == 0).count())
        .collect();

    let mut swaps = 0;
    for i in 0..n {
        let target = n - i - 1;
        let j = (i..n).find(|&j| zeroes[j] >= target);
        match j {
            Some(j) => {
                zeroes[i..=j].rotate_right(1);
                swaps += (j - i) as i32;
            }
            None => return -1,
        }
    }

    swaps
}

#[cfg(test)]
mod tests {
    use crate::leetcode::minimum_swaps_to_arrange_a_binary_grid::min_swaps;

    #[test]
    fn test_min_swaps() {
        let grid = vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];
        assert_eq!(3, min_swaps(grid))
    }
}
