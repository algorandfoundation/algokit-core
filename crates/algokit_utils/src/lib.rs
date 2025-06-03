use algokit_transact::AlgorandMsgpack;
use algokit_transact::Transaction;

pub struct Composer {
    transactions: Vec<Transaction>,
}

impl Composer {
    pub fn new() -> Self {
        Composer {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        if self.transactions.len() >= 16 {
            return Err("Cannot add more than 16 transactions to a Composer".to_string());
        }
        self.transactions.push(transaction);
        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<Vec<u8>>, String> {
        self.transactions
            .iter()
            .map(|tx| tx.encode())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TransactionMother;

    #[test]
    fn test_add_transaction() {
        let mut composer = Composer::new();
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_ok());
    }

    #[test]
    fn test_add_too_many_transactions() {
        let mut composer = Composer::new();
        for _ in 0..16 {
            let txn = TransactionMother::simple_payment().build().unwrap();
            assert!(composer.add_transaction(txn).is_ok());
        }
        let txn = TransactionMother::simple_payment().build().unwrap();
        assert!(composer.add_transaction(txn).is_err());
    }

    #[test]
    fn test_encode_transactions() {
        let mut composer = Composer::new();
        for _ in 0..5 {
            let txn = TransactionMother::simple_payment().build().unwrap();
            assert!(composer.add_transaction(txn).is_ok());
        }
        let encoded = composer.encode();
        assert!(encoded.is_ok());
        assert_eq!(encoded.unwrap().len(), 5);
    }
}
