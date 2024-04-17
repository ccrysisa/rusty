/*

Problem:
    Climbing Stairs (k steps)

    You are climbing a stair case. It takes n steps to reach to the top.
    Each time you can climb 1..k steps.
    In how many distinct ways can you climb to the top?

Framework for Solving DP Problems:
    1. Define the objective function
        f(i) is the number of distinct ways to reach the i-th stair by making 1 to k steps.
    2. Identify base cases
        f(0) = 1
        f(1) = 1
    3. Write down a recurrence relation for the optimized objective function
        f(n) = f(n-1) + f(n-2) + ... + f(n-k)
    4. What's the order of execution?
        bottom-up
    5. Where to look for the answer?
        f(n)
*/

// Time complexity: O(n)
// Space complexity: O(n)
#[allow(dead_code)]
fn climb_stairs_k_steps(n: usize, k: usize) -> usize {
    let mut dp = vec![0; std::cmp::max(2, n + 1)];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        for j in 1..=k {
            // if i - j < 0
            if i < j {
                break;
            }
            dp[i] += dp[i - j];
        }
    }
    dp[n]
}

// Time complexity: O(n)
// Space complexity: O(k)
#[allow(dead_code)]
fn climb_stairs_k_steps_opt(n: usize, k: usize) -> usize {
    let mut dp = vec![0; k];
    dp[0] = 1;
    for i in 1..=n {
        for j in 1..k {
            // if i - k < 0
            if i < j {
                break;
            }
            // dp[i % k] += dp[(i - j) % k];
            dp[i % k] = (dp[i % k] as usize).wrapping_add(dp[(i - j) % k]);
        }
    }
    dp[n % k]
}

/*
Problem:
    Climbing Stairs (k steps, space optimized, skip red steps)

    You are climbing a stair case. It takes n steps to reach to the top.
    Each time you can climb 1..k steps. You are not allowed to step on red stairs.
    In how many distinct ways can you climb to the top?
*/

// Time complexity: O(n)
// Space complexity: O(k)
#[allow(dead_code)]
fn climb_stairs_k_steps_skip_red(n: usize, k: usize, stairs: &[bool]) -> usize {
    let mut dp = vec![0; k];
    dp[0] = 1;
    for i in 1..=n {
        if stairs[i - 1] {
            dp[i % k] = 0;
            continue;
        }
        for j in 1..k {
            // if i - k < 0
            if i < j {
                break;
            }
            // dp[i % k] += dp[(i - j) % k];
            dp[i % k] = (dp[i % k] as usize).wrapping_add(dp[(i - j) % k]);
        }
    }
    dp[n % k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test1() {
        assert_eq!(climb_stairs_k_steps(3, 2), 3);
        assert_eq!(climb_stairs_k_steps_opt(3, 2), 3);
    }

    #[test]
    fn simple_test2() {
        assert_eq!(climb_stairs_k_steps(5, 2), 8);
        assert_eq!(climb_stairs_k_steps_opt(5, 2), 8);
    }

    #[test]
    fn simple_test3() {
        assert_eq!(climb_stairs_k_steps(3, 3), 4);
        assert_eq!(climb_stairs_k_steps_opt(3, 3), 4);
    }

    #[test]
    fn simple_test4() {
        assert_eq!(climb_stairs_k_steps(5, 3), 13);
        assert_eq!(climb_stairs_k_steps_opt(5, 3), 13);
    }

    #[test]
    fn memory_test() {
        assert_eq!(climb_stairs_k_steps_opt(1000000, 2), 2756670985995446685);
    }

    #[test]
    fn skip_red_test() {
        assert_eq!(
            climb_stairs_k_steps_skip_red(7, 3, &[false, true, false, true, true, false, false]),
            2
        );
    }
}
