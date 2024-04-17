/*

Problem:
    Climbing Stairs (3 steps)

    You are climbing a stair case. It takes n steps to reach to the top.
    Each time you can either climb 1, 2 or 3 steps.
    In how many distinct ways can you climb to the top?

Framework for Solving DP Problems:
    1. Define the objective function
        f(i) is the number of distinct ways to reach the i-th stair.
    2. Identify base cases
        f(0) = 1
        f(1) = 1
        f(2) = 2
    3. Write down a recurrence relation for the optimized objective function
        f(n) = f(n-1) + f(n-2) + f(n-3)
    4. What's the order of execution?
        bottom-up
    5. Where to look for the answer?
        f(n)
*/

// Time complexity: O(n)
// Space complexity: O(n)
#[allow(dead_code)]
fn climb_stairs_3_steps(n: usize) -> usize {
    let mut dp = vec![0; std::cmp::max(3, n + 1)];
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
    }
    dp[n]
}

// Time complexity: O(n)
// Space complexity: O(1)
#[allow(dead_code)]
fn climb_stairs_3_steps_opt(n: usize) -> usize {
    if n < 2 {
        return 1;
    }
    let mut a = 1;
    let mut b = 1;
    let mut c = 2;
    let mut d = 2;
    for _ in 3..=n {
        d = a + b + c;
        a = b;
        b = c;
        c = d;
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_case1() {
        assert_eq!(climb_stairs_3_steps(0), 1);
        assert_eq!(climb_stairs_3_steps_opt(0), 1);
    }

    #[test]
    fn edge_case2() {
        assert_eq!(climb_stairs_3_steps(1), 1);
        assert_eq!(climb_stairs_3_steps_opt(1), 1);
    }

    #[test]
    fn edge_case3() {
        assert_eq!(climb_stairs_3_steps(2), 2);
        assert_eq!(climb_stairs_3_steps_opt(2), 2);
    }

    #[test]
    fn simple_test1() {
        assert_eq!(climb_stairs_3_steps(3), 4);
        assert_eq!(climb_stairs_3_steps_opt(3), 4);
    }

    #[test]
    fn simple_test2() {
        assert_eq!(climb_stairs_3_steps(5), 13);
        assert_eq!(climb_stairs_3_steps_opt(5), 13);
    }
}
