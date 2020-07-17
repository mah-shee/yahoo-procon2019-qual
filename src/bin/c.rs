#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut k: u128,
        a: u128,
        b: u128
    }
    if b - a <= 2 {
        println!("{}", 1 + k);
    } else {
        let mut biscuits = 1;
        biscuits += a - 1;
        k -= a - 1;
        biscuits += k / 2 * (b - a);
        k = k % 2;
        biscuits += k;
        println!("{}", biscuits);
    }
}
