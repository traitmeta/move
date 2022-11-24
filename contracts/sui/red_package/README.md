# content

## detail

1. 创建红包（默认过期时间24小时）【要不要过期时间】
2. 撤销红包
3. 撤销全部红包 【是否只能撤销已经过期的】


sui client publish --path ./red_package --gas-budget 30000


## 创建红包

```shell
curl --location --request POST https://fullnode.devnet.sui.io:443 \
--header 'Content-Type: application/json' \
--data-raw '{ 
  "jsonrpc": "2.0",
  "method": "sui_moveCall",
  "params": [
    "0xd2a75c7f37f4d540c3b0313e8e8c97ad21b84f3a",
    "0xba8d7546e8b68f1ad56190d14eada311d320a9be",
    "package",
    "create_fair",
    ["0x2::sui::SUI"],
    ["0x3aab6a4a738b1efb019912556f2e153ed127c4f3", 1000000, 2],
    "0x13c9396fb50fc06141f7ce0c9ddd5ecd8fe41102",
    2000
  ],
  "id": 1 
}' | json_pp


{
   "id" : 1,
   "jsonrpc" : "2.0",
   "result" : {
      "gas" : {
         "digest" : "grI+96FP5GqgvVHh3KhNcN940b8A5DJ9un1BWIeOI3E=",
         "objectId" : "0x13c9396fb50fc06141f7ce0c9ddd5ecd8fe41102",
         "version" : 4
      },
      "inputObjects" : [
         {
            "ImmOrOwnedMoveObject" : {
               "digest" : "GH6dql0X9LVjqG0x3MQJdH0g9O5LYOd9vccM/t+/89E=",
               "objectId" : "0x3aab6a4a738b1efb019912556f2e153ed127c4f3",
               "version" : 1
            }
         },
         {
            "MovePackage" : "0xba8d7546e8b68f1ad56190d14eada311d320a9be"
         },
         {
            "ImmOrOwnedMoveObject" : {
               "digest" : "grI+96FP5GqgvVHh3KhNcN940b8A5DJ9un1BWIeOI3E=",
               "objectId" : "0x13c9396fb50fc06141f7ce0c9ddd5ecd8fe41102",
               "version" : 4
            }
         }
      ],
      "txBytes" : "VHJhbnNhY3Rpb25EYXRhOjoAArqNdUboto8a1WGQ0U6toxHTIKm+AQAAAAAAAAAgR0YwTQTNblGPl+KlbpTAG/ll1KmQY/YwOxhnjxmF5mwHcGFja2FnZQtjcmVhdGVfZmFpcgEHAAAAAAAAAAAAAAAAAAAAAAAAAAIDc3VpA1NVSQADAQA6q2pKc4se+wGZElVvLhU+0SfE8wEAAAAAAAAAIBh+napdF/S1Y6htMdzECXR9IPTuS2Dnfb3HDP7fv/PRAAhAQg8AAAAAAAAIAgAAAAAAAADSp1x/N/TVQMOwMT6OjJetIbhPOhPJOW+1D8BhQffODJ3dXs2P5BECBAAAAAAAAAAggrI+96FP5GqgvVHh3KhNcN940b8A5DJ9un1BWIeOI3EBAAAAAAAAANAHAAAAAAAA"
   }
}

sui keytool sign --address 0xd2a75c7f37f4d540c3b0313e8e8c97ad21b84f3a --data "VHJhbnNhY3Rpb25EYXRhOjoAArqNdUboto8a1WGQ0U6toxHTIKm+AQAAAAAAAAAgR0YwTQTNblGPl+KlbpTAG/ll1KmQY/YwOxhnjxmF5mwHcGFja2FnZQtjcmVhdGVfZmFpcgEHAAAAAAAAAAAAAAAAAAAAAAAAAAIDc3VpA1NVSQADAQA6q2pKc4se+wGZElVvLhU+0SfE8wEAAAAAAAAAIBh+napdF/S1Y6htMdzECXR9IPTuS2Dnfb3HDP7fv/PRAAhAQg8AAAAAAAAIAgAAAAAAAADSp1x/N/TVQMOwMT6OjJetIbhPOhPJOW+1D8BhQffODJ3dXs2P5BECBAAAAAAAAAAggrI+96FP5GqgvVHh3KhNcN940b8A5DJ9un1BWIeOI3EBAAAAAAAAANAHAAAAAAAA"

2022-11-24T07:44:48.762814Z  INFO sui::keytool: Data to sign : VHJhbnNhY3Rpb25EYXRhOjoAArqNdUboto8a1WGQ0U6toxHTIKm+AQAAAAAAAAAgR0YwTQTNblGPl+KlbpTAG/ll1KmQY/YwOxhnjxmF5mwHcGFja2FnZQtjcmVhdGVfZmFpcgEHAAAAAAAAAAAAAAAAAAAAAAAAAAIDc3VpA1NVSQADAQA6q2pKc4se+wGZElVvLhU+0SfE8wEAAAAAAAAAIBh+napdF/S1Y6htMdzECXR9IPTuS2Dnfb3HDP7fv/PRAAhAQg8AAAAAAAAIAgAAAAAAAADSp1x/N/TVQMOwMT6OjJetIbhPOhPJOW+1D8BhQffODJ3dXs2P5BECBAAAAAAAAAAggrI+96FP5GqgvVHh3KhNcN940b8A5DJ9un1BWIeOI3EBAAAAAAAAANAHAAAAAAAA
2022-11-24T07:44:48.763514Z  INFO sui::keytool: Address : 0xd2a75c7f37f4d540c3b0313e8e8c97ad21b84f3a
2022-11-24T07:44:48.764787Z  INFO sui::keytool: Flag Base64: AA==
2022-11-24T07:44:48.764794Z  INFO sui::keytool: Public Key Base64: SXm1Hakt49ZP48/6UOjrqYkBq0HF9JYwurW4HSyA+oc=
2022-11-24T07:44:48.764796Z  INFO sui::keytool: Signature : X8MSkxtRR4LTguF96I2RBWCLoC2j0p3ya24rlSIovH51uhNudRpK2GGN5QEfgteKswAn6uJS0hfIfZOl4flWDQ==


curl --location --request POST https://fullnode.devnet.sui.io:443 \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "sui_executeTransaction",
  "params": [ 
    "VHJhbnNhY3Rpb25EYXRhOjoAArqNdUboto8a1WGQ0U6toxHTIKm+AQAAAAAAAAAgR0YwTQTNblGPl+KlbpTAG/ll1KmQY/YwOxhnjxmF5mwHcGFja2FnZQtjcmVhdGVfZmFpcgEHAAAAAAAAAAAAAAAAAAAAAAAAAAIDc3VpA1NVSQADAQA6q2pKc4se+wGZElVvLhU+0SfE8wEAAAAAAAAAIBh+napdF/S1Y6htMdzECXR9IPTuS2Dnfb3HDP7fv/PRAAhAQg8AAAAAAAAIAgAAAAAAAADSp1x/N/TVQMOwMT6OjJetIbhPOhPJOW+1D8BhQffODJ3dXs2P5BECBAAAAAAAAAAggrI+96FP5GqgvVHh3KhNcN940b8A5DJ9un1BWIeOI3EBAAAAAAAAANAHAAAAAAAA",
    "ED25519",
    "X8MSkxtRR4LTguF96I2RBWCLoC2j0p3ya24rlSIovH51uhNudRpK2GGN5QEfgteKswAn6uJS0hfIfZOl4flWDQ==",
    "SXm1Hakt49ZP48/6UOjrqYkBq0HF9JYwurW4HSyA+oc=",
    "WaitForLocalExecution"
  ]
}' | json_pp
```
