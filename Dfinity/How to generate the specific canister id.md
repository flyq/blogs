# How to generate the specific canister id

## How canister id generated
```rs
pub const REGISTRY_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 0;
pub const GOVERNANCE_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 1;
pub const LEDGER_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 2;
pub const ROOT_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 3;
pub const CYCLES_MINTING_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 4;
pub const LIFELINE_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 5;
pub const GENESIS_TOKEN_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 6;
pub const IDENTITY_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 7;
pub const NNS_UI_CANISTER_INDEX_IN_NNS_SUBNET: u64 = 8;
```
As we can see here, The canister id is only related to a u64 type index. In fact, the canister id is obtained by u64 serialization to get 8 bytes plus two 1 (specified in the interface spec) to get a 10-bytes principal. So there is a one-to-one correspondence between canister id and u64.
The range of canister id for different subnets is divided by the premise, such as [0, 2^20) range is in the NNS system subnet, [2^20, 2^21) is another subnet, let the canister in the subnet according to In the order of creation, the corresponding index is obtained in turn. For example, registry is 0, governance is 1, and ledger is 3. The first canister in the second subnet is 2^20.

## in local env
```sh
rwlgt-iiaaa-aaaaa-aaaaa-cai	    nns/registry
rrkah-fqaaa-aaaaa-aaaaq-cai	    nns/governance
ryjl3-tyaaa-aaaaa-aaaba-cai	    nns/ledger
r7inp-6aaaa-aaaaa-aaabq-cai	    nns/root
```
When start a local env, it is like a new System subnet.
1. When deploy, there will be generated a wallet canister, which is the first canister in local, so it will be rwlgt-iiaaa-aaaaa-aaaaa-cai.
2. then we need a placehold the be the governance, that is rrkah-fqaaa-aaaaa-aaaaq-cai. so let's set canister "1", (which will deploy first, number first, and alphabet).
3. we set the ledger, to let it be the ryjl3-tyaaa-aaaaa-aaaba-cai:

```json
{
  "version": 1,
  "dfx": "0.9.3",
  "canisters": {
    "ledger": {
      "type": "rust",
      "package": "oneshot",
      "candid": "src/oneshot/oneshot.did"
    },
    "1": {
      "type": "motoko",
      "main": "src/callee/src/main.mo"
    }
  }
}
```
the result is :
```sh
dfx deploy
Creating a wallet canister on the local network.
The wallet canister on the "local" network for user "icp" is "rwlgt-iiaaa-aaaaa-aaaaa-cai"
Deploying all canisters.
Creating canisters...
Creating canister "1"...
"1" canister created with canister id: "rrkah-fqaaa-aaaaa-aaaaq-cai"
Creating canister "ledger"...
"ledger" canister created with canister id: "ryjl3-tyaaa-aaaaa-aaaba-cai"
Building canisters...
Executing: cargo build --target wasm32-unknown-unknown --release -p oneshot
```