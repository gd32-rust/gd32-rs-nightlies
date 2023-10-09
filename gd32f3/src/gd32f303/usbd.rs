#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0cs: EP0CS,
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1cs: EP1CS,
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2cs: EP2CS,
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3cs: EP3CS,
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4cs: EP4CS,
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5cs: EP5CS,
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6cs: EP6CS,
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7cs: EP7CS,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - control register"]
    pub ctl: CTL,
    #[doc = "0x44 - interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x48 - Status register"]
    pub stat: STAT,
    #[doc = "0x4c - device address register"]
    pub addr: ADDR,
    #[doc = "0x50 - Buffer address register"]
    pub baddr: BADDR,
    #[doc = "0x54 - USB LPM control and status register"]
    pub lpmcs: LPMCS,
}
#[doc = "EP0CS (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep0cs`]
module"]
pub type EP0CS = crate::Reg<ep0cs::EP0CS_SPEC>;
#[doc = "endpoint 0 register"]
pub mod ep0cs;
#[doc = "EP1CS (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep1cs`]
module"]
pub type EP1CS = crate::Reg<ep1cs::EP1CS_SPEC>;
#[doc = "endpoint 1 register"]
pub mod ep1cs;
#[doc = "EP2CS (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep2cs`]
module"]
pub type EP2CS = crate::Reg<ep2cs::EP2CS_SPEC>;
#[doc = "endpoint 2 register"]
pub mod ep2cs;
#[doc = "EP3CS (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep3cs`]
module"]
pub type EP3CS = crate::Reg<ep3cs::EP3CS_SPEC>;
#[doc = "endpoint 3 register"]
pub mod ep3cs;
#[doc = "EP4CS (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep4cs`]
module"]
pub type EP4CS = crate::Reg<ep4cs::EP4CS_SPEC>;
#[doc = "endpoint 4 register"]
pub mod ep4cs;
#[doc = "EP5CS (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep5cs`]
module"]
pub type EP5CS = crate::Reg<ep5cs::EP5CS_SPEC>;
#[doc = "endpoint 5 register"]
pub mod ep5cs;
#[doc = "EP6CS (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep6cs`]
module"]
pub type EP6CS = crate::Reg<ep6cs::EP6CS_SPEC>;
#[doc = "endpoint 6 register"]
pub mod ep6cs;
#[doc = "EP7CS (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep7cs`]
module"]
pub type EP7CS = crate::Reg<ep7cs::EP7CS_SPEC>;
#[doc = "endpoint 7 register"]
pub mod ep7cs;
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "INTF (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "ADDR (rw) register accessor: device address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "device address register"]
pub mod addr;
#[doc = "BADDR (rw) register accessor: Buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`baddr`]
module"]
pub type BADDR = crate::Reg<baddr::BADDR_SPEC>;
#[doc = "Buffer address register"]
pub mod baddr;
#[doc = "LPMCS (rw) register accessor: USB LPM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpmcs`]
module"]
pub type LPMCS = crate::Reg<lpmcs::LPMCS_SPEC>;
#[doc = "USB LPM control and status register"]
pub mod lpmcs;
