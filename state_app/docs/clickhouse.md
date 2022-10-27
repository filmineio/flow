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
  `BlockTimestamp` Int64
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
) ENGINE = ReplacingMergeTree PRIMARY KEY (ActorId)
ORDER BY
  (ActorId)
```

## Create Contracts View
```sql
CREATE MATERIALIZED VIEW flow.contracts (
  `Cid` String,
  `ContractId` String,
  `ContractRobustAddress` String,
  `OwnerId` String,
  `OwnerRobustAddress` String,
  `TransactionCount` Int64,
  `Balance` Int64,
  `OnChainBalance` Int64,
  `Bytecode` String
) ENGINE = ReplacingMergeTree PARTITION BY ContractId
ORDER BY
  ContractId SETTINGS index_granularity = 8192 AS
SELECT
  tr.Cid AS Cid,
  tr.To AS ContractId,
  tr.RobustTo AS ContractRobustAddress,
  msg.From AS OwnerId,
  msg.RobustFrom AS OwnerRobustAddress,
  msg.Params as Bytecode,
  i.Val - o.Val AS Balance,
  i.Num + o.Num AS TransactionCount,
  a.Balance AS OnChainBalance
FROM
  flow.messages AS tr
  INNER JOIN flow.messages AS msg ON tr.SubCallOf = msg.Cid
  LEFT JOIN (
    SELECT
      To,
      sum(Value) AS Val,
      count(To) AS Num
    FROM
      flow.messages
  ) AS i ON i.To = ContractId
  LEFT JOIN (
    SELECT
    From,
      sum(Value) AS Val,
      count(From) AS Num
    FROM
      flow.messages
  ) AS o ON o.From = ContractId
  LEFT JOIN (
    SELECT
      ActorId,
      Balance
    FROM
      flow.actor_bls
  ) AS a ON a.ActorId = ContractId
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
    TransactionCount,
    Balance,
    OnChainBalance,
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

## Create Block Table
```sql
CREATE TABLE flow.block (
  `Cid` String,
  `Block` String,
  `Height` Int64,
  `Timestamp` Int64
) ENGINE = ReplacingMergeTree PRIMARY KEY (
  Cid
)
ORDER BY
  (
    Cid
  )
```
