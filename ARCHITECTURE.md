# XP-Net Architecture

## Overview

XP-Net is a DAG-based Layer 1 protocol that replaces traditional blockchain consensus with entropy-weighted validation. Instead of mining or staking for block production, validators earn influence by demonstrating measurable entropy reduction in the real world.

## Core Concepts

### 1. DAG Structure
- Transactions form a Directed Acyclic Graph (DAG) instead of a linear blockchain
- Each transaction references multiple parent transactions
- Enables parallel processing and higher throughput
- No blocks - transactions are individually validated

### 2. Entropy-Based Validation
- Value creation measured by entropy reduction (ΔS)
- XP tokens minted only when validators confirm measurable order creation
- Formula: `XP = ΔS / c_L²` where c_L is causal loop closure speed
- Physical constraints prevent gaming the system

### 3. Validator Mesh
- Distributed network of validators with varying specializations
- Validators weighted by stake, reputation, and entropy contribution
- Domain-specific expertise (physical, computational, economic, etc.)
- Byzantine fault tolerance through entropy requirements

## Key Data Structures

### Transaction
```rust
pub struct Transaction {
    pub hash: Hash,
    pub tx_type: TransactionType,
    pub parents: Vec<Hash>,        // DAG references
    pub timestamp: Timestamp,
    pub nonce: u64,
    pub signature: Signature,
    pub metadata: TransactionMetadata,
}
```

Transaction types include:
- `Transfer`: XP token transfers
- `ContractDeploy`: Smart contract deployment
- `ContractCall`: Contract execution
- `EntropyReductionClaim`: Mint XP by proving entropy reduction
- `ValidatorUpdate`: Join/update validator set
- `LoopClosure`: Complete causal loops

### DAG
```rust
pub struct Dag {
    transactions: Arc<DashMap<Hash, Arc<Transaction>>>,
    graph: Arc<RwLock<DiGraph<Hash, ()>>>,
    tips: Arc<RwLock<HashSet<Hash>>>,
    account_states: Arc<DashMap<Address, AccountState>>,
}
```

The DAG maintains:
- Transaction storage and indexing
- Graph structure for traversal
- Tips tracking (transactions without children)
- Account state (balances, nonces, entropy contributions)

### Validator
```rust
pub struct Validator {
    pub address: Address,
    pub stake: XpAmount,
    pub reputation: f64,
    pub total_entropy_reduced: f64,
    pub specializations: Vec<ValidatorDomain>,
    pub metrics: ValidatorMetrics,
}
```

Validators specialize in domains:
- Physical (sensor measurements)
- Information (data analysis)
- Economic (market dynamics)
- Social (network effects)
- Computational (optimization)
- Contracts (smart contract validation)

### Entropy Measurement
```rust
pub struct EntropyReduction {
    pub initial: EntropyMeasurement,
    pub final_state: EntropyMeasurement,
    pub delta: f64,                    // Positive = order created
    pub duration_ns: u64,
    pub validators: Vec<ValidatorAttestation>,
}
```

## Consensus Mechanism

### Entropy-Weighted Consensus
1. Transaction submitted with entropy reduction claim
2. Validators selected based on:
   - Domain expertise
   - Reputation score
   - Historical entropy contribution
   - Current stake

3. Each validator measures/verifies the entropy claim
4. Responses weighted by validator influence
5. Consensus reached when weighted majority agrees
6. XP minted proportional to verified entropy reduction

### Anti-Gaming Properties
- **Sybil resistance**: New validators have zero reputation
- **Collusion prevention**: False claims require actual entropy reduction
- **Byzantine tolerance**: Entropy measurements must converge
- **Stake slashing**: Validators lose XP for false attestations

## Module Structure

### `dag_core`
Core DAG implementation:
- Transaction validation and storage
- DAG traversal algorithms
- Cycle detection
- Account state management

### `validator_mesh`
Validator network management:
- Validator registration and updates
- Entropy-weighted selection
- Consensus aggregation
- Reputation tracking

### `xp_minting`
XP token economics:
- Entropy-to-XP conversion
- Minting rules enforcement
- Supply tracking
- Reward distribution

### `contract_runtime`
Smart contract execution:
- WASM-based VM
- Gas metering by entropy
- State transitions
- Cross-contract calls

### `p2p_sync`
Network synchronization:
- Transaction propagation
- DAG tip exchange
- Validator discovery
- State sync protocols

## Example Validation Flow

1. **Alice claims she organized her workshop** (entropy reduction)
   ```rust
   let tx = Transaction::new(
       TransactionType::EntropyReductionClaim {
           claimant: alice_address,
           reduction: workshop_measurement,
       },
       parent_tips,
       nonce,
       metadata,
   );
   ```

2. **Validators selected based on expertise**
   - Physical validators check sensor data
   - Social validators verify participant feedback
   - Economic validators measure productivity gains

3. **Consensus aggregation**
   ```rust
   let consensus = EntropyWeightedConsensus {
       required_confidence: 0.8,
       majority_threshold: 0.667,
   };
   let result = consensus.aggregate_validations(responses, weights)?;
   ```

4. **XP minting if validated**
   ```rust
   if result.is_valid {
       let xp_amount = result.entropy_reduction.calculate_xp();
       dag.mint_xp(alice_address, xp_amount);
   }
   ```

## Next Steps

1. Implement remaining modules (contract runtime, p2p sync)
2. Add WebAssembly smart contract support
3. Build validator node implementation
4. Create client SDKs (Rust, TypeScript, Python)
5. Deploy testnet with entropy measurement oracles
6. Integrate with real-world sensors and data sources