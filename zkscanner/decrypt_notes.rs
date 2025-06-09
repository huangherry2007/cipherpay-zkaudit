use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime, TimeZone};

// TODO: Add a way to get the view key from the note
// TODO: Add a way to get the note id from the note
// TODO: Add a way to get the note amount from the note
// TODO: Add a way to get the note recipient from the note
// TODO: Add a way to get the note memo from the note
// TODO: Add a way to get the note expires_at from the note
// TODO: Add a way to get the note created_at from the note
// TODO: Add a way to get the note updated_at from the note

// TODO: Add a way to get the note from the note
// TODO: Connect to a real logging backend (e.g., syslog, cloud logging, or blockchain event logs)

pub struct EncryptedNote {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub metadata: String, // e.g., note id or tag
    pub expires_at: Option<i64>, // Unix timestamp (optional)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotePayload {
    pub amount: u64,
    pub recipient: String,
    pub memo: Option<String>,
}

pub struct DecryptedNote {
    pub plaintext: Vec<u8>,
    pub metadata: String,
    pub parsed: Option<NotePayload>,
    pub expired: bool,
    pub audit_trail: Vec<AuditEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEvent {
    pub timestamp: i64,
    pub action: String,
    pub details: String,
}

pub struct Wallet {
    pub view_keys: Vec<[u8; 32]>,
    pub notes: Vec<EncryptedNote>,
}

/// Trait for external logging integration
pub trait ExternalLogger {
    fn log_event(&self, event: &AuditEvent);
}

/// Scan all notes in a wallet, using all available view keys. Optionally filter by metadata substring.
pub fn scan_wallet_notes_with_logger<L: ExternalLogger>(wallet: &Wallet, filter: Option<&str>, now: i64, logger: Option<&L>) -> Vec<DecryptedNote> {
    let mut results = Vec::new();
    for key in &wallet.view_keys {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        for note in &wallet.notes {
            if let Some(f) = filter {
                if !note.metadata.contains(f) {
                    continue;
                }
            }
            let nonce = Nonce::from_slice(&note.nonce);
            if let Ok(plaintext) = cipher.decrypt(nonce, note.ciphertext.as_ref()) {
                // Multi-format parsing: try JSON, then bincode
                let parsed = serde_json::from_slice::<NotePayload>(&plaintext)
                    .or_else(|_| bincode::deserialize::<NotePayload>(&plaintext).ok())
                    .ok();
                // Expiration check
                let expired = note.expires_at.map_or(false, |ts| ts < now);
                // Advanced audit trail
                let mut audit_trail = vec![AuditEvent {
                    timestamp: now,
                    action: "decryption_attempt".to_string(),
                    details: format!("meta={} expired={} parsed={:?}", note.metadata, expired, parsed),
                }];
                if expired {
                    audit_trail.push(AuditEvent {
                        timestamp: now,
                        action: "note_expired".to_string(),
                        details: note.metadata.clone(),
                    });
                }
                if let Some(logger) = logger {
                    for event in &audit_trail {
                        logger.log_event(event);
                    }
                }
                results.push(DecryptedNote {
                    plaintext,
                    metadata: note.metadata.clone(),
                    parsed,
                    expired,
                    audit_trail,
                });
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_gcm::aead::Aead;
    use rand::RngCore;
    use chrono::Utc;
    use std::sync::{Arc, Mutex};

    struct TestLogger {
        pub events: Arc<Mutex<Vec<AuditEvent>>>,
    }
    impl ExternalLogger for TestLogger {
        fn log_event(&self, event: &AuditEvent) {
            self.events.lock().unwrap().push(event.clone());
        }
    }

    #[test]
    fn test_multi_format_parsing_and_external_logging() {
        let view_key = [42u8; 32];
        let key = Key::from_slice(&view_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = rand::random::<[u8; 12]>();
        let payload = NotePayload {
            amount: 123,
            recipient: "bob".to_string(),
            memo: Some("hello".to_string()),
        };
        // Serialize as bincode
        let plaintext = bincode::serialize(&payload).unwrap();
        let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), plaintext.as_ref()).unwrap();
        let now = Utc::now().timestamp();
        let notes = vec![EncryptedNote {
            ciphertext: ciphertext.clone(),
            nonce,
            metadata: "note1:payment".to_string(),
            expires_at: Some(now - 10), // already expired
        }];
        let wallet = Wallet {
            view_keys: vec![view_key],
            notes,
        };
        let logger = TestLogger { events: Arc::new(Mutex::new(vec![])) };
        let found = scan_wallet_notes_with_logger(&wallet, Some("payment"), now, Some(&logger));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].parsed, Some(payload));
        assert!(found[0].expired);
        // Check audit trail and logger
        let events = logger.events.lock().unwrap();
        assert!(events.iter().any(|e| e.action == "decryption_attempt"));
        assert!(events.iter().any(|e| e.action == "note_expired"));
    }
}
