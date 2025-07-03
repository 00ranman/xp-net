use common::{Result, Error};
use std::sync::atomic::{AtomicU64, Ordering};

/// Gas metering for contract execution
pub struct GasMeter {
    limit: u64,
    used: AtomicU64,
}

impl GasMeter {
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            used: AtomicU64::new(0),
        }
    }
    
    pub fn consume(&self, amount: u64) -> Result<()> {
        let new_used = self.used.fetch_add(amount, Ordering::Relaxed) + amount;
        if new_used > self.limit {
            Err(Error::Other("Out of gas".into()))
        } else {
            Ok(())
        }
    }
    
    pub fn gas_used(&self) -> u64 {
        self.used.load(Ordering::Relaxed)
    }
    
    pub fn gas_remaining(&self) -> u64 {
        self.limit.saturating_sub(self.gas_used())
    }
}

/// Entropy-based gas metering
pub struct EntropyGasMeter {
    /// Base gas cost per operation
    base_cost: u64,
    /// Entropy reduction factor (reduces gas cost)
    entropy_discount: f64,
}

impl EntropyGasMeter {
    pub fn new() -> Self {
        Self {
            base_cost: 1000,
            entropy_discount: 0.1,
        }
    }
    
    /// Calculate gas cost with entropy discount
    pub fn calculate_cost(&self, operation: GasOperation, entropy_provided: Option<f64>) -> u64 {
        let base = match operation {
            GasOperation::Storage(StorageOp::Read) => self.base_cost,
            GasOperation::Storage(StorageOp::Write) => self.base_cost * 5,
            GasOperation::Compute(complexity) => self.base_cost * complexity as u64,
            GasOperation::Memory(size) => (size / 32) * self.base_cost / 10,
            GasOperation::Call => self.base_cost * 10,
            GasOperation::Create => self.base_cost * 100,
        };
        
        // Apply entropy discount
        if let Some(entropy) = entropy_provided {
            let discount = (entropy * self.entropy_discount).min(0.9);
            (base as f64 * (1.0 - discount)) as u64
        } else {
            base
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GasOperation {
    Storage(StorageOp),
    Compute(ComputeComplexity),
    Memory(u64), // bytes
    Call,
    Create,
}

#[derive(Clone, Copy, Debug)]
pub enum StorageOp {
    Read,
    Write,
}

#[derive(Clone, Copy, Debug)]
pub enum ComputeComplexity {
    Simple = 1,
    Moderate = 10,
    Complex = 100,
    VeryComplex = 1000,
}

/// Gas schedule with entropy-aware pricing
pub struct GasSchedule {
    prices: std::collections::HashMap<String, u64>,
}

impl Default for GasSchedule {
    fn default() -> Self {
        let mut prices = std::collections::HashMap::new();
        
        // Basic operations
        prices.insert("add".to_string(), 3);
        prices.insert("mul".to_string(), 5);
        prices.insert("div".to_string(), 5);
        prices.insert("sub".to_string(), 3);
        
        // Memory operations
        prices.insert("mload".to_string(), 3);
        prices.insert("mstore".to_string(), 3);
        prices.insert("mstore8".to_string(), 3);
        
        // Storage operations
        prices.insert("sload".to_string(), 200);
        prices.insert("sstore".to_string(), 5000);
        
        // Hashing
        prices.insert("sha3".to_string(), 30);
        prices.insert("blake3".to_string(), 25);
        
        // Contract operations
        prices.insert("call".to_string(), 700);
        prices.insert("create".to_string(), 32000);
        
        // Entropy operations
        prices.insert("entropy_measure".to_string(), 1000);
        prices.insert("entropy_verify".to_string(), 500);
        prices.insert("loop_close".to_string(), 2000);
        
        Self { prices }
    }
}

impl GasSchedule {
    pub fn get_cost(&self, operation: &str) -> u64 {
        self.prices.get(operation).copied().unwrap_or(1)
    }
}