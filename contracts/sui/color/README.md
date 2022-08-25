# Record

## run record

- sui client publish --path ../ --gas-budget 10000
  
   ```
    ----- Certificate ----
    Transaction Hash: Nlv8jrmDBKtw10hkcx8vZSu54xQmJt2/vcBruXqJWGY=
    Transaction Signature: AA==@/2aOTJcJmuMGLyo2lgYS8s/VjFnEc49vqtO+DAQEn+sNX1V0nYKTjeEUoIJavwwlqiUkjUCOFJyngSAv4tf1Bg==@Vq7Nk1OjX3FCIb1gacla6aafCr/ayDaYMoFZEPcuDXQ=
    Signed Authorities Bitmap: RoaringBitmap<[0, 2, 3]>
    Transaction Kind : Publish
    ----- Transaction Effects ----
    Status : Success
    Created Objects:
    - ID: 0x95cd99feeeb3f49a52f2b4267743f551c828d5b2 , Owner: Immutable
    Mutated Objects:
    - ID: 0x0254ce378cffa7302c0ee8ddd1f599676c99098e , Owner: Account Address ( 0x1a6254d89ee1698ed62c03481d28eee88c685b94 )
    ----- Publish Results ----
    The newly published package object ID: 0x95cd99feeeb3f49a52f2b4267743f551c828d5b2

    Updated Gas : Coin { id: 0x0254ce378cffa7302c0ee8ddd1f599676c99098e, value: 49484 }
    ```

- export PACKAGE=0x95cd99feeeb3f49a52f2b4267743f551c828d5b2
- sui client call --gas-budget 1000 --package $PACKAGE --module "rgb" --function "create" --args 0 255 0

```
----- Certificate ----
    Transaction Hash: S5s2xtbdL69Fc7InUfKvidNv/GXn+x/x4ZeZB00qxxk=
    Transaction Signature: AA==@pAUkjxpbvXD7h/YNTVqvVKOlj3eI6kVJ7bDB7QLDKaqeloRs2LD9LP31ZPjC/rEuOajHcM8BPWl2PlmIBAQNAw==@Vq7Nk1OjX3FCIb1gacla6aafCr/ayDaYMoFZEPcuDXQ=
    Signed Authorities Bitmap: RoaringBitmap<[0, 2, 3]>
    Transaction Kind : Call
    Package ID : 0x95cd99feeeb3f49a52f2b4267743f551c828d5b2
    Module : rgb
    Function : create
    Arguments : ["", 255, ""]
    Type Arguments : []
    ----- Transaction Effects ----
    Status : Success
    Created Objects:
    - ID: 0xa61de7bb233df7870bca7ed3459f1261f393ec7f , Owner: Account Address (0x1a6254d89ee1698ed62c03481d28eee88c685b94 )
    Mutated Objects:
    - ID: 0x0254ce378cffa7302c0ee8ddd1f599676c99098e , Owner: Account Address (0x1a6254d89ee1698ed62c03481d28eee88c685b94 )
```

- sui client object --id 0xa61de7bb233df7870bca7ed3459f1261f393ec7f

```
    ----- Move Object (0xa61de7bb233df7870bca7ed3459f1261f393ec7f[1]) -----
    Owner: Account Address ( 0x1a6254d89ee1698ed62c03481d28eee88c685b94 )
    Version: 1
    Storage Rebate: 13
    Previous Transaction: S5s2xtbdL69Fc7InUfKvidNv/GXn+x/x4ZeZB00qxxk=
    ----- Data -----
    type: 0x95cd99feeeb3f49a52f2b4267743f551c828d5b2::rgb::ColorObject
    blue: 0
    green: 255
    id: 0xa61de7bb233df7870bca7ed3459f1261f393ec7f
    red: 0
```