use common::{Address, Hash, XpAmount, Result, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Smart contract metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractMetadata {
    pub address: Address,
    pub code_hash: Hash,
    pub deployer: Address,
    pub created_at: u64,
    pub entropy_requirement: Option<EntropyRequirement>,
    pub interface_abi: ContractABI,
}

/// Entropy requirements for contract execution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntropyRequirement {
    /// Minimum entropy reduction to call this contract
    pub min_entropy_reduction: f64,
    /// Domains that can provide entropy measurements
    pub allowed_domains: Vec<String>,
    /// Whether entropy must be fresh (not reused)
    pub require_fresh_entropy: bool,
}

/// Contract ABI for interface definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractABI {
    pub methods: Vec<MethodSignature>,
    pub events: Vec<EventSignature>,
    pub storage_layout: StorageLayout,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MethodSignature {
    pub name: String,
    pub inputs: Vec<Parameter>,
    pub outputs: Vec<Parameter>,
    pub entropy_cost: Option<f64>,
    pub is_view: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub ty: ParameterType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParameterType {
    Address,
    Uint256,
    Int256,
    Bool,
    String,
    Bytes,
    Array(Box<ParameterType>),
    Tuple(Vec<Parameter>),
    XpAmount,
    EntropyMeasurement,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageLayout {
    pub slots: HashMap<String, StorageSlot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageSlot {
    pub offset: u64,
    pub ty: ParameterType,
    pub description: String,
}

/// Contract execution context
#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub caller: Address,
    pub contract_address: Address,
    pub value: XpAmount,
    pub gas_limit: u64,
    pub block_height: u64,
    pub timestamp: u64,
    pub entropy_provided: Option<common::EntropyReduction>,
    pub call_depth: u32,
}

/// Contract execution result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub gas_used: u64,
    pub entropy_consumed: f64,
    pub return_data: Vec<u8>,
    pub logs: Vec<LogEntry>,
    pub state_changes: StateChanges,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub address: Address,
    pub topics: Vec<Hash>,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateChanges {
    pub storage_updates: HashMap<(Address, Hash), Vec<u8>>,
    pub balance_updates: HashMap<Address, XpAmount>,
    pub nonce_updates: HashMap<Address, u64>,
}

/// Contract interface trait
pub trait Contract: Send + Sync {
    /// Initialize contract from code
    fn from_code(code: &[u8]) -> Result<Self> where Self: Sized;
    
    /// Get contract metadata
    fn metadata(&self) -> &ContractMetadata;
    
    /// Execute a contract method
    fn execute(
        &self,
        method: &str,
        args: &[u8],
        context: ExecutionContext,
        state: &mut dyn StateProvider,
    ) -> Result<ExecutionResult>;
    
    /// Estimate gas for execution
    fn estimate_gas(
        &self,
        method: &str,
        args: &[u8],
        context: ExecutionContext,
    ) -> Result<u64>;
}

/// State provider trait for contract storage
pub trait StateProvider: Send + Sync {
    /// Get storage value
    fn get_storage(&self, address: &Address, key: &Hash) -> Option<Vec<u8>>;
    
    /// Set storage value
    fn set_storage(&mut self, address: &Address, key: &Hash, value: Vec<u8>);
    
    /// Get account balance
    fn get_balance(&self, address: &Address) -> XpAmount;
    
    /// Update account balance
    fn set_balance(&mut self, address: &Address, balance: XpAmount);
    
    /// Get account nonce
    fn get_nonce(&self, address: &Address) -> u64;
    
    /// Update account nonce
    fn set_nonce(&mut self, address: &Address, nonce: u64);
    
    /// Get contract code
    fn get_code(&self, address: &Address) -> Option<Vec<u8>>;
    
    /// Check if address is a contract
    fn is_contract(&self, address: &Address) -> bool;
}

/// Contract factory for different VM types
pub trait ContractFactory: Send + Sync {
    /// Create contract instance from code
    fn create_contract(&self, code: &[u8], metadata: ContractMetadata) -> Result<Box<dyn Contract>>;
    
    /// Validate contract code
    fn validate_code(&self, code: &[u8]) -> Result<()>;
    
    /// Get supported features
    fn supported_features(&self) -> Vec<String>;
}