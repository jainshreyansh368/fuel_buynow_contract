library data_structures;

use std::identity::Identity;

pub struct TokenMetaData {
    // This is left as an example. Support for dynamic length string is needed here
    name: str[35],
    metadata_uri: str[63],
    creators: [Identity; 5],
}

impl TokenMetaData {
    fn new() -> Self {
        Self {
            name: "Example Token #ph0z0o19uczI9d6pm3am",
            metadata_uri: "https://arweave.net/ktmJvqN7dr6OBDqmNCoo764v8iQiaGdtjL0xwGrqZI4",
            creators: [
                Identity::Address(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
                Identity::Address(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
                Identity::Address(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
                Identity::Address(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
                Identity::Address(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
            ],
        }
    }
}
