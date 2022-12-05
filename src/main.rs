use crypto_hash::{Algorithm, Hasher};
use std::io::Write;
use std::sync::mpsc::channel;
use std::thread;

/// Bruteforce 6 digits PIN MD5 hash
fn main() {
    let iterations = 25 * 400;
    let md5: u128 = 0xd04988522ddfed3133cc24fb6924eae9;

    let threads = 8;
    let pins_per_thread = 1_000_000 / threads;
    let (sender, receiver) = channel();

    println!("Bruteforce running...");

    for t in 0..threads {
        let t_sender = sender.clone();

        thread::spawn(move || {
            let mut hasher = Hasher::new(Algorithm::MD5);

            for n in t * pins_per_thread..(t + 1) * pins_per_thread {
                let mut hash = format!("{:0>6}", n).as_bytes().to_vec();
                for _i in 0..iterations {
                    hasher.write_all(hash.as_slice()).unwrap();
                    hash = hasher.finish();
                }
                if hash == md5.to_be_bytes() {
                    t_sender.send(format!("{:0>6}", n)).unwrap();
                    return;
                }
                if n % 5000 == 0 {
                    println!("[Thread {}] I'm alive and I'm evaluating: {}", t,
                             format!("{:0>6}", n));
                }
            }
        });
    }

    drop(sender);
    let flag = receiver.recv().expect("PIN not found");
    println!("The PIN is: {}", flag);
}
