use crate::{Address, AssetFreezeTransactionBuilder, Byte32, TransactionHeaderBuilder};
use base64::{Engine, prelude::BASE64_STANDARD};

pub struct AssetFreezeTransactionMother {}

impl AssetFreezeTransactionMother {
    pub fn asset_freeze() -> AssetFreezeTransactionBuilder {
        // mainnet-2XFGVOHMFYLAWBHOSIOI67PBT5LDRHBTD3VLX5EYBDTFNVKMCJIA
        let sender = "E4A6FVIHXSZ3F7QXRCOTYDDILVQYEBFH56HYDIIYX4SVXS2QX5GUTBVZHY"
            .parse::<Address>()
            .unwrap();
        let freeze_address = "ZJU3X2B2QN3BUBIJ64JZ565V363ANGBUDOLXAJHDXGIIMYK6WV3NSNCBQQ"
            .parse::<Address>()
            .unwrap();
        let genesis_hash: Byte32 = BASE64_STANDARD
            .decode("wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=")
            .unwrap()
            .try_into()
            .unwrap();
        let note = BASE64_STANDARD
            .decode("TkZUIGZyZWV6ZWQgYnkgbG9mdHkuYWk=")
            .unwrap();
        let group = BASE64_STANDARD
            .decode("xERjxVTlNb8jeHa16qmpxDMh4+dcDCokO69QnNESbFk=")
            .unwrap()
            .try_into()
            .unwrap();

        AssetFreezeTransactionBuilder::default()
            .header(
                TransactionHeaderBuilder::default()
                    .sender(sender)
                    .fee(1000)
                    .first_valid(37463562)
                    .last_valid(37464562)
                    .genesis_hash(genesis_hash)
                    .genesis_id("mainnet-v1.0".to_string())
                    .note(note)
                    .group(group)
                    .build()
                    .unwrap(),
            )
            .asset_id(1707148495)
            .freeze_target(freeze_address)
            .frozen(true)
            .to_owned()
    }

    pub fn asset_unfreeze() -> AssetFreezeTransactionBuilder {
        // testnet-LZ2ODDAT4ATAVJUEQW34DIKMPCMBXCCHOSIYKMWGBPEVNHLSEV2A
        let sender = "WLH5LELVSEVQL45LBRQYCLJAX6KQPGWUY5WHJXVRV2NPYZUBQAFPH22Q7A"
            .parse::<Address>()
            .unwrap();
        let freeze_address = "ZYQX7BZ6LGTD7UCS7J5RVEAKHUJPK3FNJFZV2GPUYS2TFIADVFHDBKTN7I"
            .parse::<Address>()
            .unwrap();
        let genesis_hash: Byte32 = BASE64_STANDARD
            .decode("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=")
            .unwrap()
            .try_into()
            .unwrap();
        let note = BASE64_STANDARD.decode("th4JDxFROQw=").unwrap();

        AssetFreezeTransactionBuilder::default()
            .header(
                TransactionHeaderBuilder::default()
                    .sender(sender)
                    .fee(1000)
                    .first_valid(3277583)
                    .last_valid(3278583)
                    .genesis_hash(genesis_hash)
                    .note(note)
                    .build()
                    .unwrap(),
            )
            .asset_id(185)
            .freeze_target(freeze_address)
            .frozen(false)
            .to_owned()
    }
}
