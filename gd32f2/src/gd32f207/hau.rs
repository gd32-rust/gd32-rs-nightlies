#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HAU control register"]
    pub ctl: CTL,
    #[doc = "0x04 - HAU data input register"]
    pub di: DI,
    #[doc = "0x08 - HAU configuration register"]
    pub cfg: CFG,
    #[doc = "0x0c - HAU data output register"]
    pub do0: DO0,
    #[doc = "0x10 - HAU data output register"]
    pub do1: DO1,
    #[doc = "0x14 - HAU data output register"]
    pub do2: DO2,
    #[doc = "0x18 - HAU data output register"]
    pub do3: DO3,
    #[doc = "0x1c - HAU data output register"]
    pub do4: DO4,
    #[doc = "0x20 - HAU interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x24 - HAU status and interrupt flag register"]
    pub stat: STAT,
    _reserved10: [u8; 0x02fc],
    #[doc = "0x324 - HAU data output register"]
    pub do5: DO5,
    #[doc = "0x328 - HAU data output register"]
    pub do6: DO6,
    #[doc = "0x32c - HAU data output register"]
    pub do7: DO7,
}
#[doc = "CTL (rw) register accessor: HAU control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "HAU control register"]
pub mod ctl;
#[doc = "DI (rw) register accessor: HAU data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`di::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`di::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`di`]
module"]
pub type DI = crate::Reg<di::DI_SPEC>;
#[doc = "HAU data input register"]
pub mod di;
#[doc = "CFG (rw) register accessor: HAU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "HAU configuration register"]
pub mod cfg;
#[doc = "DO0 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do0`]
module"]
pub type DO0 = crate::Reg<do0::DO0_SPEC>;
#[doc = "HAU data output register"]
pub mod do0;
#[doc = "DO1 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do1`]
module"]
pub type DO1 = crate::Reg<do1::DO1_SPEC>;
#[doc = "HAU data output register"]
pub mod do1;
#[doc = "DO2 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do2`]
module"]
pub type DO2 = crate::Reg<do2::DO2_SPEC>;
#[doc = "HAU data output register"]
pub mod do2;
#[doc = "DO3 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do3`]
module"]
pub type DO3 = crate::Reg<do3::DO3_SPEC>;
#[doc = "HAU data output register"]
pub mod do3;
#[doc = "DO4 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do4`]
module"]
pub type DO4 = crate::Reg<do4::DO4_SPEC>;
#[doc = "HAU data output register"]
pub mod do4;
#[doc = "DO5 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do5`]
module"]
pub type DO5 = crate::Reg<do5::DO5_SPEC>;
#[doc = "HAU data output register"]
pub mod do5;
#[doc = "DO6 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do6`]
module"]
pub type DO6 = crate::Reg<do6::DO6_SPEC>;
#[doc = "HAU data output register"]
pub mod do6;
#[doc = "DO7 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do7`]
module"]
pub type DO7 = crate::Reg<do7::DO7_SPEC>;
#[doc = "HAU data output register"]
pub mod do7;
#[doc = "INTEN (rw) register accessor: HAU interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "HAU interrupt enable register"]
pub mod inten;
#[doc = "STAT (rw) register accessor: HAU status and interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "HAU status and interrupt flag register"]
pub mod stat;
