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
) ENGINE = ReplacingMergeTree PRIMARY KEY (
  Cid
)
ORDER BY
    Cid
```

## Create Actor BLS Table (The Latest balance on chain)

```sql
CREATE TABLE flow.actor_bls (
  `Height` Int64,
  `Block` String,
  `ActorId` String,
  `Balance` Int64,
  `Processed` Int64
) ENGINE = ReplacingMergeTree PRIMARY KEY (ActorId, Processed, Block)
ORDER BY
  (ActorId, Processed, Block)
```

## Create Contracts View
```sql
CREATE MATERIALIZED VIEW flow.contracts (
  `Cid` String,
  `ContractId` String,
  `ContractRobustAddress` String,
  `OwnerId` String,
  `OwnerRobustAddress` String,
  `Bytecode` String
) ENGINE = ReplacingMergeTree PRIMARY KEY (ContractId, ContractRobustAddress)
ORDER BY
  (ContractId, ContractRobustAddress) AS
SELECT
  tr.Cid AS Cid,
  tr.To AS ContractId,
  tr.RobustTo AS ContractRobustAddress,
  msg.From AS OwnerId,
  msg.RobustFrom AS OwnerRobustAddress,
  msg.Params as Bytecode
FROM
  flow.messages AS tr
  INNER JOIN flow.messages AS msg ON tr.SubCallOf = msg.Cid
WHERE
  (tr.From = 't01')
  AND (tr.Method = 1)
  AND (tr.SubCallOf != '')
GROUP BY
  (
    Cid,
    ContractId,
    ContractRobustAddress,
    OwnerId,
    OwnerRobustAddress,
    Bytecode
  )
```

## Create SubCalls Count View
```sql
CREATE MATERIALIZED VIEW flow.sub_calls (
  `MessageCid` String,
  `SubCallsCount` Int64
) ENGINE = ReplacingMergeTree
ORDER BY
  MessageCid AS
SELECT
  m.Cid AS MessageCid,
  count(s.Cid) as SubCallsCount
FROM flow.messages as m
INNER JOIN (
    select Cid, SubCallOf from flow.messages GROUP BY (SubCallOf, Cid)
) as s on s.SubCallOf = m.Cid
GROUP BY (m.Cid)
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
