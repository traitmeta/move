# move

move contract dev


## Struct

aptos includes aptos rpc with rust.
sui includes sui rpc with rust.
contracts includes move contracts with sui and aptos framework.

## 疑惑点/TODO
1. 怎么使用助记词恢复私钥签名
2. 实现一个简单的钱包功能？构建交易，签名交易和发送交易


## SUI

### explorer

1. sui_getTotalTransactionNumber 获取当前最新的交易版本
2. sui_getTransactionsInRange   params = [41167, 41182] 或者这个区间的交易内容，返回高度+交易ID
3. sui_getObject  params = ["0x05"]   获取当前验证节点，下一期的验证节点的信息
4. sui_getTransaction    RPC批量获取指定的交易详情

