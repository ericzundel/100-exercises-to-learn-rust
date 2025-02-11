// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let mut result1 : i32 = 0;
    let mut result2 : i32 = 0;
    std::thread::scope(|s| {
       let  (s1, s2) = v.split_at(v.len()/2);
        s.spawn(|| {
           result1 = s1.iter().sum::<i32>();
        });
        s.spawn(|| {
            result2 = s2.iter().sum::<i32>();
        });
    });
    result1 + result2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
