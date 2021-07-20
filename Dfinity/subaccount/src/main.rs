extern crate ic_base_types;
extern crate ledger_canister;

use ic_base_types::{CanisterId, PrincipalId, PrincipalIdParseError, PrincipalIdBlobParseError};
use ledger_canister::account_identifier::{AccountIdentifier, Subaccount};
use ic_crypto_sha256::Sha256;

const CRC_LENGTH_IN_BYTES: usize = 4;

fn main() {
    let governance = CanisterId::from_u64(1);
    let caller = from_str("yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae").unwrap();
    let sub : Subaccount = compute_neuron_staking_subaccount(caller, 0);

    let to = AccountIdentifier::new(governance.clone().get(), Some(sub));
    println!("to: {:?}\n to_hex: {:?}\n sub: {:?}\n", to, to.to_hex(), sub); 
}

fn compute_neuron_staking_subaccount(controller: PrincipalId, nonce: u64) -> Subaccount {
    // The equivalent function in the UI is
    // https://github.com/dfinity/dfinity_wallet/blob/351e07d3e6d007b090117161a94ce8ec9d5a6b49/js-agent/src/canisters/createNeuron.ts#L63
    Subaccount({
        let mut state = Sha256::new();
        state.write(&[0x0c]);
        state.write(b"neuron-stake");
        state.write(&controller.as_slice());
        state.write(&nonce.to_be_bytes());
        state.finish()
    })
}

fn from_str(input: &str) -> Result<PrincipalId, PrincipalIdParseError> {
    // Strategy: Parse very liberally, then pretty-print and compare output.
    // This is both simpler and yields better error messages.
    let mut s = input.to_string();
    s.make_ascii_lowercase();
    s.retain(|c| c.is_ascii_alphanumeric());
    match base32::decode(base32::Alphabet::RFC4648 { padding: false }, &s) {
        Some(mut bytes) => {
            if bytes.len() < CRC_LENGTH_IN_BYTES {
                return Err(PrincipalIdParseError::TooShort);
            }
            if bytes.len() > PrincipalId::MAX_LENGTH_IN_BYTES + CRC_LENGTH_IN_BYTES {
                return Err(PrincipalIdParseError::TooLong);
            }
            let result =
                try_from(&bytes.split_off(CRC_LENGTH_IN_BYTES)[..]).unwrap();
            let expected = format!("{}", result);
            if input != expected {
                return Err(PrincipalIdParseError::Wrong { expected });
            }
            Ok(result)
        }
        None => Err(PrincipalIdParseError::NotBase32),
    }
}


fn try_from(blob: &[u8]) -> Result<PrincipalId, PrincipalIdBlobParseError> {
    if blob.len() != PrincipalId::MAX_LENGTH_IN_BYTES {
        return Err(PrincipalIdBlobParseError::TooLong(blob.len()));
    }
    let mut data = [0u8; PrincipalId::MAX_LENGTH_IN_BYTES];
    data[..blob.len()].copy_from_slice(&blob[..]);
    println!("{:?}\n", data);
    Ok(PrincipalId::new(blob.len(), data))
}

// output:
// data:
// [152, 239, 24, 172, 140, 12, 215, 142, 44, 41, 97, 216, 243, 158, 45, 26, 194, 160, 168, 254, 119, 25, 239, 130, 215, 66, 209, 166, 2]
//
// to: AccountIdentifier { hash: [216, 185, 241, 146, 77, 175, 16, 205, 245, 135, 159, 110, 74, 92, 50, 103, 222, 213, 167, 239, 181, 231, 64, 179, 226, 248, 90, 227] }
// to_hex: "8ca2e53dd8b9f1924daf10cdf5879f6e4a5c3267ded5a7efb5e740b3e2f85ae3"
// sub: Subaccount([151, 153, 127, 225, 241, 148, 194, 208, 107, 96, 196, 180, 189, 234, 210, 114, 65, 81, 192, 46, 165, 142, 58, 137, 118, 125, 47, 181, 92, 181, 165, 94])