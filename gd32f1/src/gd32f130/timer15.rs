#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - control register 1"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    _reserved2: [u8; 0x06],
    #[doc = "0x0c - DMA and interrupt enable register"]
    pub dmainten: crate::Reg<dmainten::DMAINTEN_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - interrupt flag register"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Software event generation register"]
    pub swevg: crate::Reg<swevg::SWEVG_SPEC>,
    _reserved5: [u8; 0x02],
    _reserved_5_chctl0: [u8; 0x02],
    _reserved6: [u8; 0x06],
    #[doc = "0x20 - Channel control register 2"]
    pub chctl2: crate::Reg<chctl2::CHCTL2_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x24 - counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x28 - Prescaler register"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x2c - Counter auto reload register"]
    pub car: crate::Reg<car::CAR_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x30 - Counter repetition register"]
    pub crep: crate::Reg<crep::CREP_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x34 - Channel 0 capture/compare value register"]
    pub ch0cv: crate::Reg<ch0cv::CH0CV_SPEC>,
    _reserved12: [u8; 0x0e],
    #[doc = "0x44 - Channel Complementary Protection register"]
    pub cchp: crate::Reg<cchp::CCHP_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x48 - DMA configuration register"]
    pub dmacfg: crate::Reg<dmacfg::DMACFG_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x4c - DMA transfer buffer register"]
    pub dmatb: crate::Reg<dmatb::DMATB_SPEC>,
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
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
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
#[doc = "Software event generation register"]
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
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "Prescaler register"]
pub mod psc;
#[doc = "CAR register accessor: an alias for `Reg<CAR_SPEC>`"]
pub type CAR = crate::Reg<car::CAR_SPEC>;
#[doc = "Counter auto reload register"]
pub mod car;
#[doc = "CREP register accessor: an alias for `Reg<CREP_SPEC>`"]
pub type CREP = crate::Reg<crep::CREP_SPEC>;
#[doc = "Counter repetition register"]
pub mod crep;
#[doc = "CH0CV register accessor: an alias for `Reg<CH0CV_SPEC>`"]
pub type CH0CV = crate::Reg<ch0cv::CH0CV_SPEC>;
#[doc = "Channel 0 capture/compare value register"]
pub mod ch0cv;
#[doc = "CCHP register accessor: an alias for `Reg<CCHP_SPEC>`"]
pub type CCHP = crate::Reg<cchp::CCHP_SPEC>;
#[doc = "Channel Complementary Protection register"]
pub mod cchp;
#[doc = "DMACFG register accessor: an alias for `Reg<DMACFG_SPEC>`"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA configuration register"]
pub mod dmacfg;
#[doc = "DMATB register accessor: an alias for `Reg<DMATB_SPEC>`"]
pub type DMATB = crate::Reg<dmatb::DMATB_SPEC>;
#[doc = "DMA transfer buffer register"]
pub mod dmatb;
