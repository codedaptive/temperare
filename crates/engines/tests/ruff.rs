use std::ffi::OsStr;
use std::fs;

use engines::{availability_in, entry_cmd, ruff::{parse_ruff, RUFF}, Availability};
use rules::{Engine, RelPath};

#[test]
fn ruff_reports_e711_but_not_other_rules_or_clean_lines() {
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-ruff", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    assert_eq!(availability_in(Engine::Ruff, &folder), Availability::Available);
    fs::write(folder.join("example.py"), "import os\nvalue = None\nif value == None:\n    pass\nif value is None:\n    pass\n").unwrap();
    let result = procio::run(&entry_cmd(&RUFF, OsStr::new("E711"), &[], None, &folder))
        .map(|out| parse_ruff(&out, "E711", &folder, &folder));
    let cache_exists = folder.join(".ruff_cache").exists();
    fs::remove_dir_all(&folder).unwrap();
    assert!(!cache_exists, "ruff left a .ruff_cache directory");
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("example.py"));
    assert_eq!(hits[0].line, Some(3));
    assert_eq!(hits[0].col, Some(13));
    assert!(hits[0].what.contains("None"), "{:?}", hits[0]);
}

#[test]
fn d1_ruff_prefix_finds_both_real_diagnostics() {
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-ruff-prefix", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    assert_eq!(availability_in(Engine::Ruff, &folder), Availability::Available);
    fs::write(folder.join("example.py"), "value = None\nif value == None: pass\n").unwrap();
    let result = procio::run(&entry_cmd(&RUFF, OsStr::new("E7"), &[], None, &folder))
        .map(|out| parse_ruff(&out, "E7", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 2, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("example.py"));
    assert_eq!(hits[0].line, Some(2));
    assert_eq!(hits[0].col, Some(13));
    assert_eq!(hits[0].what, "Comparison to `None` should be `cond is None`");
    assert_eq!(hits[1].path, RelPath::new("example.py"));
    assert_eq!(hits[1].line, Some(2));
    assert_eq!(hits[1].col, Some(17));
    assert_eq!(hits[1].what, "Multiple statements on one line (colon)");
}
