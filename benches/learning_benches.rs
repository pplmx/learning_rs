#![feature(test)]

extern crate rand;
extern crate test;

use test::Bencher;

use rand::seq::SliceRandom;
use rand::thread_rng;

use learning_rs::{qs, quick_sort};

fn random_array(size: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut arr: Vec<i32> = (0..size).map(|x| x as i32).collect(); // Cast usize to i32
    arr.shuffle(&mut rng);
    arr
}

#[bench]
fn bench_std_sort(b: &mut Bencher) {
    let mut arr = random_array(10000); // Change the size as needed
    b.iter(|| {
        arr.sort();
    });
}

#[bench]
fn bench_qs(b: &mut Bencher) {
    let arr = random_array(10000); // Change the size as needed
    b.iter(|| {
        qs(arr.clone());
    });
}

#[bench]
fn bench_quick_sort(b: &mut Bencher) {
    let mut arr = random_array(10000); // Change the size as needed
    b.iter(|| {
        quick_sort(&mut arr);
    });
}
