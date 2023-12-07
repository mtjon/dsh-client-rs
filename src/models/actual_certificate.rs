/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.6.6
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ActualCertificate : information on a certificate which is provisioned on the platform



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActualCertificate {
    #[serde(rename = "keySecret")]
    pub key_secret: String,
    #[serde(rename = "certChainSecret")]
    pub cert_chain_secret: String,
    #[serde(rename = "passphraseSecret", skip_serializing_if = "Option::is_none")]
    pub passphrase_secret: Option<String>,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "notBefore")]
    pub not_before: String,
    #[serde(rename = "notAfter")]
    pub not_after: String,
    #[serde(rename = "distinguishedName")]
    pub distinguished_name: String,
    #[serde(rename = "dnsNames")]
    pub dns_names: Vec<String>,
}

impl ActualCertificate {
    /// information on a certificate which is provisioned on the platform
    pub fn new(key_secret: String, cert_chain_secret: String, serial_number: String, not_before: String, not_after: String, distinguished_name: String, dns_names: Vec<String>) -> ActualCertificate {
        ActualCertificate {
            key_secret,
            cert_chain_secret,
            passphrase_secret: None,
            serial_number,
            not_before,
            not_after,
            distinguished_name,
            dns_names,
        }
    }
}

