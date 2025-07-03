use crate::contract::StateProvider;
use common::{Address, Hash, XpAmount};
use dashmap::DashMap;
use std::sync::Arc;

/// In-memory state provider for testing and development
pub struct InMemoryState {
    storage: Arc<DashMap<(Address, Hash), Vec<u8>>>,
    balances: Arc<DashMap<Address, XpAmount>>,
    nonces: Arc<DashMap<Address, u64>>,
    code: Arc<DashMap<Address, Vec<u8>>>,
}

impl InMemoryState {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(DashMap::new()),
            balances: Arc::new(DashMap::new()),
            nonces: Arc::new(DashMap::new()),
            code: Arc::new(DashMap::new()),
        }
    }
    
    pub fn deploy_contract(&self, address: Address, code: Vec<u8>) {
        self.code.insert(address, code);
    }
}

impl StateProvider for InMemoryState {
    fn get_storage(&self, address: &Address, key: &Hash) -> Option<Vec<u8>> {
        self.storage.get(&(*address, *key)).map(|entry| entry.clone())
    }
    
    fn set_storage(&mut self, address: &Address, key: &Hash, value: Vec<u8>) {
        self.storage.insert((*address, *key), value);
    }
    
    fn get_balance(&self, address: &Address) -> XpAmount {
        self.balances.get(address)
            .map(|entry| *entry)
            .unwrap_or_else(XpAmount::zero)
    }
    
    fn set_balance(&mut self, address: &Address, balance: XpAmount) {
        self.balances.insert(*address, balance);
    }
    
    fn get_nonce(&self, address: &Address) -> u64 {
        self.nonces.get(address)
            .map(|entry| *entry)
            .unwrap_or(0)
    }
    
    fn set_nonce(&mut self, address: &Address, nonce: u64) {
        self.nonces.insert(*address, nonce);
    }
    
    fn get_code(&self, address: &Address) -> Option<Vec<u8>> {
        self.code.get(address).map(|entry| entry.clone())
    }
    
    fn is_contract(&self, address: &Address) -> bool {
        self.code.contains_key(address)
    }
}

/// State diff tracking for transaction execution
#[derive(Clone, Debug, Default)]
pub struct StateDiff {
    pub storage_changes: Vec<StorageChange>,
    pub balance_changes: Vec<BalanceChange>,
    pub nonce_changes: Vec<NonceChange>,
    pub code_deployments: Vec<CodeDeployment>,
}

#[derive(Clone, Debug)]
pub struct StorageChange {
    pub address: Address,
    pub key: Hash,
    pub old_value: Option<Vec<u8>>,
    pub new_value: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct BalanceChange {
    pub address: Address,
    pub old_balance: XpAmount,
    pub new_balance: XpAmount,
}

#[derive(Clone, Debug)]
pub struct NonceChange {
    pub address: Address,
    pub old_nonce: u64,
    pub new_nonce: u64,
}

#[derive(Clone, Debug)]
pub struct CodeDeployment {
    pub address: Address,
    pub code_hash: Hash,
    pub code: Vec<u8>,
}

/// State provider with diff tracking
pub struct DiffTrackingState<S: StateProvider> {
    inner: S,
    diff: StateDiff,
}

impl<S: StateProvider> DiffTrackingState<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            diff: StateDiff::default(),
        }
    }
    
    pub fn into_diff(self) -> StateDiff {
        self.diff
    }
}

impl<S: StateProvider> StateProvider for DiffTrackingState<S> {
    fn get_storage(&self, address: &Address, key: &Hash) -> Option<Vec<u8>> {
        self.inner.get_storage(address, key)
    }
    
    fn set_storage(&mut self, address: &Address, key: &Hash, value: Vec<u8>) {
        let old_value = self.inner.get_storage(address, key);
        
        self.diff.storage_changes.push(StorageChange {
            address: *address,
            key: *key,
            old_value: old_value.clone(),
            new_value: Some(value.clone()),
        });
        
        self.inner.set_storage(address, key, value);
    }
    
    fn get_balance(&self, address: &Address) -> XpAmount {
        self.inner.get_balance(address)
    }
    
    fn set_balance(&mut self, address: &Address, balance: XpAmount) {
        let old_balance = self.inner.get_balance(address);
        
        if old_balance.amount != balance.amount {
            self.diff.balance_changes.push(BalanceChange {
                address: *address,
                old_balance,
                new_balance: balance,
            });
        }
        
        self.inner.set_balance(address, balance);
    }
    
    fn get_nonce(&self, address: &Address) -> u64 {
        self.inner.get_nonce(address)
    }
    
    fn set_nonce(&mut self, address: &Address, nonce: u64) {
        let old_nonce = self.inner.get_nonce(address);
        
        if old_nonce != nonce {
            self.diff.nonce_changes.push(NonceChange {
                address: *address,
                old_nonce,
                new_nonce: nonce,
            });
        }
        
        self.inner.set_nonce(address, nonce);
    }
    
    fn get_code(&self, address: &Address) -> Option<Vec<u8>> {
        self.inner.get_code(address)
    }
    
    fn is_contract(&self, address: &Address) -> bool {
        self.inner.is_contract(address)
    }
}