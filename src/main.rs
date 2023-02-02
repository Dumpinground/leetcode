fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use leetcode::{*, solution::Solution};

    #[test]
    fn test1() {
        Solution::two_sum(vec![1, 2, 3], 2);
    }

    #[test]
    fn test13() {
        Solution::roman_to_int(String::from("123"));
    }

    #[test]
    fn test14() {
        Solution::longest_common_prefix(vec![String::from("123")]);
    }
}
