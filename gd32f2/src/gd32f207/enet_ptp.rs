#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register (PTP_TSCTL)"]
    pub ptp_tsctl: PTP_TSCTL,
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    pub ptp_ssinc: PTP_SSINC,
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    pub ptp_tsh: PTP_TSH,
    #[doc = "0x0c - Ethernet PTP time stamp low register (PTP_TSL)"]
    pub ptp_tsl: PTP_TSL,
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    pub ptp_tsuh: PTP_TSUH,
    #[doc = "0x14 - Ethernet PTP time stamp low update register (PTP_TSUL)"]
    pub ptp_tsul: PTP_TSUL,
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    pub ptp_tsaddend: PTP_TSADDEND,
    #[doc = "0x1c - Ethernet PTP expected time high register"]
    pub ptp_eth: PTP_ETH,
    #[doc = "0x20 - Ethernet PTP expected time low register"]
    pub ptp_etl: PTP_ETL,
}
#[doc = "PTP_TSCTL (rw) register accessor: Ethernet PTP time stamp control register (PTP_TSCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_tsctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_tsctl`]
module"]
pub type PTP_TSCTL = crate::Reg<ptp_tsctl::PTP_TSCTL_SPEC>;
#[doc = "Ethernet PTP time stamp control register (PTP_TSCTL)"]
pub mod ptp_tsctl;
#[doc = "PTP_SSINC (rw) register accessor: Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_ssinc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_ssinc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_ssinc`]
module"]
pub type PTP_SSINC = crate::Reg<ptp_ssinc::PTP_SSINC_SPEC>;
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptp_ssinc;
#[doc = "PTP_TSH (r) register accessor: Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_tsh`]
module"]
pub type PTP_TSH = crate::Reg<ptp_tsh::PTP_TSH_SPEC>;
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptp_tsh;
#[doc = "PTP_TSL (r) register accessor: Ethernet PTP time stamp low register (PTP_TSL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_tsl`]
module"]
pub type PTP_TSL = crate::Reg<ptp_tsl::PTP_TSL_SPEC>;
#[doc = "Ethernet PTP time stamp low register (PTP_TSL)"]
pub mod ptp_tsl;
#[doc = "PTP_TSUH (rw) register accessor: Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsuh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_tsuh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_tsuh`]
module"]
pub type PTP_TSUH = crate::Reg<ptp_tsuh::PTP_TSUH_SPEC>;
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptp_tsuh;
#[doc = "PTP_TSUL (rw) register accessor: Ethernet PTP time stamp low update register (PTP_TSUL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_tsul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_tsul`]
module"]
pub type PTP_TSUL = crate::Reg<ptp_tsul::PTP_TSUL_SPEC>;
#[doc = "Ethernet PTP time stamp low update register (PTP_TSUL)"]
pub mod ptp_tsul;
#[doc = "PTP_TSADDEND (rw) register accessor: Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsaddend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_tsaddend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_tsaddend`]
module"]
pub type PTP_TSADDEND = crate::Reg<ptp_tsaddend::PTP_TSADDEND_SPEC>;
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptp_tsaddend;
#[doc = "PTP_ETH (rw) register accessor: Ethernet PTP expected time high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_eth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_eth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_eth`]
module"]
pub type PTP_ETH = crate::Reg<ptp_eth::PTP_ETH_SPEC>;
#[doc = "Ethernet PTP expected time high register"]
pub mod ptp_eth;
#[doc = "PTP_ETL (rw) register accessor: Ethernet PTP expected time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_etl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_etl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptp_etl`]
module"]
pub type PTP_ETL = crate::Reg<ptp_etl::PTP_ETL_SPEC>;
#[doc = "Ethernet PTP expected time low register"]
pub mod ptp_etl;
