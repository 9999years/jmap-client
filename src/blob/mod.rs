use crate::core::session::URLParser;

pub mod copy;
pub mod download;
pub mod helpers;
pub mod upload;

pub enum URLParameter {
    AccountId,
    BlobId,
    Name,
    Type,
}

impl URLParser for URLParameter {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "accountId" => Some(URLParameter::AccountId),
            "blobId" => Some(URLParameter::BlobId),
            "name" => Some(URLParameter::Name),
            "type" => Some(URLParameter::Type),
            _ => None,
        }
    }
}
