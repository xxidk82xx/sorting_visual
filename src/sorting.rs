
use rand::seq::SliceRandom;
use rand::thread_rng;
use core::time;
use std::thread;
use std::sync::mpsc::Sender;
pub fn sort<T:Ord+Clone+Copy>(mut arr:Vec<T>, tx:Sender<Vec<usize>>, tax:Sender<Vec<T>>) {
    //let mut array: Vec<u16> = (1..10).collect();
    arr.shuffle(&mut thread_rng());
    let mut swapped;
    loop {
        swapped = false;
        for i in 1..arr.len() {
            tx.send(vec![i, i-1]).unwrap_or(());
            tax.send(arr.clone()).unwrap_or(());
            swapped = bubble_iter(&mut arr, i) || swapped;
            thread::sleep(time::Duration::from_secs_f32(0.1));
        }
        if !swapped {
            return;
        }
    }
}
fn bubble_iter<T:Ord+Copy>(arr: &mut Vec<T>, i:usize) -> bool {
	if i == arr.len() {
		false
	} else {
		if arr[i-1] >= arr[i] {
			let a = arr[i-1];
			arr[i-1] = arr[i];
			arr[i] = a;
			true
		}
		else {
			false
		}
	}
}

