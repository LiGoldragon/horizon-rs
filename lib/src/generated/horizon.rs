#![allow(dead_code)]
pub type NodeName = protos::Text;
pub type ClusterName = protos::Text;
pub type UserName = protos::Text;
pub type DomainName = protos::Text;
pub type ModelName = protos::Text;
pub type Location = protos::Text;
pub type Interface = protos::Text;
pub type WirelessNetworkName = protos::Text;
pub type SecretName = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecretReference(pub SecretName);
impl datom_codec::Datomic for SecretReference {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: SecretName = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SecretReference {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type GithubId = protos::Text;
pub type Keygrip = protos::Text;
pub type DevicePath = protos::Text;
pub type MountPath = protos::Text;
pub type SshPubKey = protos::Text;
pub type NixPubKey = protos::Text;
pub type WireguardPubKey = protos::Text;
pub type YggPubKey = protos::Text;
pub type YggAddress = protos::Text;
pub type YggSubnet = protos::Text;
pub type LinkLocalIp = protos::Text;
pub type NodeIp = protos::Text;
pub type TapSubnet = protos::Text;
pub type SiteSource = protos::Text;
pub type ServedDomain = protos::Text;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserRole {
    Code,
    Multimedia,
    Unlimited,
}
impl datom_codec::Datomic for UserRole {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Code" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Code)
            }
            "Multimedia" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Multimedia)
            }
            "Unlimited" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Unlimited)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for UserRole {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Code => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Code").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Multimedia => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Multimedia")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Unlimited => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Unlimited").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    Arm64,
}
impl datom_codec::Datomic for Architecture {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "X86_64" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::X86_64)
            }
            "Arm64" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Arm64)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Architecture {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::X86_64 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("X86_64").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Arm64 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Arm64").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Magnitude {
    Zero,
    Min,
    Medium,
    Large,
    Max,
}
impl datom_codec::Datomic for Magnitude {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Zero" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Zero)
            }
            "Min" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Min)
            }
            "Medium" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Medium)
            }
            "Large" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Large)
            }
            "Max" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Max)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Magnitude {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Zero => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Zero").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Min => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Min").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Medium => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Medium").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Large => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Large").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Max => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Max").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyboard {
    Qwerty,
    Colemak,
}
impl datom_codec::Datomic for Keyboard {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Qwerty" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Qwerty)
            }
            "Colemak" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Colemak)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Keyboard {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Qwerty => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Qwerty").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Colemak => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Colemak").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Vim,
    Emacs,
}
impl datom_codec::Datomic for Style {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Vim" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Vim)
            }
            "Emacs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Emacs)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Style {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Vim => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Vim").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Emacs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Emacs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Editor {
    Codium,
    Emacs,
}
impl datom_codec::Datomic for Editor {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Codium" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Codium)
            }
            "Emacs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Emacs)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Editor {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Codium => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Codium").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Emacs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Emacs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}
impl datom_codec::Datomic for TextSize {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "ExtraSmall" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ExtraSmall)
            }
            "Small" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Small)
            }
            "Medium" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Medium)
            }
            "Large" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Large)
            }
            "ExtraLarge" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ExtraLarge)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for TextSize {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::ExtraSmall => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ExtraSmall")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Small => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Small").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Medium => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Medium").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Large => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Large").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::ExtraLarge => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ExtraLarge")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootloader {
    Uefi,
    Mbr,
    Uboot,
}
impl datom_codec::Datomic for Bootloader {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Uefi" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Uefi)
            }
            "Mbr" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Mbr)
            }
            "Uboot" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Uboot)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Bootloader {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Uefi => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Uefi").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Mbr => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Mbr").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Uboot => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Uboot").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotherBoard {
    Ondyfaind,
}
impl datom_codec::Datomic for MotherBoard {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Ondyfaind" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Ondyfaind)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for MotherBoard {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Ondyfaind => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Ondyfaind").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainProvider {
    Cloudflare,
}
impl datom_codec::Datomic for DomainProvider {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Cloudflare" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Cloudflare)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DomainProvider {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Cloudflare => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Cloudflare")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WlanBand {
    TwoG,
    FiveG,
    SixG,
}
impl datom_codec::Datomic for WlanBand {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "TwoG" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::TwoG)
            }
            "FiveG" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::FiveG)
            }
            "SixG" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SixG)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for WlanBand {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::TwoG => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("TwoG").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::FiveG => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("FiveG").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::SixG => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SixG").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WlanStandard {
    Wifi4,
    Wifi6,
    Wifi7,
}
impl datom_codec::Datomic for WlanStandard {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Wifi4" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Wifi4)
            }
            "Wifi6" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Wifi6)
            }
            "Wifi7" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Wifi7)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for WlanStandard {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Wifi4 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Wifi4").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Wifi6 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Wifi6").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Wifi7 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Wifi7").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KvmAvailability {
    Available,
    Absent,
}
impl datom_codec::Datomic for KvmAvailability {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Available" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Available)
            }
            "Absent" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Absent)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for KvmAvailability {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Available => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Available").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Absent => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Absent").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SiteRenderer {
    MarkdownStatic,
}
impl datom_codec::Datomic for SiteRenderer {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "MarkdownStatic" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::MarkdownStatic)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for SiteRenderer {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::MarkdownStatic => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("MarkdownStatic")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsType {
    Ext2,
    Ext3,
    Ext4,
    Btrfs,
    Xfs,
    Zfs,
    F2fs,
    Bcachefs,
    Vfat,
    Exfat,
    Ntfs,
    Tmpfs,
}
impl datom_codec::Datomic for FsType {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Ext2" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Ext2)
            }
            "Ext3" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Ext3)
            }
            "Ext4" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Ext4)
            }
            "Btrfs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Btrfs)
            }
            "Xfs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Xfs)
            }
            "Zfs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Zfs)
            }
            "F2fs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::F2fs)
            }
            "Bcachefs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Bcachefs)
            }
            "Vfat" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Vfat)
            }
            "Exfat" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Exfat)
            }
            "Ntfs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Ntfs)
            }
            "Tmpfs" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Tmpfs)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for FsType {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Ext2 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Ext2").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Ext3 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Ext3").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Ext4 => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Ext4").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Btrfs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Btrfs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Xfs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Xfs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Zfs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Zfs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::F2fs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("F2fs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Bcachefs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Bcachefs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Vfat => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Vfat").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Exfat => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Exfat").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Ntfs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Ntfs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Tmpfs => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Tmpfs").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hardware(
    pub protos::Integer,
    pub std::option::Option<ModelName>,
    pub std::option::Option<MotherBoard>,
    pub std::option::Option<protos::Integer>,
    pub std::option::Option<protos::Integer>,
    pub std::option::Option<Location>,
);
impl datom_codec::Datomic for Hardware {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 6)?;
        let p0: protos::Integer = datom_codec::Positional::position(&mut p)?;
        let p1: std::option::Option<ModelName> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p2: std::option::Option<MotherBoard> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p3: std::option::Option<protos::Integer> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p4: std::option::Option<protos::Integer> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p5: std::option::Option<Location> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Hardware {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VirtualMachineHost {
    Cluster(
        NodeName,
        std::vec::Vec<NodeName>,
        std::option::Option<UserName>,
        std::option::Option<Architecture>,
    ),
    External(protos::Text, Architecture),
}
impl datom_codec::Datomic for VirtualMachineHost {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Cluster" => {
                let mut p = datom_codec::Headed::positions(v, 4)?;
                let p0: NodeName = datom_codec::Positional::position(&mut p)?;
                let p1: std::vec::Vec<NodeName> = datom_codec::Positional::position(
                    &mut p,
                )?;
                let p2: std::option::Option<UserName> = datom_codec::Positional::position(
                    &mut p,
                )?;
                let p3: std::option::Option<Architecture> = datom_codec::Positional::position(
                    &mut p,
                )?;
                std::result::Result::Ok(Self::Cluster(p0, p1, p2, p3))
            }
            "External" => {
                let mut p = datom_codec::Headed::positions(v, 2)?;
                let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
                let p1: Architecture = datom_codec::Positional::position(&mut p)?;
                std::result::Result::Ok(Self::External(p0, p1))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for VirtualMachineHost {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Cluster(p0, p1, p2, p3) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Cluster").expect("static variant"),
                            std::boxed::Box::new(
                                datom_codec::Datom::Struct(
                                    vec![
                                        protos::Conceivable::conceive(p0)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p1)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p2)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p3)
                                        .expect("infallible datom ascent").1
                                    ],
                                ),
                            ),
                        )
                    }
                    Self::External(p0, p1) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("External")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                datom_codec::Datom::Struct(
                                    vec![
                                        protos::Conceivable::conceive(p0)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p1)
                                        .expect("infallible datom ascent").1
                                    ],
                                ),
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MachineDefinition {
    Metal(Architecture, Hardware),
    VirtualMachine(VirtualMachineHost, Hardware, std::option::Option<protos::Integer>),
}
impl datom_codec::Datomic for MachineDefinition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Metal" => {
                let mut p = datom_codec::Headed::positions(v, 2)?;
                let p0: Architecture = datom_codec::Positional::position(&mut p)?;
                let p1: Hardware = datom_codec::Positional::position(&mut p)?;
                std::result::Result::Ok(Self::Metal(p0, p1))
            }
            "VirtualMachine" => {
                let mut p = datom_codec::Headed::positions(v, 3)?;
                let p0: VirtualMachineHost = datom_codec::Positional::position(&mut p)?;
                let p1: Hardware = datom_codec::Positional::position(&mut p)?;
                let p2: std::option::Option<protos::Integer> = datom_codec::Positional::position(
                    &mut p,
                )?;
                std::result::Result::Ok(Self::VirtualMachine(p0, p1, p2))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for MachineDefinition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Metal(p0, p1) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Metal").expect("static variant"),
                            std::boxed::Box::new(
                                datom_codec::Datom::Struct(
                                    vec![
                                        protos::Conceivable::conceive(p0)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p1)
                                        .expect("infallible datom ascent").1
                                    ],
                                ),
                            ),
                        )
                    }
                    Self::VirtualMachine(p0, p1, p2) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("VirtualMachine")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                datom_codec::Datom::Struct(
                                    vec![
                                        protos::Conceivable::conceive(p0)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p1)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p2)
                                        .expect("infallible datom ascent").1
                                    ],
                                ),
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiskLayout(
    pub DevicePath,
    pub MountPath,
    pub FsType,
    pub std::vec::Vec<protos::Text>,
);
impl datom_codec::Datomic for DiskLayout {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 4)?;
        let p0: DevicePath = datom_codec::Positional::position(&mut p)?;
        let p1: MountPath = datom_codec::Positional::position(&mut p)?;
        let p2: FsType = datom_codec::Positional::position(&mut p)?;
        let p3: std::vec::Vec<protos::Text> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DiskLayout {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapDevice(pub DevicePath, pub std::option::Option<protos::Integer>);
impl datom_codec::Datomic for SwapDevice {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: DevicePath = datom_codec::Positional::position(&mut p)?;
        let p1: std::option::Option<protos::Integer> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SwapDevice {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompressedSwap(pub protos::Integer);
impl datom_codec::Datomic for CompressedSwap {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: protos::Integer = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for CompressedSwap {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeEnvironment(pub Keyboard, pub std::option::Option<CompressedSwap>);
impl datom_codec::Datomic for NodeEnvironment {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: Keyboard = datom_codec::Positional::position(&mut p)?;
        let p1: std::option::Option<CompressedSwap> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeEnvironment {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Installation(
    pub Bootloader,
    pub std::vec::Vec<DiskLayout>,
    pub std::vec::Vec<SwapDevice>,
);
impl datom_codec::Datomic for Installation {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: Bootloader = datom_codec::Positional::position(&mut p)?;
        let p1: std::vec::Vec<DiskLayout> = datom_codec::Positional::position(&mut p)?;
        let p2: std::vec::Vec<SwapDevice> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Installation {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveDefinition();
impl datom_codec::Datomic for LiveDefinition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 0)?;
        std::result::Result::Ok(Self())
    }
}
impl protos::Conceivable<datom_codec::Datom> for LiveDefinition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(vec![]),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeVariant {
    Live(LiveDefinition),
    Installation(Installation),
}
impl datom_codec::Datomic for NodeVariant {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Live" => {
                std::result::Result::Ok(Self::Live(datom_codec::Carrying::body(v)?))
            }
            "Installation" => {
                std::result::Result::Ok(
                    Self::Installation(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeVariant {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Live(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Live").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Installation(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Installation")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct YggdrasilKey(pub YggPubKey, pub YggAddress, pub YggSubnet);
impl datom_codec::Datomic for YggdrasilKey {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: YggPubKey = datom_codec::Positional::position(&mut p)?;
        let p1: YggAddress = datom_codec::Positional::position(&mut p)?;
        let p2: YggSubnet = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for YggdrasilKey {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeKeys(
    pub SshPubKey,
    pub std::option::Option<NixPubKey>,
    pub std::option::Option<YggdrasilKey>,
);
impl datom_codec::Datomic for NodeKeys {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: SshPubKey = datom_codec::Positional::position(&mut p)?;
        let p1: std::option::Option<NixPubKey> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p2: std::option::Option<YggdrasilKey> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeKeys {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WireguardProxy(pub WireguardPubKey, pub protos::Text, pub NodeIp);
impl datom_codec::Datomic for WireguardProxy {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: WireguardPubKey = datom_codec::Positional::position(&mut p)?;
        let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p2: NodeIp = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for WireguardProxy {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackupWireless(
    pub Interface,
    pub WirelessNetworkName,
    pub WlanBand,
    pub protos::Integer,
    pub WlanStandard,
    pub SecretReference,
);
impl datom_codec::Datomic for BackupWireless {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 6)?;
        let p0: Interface = datom_codec::Positional::position(&mut p)?;
        let p1: WirelessNetworkName = datom_codec::Positional::position(&mut p)?;
        let p2: WlanBand = datom_codec::Positional::position(&mut p)?;
        let p3: protos::Integer = datom_codec::Positional::position(&mut p)?;
        let p4: WlanStandard = datom_codec::Positional::position(&mut p)?;
        let p5: SecretReference = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5))
    }
}
impl protos::Conceivable<datom_codec::Datom> for BackupWireless {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouterInterfaces(
    pub Interface,
    pub Interface,
    pub WlanBand,
    pub protos::Integer,
    pub WlanStandard,
    pub std::option::Option<SecretReference>,
    pub std::option::Option<BackupWireless>,
);
impl datom_codec::Datomic for RouterInterfaces {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 7)?;
        let p0: Interface = datom_codec::Positional::position(&mut p)?;
        let p1: Interface = datom_codec::Positional::position(&mut p)?;
        let p2: WlanBand = datom_codec::Positional::position(&mut p)?;
        let p3: protos::Integer = datom_codec::Positional::position(&mut p)?;
        let p4: WlanStandard = datom_codec::Positional::position(&mut p)?;
        let p5: std::option::Option<SecretReference> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p6: std::option::Option<BackupWireless> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RouterInterfaces {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeNetwork(
    pub std::vec::Vec<LinkLocalIp>,
    pub std::option::Option<NodeIp>,
    pub std::option::Option<WireguardPubKey>,
    pub std::vec::Vec<WireguardProxy>,
    pub std::option::Option<RouterInterfaces>,
);
impl datom_codec::Datomic for NodeNetwork {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 5)?;
        let p0: std::vec::Vec<LinkLocalIp> = datom_codec::Positional::position(&mut p)?;
        let p1: std::option::Option<NodeIp> = datom_codec::Positional::position(&mut p)?;
        let p2: std::option::Option<WireguardPubKey> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p3: std::vec::Vec<WireguardProxy> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p4: std::option::Option<RouterInterfaces> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4))
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeNetwork {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostedSite(pub ServedDomain, pub SiteSource, pub SiteRenderer);
impl datom_codec::Datomic for HostedSite {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: ServedDomain = datom_codec::Positional::position(&mut p)?;
        let p1: SiteSource = datom_codec::Positional::position(&mut p)?;
        let p2: SiteRenderer = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for HostedSite {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersonaCapability {
    GitoliteServer,
}
impl datom_codec::Datomic for PersonaCapability {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "GitoliteServer" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::GitoliteServer)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for PersonaCapability {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::GitoliteServer => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("GitoliteServer")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoSettings();
impl datom_codec::Datomic for NoSettings {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 0)?;
        std::result::Result::Ok(Self())
    }
}
impl protos::Conceivable<datom_codec::Datom> for NoSettings {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(vec![]),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeCapability {
    Graphical(NoSettings),
    Center(NoSettings),
    LargeAi(NoSettings),
    Router(NoSettings),
    Edge(NoSettings),
    NextGeneration(NoSettings),
    LowPower(NoSettings),
    TestVm(NoSettings),
    VmTesting(protos::Boolean, protos::Text, std::option::Option<protos::Text>),
    CloudNode(NoSettings),
    Printing(NoSettings),
    HardwareVideo(NoSettings),
    Nordvpn(NoSettings),
    WifiCertificate(NoSettings),
    TailnetClient(NoSettings),
    TailnetController(NoSettings),
    NixBuilder(std::option::Option<protos::Integer>),
    NixCache(NoSettings),
    PersonaDevelopment(std::vec::Vec<PersonaCapability>),
    VmHost(TapSubnet, KvmAvailability, std::option::Option<protos::Integer>),
    WebHost(std::vec::Vec<HostedSite>),
}
impl datom_codec::Datomic for NodeCapability {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Graphical" => {
                std::result::Result::Ok(Self::Graphical(datom_codec::Carrying::body(v)?))
            }
            "Center" => {
                std::result::Result::Ok(Self::Center(datom_codec::Carrying::body(v)?))
            }
            "LargeAi" => {
                std::result::Result::Ok(Self::LargeAi(datom_codec::Carrying::body(v)?))
            }
            "Router" => {
                std::result::Result::Ok(Self::Router(datom_codec::Carrying::body(v)?))
            }
            "Edge" => {
                std::result::Result::Ok(Self::Edge(datom_codec::Carrying::body(v)?))
            }
            "NextGeneration" => {
                std::result::Result::Ok(
                    Self::NextGeneration(datom_codec::Carrying::body(v)?),
                )
            }
            "LowPower" => {
                std::result::Result::Ok(Self::LowPower(datom_codec::Carrying::body(v)?))
            }
            "TestVm" => {
                std::result::Result::Ok(Self::TestVm(datom_codec::Carrying::body(v)?))
            }
            "VmTesting" => {
                let mut p = datom_codec::Headed::positions(v, 3)?;
                let p0: protos::Boolean = datom_codec::Positional::position(&mut p)?;
                let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
                let p2: std::option::Option<protos::Text> = datom_codec::Positional::position(
                    &mut p,
                )?;
                std::result::Result::Ok(Self::VmTesting(p0, p1, p2))
            }
            "CloudNode" => {
                std::result::Result::Ok(Self::CloudNode(datom_codec::Carrying::body(v)?))
            }
            "Printing" => {
                std::result::Result::Ok(Self::Printing(datom_codec::Carrying::body(v)?))
            }
            "HardwareVideo" => {
                std::result::Result::Ok(
                    Self::HardwareVideo(datom_codec::Carrying::body(v)?),
                )
            }
            "Nordvpn" => {
                std::result::Result::Ok(Self::Nordvpn(datom_codec::Carrying::body(v)?))
            }
            "WifiCertificate" => {
                std::result::Result::Ok(
                    Self::WifiCertificate(datom_codec::Carrying::body(v)?),
                )
            }
            "TailnetClient" => {
                std::result::Result::Ok(
                    Self::TailnetClient(datom_codec::Carrying::body(v)?),
                )
            }
            "TailnetController" => {
                std::result::Result::Ok(
                    Self::TailnetController(datom_codec::Carrying::body(v)?),
                )
            }
            "NixBuilder" => {
                std::result::Result::Ok(
                    Self::NixBuilder(datom_codec::Carrying::body(v)?),
                )
            }
            "NixCache" => {
                std::result::Result::Ok(Self::NixCache(datom_codec::Carrying::body(v)?))
            }
            "PersonaDevelopment" => {
                std::result::Result::Ok(
                    Self::PersonaDevelopment(datom_codec::Carrying::body(v)?),
                )
            }
            "VmHost" => {
                let mut p = datom_codec::Headed::positions(v, 3)?;
                let p0: TapSubnet = datom_codec::Positional::position(&mut p)?;
                let p1: KvmAvailability = datom_codec::Positional::position(&mut p)?;
                let p2: std::option::Option<protos::Integer> = datom_codec::Positional::position(
                    &mut p,
                )?;
                std::result::Result::Ok(Self::VmHost(p0, p1, p2))
            }
            "WebHost" => {
                std::result::Result::Ok(Self::WebHost(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeCapability {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Graphical(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Graphical")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Center(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Center").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::LargeAi(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("LargeAi").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Router(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Router").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Edge(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Edge").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::NextGeneration(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("NextGeneration")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::LowPower(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("LowPower")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::TestVm(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("TestVm").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::VmTesting(p0, p1, p2) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("VmTesting")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                datom_codec::Datom::Struct(
                                    vec![
                                        protos::Conceivable::conceive(p0)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p1)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p2)
                                        .expect("infallible datom ascent").1
                                    ],
                                ),
                            ),
                        )
                    }
                    Self::CloudNode(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("CloudNode")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Printing(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Printing")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::HardwareVideo(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("HardwareVideo")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Nordvpn(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Nordvpn").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::WifiCertificate(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("WifiCertificate")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::TailnetClient(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("TailnetClient")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::TailnetController(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("TailnetController")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::NixBuilder(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("NixBuilder")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::NixCache(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("NixCache")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::PersonaDevelopment(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("PersonaDevelopment")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::VmHost(p0, p1, p2) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("VmHost").expect("static variant"),
                            std::boxed::Box::new(
                                datom_codec::Datom::Struct(
                                    vec![
                                        protos::Conceivable::conceive(p0)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p1)
                                        .expect("infallible datom ascent").1,
                                        protos::Conceivable::conceive(p2)
                                        .expect("infallible datom ascent").1
                                    ],
                                ),
                            ),
                        )
                    }
                    Self::WebHost(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("WebHost").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub type Capabilities = std::vec::Vec<NodeCapability>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeDefinition(
    pub NodeName,
    pub NodeVariant,
    pub Magnitude,
    pub Magnitude,
    pub MachineDefinition,
    pub NodeEnvironment,
    pub NodeNetwork,
    pub NodeKeys,
    pub std::option::Option<protos::Boolean>,
    pub Capabilities,
);
impl datom_codec::Datomic for NodeDefinition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 10)?;
        let p0: NodeName = datom_codec::Positional::position(&mut p)?;
        let p1: NodeVariant = datom_codec::Positional::position(&mut p)?;
        let p2: Magnitude = datom_codec::Positional::position(&mut p)?;
        let p3: Magnitude = datom_codec::Positional::position(&mut p)?;
        let p4: MachineDefinition = datom_codec::Positional::position(&mut p)?;
        let p5: NodeEnvironment = datom_codec::Positional::position(&mut p)?;
        let p6: NodeNetwork = datom_codec::Positional::position(&mut p)?;
        let p7: NodeKeys = datom_codec::Positional::position(&mut p)?;
        let p8: std::option::Option<protos::Boolean> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p9: Capabilities = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9))
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeDefinition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.8)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.9)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type GenericNodeNames = std::vec::Vec<NodeName>;
pub type GenericNodes = std::vec::Vec<NodeDefinition>;
pub type ClusterNodes = std::vec::Vec<NodeDefinition>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserPubKey(pub NodeName, pub SshPubKey, pub Keygrip);
impl datom_codec::Datomic for UserPubKey {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: NodeName = datom_codec::Positional::position(&mut p)?;
        let p1: SshPubKey = datom_codec::Positional::position(&mut p)?;
        let p2: Keygrip = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for UserPubKey {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDefinition(
    pub UserName,
    pub UserRole,
    pub Magnitude,
    pub Keyboard,
    pub Style,
    pub std::option::Option<GithubId>,
    pub std::option::Option<protos::Boolean>,
    pub std::vec::Vec<UserPubKey>,
    pub std::option::Option<Editor>,
    pub std::option::Option<TextSize>,
);
impl datom_codec::Datomic for UserDefinition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 10)?;
        let p0: UserName = datom_codec::Positional::position(&mut p)?;
        let p1: UserRole = datom_codec::Positional::position(&mut p)?;
        let p2: Magnitude = datom_codec::Positional::position(&mut p)?;
        let p3: Keyboard = datom_codec::Positional::position(&mut p)?;
        let p4: Style = datom_codec::Positional::position(&mut p)?;
        let p5: std::option::Option<GithubId> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p6: std::option::Option<protos::Boolean> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p7: std::vec::Vec<UserPubKey> = datom_codec::Positional::position(&mut p)?;
        let p8: std::option::Option<Editor> = datom_codec::Positional::position(&mut p)?;
        let p9: std::option::Option<TextSize> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9))
    }
}
impl protos::Conceivable<datom_codec::Datom> for UserDefinition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.8)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.9)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type Users = std::vec::Vec<UserDefinition>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainDefinition(pub DomainName, pub DomainProvider);
impl datom_codec::Datomic for DomainDefinition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: DomainName = datom_codec::Positional::position(&mut p)?;
        let p1: DomainProvider = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DomainDefinition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type Domains = std::vec::Vec<DomainDefinition>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClusterTrustEntry(pub ClusterName, pub Magnitude);
impl datom_codec::Datomic for ClusterTrustEntry {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: Magnitude = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for ClusterTrustEntry {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeTrustEntry(pub NodeName, pub Magnitude);
impl datom_codec::Datomic for NodeTrustEntry {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: NodeName = datom_codec::Positional::position(&mut p)?;
        let p1: Magnitude = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeTrustEntry {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserTrustEntry(pub UserName, pub Magnitude);
impl datom_codec::Datomic for UserTrustEntry {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: UserName = datom_codec::Positional::position(&mut p)?;
        let p1: Magnitude = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for UserTrustEntry {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClusterTrust(
    pub Magnitude,
    pub std::vec::Vec<ClusterTrustEntry>,
    pub std::vec::Vec<NodeTrustEntry>,
    pub std::vec::Vec<UserTrustEntry>,
);
impl datom_codec::Datomic for ClusterTrust {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 4)?;
        let p0: Magnitude = datom_codec::Positional::position(&mut p)?;
        let p1: std::vec::Vec<ClusterTrustEntry> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p2: std::vec::Vec<NodeTrustEntry> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p3: std::vec::Vec<UserTrustEntry> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3))
    }
}
impl protos::Conceivable<datom_codec::Datom> for ClusterTrust {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainConfiguration(pub protos::Text, pub std::vec::Vec<DomainName>);
impl datom_codec::Datomic for DomainConfiguration {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: std::vec::Vec<DomainName> = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DomainConfiguration {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClusterDefinition(
    pub ClusterName,
    pub ClusterNodes,
    pub GenericNodeNames,
    pub Users,
    pub Domains,
    pub ClusterTrust,
);
impl datom_codec::Datomic for ClusterDefinition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 6)?;
        let p0: ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: ClusterNodes = datom_codec::Positional::position(&mut p)?;
        let p2: GenericNodeNames = datom_codec::Positional::position(&mut p)?;
        let p3: Users = datom_codec::Positional::position(&mut p)?;
        let p4: Domains = datom_codec::Positional::position(&mut p)?;
        let p5: ClusterTrust = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4, p5))
    }
}
impl protos::Conceivable<datom_codec::Datom> for ClusterDefinition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HorizonConfiguration(pub GenericNodes, pub DomainConfiguration);
impl datom_codec::Datomic for HorizonConfiguration {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: GenericNodes = datom_codec::Positional::position(&mut p)?;
        let p1: DomainConfiguration = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for HorizonConfiguration {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HorizonDefinition(pub HorizonConfiguration, pub ClusterDefinition);
impl datom_codec::Datomic for HorizonDefinition {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: HorizonConfiguration = datom_codec::Positional::position(&mut p)?;
        let p1: ClusterDefinition = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for HorizonDefinition {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompositionRequest(pub protos::Text, pub protos::Text);
impl datom_codec::Datomic for CompositionRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for CompositionRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompositionCommand {
    Compose(CompositionRequest),
}
impl datom_codec::Datomic for CompositionCommand {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Compose" => {
                std::result::Result::Ok(Self::Compose(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for CompositionCommand {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Compose(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Compose").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
