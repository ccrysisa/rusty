/*

Problem:
    Climbing Stairs

    You are climbing a stair case. It takes n steps to reach to the top.
    Each time you can either climb 1 or 2 steps.
    In how many distinct ways can you climb to the top?

Framework for Solving DP Problems:
    1. Define the objective function
        f(i) is the number of distinct ways to reach the i-th stair.
    2. Identify base cases
        f(0) = 1
        f(1) = 1
    3. Write down a recurrence relation for the optimized objective function
        f(n) = f(n-1) + f(n-2)
    4. What's the order of execution?
        bottom-up
    5. Where to look for the answer?
        f(n)
*/

// Time complexity: O(n)
// Space complexity: O(n)
#[allow(dead_code)]
fn climb_stairs(n: usize) -> usize {
    let mut dp = vec![0; std::cmp::max(2, n + 1)];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

// Time complexity: O(n)
// Space complexity: O(1)
#[allow(dead_code)]
fn climb_stairs_opt(n: usize) -> usize {
    //  a b c
    //    a b c
    let mut a: usize = 1;
    let mut b: usize = 1;
    let mut c: usize = 1;
    for _ in 2..=n {
        c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_case1() {
        assert_eq!(climb_stairs(0), 1);
        assert_eq!(climb_stairs_opt(0), 1);
    }

    #[test]
    fn edge_case2() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs_opt(1), 1);
    }

    #[test]
    fn simple_test1() {
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs_opt(3), 3);
    }

    #[test]
    fn simple_test2() {
        assert_eq!(climb_stairs(5), 8);
        assert_eq!(climb_stairs_opt(5), 8);
    }

    #[test]
    fn memory_test() {
        // assert_eq!(climb_stairs(1000000), 2756670985995446685);
        assert_eq!(climb_stairs_opt(1000000), 2756670985995446685);
    }
}
