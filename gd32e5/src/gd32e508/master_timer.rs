#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHRTIMER Master_TIMER control register 0"]
    pub mtctl0: MTCTL0,
    #[doc = "0x04 - SHRTIMER Master_TIMER interrupt flag register"]
    pub mtintf: MTINTF,
    #[doc = "0x08 - SHRTIMER Master_TIMER interrupt flag clear register"]
    pub mtintc: MTINTC,
    #[doc = "0x0c - SHRTIMER Master_TIMER DMA and interrupt enable register"]
    pub mtdmainten: MTDMAINTEN,
    #[doc = "0x10 - SHRTIMER Master_TIMER counter register"]
    pub mtcnt: MTCNT,
    #[doc = "0x14 - SHRTIMER Master_TIMER counter auto reload register"]
    pub mtcar: MTCAR,
    #[doc = "0x18 - SHRTIMER Master_TIMER counter repetition register"]
    pub mtcrep: MTCREP,
    #[doc = "0x1c - SHRTIMER Master_TIMER compare 0 value register"]
    pub mtcmp0v: MTCMP0V,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - SHRTIMER Master_TIMER compare 1 value register"]
    pub mtcmp1v: MTCMP1V,
    #[doc = "0x28 - SHRTIMER Master_TIMER compare 2 value register"]
    pub mtcmp2v: MTCMP2V,
    #[doc = "0x2c - SHRTIMER Master_TIMER compare 3 value register"]
    pub mtcmp3v: MTCMP3V,
    _reserved11: [u8; 0x4c],
    #[doc = "0x7c - SHRTIMER Master_TIMER additional control register"]
    pub mtactl: MTACTL,
}
#[doc = "MTCTL0 (rw) register accessor: SHRTIMER Master_TIMER control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtctl0`]
module"]
pub type MTCTL0 = crate::Reg<mtctl0::MTCTL0_SPEC>;
#[doc = "SHRTIMER Master_TIMER control register 0"]
pub mod mtctl0;
#[doc = "MTINTF (r) register accessor: SHRTIMER Master_TIMER interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtintf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtintf`]
module"]
pub type MTINTF = crate::Reg<mtintf::MTINTF_SPEC>;
#[doc = "SHRTIMER Master_TIMER interrupt flag register"]
pub mod mtintf;
#[doc = "MTINTC (w) register accessor: SHRTIMER Master_TIMER interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtintc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtintc`]
module"]
pub type MTINTC = crate::Reg<mtintc::MTINTC_SPEC>;
#[doc = "SHRTIMER Master_TIMER interrupt flag clear register"]
pub mod mtintc;
#[doc = "MTDMAINTEN (rw) register accessor: SHRTIMER Master_TIMER DMA and interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtdmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtdmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtdmainten`]
module"]
pub type MTDMAINTEN = crate::Reg<mtdmainten::MTDMAINTEN_SPEC>;
#[doc = "SHRTIMER Master_TIMER DMA and interrupt enable register"]
pub mod mtdmainten;
#[doc = "MTCNT (rw) register accessor: SHRTIMER Master_TIMER counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtcnt`]
module"]
pub type MTCNT = crate::Reg<mtcnt::MTCNT_SPEC>;
#[doc = "SHRTIMER Master_TIMER counter register"]
pub mod mtcnt;
#[doc = "MTCAR (rw) register accessor: SHRTIMER Master_TIMER counter auto reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtcar`]
module"]
pub type MTCAR = crate::Reg<mtcar::MTCAR_SPEC>;
#[doc = "SHRTIMER Master_TIMER counter auto reload register"]
pub mod mtcar;
#[doc = "MTCREP (rw) register accessor: SHRTIMER Master_TIMER counter repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcrep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcrep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtcrep`]
module"]
pub type MTCREP = crate::Reg<mtcrep::MTCREP_SPEC>;
#[doc = "SHRTIMER Master_TIMER counter repetition register"]
pub mod mtcrep;
#[doc = "MTCMP0V (rw) register accessor: SHRTIMER Master_TIMER compare 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcmp0v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcmp0v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtcmp0v`]
module"]
pub type MTCMP0V = crate::Reg<mtcmp0v::MTCMP0V_SPEC>;
#[doc = "SHRTIMER Master_TIMER compare 0 value register"]
pub mod mtcmp0v;
#[doc = "MTCMP1V (rw) register accessor: SHRTIMER Master_TIMER compare 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcmp1v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcmp1v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtcmp1v`]
module"]
pub type MTCMP1V = crate::Reg<mtcmp1v::MTCMP1V_SPEC>;
#[doc = "SHRTIMER Master_TIMER compare 1 value register"]
pub mod mtcmp1v;
#[doc = "MTCMP2V (rw) register accessor: SHRTIMER Master_TIMER compare 2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcmp2v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcmp2v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtcmp2v`]
module"]
pub type MTCMP2V = crate::Reg<mtcmp2v::MTCMP2V_SPEC>;
#[doc = "SHRTIMER Master_TIMER compare 2 value register"]
pub mod mtcmp2v;
#[doc = "MTCMP3V (rw) register accessor: SHRTIMER Master_TIMER compare 3 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcmp3v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcmp3v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtcmp3v`]
module"]
pub type MTCMP3V = crate::Reg<mtcmp3v::MTCMP3V_SPEC>;
#[doc = "SHRTIMER Master_TIMER compare 3 value register"]
pub mod mtcmp3v;
#[doc = "MTACTL (rw) register accessor: SHRTIMER Master_TIMER additional control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtactl`]
module"]
pub type MTACTL = crate::Reg<mtactl::MTACTL_SPEC>;
#[doc = "SHRTIMER Master_TIMER additional control register"]
pub mod mtactl;
