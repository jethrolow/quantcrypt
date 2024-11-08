use crate::kem::common::kem_type::KemType;

/// A trait to get the length of the shared secret
pub trait SSLen {
    /// Get the length of the shared secret
    ///
    /// # Returns
    ///
    /// The length of the shared secret in bytes
    fn get_ss_len(&self) -> usize;
}

impl SSLen for KemType {
    /// Get the length of the shared secret
    ///
    /// # Returns
    ///
    /// The length of the shared secret in bytes
    fn get_ss_len(&self) -> usize {
        match self {
            // These are NOT Nsecret length as per RFC 9180
            // as there is no hash function used in the KEM
            // for the traditional KEMs
            KemType::P256 => 32,
            KemType::P384 => 48,
            KemType::X25519 => 32,
            KemType::BrainpoolP256r1 => 32,
            KemType::BrainpoolP384r1 => 48,
            KemType::X448 => 56,
            // RSA is always 32 bytes
            KemType::RsaOAEP2048 => 32,
            KemType::RsaOAEP3072 => 32,
            KemType::RsaOAEP4096 => 32,
            // ML is always 32 bytes
            KemType::MlKem512 => 32,
            KemType::MlKem768 => 32,
            KemType::MlKem1024 => 32,

            // Composite types follow hash size
            KemType::MlKem768BrainpoolP256r1 => 32,
            KemType::MlKem768X25519 => 32,
            KemType::MlKem1024P384 => 32,
            KemType::MlKem1024BrainpoolP384r1 => 32,
            KemType::MlKem1024X448 => 32,
            KemType::MlKem768Rsa2048 => 32,
            KemType::MlKem768Rsa3072 => 32,
            KemType::MlKem768Rsa4096 => 32,
            KemType::MlKem768P384 => 32,
            KemType::XWing => 32,
        }
    }
}
