extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::Zero;

fn main() {
    let s =String::from("80DD5113FEDED638E5500E65779613BDD3BDDBEB8EB5D86CDD3370E629B02E92CDE3C3FC9D");
    
    println!("{:?}", hexstring_to_base58(&s));
}

fn hexstring_to_base58(s: &String) -> String {
    let base_char = String::from("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
    let base_char_vec: Vec<char> = base_char.chars().collect();

    
    let mut a: BigUint = BigUint::parse_bytes(s.as_bytes(),16).unwrap();

    let mut res = vec![];
    while a.clone() > Zero::zero() {
        let index = a.clone() % BigUint::from(58u32);
        let index = *index.to_bytes_le().get(0).unwrap() as usize;
        res.push(base_char_vec[index]);
        a /= BigUint::from(58u32);
    }

    let res: String = res.iter().rev().collect();
    res
}

