#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCI Control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - DCI Status register 0"]
    pub stat0: crate::Reg<stat0::STAT0_SPEC>,
    #[doc = "0x08 - DCI Status register1"]
    pub stat1: crate::Reg<stat1::STAT1_SPEC>,
    #[doc = "0x0c - DCI inrerrupt enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x10 - DCI Interrupt flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x14 - DCI Interrupt flag clear register"]
    pub intc: crate::Reg<intc::INTC_SPEC>,
    #[doc = "0x18 - DCI Synchronization codes register"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x1c - DCI Synchronization codes unmask register"]
    pub scumsk: crate::Reg<scumsk::SCUMSK_SPEC>,
    #[doc = "0x20 - DCI Cropping window start position register"]
    pub cwspos: crate::Reg<cwspos::CWSPOS_SPEC>,
    #[doc = "0x24 - DCI Cropping window size register"]
    pub cwsz: crate::Reg<cwsz::CWSZ_SPEC>,
    #[doc = "0x28 - DCI DATA register"]
    pub data: crate::Reg<data::DATA_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "DCI Control register"]
pub mod ctl;
#[doc = "STAT0 register accessor: an alias for `Reg<STAT0_SPEC>`"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "DCI Status register 0"]
pub mod stat0;
#[doc = "STAT1 register accessor: an alias for `Reg<STAT1_SPEC>`"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "DCI Status register1"]
pub mod stat1;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "DCI inrerrupt enable register"]
pub mod inten;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "DCI Interrupt flag register"]
pub mod intf;
#[doc = "INTC register accessor: an alias for `Reg<INTC_SPEC>`"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "DCI Interrupt flag clear register"]
pub mod intc;
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "DCI Synchronization codes register"]
pub mod sc;
#[doc = "SCUMSK register accessor: an alias for `Reg<SCUMSK_SPEC>`"]
pub type SCUMSK = crate::Reg<scumsk::SCUMSK_SPEC>;
#[doc = "DCI Synchronization codes unmask register"]
pub mod scumsk;
#[doc = "CWSPOS register accessor: an alias for `Reg<CWSPOS_SPEC>`"]
pub type CWSPOS = crate::Reg<cwspos::CWSPOS_SPEC>;
#[doc = "DCI Cropping window start position register"]
pub mod cwspos;
#[doc = "CWSZ register accessor: an alias for `Reg<CWSZ_SPEC>`"]
pub type CWSZ = crate::Reg<cwsz::CWSZ_SPEC>;
#[doc = "DCI Cropping window size register"]
pub mod cwsz;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DCI DATA register"]
pub mod data;
