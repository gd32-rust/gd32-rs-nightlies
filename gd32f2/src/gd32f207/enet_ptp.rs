#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register (PTP_TSCTL)"]
    pub ptp_tsctl: crate::Reg<ptp_tsctl::PTP_TSCTL_SPEC>,
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    pub ptp_ssinc: crate::Reg<ptp_ssinc::PTP_SSINC_SPEC>,
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    pub ptp_tsh: crate::Reg<ptp_tsh::PTP_TSH_SPEC>,
    #[doc = "0x0c - Ethernet PTP time stamp low register (PTP_TSL)"]
    pub ptp_tsl: crate::Reg<ptp_tsl::PTP_TSL_SPEC>,
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    pub ptp_tsuh: crate::Reg<ptp_tsuh::PTP_TSUH_SPEC>,
    #[doc = "0x14 - Ethernet PTP time stamp low update register (PTP_TSUL)"]
    pub ptp_tsul: crate::Reg<ptp_tsul::PTP_TSUL_SPEC>,
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    pub ptp_tsaddend: crate::Reg<ptp_tsaddend::PTP_TSADDEND_SPEC>,
    #[doc = "0x1c - Ethernet PTP expected time high register"]
    pub ptp_eth: crate::Reg<ptp_eth::PTP_ETH_SPEC>,
    #[doc = "0x20 - Ethernet PTP expected time low register"]
    pub ptp_etl: crate::Reg<ptp_etl::PTP_ETL_SPEC>,
}
#[doc = "PTP_TSCTL register accessor: an alias for `Reg<PTP_TSCTL_SPEC>`"]
pub type PTP_TSCTL = crate::Reg<ptp_tsctl::PTP_TSCTL_SPEC>;
#[doc = "Ethernet PTP time stamp control register (PTP_TSCTL)"]
pub mod ptp_tsctl;
#[doc = "PTP_SSINC register accessor: an alias for `Reg<PTP_SSINC_SPEC>`"]
pub type PTP_SSINC = crate::Reg<ptp_ssinc::PTP_SSINC_SPEC>;
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptp_ssinc;
#[doc = "PTP_TSH register accessor: an alias for `Reg<PTP_TSH_SPEC>`"]
pub type PTP_TSH = crate::Reg<ptp_tsh::PTP_TSH_SPEC>;
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptp_tsh;
#[doc = "PTP_TSL register accessor: an alias for `Reg<PTP_TSL_SPEC>`"]
pub type PTP_TSL = crate::Reg<ptp_tsl::PTP_TSL_SPEC>;
#[doc = "Ethernet PTP time stamp low register (PTP_TSL)"]
pub mod ptp_tsl;
#[doc = "PTP_TSUH register accessor: an alias for `Reg<PTP_TSUH_SPEC>`"]
pub type PTP_TSUH = crate::Reg<ptp_tsuh::PTP_TSUH_SPEC>;
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptp_tsuh;
#[doc = "PTP_TSUL register accessor: an alias for `Reg<PTP_TSUL_SPEC>`"]
pub type PTP_TSUL = crate::Reg<ptp_tsul::PTP_TSUL_SPEC>;
#[doc = "Ethernet PTP time stamp low update register (PTP_TSUL)"]
pub mod ptp_tsul;
#[doc = "PTP_TSADDEND register accessor: an alias for `Reg<PTP_TSADDEND_SPEC>`"]
pub type PTP_TSADDEND = crate::Reg<ptp_tsaddend::PTP_TSADDEND_SPEC>;
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptp_tsaddend;
#[doc = "PTP_ETH register accessor: an alias for `Reg<PTP_ETH_SPEC>`"]
pub type PTP_ETH = crate::Reg<ptp_eth::PTP_ETH_SPEC>;
#[doc = "Ethernet PTP expected time high register"]
pub mod ptp_eth;
#[doc = "PTP_ETL register accessor: an alias for `Reg<PTP_ETL_SPEC>`"]
pub type PTP_ETL = crate::Reg<ptp_etl::PTP_ETL_SPEC>;
#[doc = "Ethernet PTP expected time low register"]
pub mod ptp_etl;
