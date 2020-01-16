extern crate hex;

const ALPHABET: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn main() {
    let input = "80DD5113FEDED638E5500E65779613BDD3BDDBEB8EB5D86CDD3370E629B02E92CDE3C3FC9D";
    let decoded = hex::decode(input).expect("Decoding failed");
    let res = to_base58(&decoded);
    println!("{:?}\n{:?}", decoded, res);
    
}

fn to_base58(v: &[u8]) -> String {
    let zcount = v.iter().take_while(|x| **x == 0).count();
    let size = (v.len() - zcount) * 138 / 100 + 1;
    let mut buffer = vec![0u8; size];

    let mut i = zcount;
    let mut high = size - 1;

    while i < v.len() {
        let mut carry = v[i] as u32;
        let mut j = size - 1;

        while j > high || carry != 0 {
            carry += 256 * buffer[j] as u32;
            buffer[j] = (carry % 58) as u8;
            carry /= 58;

            // in original trezor implementation it was underflowing
            if j  > 0 {
                j -= 1;
            }
        }

        i += 1;
        high = j;
    }

    let mut j = buffer.iter().take_while(|x| **x == 0).count();

    let mut result = String::new();
    for _ in 0..zcount {
        result.push('1');
    }

    while j < size {
        result.push(ALPHABET[buffer[j] as usize] as char);
        j += 1;
    }

    result
}
