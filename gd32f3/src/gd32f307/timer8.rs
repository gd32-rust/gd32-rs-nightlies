#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - slave mode configuration register"]
    pub smcfg: crate::Reg<smcfg::SMCFG_SPEC>,
    #[doc = "0x0c - DMA and interrupt enable register"]
    pub dmainten: crate::Reg<dmainten::DMAINTEN_SPEC>,
    #[doc = "0x10 - interrupt flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x14 - event generation register"]
    pub swevg: crate::Reg<swevg::SWEVG_SPEC>,
    _reserved_5_chctl0: [u8; 0x04],
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Channel control register 2"]
    pub chctl2: crate::Reg<chctl2::CHCTL2_SPEC>,
    #[doc = "0x24 - Counter register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - Prescaler register"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x2c - Counter auto reload register"]
    pub car: crate::Reg<car::CAR_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x34 - Channel 0 capture/compare value register"]
    pub ch0cv: crate::Reg<ch0cv::CH0CV_SPEC>,
    #[doc = "0x38 - Channel 1 capture/compare value register"]
    pub ch1cv: crate::Reg<ch1cv::CH1CV_SPEC>,
    _reserved12: [u8; 0xc0],
    #[doc = "0xfc - configuration register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x18 - Channel control register 0 (input mode)"]
    #[inline(always)]
    pub fn chctl0_input(&self) -> &crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC>)
        }
    }
    #[doc = "0x18 - Channel control register 0 (output mode)"]
    #[inline(always)]
    pub fn chctl0_output(&self) -> &crate::Reg<chctl0_output::CHCTL0_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<chctl0_output::CHCTL0_OUTPUT_SPEC>)
        }
    }
}
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "SMCFG register accessor: an alias for `Reg<SMCFG_SPEC>`"]
pub type SMCFG = crate::Reg<smcfg::SMCFG_SPEC>;
#[doc = "slave mode configuration register"]
pub mod smcfg;
#[doc = "DMAINTEN register accessor: an alias for `Reg<DMAINTEN_SPEC>`"]
pub type DMAINTEN = crate::Reg<dmainten::DMAINTEN_SPEC>;
#[doc = "DMA and interrupt enable register"]
pub mod dmainten;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "SWEVG register accessor: an alias for `Reg<SWEVG_SPEC>`"]
pub type SWEVG = crate::Reg<swevg::SWEVG_SPEC>;
#[doc = "event generation register"]
pub mod swevg;
#[doc = "CHCTL0_Output register accessor: an alias for `Reg<CHCTL0_OUTPUT_SPEC>`"]
pub type CHCTL0_OUTPUT = crate::Reg<chctl0_output::CHCTL0_OUTPUT_SPEC>;
#[doc = "Channel control register 0 (output mode)"]
pub mod chctl0_output;
#[doc = "CHCTL0_Input register accessor: an alias for `Reg<CHCTL0_INPUT_SPEC>`"]
pub type CHCTL0_INPUT = crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC>;
#[doc = "Channel control register 0 (input mode)"]
pub mod chctl0_input;
#[doc = "CHCTL2 register accessor: an alias for `Reg<CHCTL2_SPEC>`"]
pub type CHCTL2 = crate::Reg<chctl2::CHCTL2_SPEC>;
#[doc = "Channel control register 2"]
pub mod chctl2;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter register"]
pub mod cnt;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "Prescaler register"]
pub mod psc;
#[doc = "CAR register accessor: an alias for `Reg<CAR_SPEC>`"]
pub type CAR = crate::Reg<car::CAR_SPEC>;
#[doc = "Counter auto reload register"]
pub mod car;
#[doc = "CH0CV register accessor: an alias for `Reg<CH0CV_SPEC>`"]
pub type CH0CV = crate::Reg<ch0cv::CH0CV_SPEC>;
#[doc = "Channel 0 capture/compare value register"]
pub mod ch0cv;
#[doc = "CH1CV register accessor: an alias for `Reg<CH1CV_SPEC>`"]
pub type CH1CV = crate::Reg<ch1cv::CH1CV_SPEC>;
#[doc = "Channel 1 capture/compare value register"]
pub mod ch1cv;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "configuration register"]
pub mod cfg;
