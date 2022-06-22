#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x5c],
    #[doc = "0x5c - OPA control register"]
    pub opa_ctl: crate::Reg<opa_ctl::OPA_CTL_SPEC>,
    #[doc = "0x60 - OPA offset trimming for normal mode register"]
    pub opa_bt: crate::Reg<opa_bt::OPA_BT_SPEC>,
    #[doc = "0x64 - OPA offset trimming for low power mode register"]
    pub opa_lpbt: crate::Reg<opa_lpbt::OPA_LPBT_SPEC>,
    _reserved3: [u8; 0x0298],
    #[doc = "0x300 - IVREF control register"]
    pub ivref_ctl: crate::Reg<ivref_ctl::IVREF_CTL_SPEC>,
}
#[doc = "OPA_CTL register accessor: an alias for `Reg<OPA_CTL_SPEC>`"]
pub type OPA_CTL = crate::Reg<opa_ctl::OPA_CTL_SPEC>;
#[doc = "OPA control register"]
pub mod opa_ctl;
#[doc = "OPA_BT register accessor: an alias for `Reg<OPA_BT_SPEC>`"]
pub type OPA_BT = crate::Reg<opa_bt::OPA_BT_SPEC>;
#[doc = "OPA offset trimming for normal mode register"]
pub mod opa_bt;
#[doc = "OPA_LPBT register accessor: an alias for `Reg<OPA_LPBT_SPEC>`"]
pub type OPA_LPBT = crate::Reg<opa_lpbt::OPA_LPBT_SPEC>;
#[doc = "OPA offset trimming for low power mode register"]
pub mod opa_lpbt;
#[doc = "IVREF_CTL register accessor: an alias for `Reg<IVREF_CTL_SPEC>`"]
pub type IVREF_CTL = crate::Reg<ivref_ctl::IVREF_CTL_SPEC>;
#[doc = "IVREF control register"]
pub mod ivref_ctl;
