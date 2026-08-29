//! Hardware description.

use datomic::{Datomic, DatomicString, Fault, FaultProblem, PortionBuilding, PortionViewing};
use protos::{Portion, StructuralEnclosure};
use serde::{Deserialize, Serialize};

use crate::name::{ModelName, NodeName, UserName};
use crate::species::{Arch, MachineSpecies, MotherBoard};

/// Per-node hardware description. `arch` is `Option` because pod
/// (virtual) machines defer it to their super-node; resolution into
/// a concrete arch happens at projection time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    pub species: MachineSpecies,
    pub arch: Option<Arch>,
    pub cores: u32,
    pub model: Option<ModelName>,
    pub mother_board: Option<MotherBoard>,
    /// Pod-only: which node hosts this pod.
    pub super_node: Option<NodeName>,
    /// Pod-only: which user runs this pod.
    pub super_user: Option<UserName>,
    /// Intel iGPU graphics generation (e.g. 8 = Broadwell, 11 = Skylake,
    /// 12 = Tiger Lake / Alder Lake / Meteor Lake Xe-LPG, 13 = Lunar
    /// Lake Xe2). Gates the modern Intel media stack: `>= 12` enables
    /// `vpl-gpu-rt` for AV1/HEVC HW decode. None for non-Intel or
    /// unknown — modules fall back to the safe default driver.
    ///
    /// `#[serde(default)]` applies to serde/JSON decoding only. The
    /// Legacy Datomic remains count-strict while proposals still use it.
    /// The direct Datomic record below has the same 12-field positional
    /// contract: adding, removing, or reordering a field changes the
    /// machine data schema and requires every stored datum to move with it.
    #[serde(default)]
    pub chip_gen: Option<u32>,
    /// Total system RAM in gibibytes (rounded). Gates downstream
    /// resource policies: `nix.settings.maxJobs` thresholds,
    /// llama.cpp model size, language-server heap. Optional — None
    /// means the operator hasn't filled it in yet.
    ///
    /// `#[serde(default)]` is serde/JSON-only; the legacy Datomic and
    /// direct Datomic records remain positional and count-strict.
    #[serde(default)]
    pub ram_gb: Option<u32>,
    /// Virtual disk size in gibibytes for a Pod (VM) node. None for a
    /// Metal node (disk comes from the partition layout in `Io`) or a
    /// Pod whose host pre-provisions the disk. Cluster-authored: a VM's
    /// root disk is allocated at create time and is not derivable from
    /// anything else.
    ///
    /// `#[serde(default)]` is serde/JSON-only; the legacy Datomic and
    /// direct Datomic records remain positional and count-strict.
    #[serde(default)]
    pub disk_gb: Option<u32>,
    /// Physical placement of this machine — a free site/datacenter/rack
    /// label (e.g. `home-lab`, `hetzner-fsn1`). Cluster-authored,
    /// variable, and non-derivable. Optional; None means unspecified.
    /// For a Pod this MAY later resolve to the host's location at
    /// projection time.
    ///
    /// `#[serde(default)]` is serde/JSON-only; the legacy Datomic and
    /// direct Datomic records remain positional and count-strict.
    #[serde(default)]
    pub location: Option<Location>,
    /// Additional hosts permitted to hold and exchange this Pod's image,
    /// beyond the primary `super_node`. Empty (the default, and the
    /// majority) means the single-host case — the declared host-set is
    /// exactly `{super_node}`. Non-empty extends the image-distribution
    /// trust boundary to `{super_node} ∪ super_nodes`. Pod-only;
    /// cluster-authored; FIXED in the declaration. `super_node` stays the
    /// primary/canonical host (arch resolution, the guest-fold discovery
    /// predicate, the single-host majority all read it).
    ///
    /// `#[serde(default)]` is serde/JSON-only. A machine record missing
    /// this field is invalid in both legacy Datomic and direct Datomic;
    /// tail position does not make it append-safe.
    #[serde(default)]
    pub super_nodes: Vec<NodeName>,
}

impl Machine {
    /// The declared host-set: the primary `super_node` first, then the
    /// additional `super_nodes`, deduped (primary order preserved). This
    /// is the set of vmhosts permitted to hold and exchange this node's
    /// image. For the single-host majority (`super_nodes` empty) it is
    /// exactly `{super_node}`. Empty only for a machine with no
    /// `super_node` at all (a non-Pod, or a Pod missing its host — which
    /// projection rejects upstream).
    pub fn host_set(&self) -> Vec<&NodeName> {
        let mut hosts: Vec<&NodeName> = Vec::new();
        for host in self.super_node.iter().chain(self.super_nodes.iter()) {
            if !hosts.contains(&host) {
                hosts.push(host);
            }
        }
        hosts
    }
}

impl Datomic for Machine {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let Some(parts) = portion.structural(StructuralEnclosure::Braced) else {
            return Err(portion.fault(FaultProblem::Shape));
        };
        let [
            species,
            arch,
            cores,
            model,
            mother_board,
            super_node,
            super_user,
            chip_gen,
            ram_gb,
            disk_gb,
            location,
            super_nodes,
        ] = parts
        else {
            return Err(portion.fault(FaultProblem::Arity));
        };

        Ok(Self {
            species: MachineSpecies::embody(species)?,
            arch: Option::<Arch>::embody(arch)?,
            cores: u32_from_portion(cores)?,
            model: Option::<ModelName>::embody(model)?,
            mother_board: Option::<MotherBoard>::embody(mother_board)?,
            super_node: Option::<NodeName>::embody(super_node)?,
            super_user: Option::<UserName>::embody(super_user)?,
            chip_gen: optional_u32_from_portion(chip_gen)?,
            ram_gb: optional_u32_from_portion(ram_gb)?,
            disk_gb: optional_u32_from_portion(disk_gb)?,
            location: Option::<Location>::embody(location)?,
            super_nodes: Vec::<NodeName>::embody(super_nodes)?,
        })
    }

    fn portion(&self) -> Portion {
        PortionBuilding::structural(
            "",
            StructuralEnclosure::Braced,
            vec![
                self.species.portion(),
                self.arch.portion(),
                i64::from(self.cores).portion(),
                self.model.portion(),
                self.mother_board.portion(),
                self.super_node.portion(),
                self.super_user.portion(),
                optional_u32_portion(self.chip_gen),
                optional_u32_portion(self.ram_gb),
                optional_u32_portion(self.disk_gb),
                self.location.portion(),
                self.super_nodes.portion(),
            ],
        )
    }
}

fn u32_from_portion(portion: &Portion) -> std::result::Result<u32, Fault> {
    u32::try_from(i64::embody(portion)?).map_err(|_| portion.fault(FaultProblem::Value))
}

fn optional_u32_from_portion(portion: &Portion) -> std::result::Result<Option<u32>, Fault> {
    Option::<i64>::embody(portion)?
        .map(|value| u32::try_from(value).map_err(|_| portion.fault(FaultProblem::Value)))
        .transpose()
}

fn optional_u32_portion(value: Option<u32>) -> Portion {
    value.map(i64::from).portion()
}

/// Physical placement label for a machine — a free site, datacenter,
/// or rack name (`home-lab`, `hetzner-fsn1`). A transparent newtype
/// over `String` so the wire form is a bare string while the type
/// stays distinct from any other string-shaped value (mirrors
/// `ModelName`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Location(pub(crate) String);

impl Location {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Datomic for Location {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        Ok(Self(DatomicString::embody(portion)?.as_ref().to_owned()))
    }

    fn portion(&self) -> Portion {
        let value = DatomicString::try_from(self.0.clone())
            .expect("machine location must be Datomic-representable");
        Datomic::portion(&value)
    }
}
