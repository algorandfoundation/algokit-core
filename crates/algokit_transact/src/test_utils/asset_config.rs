use crate::{AssetConfigTransactionBuilder, Byte32, KeyPairAccount, TransactionHeaderBuilder};
use base64::{prelude::BASE64_STANDARD, Engine};

pub struct AssetConfigTransactionMother {}

impl AssetConfigTransactionMother {
    pub fn asset_create() -> AssetConfigTransactionBuilder {
        // mainnet - NXAHS2NA46DJHIULXYPJV2NOJSKKFFNFFXRZP35TA5IDCZNE2MUA
        let sender = "KPVZ66IFE7KHQ6623XHTPVS3IL7BXBE3HXQG35J65CVDA54VLRPP4SVOU4"
            .parse::<KeyPairAccount>()
            .unwrap();
        let reserve = "YQTVEPKB4O5F26H76L5I7BA6VGCMRC6P2QSWRKG4KVJLJ62MVYTDJPM6KE"
            .parse::<KeyPairAccount>()
            .unwrap();
        let genesis_hash: Byte32 = BASE64_STANDARD
            .decode("wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=")
            .unwrap()
            .try_into()
            .unwrap();
        let note = BASE64_STANDARD.decode("eyJuYW1lIjoiRnJhY2N0YWwgVG9rZW4iLCJ1bml0TmFtZSI6IkZSQUNDIiwiZXh0ZXJuYWxfdXJsIjoid3d3LmZyYWNjdGFsbW9uc3RlcnNuZnQuY29tIiwiaW1hZ2VfbWltZXR5cGUiOiJpbWFnZS9wbmciLCJkZXNjcmlwdGlvbiI6IkZyYWNjdGFsIFRva2VucyBhcmUgaW4tZ2FtZSBjdXJyZW5jeSBmb3IgdGhlIEZyYWNjdGFsIE1vbnN0ZXJzIGdhbWUhIn0=").unwrap();

        AssetConfigTransactionBuilder::default()
            .header(
                TransactionHeaderBuilder::default()
                    .sender(sender.clone().address())
                    .fee(1000)
                    .first_valid(26594258)
                    .last_valid(26595258)
                    .genesis_hash(genesis_hash)
                    .genesis_id("mainnet-v1.0".to_string())
                    .note(note)
                    .build()
                    .unwrap(),
            )
            .asset_id(0) // Asset ID 0 for asset creation
            .total(10000000000)
            .decimals(0)
            .default_frozen(false)
            .asset_name("Fracctal Token".to_string())
            .unit_name("FRACC".to_string())
            .url("template-ipfs://{ipfscid:0:dag-pb:reserve:sha2-256}".to_string())
            .manager(sender.address().clone())
            .reserve(reserve.address().clone())
            .freeze(sender.address().clone())
            .clawback(sender.address().clone())
            .to_owned()
    }

    pub fn asset_destroy() -> AssetConfigTransactionBuilder {
        // mainnet - U4XH6AS5UUYQI4IZ3E5JSUEIU64Y3FGNYKLH26W4HRY7T6PK745A
        let sender = "MBX2M6J44LQ22L3FROYRBKUAG4FWENPSLPTI7EBR4ECQ2APDMI6XTENHWQ"
            .parse::<KeyPairAccount>()
            .unwrap();
        let genesis_hash: Byte32 = BASE64_STANDARD
            .decode("wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=")
            .unwrap()
            .try_into()
            .unwrap();
        let note = BASE64_STANDARD.decode("fSaN7lZKDoU=").unwrap();

        AssetConfigTransactionBuilder::default()
            .header(
                TransactionHeaderBuilder::default()
                    .sender(sender.address())
                    .fee(1000)
                    .first_valid(6354623)
                    .last_valid(6355623)
                    .genesis_hash(genesis_hash)
                    .note(note)
                    .build()
                    .unwrap(),
            )
            .asset_id(917559) // Asset ID to destroy
            .to_owned()
    }

    pub fn asset_reconfigure() -> AssetConfigTransactionBuilder {
        // mainnet - GAMRAG3KCG23U2HOELJF32OQAWAISLIFBB5RLDDDYHUSOZNYN7MQ
        let sender = "EHYQCYHUC6CIWZLBX5TDTLVJ4SSVE4RRTMKFDCG4Z4Q7QSQ2XWIQPMKBPU"
            .parse::<KeyPairAccount>()
            .unwrap();
        let manager = "EHYQCYHUC6CIWZLBX5TDTLVJ4SSVE4RRTMKFDCG4Z4Q7QSQ2XWIQPMKBPU"
            .parse::<KeyPairAccount>()
            .unwrap();
        let reserve = "POMY37RQ5PYG2NHKEFVDVDKGWZLZ4NHUWUW57CVGZVIPZCTNAFE2JM7XQU"
            .parse::<KeyPairAccount>()
            .unwrap();
        let genesis_hash: Byte32 = BASE64_STANDARD
            .decode("wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=")
            .unwrap()
            .try_into()
            .unwrap();
        let note = BASE64_STANDARD.decode("eyJzdGFuZGFyZCI6ImFyYzY5IiwiZGVzY3JpcHRpb24iOiJUaGlzIGlzIGEgdmVyaWZpYWJseSBhdXRoZW50aWMgZGlnaXRhbCBoaXN0b3JpY2FsIGFydGlmYWN0IG1pbnRlZCBieSBUaGUgRGF0YSBIaXN0b3J5IE11c2V1bS4gSXQgcmVwcmVzZW50cyBhIE1hZ25pdHVkZSA1LjMgZWFydGhxdWFrZSB3aXRoIElEIHVzNzAwMG05NzYgd2hpY2ggaGFzIGFuIGVwaWNlbnRyZSBub3J0aGVybiBFYXN0IFBhY2lmaWMgUmlzZSBhbmQgb2NjdXJyZWQgYXQgTW9uLCAwMSBBcHIgMjAyNCAxNDo0NToxNiBHTVQuIFRoZSB2ZXJpZmllZCBzb3VyY2Ugb2YgdGhpcyBkYXRhIGFydGlmYWN0IHdhcyB0aGUgVW5pdGVkIFN0YXRlcyBHZW9sb2dpY2FsIFN1cnZleSAoVVNHUykuIEZvciBtb3JlIGluZm9ybWF0aW9uIHZpc2l0IGh0dHBzOi8vZGF0YWhpc3Rvcnkub3JnLy4iLCJleHRlcm5hbF91cmwiOiJodHRwczovL211c2V1bS5kYXRhaGlzdG9yeS5vcmcvZXZlbnQvUVVBS0UvdXM3MDAwbTk3NiIsInByb3BlcnRpZXMiOnsibWFnbml0dWRlIjo1LjMsImNsYXNzIjoiTTUiLCJkZXB0aCI6MTAsImxhdGl0dWRlIjo4LjI1MSwibG9uZ2l0dWRlIjotMTAzLjIyNiwicGxhY2UiOiJub3J0aGVybiBFYXN0IFBhY2lmaWMgUmlzZSIsInNvdXJjZSI6IlVTR1MiLCJzdWJUeXBlIjoiZWFydGhxdWFrZSIsInRpbWUiOiIyMDI0LTA0LTAxVDE0OjQ1OjE2LjEwOVoiLCJ0eXBlIjoicXVha2UiLCJ1cmwiOiJodHRwczovL2VhcnRocXVha2UudXNncy5nb3YvZWFydGhxdWFrZXMvZXZlbnRwYWdlL3VzNzAwMG05NzYifSwibWltZV90eXBlIjoiaW1hZ2UvcG5nIiwiaWQiOiJ1czcwMDBtOTc2IiwidGl0bGUiOiJNIDUuMyAtIG5vcnRoZXJuIEVhc3QgUGFjaWZpYyBSaXNlIn0=").unwrap();

        AssetConfigTransactionBuilder::default()
            .header(
                TransactionHeaderBuilder::default()
                    .sender(sender.address())
                    // .auth(auth)
                    .fee(1000)
                    .first_valid(37544842)
                    .last_valid(37545842)
                    .genesis_hash(genesis_hash)
                    .genesis_id("mainnet-v1.0".to_string())
                    .note(note)
                    .build()
                    .unwrap(),
            )
            .asset_id(1715458296) // Asset ID to reconfigure
            .manager(manager.address())
            .reserve(reserve.address())
            .to_owned()
    }
}
