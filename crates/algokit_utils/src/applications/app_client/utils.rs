use super::AppClient;
use super::error_transformation::extract_logic_error_data;
use crate::AppClientError;
use crate::transactions::TransactionSenderError;
use std::str::FromStr;

fn contains_logic_error(s: &str) -> bool {
    extract_logic_error_data(s).is_some()
}

/// Transform transaction errors to include enhanced logic error details when applicable.
pub fn transform_transaction_error(
    client: &AppClient,
    err: TransactionSenderError,
    is_clear_state_program: bool,
) -> AppClientError {
    let err_str = err.to_string();
    if contains_logic_error(&err_str) {
        // Only transform errors that are for this app (when app_id is known)
        if client.app_id() != 0 {
            let app_tag = format!("app={}", client.app_id());
            if !err_str.contains(&app_tag) {
                return AppClientError::TransactionSenderError { source: err };
            }
        }
        let tx_err = crate::transactions::TransactionResultError::ParsingError {
            message: err_str.clone(),
        };
        let logic = client.expose_logic_error(&tx_err, is_clear_state_program);
        return AppClientError::LogicError {
            message: logic.message.clone(),
            logic: Box::new(logic),
        };
    }

    AppClientError::TransactionSenderError { source: err }
}

/// Parse optional account reference strings into Address objects.
pub fn parse_account_refs_to_addresses(
    account_refs: &Option<Vec<String>>,
) -> Result<Option<Vec<algokit_transact::Address>>, AppClientError> {
    match account_refs {
        None => Ok(None),
        Some(refs) => {
            let mut result = Vec::with_capacity(refs.len());
            for s in refs {
                result.push(
                    algokit_transact::Address::from_str(s)
                        .map_err(|e| AppClientError::TransactError { source: e })?,
                );
            }
            Ok(Some(result))
        }
    }
}
