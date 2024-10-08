use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::util::types::ByteArray32;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountEvent {
    Lifecycle(LifecycleEvent),
    Transaction {
        timestamp: u64,
        txid: ByteArray32,
        event: TransactionEvent,
    },
}

impl AccountEvent {
    pub fn account_opened(account_id: String) -> Self {
        AccountEvent::Lifecycle(LifecycleEvent::Opened { account_id })
    }

    pub fn account_disabled() -> Self {
        AccountEvent::Lifecycle(LifecycleEvent::Disabled)
    }

    pub fn account_enabled() -> Self {
        AccountEvent::Lifecycle(LifecycleEvent::Enabled)
    }

    pub fn account_closed() -> Self {
        AccountEvent::Lifecycle(LifecycleEvent::Closed)
    }

    pub fn deposited(txid: ByteArray32, timestamp: u64, asset: String, amount: u64) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Deposited { asset, amount },
        }
    }

    pub fn debited(
        txid: ByteArray32,
        timestamp: u64,
        to_account: String,
        asset: String,
        amount: u64,
    ) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Debited {
                to_account,
                asset,
                amount,
            },
        }
    }

    pub fn debit_reversed(
        txid: ByteArray32,
        timestamp: u64,
        to_account: String,
        asset: String,
        amount: u64,
    ) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::DebitReversed {
                to_account,
                asset,
                amount,
            },
        }
    }

    pub fn credited(
        txid: ByteArray32,
        timestamp: u64,
        from_account: String,
        asset: String,
        amount: u64,
    ) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Credited {
                from_account,
                asset,
                amount,
            },
        }
    }

    pub fn credit_reversed(
        txid: ByteArray32,
        timestamp: u64,
        from_account: String,
        asset: String,
        amount: u64,
    ) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::CreditReversed {
                from_account,
                asset,
                amount,
            },
        }
    }

    pub fn withdrew(txid: ByteArray32, timestamp: u64, asset: String, amount: u64) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Withdrew { asset, amount },
        }
    }

    pub fn funds_locked(
        txid: ByteArray32,
        timestamp: u64,
        asset: String,
        amount: u64,
    ) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::FundsLocked {
                asset,
                amount,
            },
        }
    }

    pub fn funds_unlocked(txid: ByteArray32,
                          timestamp: u64,
                          asset: String,
                          amount: u64,) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::FundsUnlocked {
                asset,
                amount
            },
        }
    }

    pub fn settlement(
        txid: ByteArray32,
        timestamp: u64,
        to_account: String,
        send_asset: String,
        send_amount: u64,
        receive_asset: String,
        receive_amount: u64
    ) -> Self {
        AccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Settled {
                to_account,
                send_asset,
                send_amount,
                receive_asset,
                receive_amount
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LifecycleEvent {
    Opened { account_id: String },
    Disabled,
    Enabled,
    Closed,
}

impl LifecycleEvent {
    fn event_name(&self) -> String {
        match self {
            LifecycleEvent::Opened { .. } => "Opened".to_string(),
            LifecycleEvent::Disabled => "Disabled".to_string(),
            LifecycleEvent::Enabled => "Enabled".to_string(),
            LifecycleEvent::Closed => "Closed".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionEvent {
    Deposited {
        asset: String,
        amount: u64,
    },
    Withdrew {
        asset: String,
        amount: u64,
    },
    Debited {
        to_account: String,
        asset: String,
        amount: u64,
    },
    DebitReversed {
        to_account: String,
        asset: String,
        amount: u64,
    },
    Credited {
        from_account: String,
        asset: String,
        amount: u64,
    },
    CreditReversed {
        from_account: String,
        asset: String,
        amount: u64,
    },
    FundsLocked {
        asset: String,
        amount: u64,
    },
    FundsUnlocked {
        asset: String,
        amount: u64,
    },
    Settled {
        to_account: String,
        send_asset: String,
        send_amount: u64,
        receive_asset: String,
        receive_amount: u64,
    },
}

impl TransactionEvent {
    fn event_name(&self) -> String {
        match self {
            TransactionEvent::Deposited { .. } => "CustomerDepositedMoney".to_string(),
            TransactionEvent::Withdrew { .. } => "CustomerWithdrewCash".to_string(),
            TransactionEvent::Debited { .. } => "Debited".to_string(),
            TransactionEvent::DebitReversed { .. } => "DebitReversed".to_string(),
            TransactionEvent::Credited { .. } => "Credited".to_string(),
            TransactionEvent::CreditReversed { .. } => "CreditReversed".to_string(),
            TransactionEvent::FundsLocked { .. } => "FundsLocked".to_string(),
            TransactionEvent::FundsUnlocked { .. } => "FundsUnlocked".to_string(),
            TransactionEvent::Settled { .. } => "Settled".to_string(),
        }
    }
}

impl DomainEvent for AccountEvent {
    fn event_type(&self) -> String {
        match self {
            AccountEvent::Lifecycle(account_event) => {
                format!("Lifecycle::{}", account_event.event_name())
            }
            AccountEvent::Transaction {
                timestamp: _,
                txid: _,
                event,
            } => format!("Transaction::{}", event.event_name()),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum AccountError {
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Account not found")]
    AccountNotFound,
    #[error("Account already exists")]
    AccountAlreadyExists,
    #[error("Account is disabled")]
    AccountNotDisabled,
    #[error("Account is not in service")]
    AccountNotInService,
    #[error("Account is not empty")]
    AccountNotEmpty,
    #[error("Lock not found, please check the transaction id and make sure it not expired")]
    LockNotFound,
    #[error("Invalid transaction")]
    InvalidTransaction,
    #[error("Duplicate lock, this lock has already been processed")]
    DuplicateLock,
    #[error("duplicate transaction, this transaction has already been processed at {0}")]
    DuplicateTransaction(u64),
    #[error("Transaction not found, please check the transaction and make sure it not expired")]
    TransactionNotFound,
}
