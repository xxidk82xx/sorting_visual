
use rand::seq::SliceRandom;
use rand::thread_rng;
use core::time;
use std::thread;
use std::sync::mpsc::Sender;
pub fn sort<T:Ord+Clone>(mut arr:Vec<T>, tx:Sender<Vec<usize>>, tax:Sender<Vec<T>>) {
    arr.shuffle(&mut thread_rng());
    let mut swapped;
    for i in 0..arr.len() - 1 {
        swapped = false;
        for j in 1..arr.len()-i {
            tx.send(vec![j, j-1]).unwrap_or(());
            tax.send(arr.clone()).unwrap_or(());
            swapped = if i == arr.len() {
                false
            } else {
                if arr[j-1] >= arr[j] {
                    arr.swap(j-1, j);
                    true
                }
                else {
                    false
                }
            } || swapped;
            thread::sleep(time::Duration::from_secs_f32(0.3));
        }
        if !swapped {
            break;
        }
    }
    if verify_sorted(arr, tx, tax) == false {
        println!("didnt sort properly");
    } 
}

pub fn verify_sorted<T:Ord+Clone>(arr:Vec<T>, tx:Sender<Vec<usize>>, tax:Sender<Vec<T>>) ->bool {
    let mut sorted = true;
    for i in 1..arr.len() {
        tx.send(vec![i]).unwrap_or(());
        tax.send(arr.clone()).unwrap_or(());
        sorted = sorted && arr[i-1] < arr[i];
        thread::sleep(time::Duration::from_secs_f32(0.3))
    }
    sorted
}
