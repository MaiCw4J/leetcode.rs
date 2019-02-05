// https://leetcode.com/problems/fibonacci-number/

pub fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

pub fn fib_ii(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    let n = n as usize;
    let mut c = vec![0; n + 1];
    c[0] = 0;
    c[1] = 1;
    for i in 2..=n {
        c[i] = c[i - 1] + c[i - 2];
    }
    c[n]
}

pub fn fib_iii(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    let (mut a, mut b, mut c) = (0, 1, 0);
    for i in 1..n {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = 4;
        let output = 3;

        assert_eq!(output, fib(input));
        assert_eq!(output, fib_ii(input));
        assert_eq!(output, fib_iii(input));
    }

}