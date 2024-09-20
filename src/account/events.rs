use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::commands::ByteArray32;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BankAccountEvent {
    Account(AccountEvent),
    Transaction {
        timestamp: u64,
        txid: ByteArray32,
        event: TransactionEvent,
    },
}

impl BankAccountEvent {
    pub fn account_opened(account_id: String) -> Self {
        BankAccountEvent::Account(AccountEvent::AccountOpened { account_id })
    }

    pub fn account_disabled() -> Self {
        BankAccountEvent::Account(AccountEvent::AccountDisabled)
    }

    pub fn account_enabled() -> Self {
        BankAccountEvent::Account(AccountEvent::AccountEnabled)
    }

    pub fn account_closed() -> Self {
        BankAccountEvent::Account(AccountEvent::AccountClosed)
    }

    pub fn deposited(txid: ByteArray32, timestamp: u64, asset: String, amount: u64) -> Self {
        BankAccountEvent::Transaction {
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
        BankAccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Debited {
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
        BankAccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Credited {
                from_account,
                asset,
                amount,
            },
        }
    }

    pub fn withdrew(txid: ByteArray32, timestamp: u64, asset: String, amount: u64) -> Self {
        BankAccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Withdrew { asset, amount },
        }
    }

    pub fn funds_locked(
        txid: ByteArray32,
        timestamp: u64,
        order_id: ByteArray32,
        asset: String,
        amount: u64,
        expiration: u64,
    ) -> Self {
        BankAccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::FundsLocked {
                order_id,
                asset,
                amount,
                expiration,
            },
        }
    }

    pub fn funds_unlocked(txid: ByteArray32, timestamp: u64, order_id: ByteArray32) -> Self {
        BankAccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::FundsUnlocked { order_id },
        }
    }

    pub fn settlement(
        txid: ByteArray32,
        timestamp: u64,
        order_id: ByteArray32,
        to_account: String,
    ) -> Self {
        BankAccountEvent::Transaction {
            timestamp,
            txid,
            event: TransactionEvent::Settled {
                order_id,
                to_account,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountEvent {
    AccountOpened { account_id: String },
    AccountDisabled,
    AccountEnabled,
    AccountClosed,
}

impl AccountEvent {
    fn event_name(&self) -> String {
        match self {
            AccountEvent::AccountOpened { .. } => "AccountOpened".to_string(),
            AccountEvent::AccountDisabled => "AccountDisabled".to_string(),
            AccountEvent::AccountEnabled => "AccountEnabled".to_string(),
            AccountEvent::AccountClosed => "AccountClosed".to_string(),
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
        order_id: ByteArray32,
        asset: String,
        amount: u64,
        expiration: u64,
    },
    FundsUnlocked {
        order_id: ByteArray32,
    },
    Settled {
        order_id: ByteArray32,
        to_account: String,
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

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        match self {
            BankAccountEvent::Account(account_event) => {
                format!("Account::{}", account_event.event_name())
            }
            BankAccountEvent::Transaction {
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
pub enum BankAccountError {
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
    #[error("duplicate transaction, this transaction has already been processed at {0}")]
    DuplicateTransaction(u64),
    #[error("Transaction not found, please check the transaction and make sure it not expired")]
    TransactionNotFound,
}
