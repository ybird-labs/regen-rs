use thiserror::Error;

/// Main error type for the Regen client library
#[derive(Error, Debug)]
pub enum RegenError {
    // Key management errors
    #[error("BIP39 mnemonic error: {0}")]
    Mnemonic(#[from] bip39::Error),

    #[error("BIP32 key derivation error: {0}")]
    KeyDerivation(#[from] bip32::Error),

    // Transaction handling errors
    #[error("Cosmos SDK error: {0}")]
    Cosmos(#[from] cosmrs::ErrorReport),

    #[error("Transaction encoding error: {0}")]
    Encoding(#[from] prost::EncodeError),

    #[error("Transaction decoding error: {0}")]
    Decoding(#[from] prost::DecodeError),

    // Network errors
    #[error("gRPC error: {0}")]
    Grpc(#[from] Box<tonic::Status>),

    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    // Cosmos SDK errors
    #[error("Invalid chain ID: '{0}'")]
    InvalidChainId(String),

    #[error("Invalid address format: '{0}'")]
    InvalidAddress(String),

    #[error("Invalid derivation path: '{0}'")]
    InvalidDerivationPath(String),

    #[error("Signer not configured - call attach_signer() first")]
    NoSigner,

    #[error("Account '{account}' not found on chain '{chain_id}'")]
    AccountNotFound { account: String, chain_id: String },

    #[error("Invalid account sequence: expected {expected}, got {actual}")]
    InvalidSequence { expected: u64, actual: u64 },

    #[error("Insufficient balance: need {required}, have {available}")]
    InsufficientBalance { required: u64, available: u64 },

    #[error("Invalid private key length: expected 32 bytes, got {0}")]
    InvalidPrivateKeyLength(usize),

    #[error(
        "Transaction timeout: transaction not included in block after {timeout_seconds} seconds"
    )]
    TransactionTimeout { timeout_seconds: u64 },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),

    // Signer errors
    #[error("Signer error: {0}")]
    Signer(String),

    #[error("Invalid configuration: {0}")]
    InvalidSignerConfiguration(String),
}

/// Convenience type alias for Results using RegenError
pub type Result<T> = std::result::Result<T, RegenError>;
