//! Tests for cluster-level AI provider selections.

use horizon_lib::error::Error;
use horizon_lib::name::NodeName;
use horizon_lib::proposal::{
    AiProvider, AiProviderName, AiProviderProfile, SecretPurpose, SecretReference,
};
use nota_codec::{Decoder, NotaDecode};

#[test]
fn ai_provider_name_accepts_dash_name() {
    let name = AiProviderName::try_new("criomos-local").unwrap();
    assert_eq!(name.as_str(), "criomos-local");
}

#[test]
fn ai_provider_name_rejects_empty() {
    let error = AiProviderName::try_new("").unwrap_err();
    assert!(matches!(error, Error::InvalidAiProviderName { .. }));
}

#[test]
fn ai_provider_name_rejects_dot() {
    let error = AiProviderName::try_new("with.dot").unwrap_err();
    assert!(matches!(error, Error::InvalidAiProviderName { .. }));
}

#[test]
fn ai_provider_selection_decodes_minimal_record() {
    let text = r#"(AiProvider "criomos-local" prometheus CriomosLocalLlama None)"#;
    let mut decoder = Decoder::new(text);
    let provider = AiProvider::decode(&mut decoder).unwrap();

    assert_eq!(provider.name.as_str(), "criomos-local");
    assert_eq!(provider.serving_node.as_str(), "prometheus");
    assert!(matches!(provider.profile, AiProviderProfile::CriomosLocalLlama));
    assert!(provider.api_key.is_none());
}

#[test]
fn ai_provider_selection_can_reference_secret_api_key() {
    let provider = AiProvider {
        name: AiProviderName::try_new("cloud").unwrap(),
        serving_node: NodeName::try_new("prometheus").unwrap(),
        profile: AiProviderProfile::CriomosLocalLlama,
        api_key: Some(SecretReference {
            name: "cloud-ai-key".parse().unwrap(),
            purpose: SecretPurpose::AiProviderApiKey,
        }),
    };

    assert_eq!(provider.name.as_str(), "cloud");
    assert_eq!(
        provider.api_key.as_ref().unwrap().purpose,
        SecretPurpose::AiProviderApiKey
    );
}
