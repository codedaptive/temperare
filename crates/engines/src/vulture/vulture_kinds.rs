//! `VULTURE_KINDS`: each vulture code with the message starts vulture prints for it.

/// Each code vulture 2.16 gives a finding (its `core.py` ERROR_CODES, `reachability.py` for V201), with the start of
/// every message it prints for that code. V107 covers unused arguments too, which vulture reports as variables. The
/// one table both `vulture_refusal` (the codes a rule may name) and `parse_vulture` (a message's code) read.
pub(crate) const VULTURE_KINDS: &[(&str, &[&str])] = &[
    ("V101", &["unused attribute '"]),
    ("V102", &["unused class '"]),
    ("V103", &["unused function '"]),
    ("V104", &["unused import '"]),
    ("V105", &["unused method '"]),
    ("V106", &["unused property '"]),
    ("V107", &["unused variable '"]),
    (
        "V201",
        &[
            "unreachable code after '",
            "unsatisfiable 'if' condition",
            "unreachable 'else' block",
            "redundant if-condition",
            "unsatisfiable 'ternary' condition",
            "unreachable 'else' expression",
            "unsatisfiable 'while' condition",
        ],
    ),
];
