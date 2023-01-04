library data_structures;

use std::identity::Identity;

pub struct TokenMetaData {
    // This is left as an example. Support for dynamic length string is needed here
    metadata_uri: str[46],
    name: str[35],
    creators: [Identity; 5],
}

impl TokenMetaData {
    pub fn new(name: str[35], metadata_uri: str[46], creators: [Identity; 5]) -> Self {
        Self {
            name,
            metadata_uri,
            creators,
        }
    }
}
