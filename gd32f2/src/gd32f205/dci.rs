#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCI Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - DCI Status register 0"]
    pub stat0: STAT0,
    #[doc = "0x08 - DCI Status register1"]
    pub stat1: STAT1,
    #[doc = "0x0c - DCI inrerrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x10 - DCI Interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x14 - DCI Interrupt flag clear register"]
    pub intc: INTC,
    #[doc = "0x18 - DCI Synchronization codes register"]
    pub sc: SC,
    #[doc = "0x1c - DCI Synchronization codes unmask register"]
    pub scumsk: SCUMSK,
    #[doc = "0x20 - DCI Cropping window start position register"]
    pub cwspos: CWSPOS,
    #[doc = "0x24 - DCI Cropping window size register"]
    pub cwsz: CWSZ,
    #[doc = "0x28 - DCI DATA register"]
    pub data: DATA,
}
#[doc = "CTL (rw) register accessor: DCI Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "DCI Control register"]
pub mod ctl;
#[doc = "STAT0 (r) register accessor: DCI Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat0`]
module"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "DCI Status register 0"]
pub mod stat0;
#[doc = "STAT1 (r) register accessor: DCI Status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat1`]
module"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "DCI Status register1"]
pub mod stat1;
#[doc = "INTEN (rw) register accessor: DCI inrerrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "DCI inrerrupt enable register"]
pub mod inten;
#[doc = "INTF (r) register accessor: DCI Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "DCI Interrupt flag register"]
pub mod intf;
#[doc = "INTC (w) register accessor: DCI Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intc`]
module"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "DCI Interrupt flag clear register"]
pub mod intc;
#[doc = "SC (rw) register accessor: DCI Synchronization codes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sc`]
module"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "DCI Synchronization codes register"]
pub mod sc;
#[doc = "SCUMSK (rw) register accessor: DCI Synchronization codes unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scumsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scumsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scumsk`]
module"]
pub type SCUMSK = crate::Reg<scumsk::SCUMSK_SPEC>;
#[doc = "DCI Synchronization codes unmask register"]
pub mod scumsk;
#[doc = "CWSPOS (rw) register accessor: DCI Cropping window start position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwspos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwspos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cwspos`]
module"]
pub type CWSPOS = crate::Reg<cwspos::CWSPOS_SPEC>;
#[doc = "DCI Cropping window start position register"]
pub mod cwspos;
#[doc = "CWSZ (rw) register accessor: DCI Cropping window size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cwsz`]
module"]
pub type CWSZ = crate::Reg<cwsz::CWSZ_SPEC>;
#[doc = "DCI Cropping window size register"]
pub mod cwsz;
#[doc = "DATA (r) register accessor: DCI DATA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DCI DATA register"]
pub mod data;
