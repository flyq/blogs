# Source Code for Cycle Minting Canister

## contents
- [Source Code for Cycle Minting Canister](#source-code-for-cycle-minting-canister)
  - [contents](#contents)
  - [struct](#struct)
    - [CyclesCanisterInitPayload](#cyclescanisterinitpayload)
    - [CreateCanisterResult](#createcanisterresult)
    - [TopUpCanisterResult](#topupcanisterresult)
    - [IcptsToCycles](#icptstocycles)
    - [SetAuthorizedSubnetworkListArgs](#setauthorizedsubnetworklistargs)
    - [State](#state)


## struct

### CyclesCanisterInitPayload
```rs
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct CyclesCanisterInitPayload {
    pub ledger_canister_id: CanisterId,
    pub governance_canister_id: CanisterId,
    pub minting_account_id: Option<AccountIdentifier>,
}
```

### CreateCanisterResult
```rs
/// The result of create_canister transaction notification. In case of
/// an error, contains the index of the refund block.
pub type CreateCanisterResult = Result<CanisterId, (String, Option<BlockHeight>)>;
```

### TopUpCanisterResult
```rs
/// The result of top_up_canister transaction notification. In case of
/// an error, contains the index of the refund block.
pub type TopUpCanisterResult = Result<(), (String, Option<BlockHeight>)>;
```

### IcptsToCycles
```rs
pub struct IcptsToCycles {
    /// Number of 1/10,000ths of XDR that 1 ICP is worth.
    pub xdr_permyriad_per_icp: u64,
    /// Number of cycles that 1 XDR is worth.
    pub cycles_per_xdr: Cycles,
}
```

```rs
impl IcptsToCycles {
    pub fn to_cycles(&self, icpts: ICPTs) -> Cycles {
        Cycles::new(
            icpts.get_e8s() as u128
                * self.xdr_permyriad_per_icp as u128
                * self.cycles_per_xdr.get() as u128
                / (ledger_canister::ICP_SUBDIVIDABLE_BY as u128 * 10_000),
        )
    }
}
```

### SetAuthorizedSubnetworkListArgs
```rs
/// Argument taken by the set_authorized_subnetwork_list endpoint
#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq)]
pub struct SetAuthorizedSubnetworkListArgs {
    pub who: Option<PrincipalId>,
    pub subnets: Vec<SubnetId>,
}
```

### State
```rs
#[derive(Serialize, Deserialize, Clone, CandidType, Eq, PartialEq, Debug)]
struct State {
    ledger_canister_id: CanisterId,

    governance_canister_id: CanisterId,

    /// Account used to burn funds.
    minting_account_id: Option<AccountIdentifier>,

    authorized_subnets: BTreeMap<PrincipalId, Vec<SubnetId>>,

    default_subnets: Vec<SubnetId>,

    /// How many cycles 1 XDR is worth.
    cycles_per_xdr: Cycles,

    /// How many cycles are allowed to be minted in an hour.
    cycles_limit: Cycles,

    /// Maintain a count of how many cycles have been minted in the last hour.
    limiter: limiter::Limiter,

    total_cycles_minted: Cycles,
}
```