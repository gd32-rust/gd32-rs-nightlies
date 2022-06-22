#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MSC control register (MSC_CTL)"]
    pub msc_ctl: crate::Reg<msc_ctl::MSC_CTL_SPEC>,
    #[doc = "0x04 - Ethernet MSC receive interrupt flag register (MSC_RINTF)"]
    pub msc_rintf: crate::Reg<msc_rintf::MSC_RINTF_SPEC>,
    #[doc = "0x08 - Ethernet MSC transmit interrupt flag register (MSC_TINTF)"]
    pub msc_tintf: crate::Reg<msc_tintf::MSC_TINTF_SPEC>,
    #[doc = "0x0c - Ethernet MSC receive interrupt mask register (MSC_RINTMSK)"]
    pub msc_rintmsk: crate::Reg<msc_rintmsk::MSC_RINTMSK_SPEC>,
    #[doc = "0x10 - Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)"]
    pub msc_tintmsk: crate::Reg<msc_tintmsk::MSC_TINTMSK_SPEC>,
    _reserved5: [u8; 0x38],
    #[doc = "0x4c - Ethernet MSC transmitted good frames after a single collision counter"]
    pub msc_sccnt: crate::Reg<msc_sccnt::MSC_SCCNT_SPEC>,
    #[doc = "0x50 - Ethernet MSC transmitted good frames after more than a single collision"]
    pub msc_msccnt: crate::Reg<msc_msccnt::MSC_MSCCNT_SPEC>,
    _reserved7: [u8; 0x14],
    #[doc = "0x68 - Ethernet MSC transmitted good frames counter register"]
    pub msc_tgfcnt: crate::Reg<msc_tgfcnt::MSC_TGFCNT_SPEC>,
    _reserved8: [u8; 0x28],
    #[doc = "0x94 - Ethernet MSC received frames with CRC error counter register"]
    pub msc_rfcecnt: crate::Reg<msc_rfcecnt::MSC_RFCECNT_SPEC>,
    #[doc = "0x98 - Ethernet MSC received frames with alignment error counter register"]
    pub msc_rfaecnt: crate::Reg<msc_rfaecnt::MSC_RFAECNT_SPEC>,
    _reserved10: [u8; 0x28],
    #[doc = "0xc4 - MSC received good unicast frames counter register"]
    pub msc_rgufcnt: crate::Reg<msc_rgufcnt::MSC_RGUFCNT_SPEC>,
}
#[doc = "MSC_CTL register accessor: an alias for `Reg<MSC_CTL_SPEC>`"]
pub type MSC_CTL = crate::Reg<msc_ctl::MSC_CTL_SPEC>;
#[doc = "Ethernet MSC control register (MSC_CTL)"]
pub mod msc_ctl;
#[doc = "MSC_RINTF register accessor: an alias for `Reg<MSC_RINTF_SPEC>`"]
pub type MSC_RINTF = crate::Reg<msc_rintf::MSC_RINTF_SPEC>;
#[doc = "Ethernet MSC receive interrupt flag register (MSC_RINTF)"]
pub mod msc_rintf;
#[doc = "MSC_TINTF register accessor: an alias for `Reg<MSC_TINTF_SPEC>`"]
pub type MSC_TINTF = crate::Reg<msc_tintf::MSC_TINTF_SPEC>;
#[doc = "Ethernet MSC transmit interrupt flag register (MSC_TINTF)"]
pub mod msc_tintf;
#[doc = "MSC_RINTMSK register accessor: an alias for `Reg<MSC_RINTMSK_SPEC>`"]
pub type MSC_RINTMSK = crate::Reg<msc_rintmsk::MSC_RINTMSK_SPEC>;
#[doc = "Ethernet MSC receive interrupt mask register (MSC_RINTMSK)"]
pub mod msc_rintmsk;
#[doc = "MSC_TINTMSK register accessor: an alias for `Reg<MSC_TINTMSK_SPEC>`"]
pub type MSC_TINTMSK = crate::Reg<msc_tintmsk::MSC_TINTMSK_SPEC>;
#[doc = "Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)"]
pub mod msc_tintmsk;
#[doc = "MSC_SCCNT register accessor: an alias for `Reg<MSC_SCCNT_SPEC>`"]
pub type MSC_SCCNT = crate::Reg<msc_sccnt::MSC_SCCNT_SPEC>;
#[doc = "Ethernet MSC transmitted good frames after a single collision counter"]
pub mod msc_sccnt;
#[doc = "MSC_MSCCNT register accessor: an alias for `Reg<MSC_MSCCNT_SPEC>`"]
pub type MSC_MSCCNT = crate::Reg<msc_msccnt::MSC_MSCCNT_SPEC>;
#[doc = "Ethernet MSC transmitted good frames after more than a single collision"]
pub mod msc_msccnt;
#[doc = "MSC_TGFCNT register accessor: an alias for `Reg<MSC_TGFCNT_SPEC>`"]
pub type MSC_TGFCNT = crate::Reg<msc_tgfcnt::MSC_TGFCNT_SPEC>;
#[doc = "Ethernet MSC transmitted good frames counter register"]
pub mod msc_tgfcnt;
#[doc = "MSC_RFCECNT register accessor: an alias for `Reg<MSC_RFCECNT_SPEC>`"]
pub type MSC_RFCECNT = crate::Reg<msc_rfcecnt::MSC_RFCECNT_SPEC>;
#[doc = "Ethernet MSC received frames with CRC error counter register"]
pub mod msc_rfcecnt;
#[doc = "MSC_RFAECNT register accessor: an alias for `Reg<MSC_RFAECNT_SPEC>`"]
pub type MSC_RFAECNT = crate::Reg<msc_rfaecnt::MSC_RFAECNT_SPEC>;
#[doc = "Ethernet MSC received frames with alignment error counter register"]
pub mod msc_rfaecnt;
#[doc = "MSC_RGUFCNT register accessor: an alias for `Reg<MSC_RGUFCNT_SPEC>`"]
pub type MSC_RGUFCNT = crate::Reg<msc_rgufcnt::MSC_RGUFCNT_SPEC>;
#[doc = "MSC received good unicast frames counter register"]
pub mod msc_rgufcnt;
