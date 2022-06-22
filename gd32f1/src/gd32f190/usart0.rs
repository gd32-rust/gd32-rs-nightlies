#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x04 - Control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    #[doc = "0x08 - Control register 2"]
    pub ctl2: crate::Reg<ctl2::CTL2_SPEC>,
    #[doc = "0x0c - Baud rate generator register"]
    pub baud: crate::Reg<baud::BAUD_SPEC>,
    #[doc = "0x10 - Prescaler and guard time configuration register"]
    pub gp: crate::Reg<gp::GP_SPEC>,
    #[doc = "0x14 - Receiver timeout register"]
    pub rt: crate::Reg<rt::RT_SPEC>,
    #[doc = "0x18 - Request register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x1c - Interrupt & status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x20 - Interrupt flag clear register"]
    pub intc: crate::Reg<intc::INTC_SPEC>,
    #[doc = "0x24 - Receive data register"]
    pub rdata: crate::Reg<rdata::RDATA_SPEC>,
    #[doc = "0x28 - Transmit data register"]
    pub tdata: crate::Reg<tdata::TDATA_SPEC>,
}
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "CTL2 register accessor: an alias for `Reg<CTL2_SPEC>`"]
pub type CTL2 = crate::Reg<ctl2::CTL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctl2;
#[doc = "BAUD register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud rate generator register"]
pub mod baud;
#[doc = "GP register accessor: an alias for `Reg<GP_SPEC>`"]
pub type GP = crate::Reg<gp::GP_SPEC>;
#[doc = "Prescaler and guard time configuration register"]
pub mod gp;
#[doc = "RT register accessor: an alias for `Reg<RT_SPEC>`"]
pub type RT = crate::Reg<rt::RT_SPEC>;
#[doc = "Receiver timeout register"]
pub mod rt;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Request register"]
pub mod cmd;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Interrupt & status register"]
pub mod stat;
#[doc = "INTC register accessor: an alias for `Reg<INTC_SPEC>`"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "RDATA register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "Receive data register"]
pub mod rdata;
#[doc = "TDATA register accessor: an alias for `Reg<TDATA_SPEC>`"]
pub type TDATA = crate::Reg<tdata::TDATA_SPEC>;
#[doc = "Transmit data register"]
pub mod tdata;
