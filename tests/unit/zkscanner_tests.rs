use zkscanner::{Note, scan_notes, NoteError};

#[test]
fn test_scan_notes_basic() {
    let notes = vec![
        Note { id: 1, view_key: "vk1".to_string(), data: "note1".to_string() },
        Note { id: 2, view_key: "vk2".to_string(), data: "note2".to_string() },
        Note { id: 3, view_key: "vk1".to_string(), data: "note3".to_string() },
    ];
    let found = scan_notes(&notes, "vk1").unwrap();
    assert_eq!(found.len(), 2);
    assert_eq!(found[0].data, "note1");
    assert_eq!(found[1].data, "note3");
}

#[test]
fn test_scan_notes_empty() {
    let notes: Vec<Note> = vec![];
    let found = scan_notes(&notes, "vk1").unwrap();
    assert_eq!(found.len(), 0);
}

#[test]
fn test_scan_notes_no_matches() {
    let notes = vec![
        Note { id: 1, view_key: "vk2".to_string(), data: "note1".to_string() },
        Note { id: 2, view_key: "vk3".to_string(), data: "note2".to_string() },
    ];
    let found = scan_notes(&notes, "vk1").unwrap();
    assert_eq!(found.len(), 0);
}

#[test]
fn test_scan_notes_invalid_view_key() {
    let notes = vec![
        Note { id: 1, view_key: "vk1".to_string(), data: "note1".to_string() },
    ];
    assert!(matches!(scan_notes(&notes, ""), Err(NoteError::InvalidViewKey)));
}

#[test]
fn test_scan_notes_duplicate_ids() {
    let notes = vec![
        Note { id: 1, view_key: "vk1".to_string(), data: "note1".to_string() },
        Note { id: 1, view_key: "vk1".to_string(), data: "note2".to_string() },
    ];
    assert!(matches!(scan_notes(&notes, "vk1"), Err(NoteError::DuplicateNoteId)));
} 