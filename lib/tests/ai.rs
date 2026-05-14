//! Tests for `proposal::ai` — `AiProvider`, `AiModel`, `AiProtocol`,
//! and the validation newtypes (`AiProviderName`, `AiModelId`).

use horizon_lib::error::Error;
use horizon_lib::name::NodeName;
use horizon_lib::proposal::{
    AiModel, AiModelId, AiProtocol, AiProvider, AiProviderName,
};
use nota_codec::{Decoder, NotaDecode};

#[test]
fn ai_provider_name_accepts_letters_digits_dashes() {
    let name = AiProviderName::try_new("criomos-local").unwrap();
    assert_eq!(name.as_str(), "criomos-local");
}

#[test]
fn ai_provider_name_rejects_empty() {
    let error = AiProviderName::try_new("").unwrap_err();
    assert!(matches!(error, Error::InvalidAiProviderName { .. }));
}

#[test]
fn ai_provider_name_rejects_dots() {
    // dots are reserved for model ids; provider names stay path-clean
    let error = AiProviderName::try_new("with.dot").unwrap_err();
    assert!(matches!(error, Error::InvalidAiProviderName { .. }));
}

#[test]
fn ai_model_id_accepts_dots_and_underscores() {
    // model ids commonly include version dots: qwen3.5-122b-a10b
    let id = AiModelId::try_new("qwen3.5-122b-a10b").unwrap();
    assert_eq!(id.as_str(), "qwen3.5-122b-a10b");

    // and underscores: gpt_oss_120b
    let id2 = AiModelId::try_new("gpt_oss_120b").unwrap();
    assert_eq!(id2.as_str(), "gpt_oss_120b");
}

#[test]
fn ai_model_id_rejects_spaces() {
    let error = AiModelId::try_new("has space").unwrap_err();
    assert!(matches!(error, Error::InvalidAiModelId { .. }));
}

#[test]
fn ai_model_id_rejects_slashes() {
    let error = AiModelId::try_new("a/b").unwrap_err();
    assert!(matches!(error, Error::InvalidAiModelId { .. }));
}

#[test]
fn ai_protocol_decodes_openai_compat() {
    let mut decoder = Decoder::new("OpenAiCompat");
    let protocol = AiProtocol::decode(&mut decoder).unwrap();
    assert!(matches!(protocol, AiProtocol::OpenAiCompat));
}

#[test]
fn ai_model_decodes_from_nota_record() {
    // Cloud-served model: `serving = None` (no fetcher / runtime
    // override / load-on-startup — the model is hosted elsewhere).
    let text = r#"(AiModel "qwen3.5-122b-a10b" "Qwen3.5 122B-A10B Q4_K_M" true 131072 4096 None)"#;
    let mut decoder = Decoder::new(text);
    let model = AiModel::decode(&mut decoder).unwrap();
    assert_eq!(model.id.as_str(), "qwen3.5-122b-a10b");
    assert_eq!(model.descriptor, "Qwen3.5 122B-A10B Q4_K_M");
    assert!(model.reasoning);
    assert_eq!(model.context_window, 131072);
    assert_eq!(model.max_tokens, 4096);
    assert!(model.serving.is_none());
}

#[test]
fn ai_provider_decodes_with_no_models_no_api_key_no_base_path() {
    let text = r#"(AiProvider "minimal" prometheus OpenAiCompat 11434 None [] None None)"#;
    let mut decoder = Decoder::new(text);
    let provider = AiProvider::decode(&mut decoder).unwrap();
    assert_eq!(provider.name.as_str(), "minimal");
    assert_eq!(provider.serving_node.as_str(), "prometheus");
    assert!(matches!(provider.protocol, AiProtocol::OpenAiCompat));
    assert_eq!(provider.port, 11434);
    assert!(provider.base_path.is_none());
    assert!(provider.models.is_empty());
    assert!(provider.api_key.is_none());
    assert!(provider.serving_config.is_none());
}

#[test]
fn ai_provider_decodes_with_models_and_base_path() {
    let text = r#"(AiProvider "criomos-local" prometheus OpenAiCompat 11434
                    "/v1"
                    [(AiModel "qwen3.5-122b-a10b" "Qwen3.5 122B-A10B" true 131072 4096 None)
                     (AiModel "gpt-oss-120b" "GPT-OSS 120B" false 131072 4096 None)]
                    None
                    None)"#;
    let mut decoder = Decoder::new(text);
    let provider = AiProvider::decode(&mut decoder).unwrap();
    assert_eq!(provider.name.as_str(), "criomos-local");
    assert_eq!(provider.base_path.as_deref(), Some("/v1"));
    assert_eq!(provider.models.len(), 2);
    assert_eq!(provider.models[0].id.as_str(), "qwen3.5-122b-a10b");
    assert!(provider.models[0].reasoning);
    assert_eq!(provider.models[1].id.as_str(), "gpt-oss-120b");
    assert!(!provider.models[1].reasoning);
}

#[test]
fn ai_provider_constructs_via_rust_literal() {
    let provider = AiProvider {
        name: AiProviderName::try_new("test").unwrap(),
        serving_node: NodeName::try_new("test-node").unwrap(),
        protocol: AiProtocol::OpenAiCompat,
        port: 11434,
        base_path: Some("/v1".to_string()),
        models: vec![AiModel {
            id: AiModelId::try_new("test-model").unwrap(),
            descriptor: "Test".to_string(),
            reasoning: false,
            context_window: 8192,
            max_tokens: 2048,
            serving: None,
        }],
        api_key: None,
        serving_config: None,
    };
    assert_eq!(provider.models.len(), 1);
}

#[test]
fn ai_provider_with_local_serving_decodes_from_nota() {
    use horizon_lib::proposal::{
        AiFit, AiModelFetchurl, AiModelServing, AiModelSource, AiServingConfig,
    };
    let text = r#"(AiProvider "criomos-local" prometheus OpenAiCompat 11434
                    "/v1"
                    [(AiModel "test-7b" "Test 7B" false 8192 2048
                       (AiModelServing
                         (AiModelFetchurl "https://example.com/test-7b.gguf"
                                          "sha256-deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
                                          "test-7b.gguf")
                         8192
                         true))]
                    None
                    (AiServingConfig 1 300 99 true true Off 1 None 110 100))"#;
    let mut decoder = nota_codec::Decoder::new(text);
    let provider = AiProvider::decode(&mut decoder).unwrap();
    assert_eq!(provider.models.len(), 1);
    let model = &provider.models[0];
    let serving = model.serving.as_ref().expect("serving present");
    assert_eq!(serving.runtime_context_size, 8192);
    assert!(serving.load_on_startup);
    match &serving.source {
        AiModelSource::AiModelFetchurl(f) => assert_eq!(f.filename, "test-7b.gguf"),
        other => panic!("expected AiModelFetchurl, got {other:?}"),
    }
    let serving_config = provider
        .serving_config
        .as_ref()
        .expect("serving_config present");
    assert_eq!(serving_config.max_loaded_models, 1);
    assert_eq!(serving_config.idle_unload_seconds, 300);
    assert!(matches!(serving_config.fit, AiFit::Off));
    assert_eq!(serving_config.gpu_override, None);
    assert_eq!(serving_config.memory_max_gb, 110);
    assert_eq!(serving_config.memory_high_gb, 100);
    let _: &AiServingConfig = serving_config; // type check
    let _: AiModelServing = serving.clone(); // type check
}
