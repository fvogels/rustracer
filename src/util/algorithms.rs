pub struct VecDiffResult {
    shared: Vec<(usize, usize)>,
    left_only: Vec<usize>,
    right_only: Vec<usize>,
}

pub fn vec_diff<T : PartialEq>(xs: &Vec<T>, ys: &Vec<T>) -> VecDiffResult {
    let mut ys_used = Vec::new();
    let mut shared = Vec::new();
    let mut left_only = Vec::new();
    let mut right_only = Vec::new();

    ys_used.resize(ys.len(), false);

    for i in 0..xs.len() {
        match (0..ys.len()).find(|j| !ys_used[*j] && xs[i] == ys[*j]) {
            None => {
                left_only.push(i);
            },
            Some(j) => {
                ys_used[j] = true;
                shared.push((i, j));
            }
        }
    }

    for j in 0..ys_used.len() {
        if !ys_used[j] {
            right_only.push(j);
        }
    }

    VecDiffResult { shared, left_only, right_only }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[rstest]
    #[case(vec![], vec![], vec![], vec![], vec![])]
    #[case(vec![0], vec![], vec![], vec![0], vec![])]
    #[case(vec![], vec![0], vec![], vec![], vec![0])]
    #[case(vec![0], vec![0], vec![(0, 0)], vec![], vec![])]
    #[case(vec![0, 1], vec![0], vec![(0, 0)], vec![1], vec![])]
    #[case(vec![0, 1], vec![0, 2], vec![(0, 0)], vec![1], vec![1])]
    #[case(vec![0, 1], vec![1, 0], vec![(0, 1), (1, 0)], vec![], vec![])]
    #[case(vec![0, 1, 2], vec![1, 0, 3], vec![(0, 1), (1, 0)], vec![2], vec![2])]
    fn test_vec_diff(#[case] xs: Vec<i32>, #[case] ys: Vec<i32>, #[case] expected_shared: Vec<(usize, usize)>, #[case] expected_left: Vec<usize>, #[case] expected_right: Vec<usize>) {
        let actual = vec_diff(&xs, &ys);

        assert_eq!(expected_shared, actual.shared);
        assert_eq!(expected_left, actual.left_only);
        assert_eq!(expected_right, actual.right_only);
    }
}
