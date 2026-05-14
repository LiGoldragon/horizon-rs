//! Cluster-level AI provider records.
//!
//! Replaces the per-consumer scan that derives the inference endpoint
//! from `node.typeIs.largeAiRouter` plus the `serverPort` + `models[]`
//! literals in `CriomOS-lib/data/largeAI/llm.json`. The cluster now
//! authors which provider exists, where it serves, and what models it
//! advertises; CriomOS-home modules read this directly.
//!
//! Source: `~/primary/reports/system-specialist/119-horizon-data-needed-to-purge-criomos-literals.md` §8
//! and report 14 §8 row 6.

use nota_codec::{NotaEnum, NotaRecord, NotaSum, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::name::NodeName;
use crate::proposal::secret::SecretReference;

/// Identifier the operator gives to one AI provider entry. Distinct
/// from the model id; one provider may host many models. Validated
/// like other names: non-empty ASCII letters, digits, dashes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct AiProviderName(pub(crate) String);

impl AiProviderName {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::InvalidAiProviderName { got: s });
        }
        if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(Error::InvalidAiProviderName { got: s });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AiProviderName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AiProviderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// On-the-wire shape of a model identifier (e.g. `"qwen3.5-122b-a10b"`).
/// Closed character set so config files / paths can be derived
/// without quoting concerns.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct AiModelId(pub(crate) String);

impl AiModelId {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::InvalidAiModelId { got: s });
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_')
        {
            return Err(Error::InvalidAiModelId { got: s });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AiModelId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AiModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Wire protocol the provider speaks. Closed enum — adding a new
/// protocol is a coordinated workspace change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaEnum)]
pub enum AiProtocol {
    /// OpenAI-compatible completions / chat-completions API.
    /// llama.cpp's HTTP server, vllm, ollama with the OpenAI shim,
    /// and many cloud providers all speak this.
    OpenAiCompat,
}

/// One model the provider advertises. Consumer-facing metadata
/// (id, descriptor, reasoning flag, context window) lives at the
/// top level; server-side runtime config (fetcher source, runtime
/// override of context size, load-on-startup flag) lives in
/// `serving` when this model is locally served. Cloud-served models
/// (provider's `serving == None`) have `model.serving == None` too.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct AiModel {
    pub id: AiModelId,
    pub descriptor: String,
    /// Whether the model emits a `<think>...</think>` style reasoning
    /// block ahead of its answer. Consumers gate UI rendering on this.
    pub reasoning: bool,
    /// Maximum context window in tokens (the model's natural ceiling).
    pub context_window: u32,
    /// Maximum output tokens per single completion.
    pub max_tokens: u32,
    /// Server-side runtime config. `Some` when this model is
    /// self-served via the provider's `serving_config`; `None` when
    /// the provider is a cloud endpoint that hosts the model
    /// elsewhere.
    #[serde(default)]
    pub serving: Option<AiModelServing>,
}

/// Per-model runtime config consumed by the local serving stack
/// (today's llama.cpp router). Cloud-only models have no `serving`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct AiModelServing {
    pub source: AiModelSource,
    /// Runtime override of context size (often smaller than
    /// `context_window` to fit GPU memory).
    pub runtime_context_size: u32,
    /// Whether the router preloads this model on service start.
    pub load_on_startup: bool,
}

/// Where the model weights come from. Closed sum — operator-authored
/// fetcher targets feed `nix.fetchurl` (single file) or a multi-shard
/// runCommand (sharded GGUF). NotaSum convention: variant name equals
/// payload type name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum AiModelSource {
    AiModelFetchurl(AiModelFetchurl),
    AiModelMultiShard(AiModelMultiShard),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct AiModelFetchurl {
    pub url: String,
    pub sha256: String,
    pub filename: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct AiModelMultiShard {
    pub shards: Vec<AiModelShard>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct AiModelShard {
    pub url: String,
    pub sha256: String,
    pub filename: String,
}

/// One AI provider in the cluster. The operator authors one entry per
/// distinct endpoint (one per local llama.cpp server, one per cloud
/// account, etc.). `serving_node` names the node that hosts the
/// endpoint; consumers compose the URL as
/// `<protocol>://<servingNode.criomeDomainName>:<port><basePath?>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct AiProvider {
    pub name: AiProviderName,
    pub serving_node: NodeName,
    pub protocol: AiProtocol,
    pub port: u16,
    /// Optional URL suffix (e.g. `"/v1"` for OpenAI-compatible).
    /// `None` means no suffix; many endpoints serve the API at the
    /// host root.
    #[serde(default)]
    pub base_path: Option<String>,
    pub models: Vec<AiModel>,
    /// `None` for endpoints that need no key (the local llama.cpp
    /// router being the canonical case). `Some(reference)` for
    /// endpoints whose runtime credential lives in the cluster's
    /// secret backend.
    #[serde(default)]
    pub api_key: Option<SecretReference>,
    /// Server-side runtime config when this provider is hosted
    /// locally on `serving_node`. `None` for cloud providers (the
    /// model is hosted elsewhere; we just send requests).
    #[serde(default)]
    pub serving_config: Option<AiServingConfig>,
}

/// Per-provider runtime config for the local serving stack
/// (today: llama.cpp router). Cloud providers have no `serving_config`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct AiServingConfig {
    /// llama.cpp router `--models-max`: how many models to keep
    /// loaded simultaneously. `1` means LRU-evict on every swap.
    pub max_loaded_models: u32,
    /// llama.cpp router `--sleep-idle-seconds`: unload weights
    /// after this many idle seconds.
    pub idle_unload_seconds: u32,
    /// llama.cpp `n-gpu-layers`: number of transformer layers
    /// offloaded to GPU. `99` means "all".
    pub gpu_layers: u32,
    pub no_mmap: bool,
    pub no_warmup: bool,
    pub fit: AiFit,
    pub parallel: u32,
    /// Optional `HSA_OVERRIDE_GFX_VERSION` value for the systemd unit
    /// (AMD ROCm runtimes that need to spoof a supported GFX revision —
    /// e.g. RDNA3 reporting `11.5.1`). `None` means no override.
    #[serde(default)]
    pub gpu_override: Option<String>,
    /// systemd `MemoryMax=` for the serving unit, in GiB. Hard cap to
    /// prevent OOM from killing system services (hostapd, SSH).
    pub memory_max_gb: u32,
    /// systemd `MemoryHigh=` for the serving unit, in GiB. Soft cap
    /// (throttling threshold) — must be < `memory_max_gb`.
    pub memory_high_gb: u32,
}

/// llama.cpp router `--fit` setting: whether to slot models into
/// KV-cache budget automatically. `Off` is conservative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaEnum)]
pub enum AiFit {
    Off,
    On,
}
