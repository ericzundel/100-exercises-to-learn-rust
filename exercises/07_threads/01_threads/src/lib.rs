// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

use std::sync::mpsc;
// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let half = v.len() / 2;
    let v1 : Vec<i32> = v[0..half].to_vec();
    let v2: Vec<i32>  = v[half..].to_vec();

    let (tx1, rx1) = mpsc::channel(); 
    let (tx2, rx2) = mpsc::channel();
    let handle1 =  thread::spawn(move || {
        let mut result: i32 = 0;
        for num in v1 {
            result += num;
        }
        tx1.send(result).unwrap();
    });

    let handle2 = thread::spawn(move || {
        let mut result: i32 = 0;
        for num in v2 {
            result += num;
        }
        tx2.send(result).unwrap();
    });
    let result1 = rx1.recv().unwrap();
    let result2 = rx2.recv().unwrap();
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    return result1 + result2;
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
