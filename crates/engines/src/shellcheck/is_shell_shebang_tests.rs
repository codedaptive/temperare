//! Unit tests for is_shell_shebang.rs on literal heads: each shell form kept, other shells and non-shebangs left out, and
//! the 256-byte bound on the first line.

use super::is_shell_shebang;

/// `#!/bin/sh` padded with spaces to `len` bytes.
fn padded(len: usize) -> Vec<u8> {
    let mut line = b"#!/bin/sh".to_vec();
    line.resize(len, b' ');
    line
}

#[test]
fn the_shell_forms_are_shell_shebangs() {
    let heads: [&[u8]; 13] = [
        b"#!/usr/bin/env -u NAME bash\n",
        b"#!/usr/bin/env -C /tmp sh\n",
        b"#!/usr/bin/env -P /bin ksh\n",
        b"#!/usr/bin/env -i bash\n",
        b"#!/bin/sh\n",
        b"#!/bin/bash\n",
        b"#!/usr/bin/env bash\n",
        b"#!/bin/dash\n",
        b"#!/bin/ksh\n",
        b"#! /bin/sh\n",
        b"#!/usr/bin/env -S bash -e\n",
        b"#!/bin/bash\r\n",
        b"#!/bin/sh\techo",
    ];
    for head in heads {
        assert!(is_shell_shebang(head), "{:?}", String::from_utf8_lossy(head));
    }
}

#[test]
fn other_shells_and_other_heads_are_not() {
    let heads: [&[u8]; 10] = [
        b"#!/usr/bin/env -u bash\n",
        b"#!/usr/bin/env -u NAME zsh\n",
        b"#!/bin/zsh\n",
        b"#!/usr/bin/fish\n",
        b"#!/usr/bin/env zsh\n",
        b"#!/usr/bin/env\n",
        b"#!/bin/shell\n",
        b"echo hi\n",
        b"",
        b"\x7fELF\x02\x01\x01\0\0\0\0\0\0\0\0\0\x03\0>\0",
    ];
    for head in heads {
        assert!(!is_shell_shebang(head), "{:?}", String::from_utf8_lossy(head));
    }
}

#[test]
fn only_the_first_line_counts() {
    assert!(!is_shell_shebang(b"echo hi\n#!/bin/sh\n"));
    assert!(is_shell_shebang(b"#!/bin/sh\necho done"));
}

#[test]
fn a_first_line_of_256_bytes_counts() {
    let head = [padded(256), b"\n".to_vec()].concat();
    assert_eq!(head.len(), 257);
    assert!(is_shell_shebang(&head));
}

#[test]
fn a_first_line_of_257_bytes_does_not() {
    let head = [padded(257), b"\n".to_vec()].concat();
    assert!(!is_shell_shebang(&head));
}

#[test]
fn a_head_of_257_bytes_with_no_newline_does_not() {
    assert!(!is_shell_shebang(&padded(257)));
}

#[test]
fn a_head_of_256_bytes_with_no_newline_counts() {
    assert!(is_shell_shebang(&padded(256)));
}
