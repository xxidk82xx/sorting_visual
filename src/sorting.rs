
use rand::seq::SliceRandom;
use rand::thread_rng;
use core::time;
use std::thread;
use std::sync::mpsc::Sender;
pub fn sort(tx:Sender<usize>, tax:Sender<Vec<u32>>) {
    let mut array: Vec<u32> = (1..10).collect();
    array.shuffle(&mut thread_rng());
    let mut swapped;
    loop {
        swapped = false;
        println!("test");
        for i in 1..array.len() {
            tx.send(i).unwrap_or(());
            tax.send(array.clone()).unwrap_or(());
            swapped = bubble_iter(&mut array, i) || swapped;
            thread::sleep(time::Duration::from_secs_f32(0.1));
        }
        if !swapped {
            return;
        }
    }
}
fn bubble_iter(arr: &mut Vec<u32>, i:usize) -> bool {
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
