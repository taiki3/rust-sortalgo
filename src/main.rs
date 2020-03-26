#![feature(test)]

extern crate test;
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let mut y :Vec<i32>= (1..100).collect();
    let z :Vec<i32>= (1..100).collect();
    y.shuffle(&mut rng);
    println!("Unshuffled: {:?}", z);
    println!("Shuffled:   {:?}", y);
}

fn bbsort<T: PartialOrd>(source: &mut [T]){
    for _i in 0..source.len() {
        for j in 1..source.len() {
            if source[j] < source[j - 1]{
                source.swap(j, j-1)
            }
        }
    }
}

fn bbsort2<T: PartialOrd>(source: &mut [T]){
    for i in 0..source.len() {
        for j in 1..source.len()-i {
            if source[j] < source[j - 1]{
                source.swap(j, j-1)
            }
        }
    }
}

fn comsort<T: PartialOrd> (source: &mut [T]){
    let mut h = source.len();
    let mut is_swapped = true;

    while is_swapped || h>1 {
        if h > 1{
            h = h * 10 / 13;
        }

        let mut i = 0;
        is_swapped = false;
        while i < source.len() - h {
            if source[i] > source[i+h] {
                source.swap(i, i+h);
                is_swapped = true;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    // prepare values
    #[test]
    fn sort_test() {
        let mut rng = thread_rng();
        let mut y :Vec<i32>= (1..100).collect();
        let z :Vec<i32>= (1..100).collect();
        y.shuffle(&mut rng);
        bbsort(&mut y);
        assert_eq!(y, z);
        y.shuffle(&mut rng);
        bbsort2(&mut y);
        assert_eq!(y, z);
        y.shuffle(&mut rng);
        comsort(&mut y);
        assert_eq!(y, z);
    }
    #[bench]
    fn bench_bbsort(b: &mut Bencher) {
        let mut rng = thread_rng();
        let mut y :Vec<i32>= (1..1000_0).collect();
        y.shuffle(&mut rng);
        b.iter(|| bbsort(&mut y));
    }
    
    #[bench]
    fn bench_bbsort2(b: &mut Bencher) {
        let mut rng = thread_rng();
        let mut y :Vec<i32>= (1..1000_0).collect();
        y.shuffle(&mut rng);
        b.iter(|| bbsort2(&mut y));
    }

    #[bench]
    fn bench_comsort(b: &mut Bencher) {
        let mut rng = thread_rng();
        let mut y :Vec<i32>= (1..1000_0).collect();
        y.shuffle(&mut rng);
        b.iter(|| comsort(&mut y));
    }
    
}