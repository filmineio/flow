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
  `Value` Int64
) ENGINE = ReplacingMergeTree PRIMARY KEY (
  Cid,
  From,
    To,
    RobustFrom,
    RobustTo
)
ORDER BY
  (
    Cid,
    From,
      To,
      RobustFrom,
      RobustTo,
      Height
  ) SETTINGS index_granularity = 8192
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
  (ActorId) SETTINGS index_granularity = 8192
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
  `OnChainBalance` Int64
) ENGINE = ReplacingMergeTree PARTITION BY ContractId
ORDER BY
  ContractId SETTINGS index_granularity = 8192 AS
SELECT
  tr.Cid AS Cid,
  tr.To AS ContractId,
  tr.RobustTo AS ContractRobustAddress,
  msg.From AS OwnerId,
  msg.RobustFrom AS OwnerRobustAddress,
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
      count(Cid) AS Num
    FROM
      flow.messages
    GROUP BY
      To
  ) AS i ON i.To = ContractId
  LEFT JOIN (
    SELECT
    From,
      sum(Value) AS Val,
      count(Cid) AS Num
    FROM
      flow.messages
    GROUP BY
    From
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
    OnChainBalance
  )
```