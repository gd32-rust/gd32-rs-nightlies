#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmainten: crate::Reg<dmainten::DMAINTEN_SPEC>,
    #[doc = "0x10 - interrupt flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x14 - event generation register"]
    pub swevg: crate::Reg<swevg::SWEVG_SPEC>,
    _reserved_4_chctl0: [u8; 0x04],
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - capture/compare enable register"]
    pub chctl2: crate::Reg<chctl2::CHCTL2_SPEC>,
    #[doc = "0x24 - counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - prescaler"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x2c - auto-reload register"]
    pub car: crate::Reg<car::CAR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - capture/compare register 0"]
    pub ch0cv: crate::Reg<ch0cv::CH0CV_SPEC>,
    _reserved10: [u8; 0x18],
    #[doc = "0x50 - channel input remap register"]
    pub irmp: crate::Reg<irmp::IRMP_SPEC>,
    _reserved11: [u8; 0xa8],
    #[doc = "0xfc - configuration register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register (input mode)"]
    #[inline(always)]
    pub fn chctl0_input(&self) -> &crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC>)
        }
    }
    #[doc = "0x18 - capture/compare mode register (output mode)"]
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
#[doc = "control register 1"]
pub mod ctl0;
#[doc = "DMAINTEN register accessor: an alias for `Reg<DMAINTEN_SPEC>`"]
pub type DMAINTEN = crate::Reg<dmainten::DMAINTEN_SPEC>;
#[doc = "DMA/Interrupt enable register"]
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
#[doc = "capture/compare mode register (output mode)"]
pub mod chctl0_output;
#[doc = "CHCTL0_Input register accessor: an alias for `Reg<CHCTL0_INPUT_SPEC>`"]
pub type CHCTL0_INPUT = crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC>;
#[doc = "capture/compare mode register (input mode)"]
pub mod chctl0_input;
#[doc = "CHCTL2 register accessor: an alias for `Reg<CHCTL2_SPEC>`"]
pub type CHCTL2 = crate::Reg<chctl2::CHCTL2_SPEC>;
#[doc = "capture/compare enable register"]
pub mod chctl2;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "CAR register accessor: an alias for `Reg<CAR_SPEC>`"]
pub type CAR = crate::Reg<car::CAR_SPEC>;
#[doc = "auto-reload register"]
pub mod car;
#[doc = "CH0CV register accessor: an alias for `Reg<CH0CV_SPEC>`"]
pub type CH0CV = crate::Reg<ch0cv::CH0CV_SPEC>;
#[doc = "capture/compare register 0"]
pub mod ch0cv;
#[doc = "IRMP register accessor: an alias for `Reg<IRMP_SPEC>`"]
pub type IRMP = crate::Reg<irmp::IRMP_SPEC>;
#[doc = "channel input remap register"]
pub mod irmp;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "configuration register"]
pub mod cfg;
