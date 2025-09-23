use super::types::LogicError;
use super::{AppClient, AppSourceMaps};
use crate::AlgorandClient;
use crate::transactions::TransactionResultError;
use algokit_abi::{Arc56Contract, arc56_contract::PcOffsetMethod};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LogicErrorData {
    pub transaction_id: String,
    pub message: String,
    pub pc: u64,
}

lazy_static! {
    static ref LOGIC_ERROR_RE: Regex = Regex::new(
        r".*transaction (?P<transaction_id>[A-Z2-7]{52}): logic eval error: (?P<message>.*)\. Details: .*pc=(?P<pc>[0-9]+).*"
    )
    .unwrap();
    static ref INNER_LOGIC_ERROR_RE: Regex =
        Regex::new(r"inner tx (\d+) failed:.*?pc=([0-9]+)").unwrap();
}

pub(crate) fn extract_logic_error_data(error_str: &str) -> Option<LogicErrorData> {
    let caps = LOGIC_ERROR_RE.captures(error_str)?;
    let pc = if let Some(inner) = INNER_LOGIC_ERROR_RE.captures(error_str) {
        inner
            .get(2)
            .and_then(|m| m.as_str().parse::<u64>().ok())
            .unwrap_or_else(|| caps["pc"].parse::<u64>().unwrap_or(0))
    } else {
        caps["pc"].parse::<u64>().unwrap_or(0)
    };
    Some(LogicErrorData {
        transaction_id: caps["transaction_id"].to_string(),
        message: caps["message"].to_string(),
        pc,
    })
}

pub(crate) struct LogicErrorContext<'logic_error_ctx> {
    pub app_id: u64,
    pub app_spec: &'logic_error_ctx Arc56Contract,
    pub algorand: &'logic_error_ctx AlgorandClient,
    pub source_maps: Option<&'logic_error_ctx AppSourceMaps>,
}

impl AppClient {
    /// Import compiled source maps for approval and clear programs.
    pub fn import_source_maps(&mut self, source_maps: AppSourceMaps) {
        self.source_maps = Some(source_maps);
    }

    /// Export compiled source maps if available.
    pub fn export_source_maps(&self) -> Option<AppSourceMaps> {
        self.source_maps.clone()
    }
}

impl LogicErrorContext<'_> {
    /// Create an enhanced LogicError from a transaction error, applying source maps if available.
    pub(crate) fn expose_logic_error(
        &self,
        error: &TransactionResultError,
        is_clear_state_program: bool,
    ) -> LogicError {
        let err_str = format!("{}", error);
        let parsed_logic_error_data = extract_logic_error_data(&err_str);
        let (mut line_no_opt, mut listing) =
            self.apply_source_map_for_message(&err_str, is_clear_state_program);
        let source_map = self.get_source_map(is_clear_state_program).cloned();
        let transaction_id = Self::extract_transaction_id(&err_str);
        let pc_opt = Self::extract_pc(&err_str);

        let mut logic = LogicError {
            message: err_str.clone(),
            program: None,
            source_map,
            transaction_id,
            pc: pc_opt,
            line_no: line_no_opt,
            lines: if listing.is_empty() {
                None
            } else {
                Some(listing.clone())
            },
            traces: None,
            logic_error_str: Some(err_str.clone()),
        };

        let (tx_id, parsed_pc, msg_msg) = if let Some(p) = parsed_logic_error_data {
            (
                Some(p.transaction_id.clone()),
                Some(p.pc),
                Some(p.message.clone()),
            )
        } else {
            (logic.transaction_id.clone(), logic.pc, None)
        };

        let mut arc56_error_message: Option<String> = None;
        let mut arc56_line_no: Option<u64> = None;
        let mut arc56_listing: Vec<String> = Vec::new();

        if let Some(si_model) = self.app_spec.source_info.as_ref() {
            let program_source_info = if is_clear_state_program {
                &si_model.clear
            } else {
                &si_model.approval
            };

            let mut arc56_pc = parsed_pc.unwrap_or(0);

            if matches!(
                program_source_info.pc_offset_method,
                PcOffsetMethod::Cblocks
            ) {
                // Apply CBLOCKS offset only if compiled program bytes are available via cache
                if let Some(bytes) = self.get_program_bytes(is_clear_state_program) {
                    let offset = Self::get_constant_block_offset(&bytes);
                    arc56_pc = arc56_pc.saturating_sub(offset as u64);
                }
            }

            if arc56_pc > 0 {
                if let Some(source_info) = program_source_info
                    .source_info
                    .iter()
                    .find(|s| s.pc.iter().any(|v| *v as u64 == arc56_pc))
                {
                    if let Some(em) = &source_info.error_message {
                        arc56_error_message = Some(em.clone());
                    }
                    if arc56_line_no.is_none() {
                        if let Some(teal_line) = source_info.teal {
                            arc56_line_no = Some(teal_line as u64);
                        }
                    }
                }
            }

            if arc56_line_no.is_some()
                && self.app_spec.source.is_some()
                && self.get_source_map(is_clear_state_program).is_none()
            {
                if let Some(teal_src) = self.decode_teal(is_clear_state_program) {
                    let center = arc56_line_no.unwrap();
                    arc56_listing = Self::annotated_teal_snippet(&teal_src, center, 3);
                }
            }
        }

        if line_no_opt.is_none() && arc56_line_no.is_some() {
            line_no_opt = arc56_line_no;
            logic.line_no = line_no_opt;
        }
        if listing.is_empty() && !arc56_listing.is_empty() {
            listing = arc56_listing;
            logic.lines = Some(listing.clone());
        }

        if let Some(emsg) = arc56_error_message.or(msg_msg) {
            let app_id_from_msg = Self::extract_app_id(&err_str);
            let app_id = app_id_from_msg
                .or_else(|| Some(self.app_id.to_string()))
                .unwrap_or_else(|| "N/A".to_string());
            let txid_str = tx_id.unwrap_or_else(|| "N/A".to_string());
            let runtime_msg = format!(
                "Runtime error when executing {} (appId: {}) in transaction {}: {}",
                self.app_spec.name, app_id, txid_str, emsg
            );
            logic.message = runtime_msg.clone();
        }

        logic
    }

    /// Extract transaction id from an error string.
    fn extract_transaction_id(error_str: &str) -> Option<String> {
        let re = regex::Regex::new(r"transaction ([A-Z2-7]{52})").unwrap();
        re.captures(error_str)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Compute line and listing using a source map when available.
    fn apply_source_map_for_message(
        &self,
        error_str: &str,
        is_clear_state_program: bool,
    ) -> (Option<u64>, Vec<String>) {
        let pc_opt = Self::extract_pc(error_str);
        if let Some(pc) = pc_opt {
            if let Some((line_no, listing)) = self.apply_source_map(pc, is_clear_state_program) {
                return (Some(line_no), listing);
            }
        }
        (None, Vec::new())
    }

    /// Extract program counter from an error string.
    fn extract_pc(s: &str) -> Option<u64> {
        for token in s.split(|c: char| c.is_whitespace() || c == ',' || c == ';') {
            if let Some(idx) = token.find('=') {
                let (k, v) = token.split_at(idx);
                if k.ends_with("pc") {
                    if let Ok(parsed_logic_error_data) = v.trim_start_matches('=').parse::<u64>() {
                        return Some(parsed_logic_error_data);
                    }
                }
            }
        }
        None
    }

    /// Map pc to TEAL line and extract a short snippet.
    fn apply_source_map(
        &self,
        pc: u64,
        is_clear_state_program: bool,
    ) -> Option<(u64, Vec<String>)> {
        let map = self.get_source_map(is_clear_state_program)?;
        let line_no = Self::map_pc_to_line(map, pc)?;
        let listing = Self::truncate_listing(map, line_no, 3);
        Some((line_no, listing))
    }

    /// Get the selected program's source map.
    fn get_source_map(&self, is_clear_state_program: bool) -> Option<&JsonValue> {
        let maps = self.source_maps?;
        if is_clear_state_program {
            maps.clear_source_map.as_ref()
        } else {
            maps.approval_source_map.as_ref()
        }
    }

    /// Map a program counter to a source line using the pc array.
    fn map_pc_to_line(map: &JsonValue, pc: u64) -> Option<u64> {
        let pcs = map.get("pc")?.as_array()?;
        let mut best_line: Option<u64> = None;
        for (i, entry) in pcs.iter().enumerate() {
            if let Some(pc_val) = entry.as_u64() {
                if pc_val == pc {
                    return Some(i as u64 + 1);
                }
                if pc_val < pc {
                    best_line = Some(i as u64 + 1);
                }
            }
        }
        best_line
    }

    /// Format a numbered snippet around a source line from a source map.
    fn truncate_listing(map: &JsonValue, center_line: u64, context: usize) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        if let Some(source) = map.get("source").and_then(|s| s.as_str()) {
            let src_lines: Vec<&str> = source.lines().collect();
            let total = src_lines.len();
            let center = center_line.saturating_sub(1) as usize;
            let start = center.saturating_sub(context);
            let end = (center + context + 1).min(total);
            for (i, line) in src_lines.iter().enumerate().take(end).skip(start) {
                lines.push(format!("{:>4} | {}", i + 1, line));
            }
        }
        lines
    }

    /// Format a numbered snippet around a source line from raw TEAL.
    fn truncate_teal_source(source: &str, center_line: u64, context: usize) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        let src_lines: Vec<&str> = source.lines().collect();
        let total = src_lines.len();
        if total == 0 {
            return lines;
        }
        let center = center_line.saturating_sub(1) as usize;
        let start = center.saturating_sub(context);
        let end = (center + context + 1).min(total);
        for (i, line) in src_lines.iter().enumerate().take(end).skip(start) {
            lines.push(format!("{:>4} | {}", i + 1, line));
        }
        lines
    }

    /// Like truncate_teal_source but adds a subtle error marker on the center line.
    fn annotated_teal_snippet(source: &str, center_line: u64, context: usize) -> Vec<String> {
        let mut lines = Self::truncate_teal_source(source, center_line, context);
        // Try to mark the line that equals center_line if present
        let needle = format!("{:>4} |", center_line);
        let mut marked = false;
        for entry in &mut lines {
            if entry.starts_with(&needle) {
                *entry = format!("{}\t<-- Error", entry);
                marked = true;
                break;
            }
        }
        // Fallback: mark middle line if exact match not found
        if !marked && !lines.is_empty() {
            let mid = lines.len() / 2;
            lines[mid] = format!("{}\t<-- Error", lines[mid]);
        }
        lines
    }

    /// Calculate the offset after initial constant blocks in a TEAL program (CBLOCKS).
    fn get_constant_block_offset(program: &[u8]) -> usize {
        const BYTE_CBLOCK: u8 = 38; // bytecblock
        const INT_CBLOCK: u8 = 32; // intcblock
        if program.is_empty() {
            return 0;
        }
        let mut i = 1; // skip version byte
        let len = program.len();
        let mut bytec_off: Option<usize> = None;
        let mut intc_off: Option<usize> = None;

        while i < len {
            let op = program[i];
            i += 1;
            if op != BYTE_CBLOCK && op != INT_CBLOCK {
                break;
            }
            if i >= len {
                break;
            }
            let values_remaining = program[i] as usize;
            i += 1;
            for _ in 0..values_remaining {
                if op == BYTE_CBLOCK {
                    if i >= len {
                        break;
                    }
                    let elem_len = program[i] as usize;
                    i += 1 + elem_len.min(len.saturating_sub(i));
                } else {
                    while i < len {
                        let b = program[i];
                        i += 1;
                        if (b & 0x80) == 0 {
                            break;
                        }
                    }
                }
            }
            let off = i;
            if op == BYTE_CBLOCK {
                bytec_off = Some(off)
            } else {
                intc_off = Some(off)
            }

            if i >= len {
                break;
            }
            let next = program[i];
            if next != BYTE_CBLOCK && next != INT_CBLOCK {
                break;
            }
        }
        bytec_off.or(intc_off).unwrap_or(0)
    }

    /// Try to get compiled program bytes for the app from the compilation cache.
    /// This avoids async calls; returns None if not available.
    fn get_program_bytes(&self, is_clear_state_program: bool) -> Option<Vec<u8>> {
        let teal_src = self.decode_teal(is_clear_state_program)?;
        self.algorand
            .app()
            .get_compilation_result(&teal_src)
            .map(|c| c.compiled_base64_to_bytes)
    }

    /// Decode base64 TEAL source from the app spec.
    fn decode_teal(&self, is_clear_state_program: bool) -> Option<String> {
        let src = self.app_spec.source.as_ref()?;
        if is_clear_state_program {
            src.get_decoded_clear().ok()
        } else {
            src.get_decoded_approval().ok()
        }
    }

    /// Extract app id from an error string.
    fn extract_app_id(error_str: &str) -> Option<String> {
        let re = regex::Regex::new(r"app=(\d+)").ok()?;
        re.captures(error_str)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }
}

impl AppClient {
    /// Create an enhanced LogicError from a transaction error, applying source maps if available.
    pub fn expose_logic_error(
        &self,
        error: &TransactionResultError,
        is_clear_state_program: bool,
    ) -> LogicError {
        let context = LogicErrorContext {
            app_id: self.app_id(),
            app_spec: self.app_spec(),
            algorand: self.algorand(),
            source_maps: self.source_maps.as_ref(),
        };

        context.expose_logic_error(error, is_clear_state_program)
    }
}
