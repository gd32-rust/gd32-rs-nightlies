#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAU control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - CAU status register 0"]
    pub stat0: crate::Reg<stat0::STAT0_SPEC>,
    #[doc = "0x08 - CAU data input register"]
    pub di: crate::Reg<di::DI_SPEC>,
    #[doc = "0x0c - CAU data output register"]
    pub do_: crate::Reg<do_::DO_SPEC>,
    #[doc = "0x10 - CAU DMA enable register"]
    pub dmaen: crate::Reg<dmaen::DMAEN_SPEC>,
    #[doc = "0x14 - CAU interrupt enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x18 - CAU interrupt status flag register 1"]
    pub stat1: crate::Reg<stat1::STAT1_SPEC>,
    #[doc = "0x1c - CAU enable interrupt status flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x20 - CAU key register"]
    pub key0h: crate::Reg<key0h::KEY0H_SPEC>,
    #[doc = "0x24 - CAU key register"]
    pub key0l: crate::Reg<key0l::KEY0L_SPEC>,
    #[doc = "0x28 - CAU key register"]
    pub key1h: crate::Reg<key1h::KEY1H_SPEC>,
    #[doc = "0x2c - CAU key register"]
    pub key1l: crate::Reg<key1l::KEY1L_SPEC>,
    #[doc = "0x30 - CAU key register"]
    pub key2h: crate::Reg<key2h::KEY2H_SPEC>,
    #[doc = "0x34 - CAU key register"]
    pub key2l: crate::Reg<key2l::KEY2L_SPEC>,
    #[doc = "0x38 - CAU key register"]
    pub key3h: crate::Reg<key3h::KEY3H_SPEC>,
    #[doc = "0x3c - CAU key register"]
    pub key3l: crate::Reg<key3l::KEY3L_SPEC>,
    #[doc = "0x40 - CAU initialization register"]
    pub iv0h: crate::Reg<iv0h::IV0H_SPEC>,
    #[doc = "0x44 - CAU initialization register"]
    pub iv0l: crate::Reg<iv0l::IV0L_SPEC>,
    #[doc = "0x48 - CAU initialization register"]
    pub iv1h: crate::Reg<iv1h::IV1H_SPEC>,
    #[doc = "0x4c - CAU initialization register"]
    pub iv1l: crate::Reg<iv1l::IV1L_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "CAU control register"]
pub mod ctl;
#[doc = "STAT0 register accessor: an alias for `Reg<STAT0_SPEC>`"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "CAU status register 0"]
pub mod stat0;
#[doc = "DI register accessor: an alias for `Reg<DI_SPEC>`"]
pub type DI = crate::Reg<di::DI_SPEC>;
#[doc = "CAU data input register"]
pub mod di;
#[doc = "DO register accessor: an alias for `Reg<DO_SPEC>`"]
pub type DO = crate::Reg<do_::DO_SPEC>;
#[doc = "CAU data output register"]
pub mod do_;
#[doc = "DMAEN register accessor: an alias for `Reg<DMAEN_SPEC>`"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "CAU DMA enable register"]
pub mod dmaen;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "CAU interrupt enable register"]
pub mod inten;
#[doc = "STAT1 register accessor: an alias for `Reg<STAT1_SPEC>`"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "CAU interrupt status flag register 1"]
pub mod stat1;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "CAU enable interrupt status flag register"]
pub mod intf;
#[doc = "KEY0H register accessor: an alias for `Reg<KEY0H_SPEC>`"]
pub type KEY0H = crate::Reg<key0h::KEY0H_SPEC>;
#[doc = "CAU key register"]
pub mod key0h;
#[doc = "KEY0L register accessor: an alias for `Reg<KEY0L_SPEC>`"]
pub type KEY0L = crate::Reg<key0l::KEY0L_SPEC>;
#[doc = "CAU key register"]
pub mod key0l;
#[doc = "KEY1H register accessor: an alias for `Reg<KEY1H_SPEC>`"]
pub type KEY1H = crate::Reg<key1h::KEY1H_SPEC>;
#[doc = "CAU key register"]
pub mod key1h;
#[doc = "KEY1L register accessor: an alias for `Reg<KEY1L_SPEC>`"]
pub type KEY1L = crate::Reg<key1l::KEY1L_SPEC>;
#[doc = "CAU key register"]
pub mod key1l;
#[doc = "KEY2H register accessor: an alias for `Reg<KEY2H_SPEC>`"]
pub type KEY2H = crate::Reg<key2h::KEY2H_SPEC>;
#[doc = "CAU key register"]
pub mod key2h;
#[doc = "KEY2L register accessor: an alias for `Reg<KEY2L_SPEC>`"]
pub type KEY2L = crate::Reg<key2l::KEY2L_SPEC>;
#[doc = "CAU key register"]
pub mod key2l;
#[doc = "KEY3H register accessor: an alias for `Reg<KEY3H_SPEC>`"]
pub type KEY3H = crate::Reg<key3h::KEY3H_SPEC>;
#[doc = "CAU key register"]
pub mod key3h;
#[doc = "KEY3L register accessor: an alias for `Reg<KEY3L_SPEC>`"]
pub type KEY3L = crate::Reg<key3l::KEY3L_SPEC>;
#[doc = "CAU key register"]
pub mod key3l;
#[doc = "IV0H register accessor: an alias for `Reg<IV0H_SPEC>`"]
pub type IV0H = crate::Reg<iv0h::IV0H_SPEC>;
#[doc = "CAU initialization register"]
pub mod iv0h;
#[doc = "IV0L register accessor: an alias for `Reg<IV0L_SPEC>`"]
pub type IV0L = crate::Reg<iv0l::IV0L_SPEC>;
#[doc = "CAU initialization register"]
pub mod iv0l;
#[doc = "IV1H register accessor: an alias for `Reg<IV1H_SPEC>`"]
pub type IV1H = crate::Reg<iv1h::IV1H_SPEC>;
#[doc = "CAU initialization register"]
pub mod iv1h;
#[doc = "IV1L register accessor: an alias for `Reg<IV1L_SPEC>`"]
pub type IV1L = crate::Reg<iv1l::IV1L_SPEC>;
#[doc = "CAU initialization register"]
pub mod iv1l;
