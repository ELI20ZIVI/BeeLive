use std::{fs::File, io::Read};

use jsonwebtoken::{decode, errors::Result, Algorithm, DecodingKey, TokenData, Validation};
use casdoor_rust_sdk::CasdoorUser;
use serde::de::DeserializeOwned;

/// Authenticator that checks JWTs.
pub struct Authenticator {
    /// The certificate to validate the signature.
    key: DecodingKey,
    /// The validation algorithm.
    validation: Validation,
}

impl Authenticator {

    /// Creates a new authenticator with a predefined signature [algo] and a string pem [cert].
    ///
    /// [algo] defaults to PS512 (RSASSA-PSS + SHA512).
    pub fn new(algo: Option<Algorithm>, cert: &str) -> Result<Self> {
        Self {
           key: DecodingKey::from_rsa_pem(cert.as_bytes())?,
           // If not provided, PS512 is used.
           validation: Validation::new(algo.unwrap_or(Algorithm::PS512)),
        }.into()
    }

    /// Creates a new authenticator with a predefined signature [algo] and a pem file [cert].
    ///
    /// [algo] defaults to PS512 (RSASSA-PSS + SHA512).
    ///
    /// Panics if the file is not a valid pem file.
    pub fn from_file(algo: Option<Algorithm>, cert_file: &mut File) -> Result<Self> {
        let mut cert = String::new();
        cert_file.read_to_string(&mut cert).expect("Cannot open the certificate file");

        Self::new(algo, &cert)
    }

    /// Validates and decodes the given [token] extracting the claims.
    ///
    /// Validation is performed using the parameters passed during construction.
    pub fn validate_and_decode<T : DeserializeOwned>(&self, token: &str) -> Result<TokenData<T>> {
        decode::<T>(token, &self.key, &self.validation)
    }

    /// Validates and decodes the given [token] extracting only the user id claim.
    ///
    /// Validation is performed using the parameters passed during construction.
    pub fn decode_user_id(&self, token: &str) -> Result<String> {
        let token_data = self.validate_and_decode::<CasdoorUser>(token)?;

        Ok(token_data.claims.id)
    }

}

impl From<Authenticator> for Result<Authenticator> {

    fn from(authenticator: Authenticator) -> Self {
        Ok(authenticator)
    }
    
}
