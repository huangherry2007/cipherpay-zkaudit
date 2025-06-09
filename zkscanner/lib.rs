mod decrypt_notes;

pub use decrypt_notes::{EncryptedNote, DecryptedNote, NotePayload, AuditEvent, Wallet, ExternalLogger, scan_wallet_notes_with_logger};

/// Unified API for scanning and decrypting wallet notes with optional filtering and external logging.
///
/// # Arguments
/// * `wallet` - The wallet containing view keys and encrypted notes.
/// * `filter` - Optional substring to filter notes by metadata.
/// * `now` - Current Unix timestamp for expiration checks.
/// * `logger` - Optional external logger for audit events.
///
/// # Returns
/// A vector of decrypted notes with parsed payloads, expiration status, and audit trails.
#[inline]
pub fn scan_notes_with_audit<'a, L: ExternalLogger>(
    wallet: &'a Wallet,
    filter: Option<&str>,
    now: i64,
    logger: Option<&L>,
) -> Vec<DecryptedNote> {
    scan_wallet_notes_with_logger(wallet, filter, now, logger)
} 