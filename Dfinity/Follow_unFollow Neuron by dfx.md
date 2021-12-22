# Follow Neuron by dfx

## Contents
- [Follow Neuron by dfx](#follow-neuron-by-dfx)
  - [Contents](#contents)
  - [在线版](#在线版)
    - [简单版](#简单版)
    - [讲解版](#讲解版)
  - [离线版](#离线版)
    - [生成签名](#生成签名)
    - [发送交易](#发送交易)
- [参考](#参考)

在线版就是操作的账号可以联网，直接 dfx 签发交易发送到脸上
离线版就是操作的账号不能联网，先在离线设备签署交易，然后将签名包转移到在线设备，发送到链上，有效期为 5 min。

打开 Mac/Linux 终端，进入一个工作目录，执行：
```
echo {} > dfx.json
```

## 在线版
### 简单版
```sh
# 获取当前 dfx 控制的所有神经元的 id
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_neuron_ids

(vec { 2_524_431_329_219_902_182 : nat64 })

# 接下来设置神经元 2_524_431_329_219_902_182 去 follow 哪些神经元

# 示例，设置神经元去 follow 5764626780264251033，27，28 这三个神经元。其中 27 是 Dfinity Foundation 的，28 是 ICA 的，5764626780264251033 是我个人的神经元。topic = 0 表示所有的主题下的投票都 follow 这三个。可以按照格式自行修改，增删。返回结果没有具体信息，表示成功执行。
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai manage_neuron "(record {id=opt record{id=2_524_431_329_219_902_182:nat64};command=opt variant{Follow=record {topic=0:int32;followees=vec {record{id=5764626780264251033:nat64};record{id=27:nat64};record{id=28:nat64}}}}})"

(record { 2_171_433_291 = opt variant { 774_571_409 = record {} } })
```

### 讲解版

操作之前，获取该神经元的信息。其中每个字段的具体信息，参考 [How to use dfx to interact with NNS.md](./How%20to%20use%20dfx%20to%20interact%20with%20NNS%20canisters%20instead%20of%20nns%20app.md)
```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_full_neuron "(2_524_431_329_219_902_182 : nat64)"

(
  variant {
    17_724 = record {
      23_515 = opt record { 23_515 = 2_524_431_329_219_902_182 : nat64 };
      79_599_772 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      349_671_467 = vec {}; # 为了不影响阅读，把一大块投票历史记录删除
      456_924_626 = true;
      852_549_734 = false;
      1_029_637_143 = 9_557_125 : nat64;
      1_257_408_332 = 200_000_000 : nat64;
      1_392_680_831 = 1_626_759_699 : nat64;
      2_399_567_118 = 1_630_336_288 : nat64;
      2_680_861_478 = vec {};
      2_707_029_165 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^";
      2_878_748_008 = null;
      3_084_775_299 = opt variant { 1_620_537_965 = 18_144_000 : nat64 };
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
    }
  },
)
```

上面最重要的是这个字段，表示该神经元正在所有主题下只 follow 了一个神经元，那就是 id 为 28 的神经元。
```sh
3_407_357_762 = vec {
    record {
        0 : int32;
        record { 3_407_357_762 = vec { record { 23_515 = 28 : nat64 } } };
    };
};
```

接着修改 follow 的神经元，注意是覆盖式修改，不是新增。设置神经元去 follow 5764626780264251033，27，28 这三个神经元。其中 27 是 Dfinity Foundation 的，28 是 ICA 的，5764626780264251033 是我个人的神经元。topic = 0 表示所有的主题下的投票都 follow 这三个。可以按照格式自行修改，增删。返回结果没有具体信息，表示成功执行。
```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai manage_neuron "(record {id=opt record{id=2_524_431_329_219_902_182:nat64};command=opt variant{Follow=record {topic=0:int32;followees=vec {record{id=5764626780264251033:nat64};record{id=27:nat64};record{id=28:nat64}}}}})"

(record { 2_171_433_291 = opt variant { 774_571_409 = record {} } })
```

查看神经元信息，已经成功修改。
```sh
dfx canister --network=https://ic0.app --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai get_full_neuron "(2_524_431_329_219_902_182 : nat64)"
5764626780264251033
(
  variant {
    17_724 = record {
      23_515 = opt record { 23_515 = 2_524_431_329_219_902_182 : nat64 };
      79_599_772 = opt principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae";
      349_671_467 = vec { }; # 为了不影响阅读，把一大块投票历史记录删除了
      456_924_626 = true;
      852_549_734 = false;
      1_029_637_143 = 9_557_125 : nat64;
      1_257_408_332 = 200_000_000 : nat64;
      1_392_680_831 = 1_626_759_699 : nat64;
      2_399_567_118 = 1_630_336_288 : nat64;
      2_680_861_478 = vec {};
      2_707_029_165 = blob "\97\99\7f\e1\f1\94\c2\d0k\60\c4\b4\bd\ea\d2rAQ\c0.\a5\8e:\89v}/\b5\5c\b5\a5^";
      2_878_748_008 = null;
      3_084_775_299 = opt variant { 1_620_537_965 = 18_144_000 : nat64 };
      3_407_357_762 = vec {
        record {
          0 : int32;
          record {
            3_407_357_762 = vec {
              record { 23_515 = 5_764_626_780_264_251_033 : nat64 };
              record { 23_515 = 27 : nat64 };
              record { 23_515 = 28 : nat64 };
            };
          };
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
    }
  },
)
```

## 离线版

### 生成签名
在离线设备中：
```sh
# 接下来设置神经元 2_524_431_329_219_902_182 去 follow 哪些神经元

# 示例，设置神经元去 follow 5764626780264251033，28 这两个神经元。其中 28 是 ICA 的，5764626780264251033 是我个人的神经元。topic = 0 表示所有的主题下的投票都 follow 这三个。可以按照格式自行修改，增删。返回结果没有具体信息，表示成功执行。
dfx canister --network ic --no-wallet sign rrkah-fqaaa-aaaaa-aaaaq-cai manage_neuron "(record {id=opt record{id=2_524_431_329_219_902_182:nat64};command=opt variant{Follow=record {topic=0:int32;followees=vec {record{id=5764626780264251033:nat64};record{id=28:nat64}}}}})"

Update message generated at [message.json]
Signed request_status append to update message in [message.json]

```
会在终端的工作目录生成一个 message.json 文件，这是已经签名好的消息。

### 发送交易
将 message.json 转移到联网设备，执行命令，中途提示 Okay？输入 y 回车

```sh
dfx canister --network ic --no-wallet send ./message.json

Will send message:
  Creation:    2021-12-22 11:12:37 UTC
  Expiration:  2021-12-22 11:17:37 UTC
  Network:     https://ic0.app
  Call type:   update
  Sender:      yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae
  Canister id: rrkah-fqaaa-aaaaa-aaaaq-cai
  Method name: manage_neuron
  Arg:         [68, 73, 68, 76, 7, 108, 2, 219, 183, 1, 1, 203, 226, 181, 139, 8, 3, 110, 2, 108, 1, 219, 183, 1, 120, 110, 4, 107, 1, 145, 139, 172, 241, 2, 5, 108, 2, 175, 163, 189, 161, 1, 117, 194, 206, 224, 216, 12, 6, 109, 2, 1, 0, 1, 230, 250, 4, 62, 235, 148, 8, 35, 1, 0, 0, 0, 0, 0, 2, 153, 190, 90, 172, 131, 17, 0, 80, 28, 0, 0, 0, 0, 0, 0, 0]

Okay? [y/N]
y
To check the status of this update call, append `--status` to current command.
e.g. `dfx canister send message.json --status`
Alternatively, if you have the correct identity on this machine, using `dfx canister request-status` with following arguments.
Request ID: 0x0a4bda177a52af197cd8bf9fc30b806d66a0a624f45af638293a8a93bcd1db1b
Canister ID: rrkah-fqaaa-aaaaa-aaaaq-cai
```

最后在联网设备查询该神经的信息，知道修改成功：
```sh
3_407_357_762 = vec {
    record {
        0 : int32;
        record {
            3_407_357_762 = vec {
                record { 23_515 = 5_764_626_780_264_251_033 : nat64 };
                record { 23_515 = 28 : nat64 };
            };
        };
    };
};
```


# 参考
* https://ic.associates/nns-command-line-guide/#get-full-neuron