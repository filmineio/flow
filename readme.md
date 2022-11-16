# FLOW  - Technical details

## TODO
- Logs 
  - Proper implementation is missing
- Metrics
  - Proper collection is missing
- Alerts
  - Define proper vector playbook 
    - This can also serve chain health service, new contract flow can be the blueprint 
- State Producer App - Bookkeeping
  - Planned - Sled (Embedded) 
- DFS Backup
  - Depends on data availability wg -- Currently `unimplemented`
- FVM Explorer API
  - Auth
    - Plan is to have IDP that unifies auth needs over all products
    - Address Verify
  - Source Code
    - Upload
    - Method Mapping
    - Params + Return deserializer (based on source code)
    - Contract interaction (method calling from API)
  - Bytecode analyzer and metadata extract -- Abandoned due to low value

## Usage
### State App
#### Dependencies:
 - Lotus
 - Sled (Bookkeeping embedded DB) ```TODO``` 
#### Stack
- Rust
#### Commands
- Start (standalone)
```shell
cargo run
```
- Start with Vector (some playbook examples are in `vector` dir)
```shell
cargo run | sudo vector --config vector/vector-all.toml
```
### New Contract Listener
#### Dependencies:
- Lotus
- Kafka
- Clickhouse
#### Stack
- Rust
#### Commands
- Start (standalone)
```shell
cargo run --bin contract_listener
```

### FVM Explorer API
#### Dependencies:
- Lotus
- Kafka
- Clickhouse
- Postgres
#### Stack
- Rust
#### Commands
- Start (standalone)
```shell
cargo run --bin fvm_explorer_api
```

## Overall Architecture

![wip (6).jpg](Technical%20details%206bdebae962f341c4891510700a1c4e14/wip_(6).jpg)

## Components

- State Producer (Rust bin)
    
    ### State Producer App
    
    - General Notes
        - Responsible for:
            - Decoding Messages (with respective sub calls)
            - Decoding Blocks
            - Resolving Actors (addresses)
        - Tech Stack
            - Rust
        - External Dependencies
            - Lotus
            - Sled (Bookkeeping embedded DB)
    - Diagrams
        
        ![wip (2).jpg](Technical%20details%206bdebae962f341c4891510700a1c4e14/wip_(2).jpg)
        
    - Compute State
        - Resolve Height
        - Resolve block details
        - Dependent on Lotus state compute, resolves block messages and sub calls
    - Enrich Data
        - Enriches Data
            - Message → Address Resolution → FlowMessage
            - Block → FlowBlock
    - Sync (emit)
        - Walks collected data and emits to stdout in JSON format
- Data Observability Pipeline (Vector.dev)
    
    ### Data Observability Pipeline
    
    - General Notes
        - Responsible for:
            - Parsing State App output
            - Branching Data flows via Filters
            - Remapping Data into an expected output format
            - Batch syncing data to output sources
        - Tech Stack
            - Vector.dev
        - External Dependencies
            - Input Sources
                - StateApp
            - Sync Targets
                - Clickhouse
                - Kafka
                - DFS
    - Diagrams
        
        ![wip (3).jpg](Technical%20details%206bdebae962f341c4891510700a1c4e14/wip_(3).jpg)
        
    - Playbooks
        
        Playbooks are vector configs files, utilizing Vector Remap Language (VRL) 
        
    - Input
        - Connects to State Producer app stdout
    - Transform
        - Parse
            - Parses the data into objects that can be processed with VRL
        - Filter
            
            In a filter stage, data flow is branching based on predicate functions. From this point, parsed input data is segregated into:
            
            - Blocks
            - Messages
                - ContractInit Message (sub-branch)
            - App Logs
            - Metrics
        - Remap
            
            Remap Phase is where data is prepared for the sync phase
            
            - Message → Clickhouse Table Data
            - Block → Clickhouse Table Data
            - Message → Kafka (new_contract) Payload
            - Metrics → essential metrics payload
    - Sync
        - Messages
            - Batch insert messages to Clickhouse
            - Batch save messages  to DFS
                - height/
                       block/
                              message_cid/
                                     data.json
                                       sub_calls/
                                               message_cid.json
        - Blocks
            - Batch insert Blocks to Clickhouse
            - Batch save block on DFS height/block
        - Logs
            - Sync to Loki
        - Metrics
            - Sync to Prometheus
    - Datasources
        - Clickhouse
            - Tables
                - Transactions
                - Contracts
            - Materialized Views
                - ContractTransactions
        - Kafka
            - Topics
                - new_contract
        - DFS
            - Archival sync
- New Contract Listener (Rust bin)
    
    ### New Contract Listener
    
    - General Notes
        - Responsible for:
            - Analyzing Flagged Transaction
            - Resolve Contract Details
            - Store New Contract
        - Tech Stack
            - Rust
        - External Dependencies
            - Kafka
            - Clickhouse
    - Diagrams
        
        ![wip (4).jpg](Technical%20details%206bdebae962f341c4891510700a1c4e14/wip_(4).jpg)
        
    - Parse Flagged Message
        - Once a message is flagged as contract initializer, app analyzes message return information. If information is valid message, it is passed to details resolution
    - Resolve Contract details
        - EFVM
            - Resolve message data (future contract addresses, owner, status…)
        - WASM
            - Resolve message data (contract actor addresses, owner, status…)
    - Sync Contract
        - Once details are resolved contract data is inserted into Clickhouse
- FVM Explorer API (Rust bin)
    - General Notes
        - Responsible for:
            - Providing Search APIs
                - Transactions
                - Blocks
                - Contracts
                - Projects
            - Address Verifier
            - Contract Interaction API
            - Auth
        - Tech Stack
            - Rust
        - External Dependencies
            - Clickhouse
            - Postgres
            - Lotus
    - Diagrams
        
        ![wip (5).jpg](Technical%20details%206bdebae962f341c4891510700a1c4e14/wip_(5).jpg)
        
    - Search API (public)
        - Search transactions by cid, block, actor_address
        - Search Contracts By f2 address, f4 address, Eth Address, actor_id, owner
        - Search Blocks by Cid
        - Search Projects by name, id, owner
    - Address verifier (public)
        - Provides address verification api
    - Auth (internal)
        - Provides user apis
    - Contract Interaction API (semi-public)
        - Download Abi
        - Download bytecode
        - Upload Source
        - Invoce method
    - Platform Database
        - Postgres
            - Accounts
            - Projects
    - Magic Auth
        - Passwordless Auth
- FVM Explorer UI (Next.js)