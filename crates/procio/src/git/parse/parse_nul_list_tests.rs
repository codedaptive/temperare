use super::parse_nul_list;

#[test]
fn empty_input_has_no_entries() {
    assert_eq!(parse_nul_list(b""), Vec::<Vec<u8>>::new());
}

#[test]
fn entries_ending_with_nul() {
    assert_eq!(parse_nul_list(b"a.rs\0dir/b c.py\0"), vec![b"a.rs".to_vec(), b"dir/b c.py".to_vec()]);
}

#[test]
fn last_entry_without_nul_is_kept() {
    assert_eq!(parse_nul_list(b"a.rs\0b.rs"), vec![b"a.rs".to_vec(), b"b.rs".to_vec()]);
}

#[test]
fn empty_entries_are_dropped() {
    assert_eq!(parse_nul_list(b"\0a\0\0b\0"), vec![b"a".to_vec(), b"b".to_vec()]);
}

#[test]
fn non_utf8_bytes_are_kept_exactly() {
    assert_eq!(parse_nul_list(b"x\xFF\x80\0caf\xC3\xA9\0"), vec![vec![b'x', 0xFF, 0x80], vec![b'c', b'a', b'f', 0xC3, 0xA9]]);
}
