use hhm_pub_lib_core::validate_json;

#[test]
fn accepts_shared_valid_cases() {
    for (model, input) in [
        (
            "ClientInfo",
            include_str!("../../../conformance/cases/valid-client-info.json"),
        ),
        (
            "PublicLocation",
            include_str!("../../../conformance/cases/valid-location.json"),
        ),
        (
            "PublicAccountContext",
            include_str!("../../../conformance/cases/valid-account.json"),
        ),
    ] {
        assert_eq!(validate_json(model, input), Ok(()));
    }
}

#[test]
fn rejects_shared_invalid_cases() {
    for (model, input) in [
        (
            "ClientInfo",
            include_str!("../../../conformance/cases/invalid-extra-field.json"),
        ),
        (
            "ClientInfo",
            include_str!("../../../conformance/cases/invalid-platform.json"),
        ),
        (
            "ClientInfo",
            include_str!("../../../conformance/cases/invalid-oversized-version.json"),
        ),
        (
            "PublicAccountContext",
            include_str!("../../../conformance/cases/invalid-uuid.json"),
        ),
        (
            "IdempotencyKey",
            include_str!("../../../conformance/cases/invalid-int64.json"),
        ),
        (
            "ClientInfo",
            include_str!("../../../conformance/cases/invalid-prototype-key.json"),
        ),
    ] {
        assert!(validate_json(model, input).is_err());
    }
}

#[test]
fn unknown_models_fail_closed() {
    assert_eq!(
        validate_json("InternalGrant", "{}").unwrap_err().code,
        "unknown_model"
    );
}

#[test]
fn every_manifest_case_is_exercised() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../../../conformance/manifest.json")).unwrap();
    assert_eq!(manifest["schemaVersion"], "hhm.public-conformance.v1");
    let cases = manifest["cases"].as_array().expect("fixture case array");
    assert!(!cases.is_empty(), "fixture manifest must not be empty");
    let mut names = std::collections::BTreeSet::new();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/cases");
    for case in cases {
        let file = case["file"].as_str().expect("fixture filename");
        let model = case["model"].as_str().expect("fixture model");
        let valid = case["valid"].as_bool().expect("fixture expected result");
        assert!(names.insert(file), "duplicate fixture: {file}");
        assert!(
            [
                "ClientInfo",
                "IdempotencyKey",
                "PublicLocation",
                "PublicAccountContext"
            ]
            .contains(&model),
            "unknown fixture model: {model}"
        );
        assert!(
            file.ends_with(".json") && !file.contains(['/', '\\']),
            "fixture must be a JSON basename"
        );
        let input = std::fs::read_to_string(root.join(file)).expect("read fixture");
        assert_eq!(
            validate_json(model, &input).is_ok(),
            valid,
            "fixture: {file}"
        );
    }
}
