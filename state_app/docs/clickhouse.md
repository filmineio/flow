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
  `Nonce` Int64
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


## Create contract_bls Table
```sql
CREATE MATERIALIZED VIEW flow.contracts_bls (
  `ContractId` String,
  `TransactionCount` Int64,
  `Balance` Int64
) ENGINE = ReplacingMergeTree PRIMARY KEY (ContractId)
ORDER BY
  (ContractId) AS
SELECT
  t.ActorId as ContractId,
  t.Balance as Balance,
  i.Num + o.Num AS TransactionCount
FROM
  flow.actor_bls AS t
  LEFT JOIN (
    SELECT
      Cid,
      RobustTo,
      count(Value) AS Num  
    FROM
      flow.messages
    GROUP BY (Cid, RobustTo)
  ) AS i ON i.RobustTo = ContractId
  LEFT JOIN (
    SELECT
    Cid,
    RobustFrom,
      count(Value) AS Num
    FROM
      flow.messages
    GROUP BY (Cid, RobustFrom)
  ) AS o ON o.RobustFrom = ContractId
GROUP BY
  (
    ContractId,
    TransactionCount,
    Balance
  )
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
