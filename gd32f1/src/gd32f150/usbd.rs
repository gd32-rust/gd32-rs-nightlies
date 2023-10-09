#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0cs: EP0CS,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1cs: EP1CS,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2cs: EP2CS,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3cs: EP3CS,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4cs: EP4CS,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5cs: EP5CS,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6cs: EP6CS,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7cs: EP7CS,
    _reserved8: [u8; 0x22],
    #[doc = "0x40 - control register"]
    pub ctl: CTL,
    _reserved9: [u8; 0x02],
    #[doc = "0x44 - interrupt flag register"]
    pub intf: INTF,
    _reserved10: [u8; 0x02],
    #[doc = "0x48 - Status register"]
    pub stat: STAT,
    _reserved11: [u8; 0x02],
    #[doc = "0x4c - device address register"]
    pub daddr: DADDR,
    _reserved12: [u8; 0x02],
    #[doc = "0x50 - Buffer address register"]
    pub baddr: BADDR,
    _reserved13: [u8; 0xae],
    #[doc = "0x100 - USB sub-endpoint 0 register"]
    pub sep0: SEP0,
    _reserved14: [u8; 0x02],
    #[doc = "0x104 - USB sub-endpoint 1 register"]
    pub sep1: SEP1,
    _reserved15: [u8; 0x02],
    #[doc = "0x108 - USB sub-endpoint 2 register"]
    pub sep2: SEP2,
    _reserved16: [u8; 0x02],
    #[doc = "0x10c - USB sub-endpoint 3 register"]
    pub sep3: SEP3,
    _reserved17: [u8; 0x02],
    #[doc = "0x110 - USB sub-endpoint 4 register"]
    pub sep4: SEP4,
    _reserved18: [u8; 0x02],
    #[doc = "0x114 - USB sub-endpoint 5 register"]
    pub sep5: SEP5,
    _reserved19: [u8; 0x02],
    #[doc = "0x118 - USB sub-endpoint 6 register"]
    pub sep6: SEP6,
    _reserved20: [u8; 0x02],
    #[doc = "0x11c - USB sub-endpoint 7 register"]
    pub sep7: SEP7,
    _reserved21: [u8; 0x22],
    #[doc = "0x140 - USB LPM control register"]
    pub lpmctl: LPMCTL,
    _reserved22: [u8; 0x02],
    #[doc = "0x144 - USB LPM interrupt flag register"]
    pub lpmintf: LPMINTF,
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
#[doc = "DADDR (rw) register accessor: device address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daddr`]
module"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "device address register"]
pub mod daddr;
#[doc = "BADDR (rw) register accessor: Buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`baddr`]
module"]
pub type BADDR = crate::Reg<baddr::BADDR_SPEC>;
#[doc = "Buffer address register"]
pub mod baddr;
#[doc = "SEP0 (rw) register accessor: USB sub-endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sep0`]
module"]
pub type SEP0 = crate::Reg<sep0::SEP0_SPEC>;
#[doc = "USB sub-endpoint 0 register"]
pub mod sep0;
#[doc = "SEP1 (rw) register accessor: USB sub-endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sep1`]
module"]
pub type SEP1 = crate::Reg<sep1::SEP1_SPEC>;
#[doc = "USB sub-endpoint 1 register"]
pub mod sep1;
#[doc = "SEP2 (rw) register accessor: USB sub-endpoint 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sep2`]
module"]
pub type SEP2 = crate::Reg<sep2::SEP2_SPEC>;
#[doc = "USB sub-endpoint 2 register"]
pub mod sep2;
#[doc = "SEP3 (rw) register accessor: USB sub-endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sep3`]
module"]
pub type SEP3 = crate::Reg<sep3::SEP3_SPEC>;
#[doc = "USB sub-endpoint 3 register"]
pub mod sep3;
#[doc = "SEP4 (rw) register accessor: USB sub-endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sep4`]
module"]
pub type SEP4 = crate::Reg<sep4::SEP4_SPEC>;
#[doc = "USB sub-endpoint 4 register"]
pub mod sep4;
#[doc = "SEP5 (rw) register accessor: USB sub-endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sep5`]
module"]
pub type SEP5 = crate::Reg<sep5::SEP5_SPEC>;
#[doc = "USB sub-endpoint 5 register"]
pub mod sep5;
#[doc = "SEP6 (rw) register accessor: USB sub-endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sep6`]
module"]
pub type SEP6 = crate::Reg<sep6::SEP6_SPEC>;
#[doc = "USB sub-endpoint 6 register"]
pub mod sep6;
#[doc = "SEP7 (rw) register accessor: USB sub-endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sep7`]
module"]
pub type SEP7 = crate::Reg<sep7::SEP7_SPEC>;
#[doc = "USB sub-endpoint 7 register"]
pub mod sep7;
#[doc = "LPMCTL (rw) register accessor: USB LPM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpmctl`]
module"]
pub type LPMCTL = crate::Reg<lpmctl::LPMCTL_SPEC>;
#[doc = "USB LPM control register"]
pub mod lpmctl;
#[doc = "LPMINTF (rw) register accessor: USB LPM interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmintf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmintf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpmintf`]
module"]
pub type LPMINTF = crate::Reg<lpmintf::LPMINTF_SPEC>;
#[doc = "USB LPM interrupt flag register"]
pub mod lpmintf;
