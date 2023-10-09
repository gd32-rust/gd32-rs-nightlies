#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event control register"]
    pub ec: EC,
    #[doc = "0x04 - AFIO port configuration register 0"]
    pub pcf0: PCF0,
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: EXTISS0,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: EXTISS1,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: EXTISS2,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: EXTISS3,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - AFIO port configuration register 1"]
    pub pcf1: PCF1,
    #[doc = "0x20 - IO compensation control register"]
    pub cpsctl: CPSCTL,
    _reserved8: [u8; 0x18],
    #[doc = "0x3c - AFIO port configuration register A"]
    pub pcfa: PCFA,
    #[doc = "0x40 - AFIO port configuration register B"]
    pub pcfb: PCFB,
    #[doc = "0x44 - AFIO port configuration register C"]
    pub pcfc: PCFC,
    #[doc = "0x48 - AFIO port configuration register D"]
    pub pcfd: PCFD,
    #[doc = "0x4c - AFIO port configuration register E"]
    pub pcfe: PCFE,
    _reserved13: [u8; 0x04],
    #[doc = "0x54 - AFIO port configuration register G"]
    pub pcfg: PCFG,
}
#[doc = "EC (rw) register accessor: Event control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ec`]
module"]
pub type EC = crate::Reg<ec::EC_SPEC>;
#[doc = "Event control register"]
pub mod ec;
#[doc = "PCF0 (rw) register accessor: AFIO port configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcf0`]
module"]
pub type PCF0 = crate::Reg<pcf0::PCF0_SPEC>;
#[doc = "AFIO port configuration register 0"]
pub mod pcf0;
#[doc = "EXTISS0 (rw) register accessor: EXTI sources selection register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extiss0`]
module"]
pub type EXTISS0 = crate::Reg<extiss0::EXTISS0_SPEC>;
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTISS1 (rw) register accessor: EXTI sources selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extiss1`]
module"]
pub type EXTISS1 = crate::Reg<extiss1::EXTISS1_SPEC>;
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTISS2 (rw) register accessor: EXTI sources selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extiss2`]
module"]
pub type EXTISS2 = crate::Reg<extiss2::EXTISS2_SPEC>;
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTISS3 (rw) register accessor: EXTI sources selection register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extiss3`]
module"]
pub type EXTISS3 = crate::Reg<extiss3::EXTISS3_SPEC>;
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "PCF1 (rw) register accessor: AFIO port configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcf1`]
module"]
pub type PCF1 = crate::Reg<pcf1::PCF1_SPEC>;
#[doc = "AFIO port configuration register 1"]
pub mod pcf1;
#[doc = "CPSCTL (rw) register accessor: IO compensation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpsctl`]
module"]
pub type CPSCTL = crate::Reg<cpsctl::CPSCTL_SPEC>;
#[doc = "IO compensation control register"]
pub mod cpsctl;
#[doc = "PCFA (rw) register accessor: AFIO port configuration register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcfa`]
module"]
pub type PCFA = crate::Reg<pcfa::PCFA_SPEC>;
#[doc = "AFIO port configuration register A"]
pub mod pcfa;
#[doc = "PCFB (rw) register accessor: AFIO port configuration register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcfb`]
module"]
pub type PCFB = crate::Reg<pcfb::PCFB_SPEC>;
#[doc = "AFIO port configuration register B"]
pub mod pcfb;
#[doc = "PCFC (rw) register accessor: AFIO port configuration register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcfc`]
module"]
pub type PCFC = crate::Reg<pcfc::PCFC_SPEC>;
#[doc = "AFIO port configuration register C"]
pub mod pcfc;
#[doc = "PCFD (rw) register accessor: AFIO port configuration register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcfd`]
module"]
pub type PCFD = crate::Reg<pcfd::PCFD_SPEC>;
#[doc = "AFIO port configuration register D"]
pub mod pcfd;
#[doc = "PCFE (rw) register accessor: AFIO port configuration register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcfe`]
module"]
pub type PCFE = crate::Reg<pcfe::PCFE_SPEC>;
#[doc = "AFIO port configuration register E"]
pub mod pcfe;
#[doc = "PCFG (rw) register accessor: AFIO port configuration register G\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcfg`]
module"]
pub type PCFG = crate::Reg<pcfg::PCFG_SPEC>;
#[doc = "AFIO port configuration register G"]
pub mod pcfg;
