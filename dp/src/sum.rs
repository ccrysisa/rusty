/*

Problem:
    Find the sum of the first N numbers.

Objective function:
    f(i) is the sum of the first i elements.

Recurrence relation:
    f(n) = f(n-1) + n

 */

// Time complexity: O(n)
// Space complexity: O(n)
#[allow(unused)]
fn sum(n: usize) -> usize {
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        dp[i] = dp[i - 1] + i;
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_case1() {
        assert_eq!(sum(0), 0);
    }

    #[test]
    fn edge_case2() {
        assert_eq!(sum(1), 1);
    }

    #[test]
    fn simple_test() {
        assert_eq!(sum(5), 15);
    }
}
