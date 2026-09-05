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
