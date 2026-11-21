//! `LooseningKind`: which way a change to a rule file loosened it.

use serde::Serialize;

/// What a `Loosening` reports; its serialized names are the report's names. `about` is a rule id for every kind but
/// the two set kinds, where it is the set's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum LooseningKind {
    /// A rule in force at the base is in neither `rules` nor `not_applicable_rules` now (removed, turned off, or
    /// retired by a newer set). `before` and `after` are None.
    #[serde(rename = "rule_gone")]
    RuleGone,
    /// A rule in force at the base is not applicable now. `after` holds now's reason; `before` is None.
    #[serde(rename = "rule_not_applicable")]
    RuleNotApplicable,
    /// A rule's severity is lower now. `before` and `after` hold the severity names ("error", "warning").
    #[serde(rename = "severity_lowered")]
    SeverityLowered,
    /// A pattern of the base's `paths` is missing from now's non-empty `paths`. `before` holds the pattern;
    /// `after` is None.
    #[serde(rename = "path_removed")]
    PathRemoved,
    /// The base's `paths` is empty (every file) and now's is not. `before` is None; `after` holds now's
    /// patterns joined with ", ".
    #[serde(rename = "paths_limited")]
    PathsLimited,
    /// A pattern of now's `exclude` is missing from the base's. `before` is None; `after` holds the pattern.
    #[serde(rename = "exclude_added")]
    ExcludeAdded,
    /// A kit of the base's `kits` is missing from now's non-empty `kits`. `before` holds the kit; `after` is None.
    #[serde(rename = "kit_removed")]
    KitRemoved,
    /// The base's `kits` is empty (every kit) and now's is not. `before` is None; `after` holds now's kits joined
    /// with ", ".
    #[serde(rename = "kits_limited")]
    KitsLimited,
    /// A kit of now's `outside_kits` is missing from the base's. `before` is None; `after` holds the kit.
    #[serde(rename = "outside_kit_added")]
    OutsideKitAdded,
    /// An exception now that no base exception covers; `about` is its rule id. `before` is None; `after` holds its
    /// path, with its `what` in parentheses when it has one.
    #[serde(rename = "exception_added")]
    ExceptionAdded,
    /// An exception now that base exceptions cover, whose `expires` is later than the base's (the latest among the
    /// covering ones, all dated); `about` is its rule id. `before` and `after` hold the base's and now's dates as
    /// "YYYY-MM-DD".
    #[serde(rename = "expiry_later")]
    ExpiryLater,
    /// An exception now with no `expires` where the base's covering exceptions all had one; `about` is its rule id.
    /// `before` holds the base's date (the latest among them) as "YYYY-MM-DD"; `after` is None.
    #[serde(rename = "expiry_dropped")]
    ExpiryDropped,
    /// A baseline set taken at the base is not taken now. `before` holds the base's version; `after` is None.
    #[serde(rename = "set_dropped")]
    SetDropped,
    /// A baseline set is taken at a lower version now. `before` and `after` hold the base's and now's versions.
    #[serde(rename = "set_older")]
    SetOlder,
}
