//! Unit test for typed_why.rs: a check that picks any rule needing type resolution, by its name or through its set, gets a
//! why naming every such rule in list order; a check that picks none gets None. The style rules are written out from
//! detekt_typed_rules.txt.

use super::typed_why;

#[test]
fn typed_why_cases() {
    let rows: [(&str, Option<&str>); 7] = [
        (
            "Deprecation",
            Some(
                "detekt needs type resolution for Deprecation, which needs a classpath that cctao does not give detekt, so \
                 they would report nothing",
            ),
        ),
        (
            "MagicNumber,UnsafeCast,Deprecation",
            Some(
                "detekt needs type resolution for Deprecation, UnsafeCast, which needs a classpath that cctao does not \
                 give detekt, so they would report nothing",
            ),
        ),
        (
            "style",
            Some(
                "detekt needs type resolution for CanBeNonNullable, ExplicitCollectionElementAccessMethod, \
                 ForbiddenAnnotation, ForbiddenMethodCall, ForbiddenVoid, MaxChainedCallsOnSameLine, \
                 MultilineLambdaItParameter, NullableBooleanCheck, ObjectLiteralToLambda, PreferToOverPairSyntax, \
                 RedundantExplicitType, RedundantHigherOrderMapUsage, UnnecessaryAbstractClass, UnnecessaryApply, \
                 UnnecessaryBracesAroundTrailingLambda, UnnecessaryFilter, UnnecessaryInnerClass, UnnecessaryLet, \
                 UseAnyOrNoneInsteadOfFind, UseCheckNotNull, UseEmptyCounterpart, UseIfEmptyOrIfBlank, \
                 UseIsNullOrEmpty, UseOrEmpty, UseRequireNotNull, UseSumOfInsteadOfFlatMapSize, UselessCallOnNotNull, \
                 VarCouldBeVal, which needs a classpath that cctao does not give detekt, so they would report nothing",
            ),
        ),
        ("empty-blocks", None),
        ("comments", None),
        ("MagicNumber", None),
        ("MagicNumber,empty-blocks", None),
    ];
    for (check, expected) in rows {
        assert_eq!(typed_why(check).as_deref(), expected, "{check}");
    }
}
