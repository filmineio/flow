# Clickhouse Setup

## Create Database
```sql
create DATABASE flow;
```

## Create Messages Table (Main)
```sql
CREATE TABLE flow.messages (
  `Cid` String,
  `Height` Int64,
  `Block` String,
  `MessageRctExitCode` Int64,
  `MessageRctReturn` String,
  `MessageRctGasUsed` Int64,
  `SubCallOf` String,
  `From` String,
  `RobustFrom` String,
  `RobustTo` String,
  `To` String,
  `GasLimit` Int64,
  `GasFeeCap` String,
  `GasPremium` String,
  `Method` Int64,
  `Params` String,
  `Value` Int64,
  `Timestamp` Int64,
  `Nonce` Int64,
  `Version` Int64
) ENGINE = ReplacingMergeTree PRIMARY KEY (
  Cid
)
ORDER BY
    Cid
```

## Create Contracts Table
```sql
CREATE TABLE flow.contracts (
  `Cid` String,
`ContractId` String,
`ContractAddress` String,
`ContractActorAddress` String,
`OwnerId` String,
`OwnerAddress` String,
`Compiler` String,
`ContractType` String,
`EthAddress` String
) ENGINE = ReplacingMergeTree PRIMARY KEY (
  ContractId
)
ORDER BY
    ContractId
```


## Create Block Table
```sql
CREATE TABLE flow.block (
  `Cid` String,
  `Block` String,
  `Height` Int64,
  `Timestamp` Int64,
  `Miner` String
) ENGINE = ReplacingMergeTree PRIMARY KEY (
  Cid
)
ORDER BY
  (
    Cid
  )
```
