use crate::{
    ApplicationCallTransactionBuilder, OnApplicationComplete, StateSchema,
    test_utils::{AccountMother, TransactionHeaderMother},
};
use base64::{Engine, prelude::BASE64_STANDARD};

pub struct ApplicationCallTransactionMother {}

impl ApplicationCallTransactionMother {
    pub fn application_create() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/L6B56N2BAXE43PUI7IDBXCJN5DEB6NLCH4AAN3ON64CXPSCTJNTA
        ApplicationCallTransactionBuilder::default()
        .header(TransactionHeaderMother::testnet()
            .sender(AccountMother::nfd_testnet().address())
            .first_valid(21038057)
            .last_valid(21039057)
            .note(
                BASE64_STANDARD
                    .decode("TkZEIFJlZ2lzdHJ5IENvbnRyYWN0")
                    .unwrap(),
            )
            .build()
            .unwrap())
        .app_id(0)
        .on_complete(OnApplicationComplete::NoOp)
        .approval_program(
            BASE64_STANDARD
            .decode("BiAKAQACCAQGEAoFAyYDBmkuYXBwcwhhZGRfYWRkcgAxFiQMQAKmMRYiCTUAI0ACmjEZIQgSQAKQMRkhBBJAAoIxGCMSQAJ5MRkkEkACcDEZIhJAAb8xGSMSQAABADYaAIADZ2FzEkABqTIEJA80ADgQIhIQNAA4IDIDEhA0ADgJMgMSEDQAOAgyAA80ADgBMgANERA0AIgCgSISEEAAAiNDMRskEjYaACkSEDEWIgk4BzEAEhBAAUsxGyQSNhoAgAtyZW1vdmVfYWRkchIQMRYiCTgHMQASEEABGTIEIQQPNhoAgARtaW50EhAxGyEEEhBAAAEAMRYhCQk4PTUBNAFyCDUENQMxFiIJiAINMRaIAggQFEAA2zEWJAmIAfwxFiQJOAgjEhBAAMI2GgEXNQIxFiQJOAcyChIxFiQJOAg0AhIQMRYiCTgHMgoSEDEWIgk4CIHAmgwSEDEYMggSEDEQIQUSEBRAAIGxIrIQMRYiCTgINAIIsgg0A7IHI7IBtiEFshAjshkxALIcNjAAsjA0AbIYNhoAsho2GgKyGjYaA7IaI7IBtiEFshAjshkxALIcNjAAsjA0AbIYgA5vZmZlcl9mb3Jfc2FsZbIaNhoBshoxFiEJCTkaA7IaI7IBs7gBOgA1yCJC/rYjQyM1AkL/PiNDMQAoIQY2GgEXiAJ1Qv6dMQAoIQY2GgEXiAIyQv6OIkMyBCQPNAA4ECISEDQAOCAyAxIQNAA4CTIDEhA0ADgIMgAPNAA4ATIADREQNACIANYiEhA0ADgHMQASEEAAAiNDMgQkDzEbIhIQNhoAgAZhc3NpZ24SEEAAJzEbJBI2GgApEhAxFiIJOAcxABIQQAABADEAKCEGNhoBF4gBsEL/vzEAgAdpLmFzYWlkMRYkCTvIZjEAgAdpLmFwcGlkMRYhCAk4PRZmIkL/lSJDIkMxADIJEkMjQyNDIzUAQv1aNQ6ACjAxMjM0NTY3ODk0DiJYiTUNNA0jEkAAKDQNIQcKIw1AAA0qNA0hBxiI/9FQQgAUNA0hBwo0DUyI/9VMNQ1C/+OAATCJNQU0BTgAgbirnShwADUHNQY0BiISNAU4CTIDEhA0BTggMgMSEIk1FzUWNRU0FTQWNBdjNRk1GDQZQAAEKkIAAjQYiTURNRA1DzQPMgg0EIj/1BUjEkAAcTQPNBBiNRIjNRM0EzQSFSUKDEAAGTQSFYF4DEAAAiOJNA80EDQSNBEWUGZCAE00EjQTJQtbNRQ0FCMSQAATNBQ0ERJAAAk0EyIINRNC/7siiTQPNBA0EiM0EyULUjQRFlA0EjQTJQslCDQSFVJQZiKJNA80EDQRFmYiiSKJNSE1IDUfNB80IGI1IiM1IzQjNCIVJQoMQQA1NCI0IyULWzQhEkAACTQjIgg1I0L/3zQfNCA0IiM0IyULUiMWUDQiNCMlCyUINCIVUlBmIokjiTULNQo1CTUIIzUMNAw0CgxBAB80CDQJNAyI/ohQNAuI/voiEkAACTQMIgg1DEL/2yKJI4k1HTUcNRs1GiM1HjQeNBwMQQAfNBo0GzQeiP5UUDQdiP9YIhJAAAk0HiIINR5C/9siiSOJ")
            .unwrap()
        )
        .clear_state_program(
            BASE64_STANDARD
            .decode("BoEBQw==")
            .unwrap()
        )
        .extra_program_pages(3)
        .local_state_schema(StateSchema {
            num_uints: 0,
            num_byte_slices: 16,
        })
        .to_owned()
    }

    pub fn application_update() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/NQVNJ5VWEDX42DMJQIQET4QPNUOW27EYIPKZ4SDWKOOEFJQB7PZA
        ApplicationCallTransactionBuilder::default()
        .header(TransactionHeaderMother::testnet()
            .sender(AccountMother::nfd_testnet().address())
            .first_valid(43679851)
            .last_valid(43679951)
            .note(
                BASE64_STANDARD
                    .decode("TkZEIFJlZ2lzdHJ5IENvbnRyYWN0OldTT1RSSlhMWUJRWVZNRkxPSUxZVlVTTklXS0JXVUJXM0dOVVdBRktHS0hOS05SWDY2TkVaSVRVTE0=")
                    .unwrap(),
            )
            .build()
            .unwrap())
        .app_id(84366825)
        .on_complete(OnApplicationComplete::UpdateApplication)
        .args(vec![
            BASE64_STANDARD.decode("dGVhbHNjcmlwdC1kdW1teQ==").unwrap(),
        ])
        .approval_program(
            BASE64_STANDARD
            .decode("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")
            .unwrap()
        )
        .clear_state_program(
            BASE64_STANDARD
            .decode("Cg==")
            .unwrap()
        )
        .to_owned()
    }

    pub fn application_call() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/6Y644M5SGTKNBH7ZX6D7QAAHDF6YL6FDJPRAGSUHNZLR4IKGVSPQ
        ApplicationCallTransactionBuilder::default()
        .header(
            TransactionHeaderMother::testnet()
                .sender("KVAGZI3WJI36TTTKJUI36ECGP3NHGR5VBJNIXG3DROHPGH2XFC36D4HENE".parse().unwrap())
                .first_valid(21038300)
                .last_valid(21039300)
                .note(BASE64_STANDARD.decode("AAAAAAAPQkA=").unwrap())
                .fee(5000)
                .group(BASE64_STANDARD.decode("ktxBY/2UFfqvhwKKxwihS9YhfG+of3hz2I3ErgNZZSo=").unwrap().try_into().unwrap())
                .build()
                .unwrap(),
        )
        .app_id(84366825)
        .on_complete(OnApplicationComplete::NoOp)
        .args(vec![
            BASE64_STANDARD.decode("bWludA==").unwrap(),
            BASE64_STANDARD.decode("AAAAAAAPQkA=").unwrap(),
            BASE64_STANDARD.decode("c2VjdXJpdGl6ZS5hbGdv").unwrap(),
            BASE64_STANDARD.decode("dGVtcGxhdGUtaXBmczovL3tpcGZzY2lkOjE6ZGFnLXBiOnJlc2VydmU6c2hhMi0yNTZ9L25mZC5qc29u").unwrap(),
        ])
        .account_references(vec![
            "KVAGZI3WJI36TTTKJUI36ECGP3NHGR5VBJNIXG3DROHPGH2XFC36D4HENE"
                .parse()
                .unwrap(),
            "KVAGZI3WJI36TTTKJUI36ECGP3NHGR5VBJNIXG3DROHPGH2XFC36D4HENE"
                .parse()
                .unwrap(),
        ])
        .asset_references(vec![84366776])
        .to_owned()
    }

    pub fn application_delete() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/mainnet/transaction/XVVC7UDLCPI622KCJZLWK3SEAWWVUEPEXUM5CO3DFLWOBH7NOPDQ
        ApplicationCallTransactionBuilder::default()
            .header(
                TransactionHeaderMother::mainnet()
                    .sender(
                        "H3OQEQIIC35RZTJNU5A75LT4PCTUCF3VKVEQTSXAJMUGNTRUKEKI4QSRW4"
                            .parse()
                            .unwrap(),
                    )
                    .first_valid(39723798)
                    .last_valid(39724798)
                    .build()
                    .unwrap(),
            )
            .app_id(1898586902)
            .on_complete(OnApplicationComplete::DeleteApplication)
            .account_references(vec![
                "MDIVKI64M2HEKCWKH7SOTUXEEW6KNOYSAOBTDTS32KUQOGUT75D43MSP5M"
                    .parse()
                    .unwrap(),
                "H3OQEQIIC35RZTJNU5A75LT4PCTUCF3VKVEQTSXAJMUGNTRUKEKI4QSRW4"
                    .parse()
                    .unwrap(),
                "MDIVKI64M2HEKCWKH7SOTUXEEW6KNOYSAOBTDTS32KUQOGUT75D43MSP5M"
                    .parse()
                    .unwrap(),
            ])
            .asset_references(vec![850924184])
            .to_owned()
    }

    pub fn application_opt_in() -> ApplicationCallTransactionBuilder {
        // https://lora.algokit.io/testnet/transaction/BNASGY47TXXUTFUZPDAGGPQKK54B4QPEEPDTJIZFDXC64WQH4GOQ
        ApplicationCallTransactionBuilder::default()
            .header(
                TransactionHeaderMother::testnet()
                    .sender(
                        "RAJ6J5E32CAU47LTXYQESPEGNTIE4AE652XZMU4V2AYBNTRVDPOF5DXOQM"
                            .parse()
                            .unwrap(),
                    )
                    .first_valid(21038233)
                    .last_valid(21039233)
                    .fee(1000)
                    .group(
                        BASE64_STANDARD
                            .decode("3T4cJx8PgfSOdwtONtjXVpkUjd46fNik93wUUZMszxY=")
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .app_id(84366825)
            .on_complete(OnApplicationComplete::OptIn)
            .args(vec![BASE64_STANDARD.decode("YXNzaWdu").unwrap()])
            .account_references(vec![
                "KVAGZI3WJI36TTTKJUI36ECGP3NHGR5VBJNIXG3DROHPGH2XFC36D4HENE"
                    .parse()
                    .unwrap(),
            ])
            .asset_references(vec![84366776])
            .to_owned()
    }

    pub fn application_call_example() -> ApplicationCallTransactionBuilder {
        ApplicationCallTransactionBuilder::default()
            .header(TransactionHeaderMother::example().build().unwrap())
            .app_id(12345)
            .on_complete(OnApplicationComplete::NoOp)
            .to_owned()
    }

    pub fn application_close_out() -> ApplicationCallTransactionBuilder {
        Self::application_call_example()
            .on_complete(OnApplicationComplete::CloseOut)
            .to_owned()
    }

    pub fn application_clear_state() -> ApplicationCallTransactionBuilder {
        Self::application_call_example()
            .on_complete(OnApplicationComplete::ClearState)
            .to_owned()
    }
}
