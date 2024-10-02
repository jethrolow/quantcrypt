use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum QuantCryptError {
    #[error("Key wrap failed")]
    KeyWrapFailed,
    #[error("Key unwrap failed")]
    KeyUnwrapFailed,
    #[error("Invalid OID")]
    InvalidOid,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid private key")]
    InvalidPrivateKey,
    #[error("Serilization to/from PEM/DER failed")]
    SerializationFailed,
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    #[error("Signature failed")]
    SignatureFailed,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Key pair generation failed")]
    KeyPairGenerationFailed,
    #[error("Missing not_after")]
    MissingNotAfter,
    #[error("Missing subject")]
    MissingSubject,
    #[error("Missing public key")]
    MissingPublicKey,
    #[error("Bad private key")]
    BadPrivateKey,
    #[error("Bad public key")]
    BadPublicKey,
    #[error("Bad subject")]
    BadSubject,
    #[error("Invalid HKDF length")]
    InvalidHkdfLength,
    #[error("Bad issuers public key")]
    BadIssuersPublicKey,
    #[error("Bad serial number key")]
    BadSerialNumber,
    #[error("Bad extension")]
    BadExtension,
    #[error("Invalid not_before. Please use an ISO 8601 date string and ensure that not_before is before not_after")]
    InvalidNotBefore,
    #[error("Invalid not after. Please use an ISO 8601 date string and ensure that not_after is after not_before. Also, ensure that not_after is not in the past")]
    InvalidNotAfter,
    #[error("Certificate is invalid")]
    InvalidCertificate,
    #[error(
        "Unsupported operation. Only DSA keys can be used for signing and KEM keys for encap/decap"
    )]
    UnsupportedOperation,
    #[error("Not implemented")]
    NotImplemented,
    #[error("Invalid ciphertext")]
    InvalidCiphertext,
    #[error("Encap failed")]
    EncapFailed,
    #[error("Decap failed")]
    DecapFailed,
    #[error("Unknown error")]
    Unknown,
}