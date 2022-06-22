#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0cs: crate::Reg<ep0cs::EP0CS_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1cs: crate::Reg<ep1cs::EP1CS_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2cs: crate::Reg<ep2cs::EP2CS_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3cs: crate::Reg<ep3cs::EP3CS_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4cs: crate::Reg<ep4cs::EP4CS_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5cs: crate::Reg<ep5cs::EP5CS_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6cs: crate::Reg<ep6cs::EP6CS_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7cs: crate::Reg<ep7cs::EP7CS_SPEC>,
    _reserved8: [u8; 0x22],
    #[doc = "0x40 - control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x44 - interrupt flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x48 - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x4c - device address register"]
    pub daddr: crate::Reg<daddr::DADDR_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x50 - Buffer address register"]
    pub baddr: crate::Reg<baddr::BADDR_SPEC>,
    _reserved13: [u8; 0xae],
    #[doc = "0x100 - USB sub-endpoint 0 register"]
    pub sep0: crate::Reg<sep0::SEP0_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x104 - USB sub-endpoint 1 register"]
    pub sep1: crate::Reg<sep1::SEP1_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x108 - USB sub-endpoint 2 register"]
    pub sep2: crate::Reg<sep2::SEP2_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x10c - USB sub-endpoint 3 register"]
    pub sep3: crate::Reg<sep3::SEP3_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x110 - USB sub-endpoint 4 register"]
    pub sep4: crate::Reg<sep4::SEP4_SPEC>,
    _reserved18: [u8; 0x02],
    #[doc = "0x114 - USB sub-endpoint 5 register"]
    pub sep5: crate::Reg<sep5::SEP5_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x118 - USB sub-endpoint 6 register"]
    pub sep6: crate::Reg<sep6::SEP6_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x11c - USB sub-endpoint 7 register"]
    pub sep7: crate::Reg<sep7::SEP7_SPEC>,
    _reserved21: [u8; 0x22],
    #[doc = "0x140 - USB LPM control register"]
    pub lpmctl: crate::Reg<lpmctl::LPMCTL_SPEC>,
    _reserved22: [u8; 0x02],
    #[doc = "0x144 - USB LPM interrupt flag register"]
    pub lpmintf: crate::Reg<lpmintf::LPMINTF_SPEC>,
}
#[doc = "EP0CS register accessor: an alias for `Reg<EP0CS_SPEC>`"]
pub type EP0CS = crate::Reg<ep0cs::EP0CS_SPEC>;
#[doc = "endpoint 0 register"]
pub mod ep0cs;
#[doc = "EP1CS register accessor: an alias for `Reg<EP1CS_SPEC>`"]
pub type EP1CS = crate::Reg<ep1cs::EP1CS_SPEC>;
#[doc = "endpoint 1 register"]
pub mod ep1cs;
#[doc = "EP2CS register accessor: an alias for `Reg<EP2CS_SPEC>`"]
pub type EP2CS = crate::Reg<ep2cs::EP2CS_SPEC>;
#[doc = "endpoint 2 register"]
pub mod ep2cs;
#[doc = "EP3CS register accessor: an alias for `Reg<EP3CS_SPEC>`"]
pub type EP3CS = crate::Reg<ep3cs::EP3CS_SPEC>;
#[doc = "endpoint 3 register"]
pub mod ep3cs;
#[doc = "EP4CS register accessor: an alias for `Reg<EP4CS_SPEC>`"]
pub type EP4CS = crate::Reg<ep4cs::EP4CS_SPEC>;
#[doc = "endpoint 4 register"]
pub mod ep4cs;
#[doc = "EP5CS register accessor: an alias for `Reg<EP5CS_SPEC>`"]
pub type EP5CS = crate::Reg<ep5cs::EP5CS_SPEC>;
#[doc = "endpoint 5 register"]
pub mod ep5cs;
#[doc = "EP6CS register accessor: an alias for `Reg<EP6CS_SPEC>`"]
pub type EP6CS = crate::Reg<ep6cs::EP6CS_SPEC>;
#[doc = "endpoint 6 register"]
pub mod ep6cs;
#[doc = "EP7CS register accessor: an alias for `Reg<EP7CS_SPEC>`"]
pub type EP7CS = crate::Reg<ep7cs::EP7CS_SPEC>;
#[doc = "endpoint 7 register"]
pub mod ep7cs;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "DADDR register accessor: an alias for `Reg<DADDR_SPEC>`"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "device address register"]
pub mod daddr;
#[doc = "BADDR register accessor: an alias for `Reg<BADDR_SPEC>`"]
pub type BADDR = crate::Reg<baddr::BADDR_SPEC>;
#[doc = "Buffer address register"]
pub mod baddr;
#[doc = "SEP0 register accessor: an alias for `Reg<SEP0_SPEC>`"]
pub type SEP0 = crate::Reg<sep0::SEP0_SPEC>;
#[doc = "USB sub-endpoint 0 register"]
pub mod sep0;
#[doc = "SEP1 register accessor: an alias for `Reg<SEP1_SPEC>`"]
pub type SEP1 = crate::Reg<sep1::SEP1_SPEC>;
#[doc = "USB sub-endpoint 1 register"]
pub mod sep1;
#[doc = "SEP2 register accessor: an alias for `Reg<SEP2_SPEC>`"]
pub type SEP2 = crate::Reg<sep2::SEP2_SPEC>;
#[doc = "USB sub-endpoint 2 register"]
pub mod sep2;
#[doc = "SEP3 register accessor: an alias for `Reg<SEP3_SPEC>`"]
pub type SEP3 = crate::Reg<sep3::SEP3_SPEC>;
#[doc = "USB sub-endpoint 3 register"]
pub mod sep3;
#[doc = "SEP4 register accessor: an alias for `Reg<SEP4_SPEC>`"]
pub type SEP4 = crate::Reg<sep4::SEP4_SPEC>;
#[doc = "USB sub-endpoint 4 register"]
pub mod sep4;
#[doc = "SEP5 register accessor: an alias for `Reg<SEP5_SPEC>`"]
pub type SEP5 = crate::Reg<sep5::SEP5_SPEC>;
#[doc = "USB sub-endpoint 5 register"]
pub mod sep5;
#[doc = "SEP6 register accessor: an alias for `Reg<SEP6_SPEC>`"]
pub type SEP6 = crate::Reg<sep6::SEP6_SPEC>;
#[doc = "USB sub-endpoint 6 register"]
pub mod sep6;
#[doc = "SEP7 register accessor: an alias for `Reg<SEP7_SPEC>`"]
pub type SEP7 = crate::Reg<sep7::SEP7_SPEC>;
#[doc = "USB sub-endpoint 7 register"]
pub mod sep7;
#[doc = "LPMCTL register accessor: an alias for `Reg<LPMCTL_SPEC>`"]
pub type LPMCTL = crate::Reg<lpmctl::LPMCTL_SPEC>;
#[doc = "USB LPM control register"]
pub mod lpmctl;
#[doc = "LPMINTF register accessor: an alias for `Reg<LPMINTF_SPEC>`"]
pub type LPMINTF = crate::Reg<lpmintf::LPMINTF_SPEC>;
#[doc = "USB LPM interrupt flag register"]
pub mod lpmintf;
