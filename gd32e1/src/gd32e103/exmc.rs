#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR Flash control registers"]
    pub snctl: SNCTL,
    #[doc = "0x04 - SRAM/NOR Flash timing configuration registers"]
    pub sntcfg: SNTCFG,
    _reserved2: [u8; 0xfc],
    #[doc = "0x104 - SRAM/NOR write timing configuration registers"]
    pub snwtcfg: SNWTCFG,
}
#[doc = "SNCTL (rw) register accessor: SRAM/NOR Flash control registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snctl`]
module"]
pub type SNCTL = crate::Reg<snctl::SNCTL_SPEC>;
#[doc = "SRAM/NOR Flash control registers"]
pub mod snctl;
#[doc = "SNTCFG (rw) register accessor: SRAM/NOR Flash timing configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sntcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sntcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sntcfg`]
module"]
pub type SNTCFG = crate::Reg<sntcfg::SNTCFG_SPEC>;
#[doc = "SRAM/NOR Flash timing configuration registers"]
pub mod sntcfg;
#[doc = "SNWTCFG (rw) register accessor: SRAM/NOR write timing configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snwtcfg`]
module"]
pub type SNWTCFG = crate::Reg<snwtcfg::SNWTCFG_SPEC>;
#[doc = "SRAM/NOR write timing configuration registers"]
pub mod snwtcfg;
