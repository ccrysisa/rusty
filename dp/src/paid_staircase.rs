/*
Problem:
    Paid Staircase

    You are climbing a paid staircase. It takes n steps to reach to the top and you have to
    pay p[i] to step on the i-th stair. Each time you can climb 1 or 2 steps.
    What's the cheapest amount you have to pay to get to the top of the staircase?
*/

// Time complexity: O(n)
// Space complexity: O(n)
#[allow(dead_code)]
fn paid_staircase(n: usize, p: &[usize]) -> usize {
    let mut dp = vec![0; std::cmp::max(2, n + 1)];
    dp[0] = 0;
    dp[1] = p[1];
    for i in 2..=n {
        dp[i] = p[i] + std::cmp::min(dp[i - 1], dp[i - 2]);
    }
    dp[n]
}

// Time complexity: O(n)
// Space complexity: O(1)
#[allow(dead_code)]
fn paid_staircase_opt(n: usize, p: &[usize]) -> usize {
    if n < 1 {
        return 0;
    }
    let mut a = 0;
    let mut b = p[1];
    let mut c = b;
    for i in 2..=n {
        c = p[i] + std::cmp::min(a, b);
        a = b;
        b = c;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_cases() {
        assert_eq!(paid_staircase(0, &[0, 2]), 0);
        assert_eq!(paid_staircase(1, &[0, 2]), 2);
        assert_eq!(paid_staircase_opt(0, &[0, 2]), 0);
        assert_eq!(paid_staircase_opt(1, &[0, 2]), 2);
    }

    #[test]
    fn simple_test() {
        assert_eq!(paid_staircase(3, &[0, 3, 2, 4]), 6);
        assert_eq!(paid_staircase_opt(3, &[0, 3, 2, 4]), 6);
    }
}
