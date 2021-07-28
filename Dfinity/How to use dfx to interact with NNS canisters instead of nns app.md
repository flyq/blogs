# How to use dfx to interact with NNS canisters instead of nns app

# Contents
- [How to use dfx to interact with NNS canisters instead of nns app](#how-to-use-dfx-to-interact-with-nns-canisters-instead-of-nns-app)
- [Contents](#contents)
- [Background](#background)
- [How](#how)
  - [dfx](#dfx)
  - [创建神经元](#创建神经元)
  - [查询神经元信息](#查询神经元信息)
  - [操作神经元](#操作神经元)
    - [设置溶解延迟](#设置溶解延迟)
    - [神经元投票](#神经元投票)

# Background
现在所有和 ICP 以及神经元治理相关的操作都需要使用 [NNS app](https://nns.ic0.app/)。

在大多数情况下，NNS app 是能够满足需求。但是在以下情况下无法满足需求：
1. 当 [Identity](http://identity.ic0.app/) 组件无法使用时，因为 NNS app 依赖于 Identity 组件的认证，因此 NNS app 也将无法使用。
   1. 比如对应种子轮用户，他们被锁定的神经元被 2017 年生成的助记词控制，并且无法将控制权转移给 Identity 里面的账号。此时，他们要想操作神经元（比如投票，解锁等），则无法使用 NNS app。
   2. 用户管理的金额非常大，希望使用基于私钥文件的离线两步验证，这时就不能使用 NNS app 了。
   3. WebAuth 还是一个比较新的标准，浏览器支持并不完善。并且仍有很多终端设备（手机，电脑等）不支持安全芯片。
2. 当需要批量操作神经元时，比如需要操作 100 个神经元去投票，如果基于 NNS app，则需要点击几百次，并且等等投票结果等等，比较麻烦

# How
## dfx
使用 dfx + 足够的链上信息，原则上是可以完成和 IC 上所有的 canister 进行交互的所有操作。

现在假设 dfx 已经配置好一个账号，icp，里面有足量的 icp（至少 1.1 个 ICP，其中 1 个 ICP 用于质押在神经元里面，0.1 个 ICP 用于手续费）。

## 创建神经元

tools:
[subaccount](https://github.com/flyq/blogs/blob/master/Dfinity/subaccount/src/main.rs)

```sh
# 获取该账号在主网上的余额
dfx ledger --network=https://ic0.app balance
1.47200000 ICP

# 获取该账号的 principal id
dfx identity get-principal
yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae

# 获取该账号的 account id
dfx ledger account-id
073ca335431d6b6f6916068b5784a241730d2e3452ae650025b4bf7a975a81f0

# 设置 governance canister 地址
governance="principal \"rrkah-fqaaa-aaaaa-aaaaq-cai\""

# 调用 Ledger canister（ryjl3-tyaaa-aaaaa-aaaba-cai），发送 1 个 ICP 到 governance 地址下的某个子账号（to）。
# 其中 to 使用 [subaccount](subaccount/src/main.rs) 生成。等于 governance_canister_id + sub
# 可以看出，其中 sub 通过 caller（发送地址） 和 memo（这里是0），然后经过一系列变化得到。
# 交易发送完成后，得到这笔交易的块高 291747
# 交易地址在 https://dashboard.internetcomputer.org/transaction/d7eec9a3105857a94754838cb2dca9568836b3f441a89b498c5b729a02a7a9f3
dfx canister --network=https://ic0.app --no-wallet call ryjl3-tyaaa-aaaaa-aaaba-cai send_dfx "(record {memo=0:nat64;amount=record {e8s=100000000:nat64};fee=record {e8s=10000:nat64};from_subaccount=null;to=\"8ca2e53dd8b9f1924daf10cdf5879f6e4a5c3267ded5a7efb5e740b3e2f85ae3\";created_at_time=null})"
(291_747 : nat64)

# 接着调用 Ledger canister（ryjl3-tyaaa-aaaaa-aaaba-cai），给 governance canister 发送一个已经成功转账的通知
# 参数 block_height 为之前那笔交易的高度。to_canister 是 governance，
# to_subaccount 使用 [subaccount](subaccount/src/main.rs) 生成。和前一笔交易类似，通过 caller（发送地址） 和 memo（这里是0），然后经过一系列变化得到。
# 最后返回神经元 id (ic_nns_common::pb::v1::NeuronId)
dfx canister --network=https://ic0.app --no-wallet call ryjl3-tyaaa-aaaaa-aaaba-cai notify_dfx "(record {block_height=291747:nat64;max_fee=record{e8s=10000:nat64};from_subaccount=null;to_canister=$governance;to_subaccount=vec {151:nat8;153:nat8;127:nat8;225:nat8;241:nat8;148:nat8;194:nat8;208:nat8;107:nat8;96:nat8;196:nat8;180:nat8;189:nat8;234:nat8;210:nat8;114:nat8;65:nat8;81:nat8;192:nat8;46:nat8;165:nat8;142:nat8;58:nat8;137:nat8;118:nat8;125:nat8;47:nat8;181:nat8;92:nat8;181:nat8;165:nat8;94:nat8}})"
(record { 23_515 = 2_524_431_329_219_902_182 : nat64 })

# 查询余额，少了 1 ICP 和部分手续费
dfx ledger --network=https://ic0.app balance
0.47180000 ICP
```

## 查询神经元信息

查询并对比 governance 的 candid 文件，添加注释

```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_full_neuron "(2_524_431_329_219_902_182 : nat64)"
(
  variant { # Result_1
    17_724 = record { # Neuron
      23_515 = opt record { 23_515 = 2_524_431_329_219_902_182 : nat64 }; # id : opt NeuronId;
      79_599_772 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae"; # controller : opt principal;
      349_671_467 = vec {}; # recent_ballots : vec BallotInfo;
      456_924_626 = true; # kyc_verified : bool;
      852_549_734 = false; # not_for_profit : bool;
      1_029_637_143 = 0 : nat64; # maturity_e8s_equivalent : nat64;
      1_257_408_332 = 100_000_000 : nat64; # cached_neuron_stake_e8s : nat64;
      1_392_680_831 = 1_626_759_699 : nat64; # created_timestamp_seconds : nat64;
      2_399_567_118 = 1_626_759_699 : nat64; # aging_since_timestamp_seconds : nat64;
      2_680_861_478 = vec {}; # hot_keys : vec principal;
      2_707_029_165 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^"; # account : vec nat8;
      3_084_775_299 = opt variant { 1_620_537_965 = 0 : nat64 }; # dissolve_state : opt DissolveState;
      3_407_357_762 = vec { # followees : vec record { int32; Followees };
        record {
          0 : int32;
          record { 3_407_357_762 = vec { record { 23_515 = 28 : nat64 } } };
        };
      };
      3_439_871_066 = 0 : nat64; # neuron_fees_e8s : nat64;
      3_664_621_355 = opt record { # transfer : opt NeuronStakeTransfer;
        1_077_262_001 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^"; #   to_subaccount : vec nat8;
        1_103_886_095 = 100_000_000 : nat64; # neuron_stake_e8s : nat64;
        1_136_829_802 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae"; # from : opt principal;
        1_213_809_850 = 0 : nat64; # memo : nat64;
        1_835_347_746 = vec {}; # from_subaccount : vec nat8;
        3_066_807_170 = 1_626_759_699 : nat64; # transfer_timestamp : nat64;
        3_583_743_961 = 291_747 : nat64; # block_height : nat64;
      };
    }
  },
)

# 返回该账号控制的神经元id，这里就刚刚创建的那个
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_neuron_ids
(vec { 2_524_431_329_219_902_182 : nat64 })

# 获取该神经元的基本信息
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_neuron_info "(2_524_431_329_219_902_182 : nat64)"
(
  variant { # Result_2
    17_724 = record { # NeuronInfo
      303_619_573 = 0 : nat64; # dissolve_delay_seconds : nat64;
      349_671_467 = vec {}; # recent_ballots : vec BallotInfo;
      1_392_680_831 = 1_626_759_699 : nat64; # created_timestamp_seconds : nat64;
      2_215_343_633 = 3 : int32; # state : int32;
      3_433_024_449 = 1_626_762_752 : nat64; # retrieved_at_timestamp_seconds : nat64;
      3_871_395_629 = 100_000_604 : nat64; # voting_power : nat64;
      4_290_862_015 = 3_053 : nat64; # age_seconds : nat64;
    }
  },
)

# 获取当前 Pending 的提案
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_pending_proposals
(
  vec {
    record { # ProposalInfo
      23_515 = opt record { 23_515 = 11_078 : nat64 }; # id : opt NeuronId;
      100_394_802 = 1 : int32; # status : int32;
      338_645_423 = 7 : int32; # topic : int32;
      568_110_681 = null; # ballots : vec record { nat64; Ballot };
      718_522_127 = vec {}; 
      1_120_297_033 = 1_626_700_972 : nat64; # proposal_timestamp_seconds : nat64;
      1_138_543_385 = 0 : nat64; # reward_event_round : nat64;
      1_453_869_204 = 0 : nat64; # failed_timestamp_seconds : nat64;
      1_659_864_270 = 100_000_000 : nat64; # reject_cost_e8s : nat64;
      2_084_260_468 = opt record { # latest_tally : opt Tally;
        24_641 = 16_425_617_043 : nat64; # no : nat64;
        6_039_847 = 2_806_966_682_423 : nat64; # yes : nat64;
        338_842_564 = 31_775_041_633_396_566 : nat64; # total : nat64;
        4_174_818_006 = 1_626_760_638 : nat64; # timestamp_seconds : nat64;
      };
      2_139_208_002 = 1 : int32; # reward_status : int32;
      2_756_235_859 = 0 : nat64; # decided_timestamp_seconds : nat64;
      3_000_310_834 = opt record { # proposal : opt Proposal;
        5_843_823 = "https://github.com/ic-association/nns-proposals/blob/main/proposals/subnet_management/20210719T1320Z.md"; # url : text
        373_701_558 = opt variant { # action : opt Action
          2_746_863_190 = record { # ExecuteNnsFunction : ExecuteNnsFunction;
            1_563_405_284 = 6 : int32; # nns_function : int32;
            3_979_722_638 = blob "DIDL\06l\06\e7\8f\b0\12x\c4\c1\81D\01\bd\86\9d\8b\04h\8c\c9\9c\ae\06\03\fc\ef\cc\a6\0d\05\f7\e0\ab\c7\0dxn\02mhn\04l\03\00q\01q\02xm{\01\00\f8&T\00\00\00\00\00\01\00\01\1dE\17\8bf\a4\07\a4\d5\9c\0aMg3\bd\a2\ce?7\c1\11\b4%~\c4\19\08\f4\f8\02\00 \1d\dc\eb\c4<_\00\efp\ee\9d5Zc}\f5ZA\e8\9f\8d\bda\ab d\10K\dc\c0\c1 \b0J\8a\a5\e83\93\16"; # payload : vec nat8
          }
        };
        2_162_756_390 = "recover subnet 5kdm2-62fc6-fwnja-hutkz-ycsnm-4z33i-woh43-4cenu-ev7mi-gii6t-4ae"; # summary : text
      };
      3_000_311_732 = opt record { 23_515 = 48 : nat64 }; # proposer : opt NeuronId;
      4_133_454_822 = 0 : nat64; # executed_timestamp_seconds : nat64;
    };
  },
)
```

提案 Action 类型，即有哪几种提案：
```did
type Action = variant {
  ManageNeuron : ManageNeuron;
  ExecuteNnsFunction : ExecuteNnsFunction;
  RewardNodeProvider : RewardNodeProvider;
  SetDefaultFollowees : SetDefaultFollowees;
  ManageNetworkEconomics : NetworkEconomics;
  ApproveGenesisKyc : ApproveGenesisKyc;
  AddOrRemoveNodeProvider : AddOrRemoveNodeProvider;
  Motion : Motion;
};
```

获取某个指定的神经元信息
```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_proposal_info "(11_077:nat64)"
(
  opt record { # ProposalInfo
    23_515 = opt record { 23_515 = 11_077 : nat64 }; # id : opt NeuronId;
    100_394_802 = 4 : int32;  # status : int32;
    338_645_423 = 2 : int32;  # topic : int32;
    568_110_681 = null;  # ballots : vec record { nat64; Ballot };
    718_522_127 = vec {};
    1_120_297_033 = 1_626_700_814 : nat64;  # proposal_timestamp_seconds : nat64;
    1_138_543_385 = 0 : nat64; # reward_event_round : nat64;
    1_453_869_204 = 0 : nat64; # failed_timestamp_seconds : nat64;
    1_659_864_270 = 100_000_000 : nat64; # reject_cost_e8s : nat64;
    2_084_260_468 = opt record { # latest_tally : opt Tally;
      24_641 = 0 : nat64; # no : nat64;
      6_039_847 = 31_765_476_162_104_117 : nat64;  # yes : nat64;
      338_842_564 = 31_775_041_889_086_177 : nat64;  # total : nat64;
      4_174_818_006 = 1_626_700_819 : nat64; # timestamp_seconds : nat64;
    };
    2_139_208_002 = 2 : int32; # reward_status : int32;
    2_756_235_859 = 1_626_700_819 : nat64;  # decided_timestamp_seconds : nat64;
    3_000_310_834 = opt record { # proposal : opt Proposal;
      5_843_823 = ""; # url : text
      373_701_558 = opt variant {  # action : opt Action
        2_746_863_190 = record { # # ExecuteNnsFunction : ExecuteNnsFunction;
          1_563_405_284 = 10 : int32;  # nns_function : int32;
          3_979_722_638 = blob "DIDL\01l\03\90\cb\8b\aa\01q\df\f5\81\a0\08x\d6\d5\da\c6\0fx\01\00 {\22icp\22:[\22Huobi\22],\22sdr\22:\22xe.com\22}\ccC\03\00\00\00\00\00\00|\f5\60\00\00\00\00"; # payload : vec nat8
        }
      };
      2_162_756_390 = "The ICP/XDR conversion rate is set to 21.3964."; # summary : text
    };
    3_000_311_732 = opt record { 23_515 = 25 : nat64 }; # proposer : opt NeuronId;
    4_133_454_822 = 1_626_700_819 : nat64; # executed_timestamp_seconds : nat64;
  },
)
```


```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai list_neurons "(record{neuron_ids=vec {2_524_431_329_219_902_182:nat64;};include_neurons_readable_by_caller=true;})"
(
  record { # type ListNeuronsResponse = record {
    1_319_168_057 = vec { #   neuron_infos : vec record { nat64; NeuronInfo };
      record { # 
        2_524_431_329_219_902_182 : nat64; # nat64
        record {  # NeuronInfo
          303_619_573 = 0 : nat64;  #   dissolve_delay_seconds : nat64;
          349_671_467 = vec {}; #   recent_ballots : vec BallotInfo;
          1_392_680_831 = 1_626_759_699 : nat64; #   created_timestamp_seconds : nat64;
          2_215_343_633 = 3 : int32; #   state : int32;
          3_433_024_449 = 1_626_769_938 : nat64; #   retrieved_at_timestamp_seconds : nat64;
          3_871_395_629 = 100_002_027 : nat64; #   voting_power : nat64;
          4_290_862_015 = 10_239 : nat64; #   age_seconds : nat64;
        };
      };
    };
    1_488_793_264 = vec { #   full_neurons : vec Neuron;
      record {  # type Neuron = record {
        23_515 = opt record { 23_515 = 2_524_431_329_219_902_182 : nat64 }; #   id : opt NeuronId;
        79_599_772 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae"; #   controller : opt principal;
        349_671_467 = vec {}; #   recent_ballots : vec BallotInfo;
        456_924_626 = true; #   kyc_verified : bool;
        852_549_734 = false;  #   not_for_profit : bool;
        1_029_637_143 = 0 : nat64; #   maturity_e8s_equivalent : nat64;
        1_257_408_332 = 100_000_000 : nat64; #   cached_neuron_stake_e8s : nat64;
        1_392_680_831 = 1_626_759_699 : nat64; #   created_timestamp_seconds : nat64;
        2_399_567_118 = 1_626_759_699 : nat64; #   aging_since_timestamp_seconds : nat64;
        2_680_861_478 = vec {}; #   hot_keys : vec principal;
        2_707_029_165 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^"; #   account : vec nat8;
        3_084_775_299 = opt variant { 1_620_537_965 = 0 : nat64 }; #   dissolve_state : opt DissolveState;
        3_407_357_762 = vec { #   followees : vec record { int32; Followees };
          record {
            0 : int32;
            record { 3_407_357_762 = vec { record { 23_515 = 28 : nat64 } } }; # type Followees = record { followees : vec NeuronId };
          };
        };
        3_439_871_066 = 0 : nat64; #   neuron_fees_e8s : nat64;
        3_664_621_355 = opt record { #   transfer : opt NeuronStakeTransfer;
          1_077_262_001 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^";
          1_103_886_095 = 100_000_000 : nat64;
          1_136_829_802 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
          1_213_809_850 = 0 : nat64;
          1_835_347_746 = vec {};
          3_066_807_170 = 1_626_759_699 : nat64;
          3_583_743_961 = 291_747 : nat64;
        };
      };
      record {
        23_515 = opt record { 23_515 = 2_524_431_329_219_902_182 : nat64 };
        79_599_772 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
        349_671_467 = vec {};
        456_924_626 = true;
        852_549_734 = false;
        1_029_637_143 = 0 : nat64;
        1_257_408_332 = 100_000_000 : nat64;
        1_392_680_831 = 1_626_759_699 : nat64;
        2_399_567_118 = 1_626_759_699 : nat64;
        2_680_861_478 = vec {};
        2_707_029_165 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^";
        3_084_775_299 = opt variant { 1_620_537_965 = 0 : nat64 };
        3_407_357_762 = vec {
          record {
            0 : int32;
            record { 3_407_357_762 = vec { record { 23_515 = 28 : nat64 } } };
          };
        };
        3_439_871_066 = 0 : nat64;
        3_664_621_355 = opt record {
          1_077_262_001 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^";
          1_103_886_095 = 100_000_000 : nat64;
          1_136_829_802 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
          1_213_809_850 = 0 : nat64;
          1_835_347_746 = vec {};
          3_066_807_170 = 1_626_759_699 : nat64;
          3_583_743_961 = 291_747 : nat64;
        };
      };
    };
  },
)

dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai list_neurons "(record{neuron_ids=vec {2_524_431_329_219_902_182:nat64;};include_neurons_readable_by_caller=false;})"
(
  record {
    1_319_168_057 = vec {
      record {
        2_524_431_329_219_902_182 : nat64;
        record {
          303_619_573 = 0 : nat64;
          349_671_467 = vec {};
          1_392_680_831 = 1_626_759_699 : nat64;
          2_215_343_633 = 3 : int32;
          3_433_024_449 = 1_626_770_001 : nat64;
          3_871_395_629 = 100_002_040 : nat64;
          4_290_862_015 = 10_302 : nat64;
        };
      };
    };
    1_488_793_264 = vec {
      record {
        23_515 = opt record { 23_515 = 2_524_431_329_219_902_182 : nat64 };
        79_599_772 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
        349_671_467 = vec {};
        456_924_626 = true;
        852_549_734 = false;
        1_029_637_143 = 0 : nat64;
        1_257_408_332 = 100_000_000 : nat64;
        1_392_680_831 = 1_626_759_699 : nat64;
        2_399_567_118 = 1_626_759_699 : nat64;
        2_680_861_478 = vec {};
        2_707_029_165 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^";
        3_084_775_299 = opt variant { 1_620_537_965 = 0 : nat64 };
        3_407_357_762 = vec {
          record {
            0 : int32;
            record { 3_407_357_762 = vec { record { 23_515 = 28 : nat64 } } };
          };
        };
        3_439_871_066 = 0 : nat64;
        3_664_621_355 = opt record {
          1_077_262_001 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^";
          1_103_886_095 = 100_000_000 : nat64;
          1_136_829_802 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
          1_213_809_850 = 0 : nat64;
          1_835_347_746 = vec {};
          3_066_807_170 = 1_626_759_699 : nat64;
          3_583_743_961 = 291_747 : nat64;
        };
      };
    };
  },
)
```

可以看成，好几个接口返回的信息基本差不多。当然，现在账号下面神经元少，就会出现这种情况。

## 操作神经元

针对神经元操作类型：
```did
type Command = variant {
  Spawn : Spawn;
  Split : Split;
  Follow : Follow;
  Configure : Configure;
  RegisterVote : RegisterVote;
  DisburseToNeuron : DisburseToNeuron;
  MakeProposal : Proposal;
  Disburse : Disburse;
};
```
1. Spawn，下蛋，即神经元成熟了可以增发出新的小神经元，里面包含锁定 7 天的 ICP。
2. Split，分叉，可以将一个神经元分叉成两个神经元
3. Follow，跟随其他神经元去投票
4. Configure，配置神经元
5. RegisterVote，主动向某个提案投票
6. DisburseToNeuron，
7. MakeProposal，发起一个提案
8. Disburse，将已经溶解了的神经元里面的 ICP 拿出来。

### 设置溶解延迟
只有当神经元的溶解延迟大于 6 个月时，才能投票。

给前面的创建的神经元设置溶解延迟，7个月：`7*30*24*3600=18144000 秒`

```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai manage_neuron "(record {id=opt record{id=2_524_431_329_219_902_182:nat64};command=opt variant{Configure=record {operation=opt variant {IncreaseDissolveDelay= record{additional_dissolve_delay_seconds=18144000:nat32}}}}})"
(record { 2_171_433_291 = opt variant { 1_647_237_574 = record {} } })
```


### 神经元投票
**没有设置溶解延迟直接投票（失败）**
使用前面生成的神经元 id 投票： 2_524_431_329_219_902_182；

我们找一个还可以投票的提案：11078 提案：

![](./images/proposal11193.png)

```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_proposal_info "(11078:nat64)"
(
  opt record { # ProposalInfo
    23_515 = opt record { 23_515 = 11_078 : nat64 }; # id : opt NeuronId;
    100_394_802 = 1 : int32;  # status : int32;
    338_645_423 = 7 : int32;  # topic : int32;
    568_110_681 = null;  # ballots : vec record { nat64; Ballot };
    718_522_127 = vec {};  
    1_120_297_033 = 1_626_700_972 : nat64;# proposal_timestamp_seconds : nat64;
    1_138_543_385 = 0 : nat64; # reward_event_round : nat64;
    1_453_869_204 = 0 : nat64; # failed_timestamp_seconds : nat64;
    1_659_864_270 = 100_000_000 : nat64; # reject_cost_e8s : nat64;
    2_084_260_468 = opt record { # latest_tally : opt Tally;
      24_641 = 16_425_617_043 : nat64;  # no : nat64;
      6_039_847 = 5_371_532_431_420 : nat64;   # yes : nat64;
      338_842_564 = 31_775_041_633_396_566 : nat64;  # total : nat64;
      4_174_818_006 = 1_626_770_088 : nat64; # timestamp_seconds : nat64;
    };
    2_139_208_002 = 1 : int32; # reward_status : int32; 
    2_756_235_859 = 0 : nat64;  # decided_timestamp_seconds : nat64;
    3_000_310_834 = opt record { # proposal : opt Proposal;
      5_843_823 = "https://github.com/ic-association/nns-proposals/blob/main/proposals/subnet_management/20210719T1320Z.md";  # url : text
      373_701_558 = opt variant { # action : opt Action
        2_746_863_190 = record { # ExecuteNnsFunction : ExecuteNnsFunction;
          1_563_405_284 = 6 : int32; # nns_function : int32;
          3_979_722_638 = blob "DIDL\06l\06\e7\8f\b0\12x\c4\c1\81D\01\bd\86\9d\8b\04h\8c\c9\9c\ae\06\03\fc\ef\cc\a6\0d\05\f7\e0\ab\c7\0dxn\02mhn\04l\03\00q\01q\02xm{\01\00\f8&T\00\00\00\00\00\01\00\01\1dE\17\8bf\a4\07\a4\d5\9c\0aMg3\bd\a2\ce?7\c1\11\b4%~\c4\19\08\f4\f8\02\00 \1d\dc\eb\c4<_\00\efp\ee\9d5Zc}\f5ZA\e8\9f\8d\bda\ab d\10K\dc\c0\c1 \b0J\8a\a5\e83\93\16";; # payload : vec nat8
        }
      };
      2_162_756_390 = "recover subnet 5kdm2-62fc6-fwnja-hutkz-ycsnm-4z33i-woh43-4cenu-ev7mi-gii6t-4ae"; # summary : text
    };
    3_000_311_732 = opt record { 23_515 = 48 : nat64 }; # proposer : opt NeuronId;
    4_133_454_822 = 0 : nat64;  # executed_timestamp_seconds : nat64;
  },
)
```

其中 vote = 1 表示支持，2 表示反对，我们支持
```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai manage_neuron "(record {id=opt record{id=2_524_431_329_219_902_182:nat64};command=opt variant{RegisterVote=record {vote=1:int32;proposal=opt record{id=11078:nat64}}}})"

(
  record {
    2_171_433_291 = opt variant {
      106_380_200 = record {
        1_389_388_560 = "Neuron not authorized to vote on proposal.";
        3_790_638_545 = 3 : int32;
      }
    };
  },
)
···
```

**设置完溶解延迟再投票**
```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai manage_neuron "(record {id=opt record{id=2_524_431_329_219_902_182:nat64};command=opt variant{RegisterVote=record {vote=1:int32;proposal=opt record{id=11078:nat64}}}})"
(
  record {
    2_171_433_291 = opt variant {
      106_380_200 = record {
        1_389_388_560 = "Neuron not authorized to vote on proposal.";
        3_790_638_545 = 3 : int32;
      }
    };
  },
)
```
结果还是失败。应该是创建神经元之后需要等一段时间才能投票：
提了一个问题 https://forum.dfinity.org/t/how-long-does-it-take-to-start-voting-after-the-neuron-is-created/6011


过了几天后，提案 11078 的已经过了投票期了，没有执行成功。

查看神经元：

```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_neuron_info "(2_524_431_329_219_902_182 : nat64)"
(
  variant {
    17_724 = record {
      303_619_573 = 18_144_000 : nat64;
      349_671_467 = vec { record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_428 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_394 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_395 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_396 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_397 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_398 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_400 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_401 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_402 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_403 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_404 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_405 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_406 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_407 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_408 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_410 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_411 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_412 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_413 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_414 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_415 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_290 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_291 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_292 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_289 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_283 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_282 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_280 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_279 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_254 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_256 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_257 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_258 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_259 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_260 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_262 : nat64;};};};
      1_392_680_831 = 1_626_759_699 : nat64;
      2_215_343_633 = 1 : int32;
      3_433_024_449 = 1_626_918_069 : nat64;
      3_871_395_629 = 107_217_302 : nat64;
      4_290_862_015 = 143_415 : nat64;
    }
  },
)
```
发现还是多了很多投票记录的，应该是跟随 神经元28 投的票，即该神经元成功创建，能正常投票。

现在没有 pending 神经元，后续再找一个 pending 神经元主动投票试试


过了几天，找个一个 open 的提案 12445，再投票：
```sh
 dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai manage_neuron "(record {id=opt record{id=2_524_431_329_219_902_182:nat64};command=opt variant{RegisterVote=record {vote=1:int32;proposal=opt record{id=12445:nat64}}}})"

(record { 2_171_433_291 = opt variant { 2_455_066_893 = record {} } })
```
成功了

对比投票前后：
```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_neuron_info "(2_524_431_329_219_902_182 : nat64)"
(
  variant {
    17_724 = record {
      303_619_573 = 18_144_000 : nat64;
      349_671_467 = vec { record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_457 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_448 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_243 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_238 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_240 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_202 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_765 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_528 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_428 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_394 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_395 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_396 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_397 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_398 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_400 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_401 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_402 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_403 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_404 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_405 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_406 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_407 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_408 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_410 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_411 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_412 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_413 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_414 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_415 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_290 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_291 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_292 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_289 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_283 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_282 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_280 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_279 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_254 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_256 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_257 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_258 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_259 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_260 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_262 : nat64;};};};
      1_392_680_831 = 1_626_759_699 : nat64;
      2_215_343_633 = 1 : int32;
      3_433_024_449 = 1_627_443_796 : nat64;
      3_871_395_629 = 107_328_906 : nat64;
      4_290_862_015 = 669_142 : nat64;
    }
  },
)

dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_neuron_info "(2_524_431_329_219_902_182 : nat64)"
(
  variant {
    17_724 = record {
      303_619_573 = 18_144_000 : nat64;
      349_671_467 = vec { record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_445 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_457 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_448 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_243 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_238 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_240 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_202 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_765 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_528 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_428 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_394 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_395 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_396 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_397 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_398 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_400 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_401 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_402 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_403 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_404 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_405 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_406 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_407 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_408 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_410 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_411 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_412 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_413 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_414 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_415 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_290 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_291 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_292 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_289 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_283 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_282 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_280 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_279 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_254 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_256 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_257 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_258 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_259 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_260 : nat64;};}; record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 11_262 : nat64;};};};
      1_392_680_831 = 1_626_759_699 : nat64;
      2_215_343_633 = 1 : int32;
      3_433_024_449 = 1_627_443_850 : nat64;
      3_871_395_629 = 107_328_917 : nat64;
      4_290_862_015 = 669_196 : nat64;
    }
  },
)
```
多了一个 `349_671_467 = vec { record { 1_314_114_794 = 1 : int32; 2_744_746_248 = opt record { 23_515 = 12_445 : nat64;};};` 12_445 的记录。


原链接：[source](https://github.com/flyq/blogs/blob/master/Dfinity/How%20to%20use%20dfx%20to%20interact%20with%20NNS%20canisters%20instead%20of%20nns%20app.md)
