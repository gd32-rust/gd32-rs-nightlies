#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHRTIMER control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - SHRTIMER control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - SHRTIMER interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x0c - SHRTIMER interrupt flag clear register"]
    pub intc: INTC,
    #[doc = "0x10 - SHRTIMER interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x14 - SHRTIMER channel output enable register"]
    pub chouten: CHOUTEN,
    #[doc = "0x18 - SHRTIMER channel output disable register"]
    pub choutdis: CHOUTDIS,
    #[doc = "0x1c - SHRTIMER channel output disable flag register"]
    pub choutdisf: CHOUTDISF,
    #[doc = "0x20 - SHRTIMER bunch mode control register"]
    pub bmctl: BMCTL,
    #[doc = "0x24 - SHRTIMER bunch mode start trigger register"]
    pub bmstrg: BMSTRG,
    #[doc = "0x28 - SHRTIMER bunch mode compare value register"]
    pub bmcmpv: BMCMPV,
    #[doc = "0x2c - SHRTIMER bunch mode counter auto reload register"]
    pub bmcar: BMCAR,
    #[doc = "0x30 - SHRTIMER external event configuration register 0"]
    pub exevcfg0: EXEVCFG0,
    #[doc = "0x34 - SHRTIMER external event configuration register 1"]
    pub exevcfg1: EXEVCFG1,
    #[doc = "0x38 - SHRTIMER external event digital filter control register"]
    pub exevdfctl: EXEVDFCTL,
    #[doc = "0x3c - SHRTIMER trigger source 0 to ADC register"]
    pub adctrigs0: ADCTRIGS0,
    #[doc = "0x40 - SHRTIMER trigger source 1 to ADC register"]
    pub adctrigs1: ADCTRIGS1,
    #[doc = "0x44 - SHRTIMER trigger source 2 to ADC register"]
    pub adctrigs2: ADCTRIGS2,
    #[doc = "0x48 - SHRTIMER trigger source 3 to ADC register"]
    pub adctrigs3: ADCTRIGS3,
    #[doc = "0x4c - SHRTIMER DLL calibration control register"]
    pub dllcctl: DLLCCTL,
    #[doc = "0x50 - SHRTIMER fault input configuration register 0"]
    pub fltincfg0: FLTINCFG0,
    #[doc = "0x54 - SHRTIMER fault input configuration register 1"]
    pub fltincfg1: FLTINCFG1,
    #[doc = "0x58 - SHRTIMER DMA update Master_TIMER register"]
    pub dmaupmtr: DMAUPMTR,
    #[doc = "0x5c - SHRTIMER DMA update Slave_TIMER0 register"]
    pub dmaupst0r: DMAUPST0R,
    #[doc = "0x60 - SHRTIMER DMA update Slave_TIMER1 register"]
    pub dmaupst1r: DMAUPST1R,
    #[doc = "0x64 - SHRTIMER DMA update Slave_TIMER2 register"]
    pub dmaupst2r: DMAUPST2R,
    #[doc = "0x68 - SHRTIMER DMA update Slave_TIMER3 register"]
    pub dmaupst3r: DMAUPST3R,
    #[doc = "0x6c - SHRTIMER DMA update Slave_TIMER4 register"]
    pub dmaupst4r: DMAUPST4R,
    #[doc = "0x70 - SHRTIMER DMA transfer buffer register"]
    pub dmatb: DMATB,
}
#[doc = "CTL0 (rw) register accessor: SHRTIMER control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "SHRTIMER control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: SHRTIMER control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "SHRTIMER control register 1"]
pub mod ctl1;
#[doc = "INTF (r) register accessor: SHRTIMER interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "SHRTIMER interrupt flag register"]
pub mod intf;
#[doc = "INTC (w) register accessor: SHRTIMER interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intc`]
module"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "SHRTIMER interrupt flag clear register"]
pub mod intc;
#[doc = "INTEN (rw) register accessor: SHRTIMER interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "SHRTIMER interrupt enable register"]
pub mod inten;
#[doc = "CHOUTEN (rw) register accessor: SHRTIMER channel output enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chouten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chouten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chouten`]
module"]
pub type CHOUTEN = crate::Reg<chouten::CHOUTEN_SPEC>;
#[doc = "SHRTIMER channel output enable register"]
pub mod chouten;
#[doc = "CHOUTDIS (w) register accessor: SHRTIMER channel output disable register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`choutdis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`choutdis`]
module"]
pub type CHOUTDIS = crate::Reg<choutdis::CHOUTDIS_SPEC>;
#[doc = "SHRTIMER channel output disable register"]
pub mod choutdis;
#[doc = "CHOUTDISF (r) register accessor: SHRTIMER channel output disable flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`choutdisf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`choutdisf`]
module"]
pub type CHOUTDISF = crate::Reg<choutdisf::CHOUTDISF_SPEC>;
#[doc = "SHRTIMER channel output disable flag register"]
pub mod choutdisf;
#[doc = "BMCTL (rw) register accessor: SHRTIMER bunch mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bmctl`]
module"]
pub type BMCTL = crate::Reg<bmctl::BMCTL_SPEC>;
#[doc = "SHRTIMER bunch mode control register"]
pub mod bmctl;
#[doc = "BMSTRG (rw) register accessor: SHRTIMER bunch mode start trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmstrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmstrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bmstrg`]
module"]
pub type BMSTRG = crate::Reg<bmstrg::BMSTRG_SPEC>;
#[doc = "SHRTIMER bunch mode start trigger register"]
pub mod bmstrg;
#[doc = "BMCMPV (rw) register accessor: SHRTIMER bunch mode compare value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcmpv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcmpv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bmcmpv`]
module"]
pub type BMCMPV = crate::Reg<bmcmpv::BMCMPV_SPEC>;
#[doc = "SHRTIMER bunch mode compare value register"]
pub mod bmcmpv;
#[doc = "BMCAR (rw) register accessor: SHRTIMER bunch mode counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bmcar`]
module"]
pub type BMCAR = crate::Reg<bmcar::BMCAR_SPEC>;
#[doc = "SHRTIMER bunch mode counter auto reload register"]
pub mod bmcar;
#[doc = "EXEVCFG0 (rw) register accessor: SHRTIMER external event configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exevcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exevcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exevcfg0`]
module"]
pub type EXEVCFG0 = crate::Reg<exevcfg0::EXEVCFG0_SPEC>;
#[doc = "SHRTIMER external event configuration register 0"]
pub mod exevcfg0;
#[doc = "EXEVCFG1 (rw) register accessor: SHRTIMER external event configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exevcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exevcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exevcfg1`]
module"]
pub type EXEVCFG1 = crate::Reg<exevcfg1::EXEVCFG1_SPEC>;
#[doc = "SHRTIMER external event configuration register 1"]
pub mod exevcfg1;
#[doc = "EXEVDFCTL (rw) register accessor: SHRTIMER external event digital filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exevdfctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exevdfctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exevdfctl`]
module"]
pub type EXEVDFCTL = crate::Reg<exevdfctl::EXEVDFCTL_SPEC>;
#[doc = "SHRTIMER external event digital filter control register"]
pub mod exevdfctl;
#[doc = "ADCTRIGS0 (rw) register accessor: SHRTIMER trigger source 0 to ADC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrigs0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrigs0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adctrigs0`]
module"]
pub type ADCTRIGS0 = crate::Reg<adctrigs0::ADCTRIGS0_SPEC>;
#[doc = "SHRTIMER trigger source 0 to ADC register"]
pub mod adctrigs0;
#[doc = "ADCTRIGS1 (rw) register accessor: SHRTIMER trigger source 1 to ADC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrigs1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrigs1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adctrigs1`]
module"]
pub type ADCTRIGS1 = crate::Reg<adctrigs1::ADCTRIGS1_SPEC>;
#[doc = "SHRTIMER trigger source 1 to ADC register"]
pub mod adctrigs1;
#[doc = "ADCTRIGS2 (rw) register accessor: SHRTIMER trigger source 2 to ADC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrigs2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrigs2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adctrigs2`]
module"]
pub type ADCTRIGS2 = crate::Reg<adctrigs2::ADCTRIGS2_SPEC>;
#[doc = "SHRTIMER trigger source 2 to ADC register"]
pub mod adctrigs2;
#[doc = "ADCTRIGS3 (rw) register accessor: SHRTIMER trigger source 3 to ADC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrigs3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrigs3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adctrigs3`]
module"]
pub type ADCTRIGS3 = crate::Reg<adctrigs3::ADCTRIGS3_SPEC>;
#[doc = "SHRTIMER trigger source 3 to ADC register"]
pub mod adctrigs3;
#[doc = "DLLCCTL (rw) register accessor: SHRTIMER DLL calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dllcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dllcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dllcctl`]
module"]
pub type DLLCCTL = crate::Reg<dllcctl::DLLCCTL_SPEC>;
#[doc = "SHRTIMER DLL calibration control register"]
pub mod dllcctl;
#[doc = "FLTINCFG0 (rw) register accessor: SHRTIMER fault input configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltincfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltincfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fltincfg0`]
module"]
pub type FLTINCFG0 = crate::Reg<fltincfg0::FLTINCFG0_SPEC>;
#[doc = "SHRTIMER fault input configuration register 0"]
pub mod fltincfg0;
#[doc = "FLTINCFG1 (rw) register accessor: SHRTIMER fault input configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltincfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltincfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fltincfg1`]
module"]
pub type FLTINCFG1 = crate::Reg<fltincfg1::FLTINCFG1_SPEC>;
#[doc = "SHRTIMER fault input configuration register 1"]
pub mod fltincfg1;
#[doc = "DMAUPMTR (rw) register accessor: SHRTIMER DMA update Master_TIMER register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaupmtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaupmtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaupmtr`]
module"]
pub type DMAUPMTR = crate::Reg<dmaupmtr::DMAUPMTR_SPEC>;
#[doc = "SHRTIMER DMA update Master_TIMER register"]
pub mod dmaupmtr;
#[doc = "DMAUPST0R (rw) register accessor: SHRTIMER DMA update Slave_TIMER0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaupst0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaupst0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaupst0r`]
module"]
pub type DMAUPST0R = crate::Reg<dmaupst0r::DMAUPST0R_SPEC>;
#[doc = "SHRTIMER DMA update Slave_TIMER0 register"]
pub mod dmaupst0r;
#[doc = "DMAUPST1R (rw) register accessor: SHRTIMER DMA update Slave_TIMER1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaupst1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaupst1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaupst1r`]
module"]
pub type DMAUPST1R = crate::Reg<dmaupst1r::DMAUPST1R_SPEC>;
#[doc = "SHRTIMER DMA update Slave_TIMER1 register"]
pub mod dmaupst1r;
#[doc = "DMAUPST2R (rw) register accessor: SHRTIMER DMA update Slave_TIMER2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaupst2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaupst2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaupst2r`]
module"]
pub type DMAUPST2R = crate::Reg<dmaupst2r::DMAUPST2R_SPEC>;
#[doc = "SHRTIMER DMA update Slave_TIMER2 register"]
pub mod dmaupst2r;
#[doc = "DMAUPST3R (rw) register accessor: SHRTIMER DMA update Slave_TIMER3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaupst3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaupst3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaupst3r`]
module"]
pub type DMAUPST3R = crate::Reg<dmaupst3r::DMAUPST3R_SPEC>;
#[doc = "SHRTIMER DMA update Slave_TIMER3 register"]
pub mod dmaupst3r;
#[doc = "DMAUPST4R (rw) register accessor: SHRTIMER DMA update Slave_TIMER4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaupst4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaupst4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaupst4r`]
module"]
pub type DMAUPST4R = crate::Reg<dmaupst4r::DMAUPST4R_SPEC>;
#[doc = "SHRTIMER DMA update Slave_TIMER4 register"]
pub mod dmaupst4r;
#[doc = "DMATB (w) register accessor: SHRTIMER DMA transfer buffer register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmatb`]
module"]
pub type DMATB = crate::Reg<dmatb::DMATB_SPEC>;
#[doc = "SHRTIMER DMA transfer buffer register"]
pub mod dmatb;
