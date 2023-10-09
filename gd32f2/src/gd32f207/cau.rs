#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAU control register"]
    pub ctl: CTL,
    #[doc = "0x04 - CAU status register 0"]
    pub stat0: STAT0,
    #[doc = "0x08 - CAU data input register"]
    pub di: DI,
    #[doc = "0x0c - CAU data output register"]
    pub do_: DO,
    #[doc = "0x10 - CAU DMA enable register"]
    pub dmaen: DMAEN,
    #[doc = "0x14 - CAU interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x18 - CAU interrupt status flag register 1"]
    pub stat1: STAT1,
    #[doc = "0x1c - CAU enable interrupt status flag register"]
    pub intf: INTF,
    #[doc = "0x20 - CAU key register"]
    pub key0h: KEY0H,
    #[doc = "0x24 - CAU key register"]
    pub key0l: KEY0L,
    #[doc = "0x28 - CAU key register"]
    pub key1h: KEY1H,
    #[doc = "0x2c - CAU key register"]
    pub key1l: KEY1L,
    #[doc = "0x30 - CAU key register"]
    pub key2h: KEY2H,
    #[doc = "0x34 - CAU key register"]
    pub key2l: KEY2L,
    #[doc = "0x38 - CAU key register"]
    pub key3h: KEY3H,
    #[doc = "0x3c - CAU key register"]
    pub key3l: KEY3L,
    #[doc = "0x40 - CAU initialization register"]
    pub iv0h: IV0H,
    #[doc = "0x44 - CAU initialization register"]
    pub iv0l: IV0L,
    #[doc = "0x48 - CAU initialization register"]
    pub iv1h: IV1H,
    #[doc = "0x4c - CAU initialization register"]
    pub iv1l: IV1L,
}
#[doc = "CTL (rw) register accessor: CAU control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "CAU control register"]
pub mod ctl;
#[doc = "STAT0 (r) register accessor: CAU status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat0`]
module"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "CAU status register 0"]
pub mod stat0;
#[doc = "DI (rw) register accessor: CAU data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`di::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`di::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`di`]
module"]
pub type DI = crate::Reg<di::DI_SPEC>;
#[doc = "CAU data input register"]
pub mod di;
#[doc = "DO (r) register accessor: CAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`do_`]
module"]
pub type DO = crate::Reg<do_::DO_SPEC>;
#[doc = "CAU data output register"]
pub mod do_;
#[doc = "DMAEN (rw) register accessor: CAU DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaen`]
module"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "CAU DMA enable register"]
pub mod dmaen;
#[doc = "INTEN (rw) register accessor: CAU interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "CAU interrupt enable register"]
pub mod inten;
#[doc = "STAT1 (r) register accessor: CAU interrupt status flag register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat1`]
module"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "CAU interrupt status flag register 1"]
pub mod stat1;
#[doc = "INTF (r) register accessor: CAU enable interrupt status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "CAU enable interrupt status flag register"]
pub mod intf;
#[doc = "KEY0H (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key0h`]
module"]
pub type KEY0H = crate::Reg<key0h::KEY0H_SPEC>;
#[doc = "CAU key register"]
pub mod key0h;
#[doc = "KEY0L (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key0l`]
module"]
pub type KEY0L = crate::Reg<key0l::KEY0L_SPEC>;
#[doc = "CAU key register"]
pub mod key0l;
#[doc = "KEY1H (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key1h`]
module"]
pub type KEY1H = crate::Reg<key1h::KEY1H_SPEC>;
#[doc = "CAU key register"]
pub mod key1h;
#[doc = "KEY1L (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key1l`]
module"]
pub type KEY1L = crate::Reg<key1l::KEY1L_SPEC>;
#[doc = "CAU key register"]
pub mod key1l;
#[doc = "KEY2H (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key2h`]
module"]
pub type KEY2H = crate::Reg<key2h::KEY2H_SPEC>;
#[doc = "CAU key register"]
pub mod key2h;
#[doc = "KEY2L (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key2l`]
module"]
pub type KEY2L = crate::Reg<key2l::KEY2L_SPEC>;
#[doc = "CAU key register"]
pub mod key2l;
#[doc = "KEY3H (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key3h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key3h`]
module"]
pub type KEY3H = crate::Reg<key3h::KEY3H_SPEC>;
#[doc = "CAU key register"]
pub mod key3h;
#[doc = "KEY3L (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key3l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key3l`]
module"]
pub type KEY3L = crate::Reg<key3l::KEY3L_SPEC>;
#[doc = "CAU key register"]
pub mod key3l;
#[doc = "IV0H (rw) register accessor: CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iv0h`]
module"]
pub type IV0H = crate::Reg<iv0h::IV0H_SPEC>;
#[doc = "CAU initialization register"]
pub mod iv0h;
#[doc = "IV0L (rw) register accessor: CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iv0l`]
module"]
pub type IV0L = crate::Reg<iv0l::IV0L_SPEC>;
#[doc = "CAU initialization register"]
pub mod iv0l;
#[doc = "IV1H (rw) register accessor: CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iv1h`]
module"]
pub type IV1H = crate::Reg<iv1h::IV1H_SPEC>;
#[doc = "CAU initialization register"]
pub mod iv1h;
#[doc = "IV1L (rw) register accessor: CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iv1l`]
module"]
pub type IV1L = crate::Reg<iv1l::IV1L_SPEC>;
#[doc = "CAU initialization register"]
pub mod iv1l;
