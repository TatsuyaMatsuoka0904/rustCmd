use std::time::Duration;
use std::thread::sleep;
extern crate rand;
use rand::Rng;

pub fn angle_koyori() {
    println!("+  ⊂＝＝⊃");
    println!(" (ᐡ o̴̶̷̤ ﻌ o̴̶̷̤ ᐡ)  +");
    println!(" i⌒/   つつ");
    println!(" .川　  ﾉ");
    println!("+   u u");
}

pub fn timekeep() {
    sleep(Duration::from_millis(500));
    println!(".");
    sleep(Duration::from_millis(500));
    println!("..");
    sleep(Duration::from_millis(500));
    println!("...");
    sleep(Duration::from_millis(500));

}

pub fn rand() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
}

