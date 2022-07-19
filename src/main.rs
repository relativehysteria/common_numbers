use std::collections::HashSet;
use std::cmp::Ordering;
use std::time::{Duration, Instant};
use std::thread;

fn main() {
    let a_orig = (0..1 << 17).collect::<Vec<u32>>();
    let b_orig = (0..1 << 16).collect::<Vec<u32>>();

    let mut threads = Vec::with_capacity(3);

    // Find the common numbers by iterating through the vectors
    let a = a_orig.clone();
    let b = b_orig.clone();
    threads.push(thread::spawn(move || {
        println!("Unsorted array: {:?}", unsorted_array(&a, &b).1);
    }));

    // Find the common numbers by turning the vectors into a set and finding
    // their intersection
    let a = a_orig.clone();
    let b = b_orig.clone();
    threads.push(thread::spawn(move || {
        println!("HashSet: {:?}", hashset(&a, &b).1);
    }));

    // Find the common numbers by sorting the vectors and iterating through them
    let mut a = a_orig.clone();
    let mut b = b_orig.clone();
    threads.push(thread::spawn(move || {
        println!("Sorted array: {:?}", sorted_array(&mut a, &mut b).1);
    }));

    threads.into_iter().for_each(|thread| { thread.join(); });
}

/// Naively find the common numbers in two vectors by iterating through them.
fn unsorted_array(a: &Vec<u32>, b: &Vec<u32>) -> (usize, Duration) {
    let now = Instant::now();

    let mut res = 0;
    for i in a.iter() {
        for j in b.iter() {
            if i == j {
                res += 1;
            }
        }
    }

    (res, now.elapsed())
}

/// Find the common numbers in two vectors by creating sets out of them
/// and then finding their intersection.
fn hashset(a: &Vec<u32>, b: &Vec<u32>) -> (usize, Duration) {
    let now = Instant::now();

    let mut a_set = HashSet::with_capacity(a.capacity());
    let mut b_set = HashSet::with_capacity(b.capacity());

    a.iter().for_each(|e| { a_set.insert(e); });
    b.iter().for_each(|e| { b_set.insert(e); });

    let res = a_set.intersection(&b_set).into_iter().count();

    (res, now.elapsed())
}

/// Find the common numbers in two vectors by *mutably* sorting them
/// and then iterating through them.
fn sorted_array(a: &mut Vec<u32>, b: &mut Vec<u32>) -> (usize, Duration) {
    let now = Instant::now();

    a.sort_unstable();
    b.sort_unstable();

    let mut i   = 0;
    let mut j   = 0;
    let mut res = 0;
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Less    => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal   => {
                res += 1;
                i   += 1;
                j   += 1;
            },
        }
    }

    (res, now.elapsed())
}
