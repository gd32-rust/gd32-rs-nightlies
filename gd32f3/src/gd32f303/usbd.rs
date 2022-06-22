#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0cs: crate::Reg<ep0cs::EP0CS_SPEC>,
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1cs: crate::Reg<ep1cs::EP1CS_SPEC>,
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2cs: crate::Reg<ep2cs::EP2CS_SPEC>,
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3cs: crate::Reg<ep3cs::EP3CS_SPEC>,
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4cs: crate::Reg<ep4cs::EP4CS_SPEC>,
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5cs: crate::Reg<ep5cs::EP5CS_SPEC>,
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6cs: crate::Reg<ep6cs::EP6CS_SPEC>,
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7cs: crate::Reg<ep7cs::EP7CS_SPEC>,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x44 - interrupt flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x48 - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x4c - device address register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x50 - Buffer address register"]
    pub baddr: crate::Reg<baddr::BADDR_SPEC>,
    #[doc = "0x54 - USB LPM control and status register"]
    pub lpmcs: crate::Reg<lpmcs::LPMCS_SPEC>,
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
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "device address register"]
pub mod addr;
#[doc = "BADDR register accessor: an alias for `Reg<BADDR_SPEC>`"]
pub type BADDR = crate::Reg<baddr::BADDR_SPEC>;
#[doc = "Buffer address register"]
pub mod baddr;
#[doc = "LPMCS register accessor: an alias for `Reg<LPMCS_SPEC>`"]
pub type LPMCS = crate::Reg<lpmcs::LPMCS_SPEC>;
#[doc = "USB LPM control and status register"]
pub mod lpmcs;
