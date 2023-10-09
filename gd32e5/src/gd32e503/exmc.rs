#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR flash control register 0"]
    pub snctl0: SNCTL0,
    #[doc = "0x04 - SRAM/NOR flash timing configuration register 0"]
    pub sntcfg0: SNTCFG0,
    #[doc = "0x08 - SRAM/NOR flash control register 1"]
    pub snctl1: SNCTL1,
    #[doc = "0x0c - SRAM/NOR flash timing configuration register 1"]
    pub sntcfg1: SNTCFG1,
    #[doc = "0x10 - SRAM/NOR flash control register 2"]
    pub snctl2: SNCTL2,
    #[doc = "0x14 - SRAM/NOR flash timing configuration register 2"]
    pub sntcfg2: SNTCFG2,
    #[doc = "0x18 - SRAM/NOR flash control register 3"]
    pub snctl3: SNCTL3,
    #[doc = "0x1c - SRAM/NOR flash timing configuration register 3"]
    pub sntcfg3: SNTCFG3,
    _reserved8: [u8; 0x40],
    #[doc = "0x60 - NAND flash/PC card control register 1"]
    pub npctl1: NPCTL1,
    #[doc = "0x64 - NAND flash/PC card interrupt enable register 1"]
    pub npinten1: NPINTEN1,
    #[doc = "0x68 - NAND flash/PC card common space timing configuration register 1"]
    pub npctcfg1: NPCTCFG1,
    #[doc = "0x6c - NAND flash/PC card attribute space timing configuration register 1"]
    pub npatcfg1: NPATCFG1,
    _reserved12: [u8; 0x04],
    #[doc = "0x74 - NAND flash ECC register 1"]
    pub necc1: NECC1,
    _reserved13: [u8; 0x08],
    #[doc = "0x80 - NAND flash/PC card control register 2"]
    pub npctl2: NPCTL2,
    #[doc = "0x84 - NAND flash/PC card interrupt enable register 2"]
    pub npinten2: NPINTEN2,
    #[doc = "0x88 - NAND flash/PC card common space timing configuration register 2"]
    pub npctcfg2: NPCTCFG2,
    #[doc = "0x8c - NAND flash/PC card attribute space timing configuration register 2"]
    pub npatcfg2: NPATCFG2,
    _reserved17: [u8; 0x04],
    #[doc = "0x94 - NAND flash ECC register 2"]
    pub necc2: NECC2,
    _reserved18: [u8; 0x08],
    #[doc = "0xa0 - NAND flash/PC card control register 3"]
    pub npctl3: NPCTL3,
    #[doc = "0xa4 - NAND flash/PC card interrupt enable register 3"]
    pub npinten3: NPINTEN3,
    #[doc = "0xa8 - NAND flash/PC card common space timing configuration register 3"]
    pub npctcfg3: NPCTCFG3,
    #[doc = "0xac - NAND flash/PC card attribute space timing configuration register 3"]
    pub npatcfg3: NPATCFG3,
    #[doc = "0xb0 - PC card I/O space timing configuration register"]
    pub piotcfg3: PIOTCFG3,
    _reserved23: [u8; 0x50],
    #[doc = "0x104 - SRAM/NOR flash write timing configuration register 0"]
    pub snwtcfg0: SNWTCFG0,
    _reserved24: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR flash write timing configuration register 1"]
    pub snwtcfg1: SNWTCFG1,
    _reserved25: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR flash write timing configuration register 2"]
    pub snwtcfg2: SNWTCFG2,
    _reserved26: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR flash write timing configuration register 3"]
    pub snwtcfg3: SNWTCFG3,
}
#[doc = "SNCTL0 (rw) register accessor: SRAM/NOR flash control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snctl0`]
module"]
pub type SNCTL0 = crate::Reg<snctl0::SNCTL0_SPEC>;
#[doc = "SRAM/NOR flash control register 0"]
pub mod snctl0;
#[doc = "SNTCFG0 (rw) register accessor: SRAM/NOR flash timing configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sntcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sntcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sntcfg0`]
module"]
pub type SNTCFG0 = crate::Reg<sntcfg0::SNTCFG0_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 0"]
pub mod sntcfg0;
#[doc = "SNCTL1 (rw) register accessor: SRAM/NOR flash control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snctl1`]
module"]
pub type SNCTL1 = crate::Reg<snctl1::SNCTL1_SPEC>;
#[doc = "SRAM/NOR flash control register 1"]
pub mod snctl1;
#[doc = "SNTCFG1 (rw) register accessor: SRAM/NOR flash timing configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sntcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sntcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sntcfg1`]
module"]
pub type SNTCFG1 = crate::Reg<sntcfg1::SNTCFG1_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 1"]
pub mod sntcfg1;
#[doc = "SNCTL2 (rw) register accessor: SRAM/NOR flash control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snctl2`]
module"]
pub type SNCTL2 = crate::Reg<snctl2::SNCTL2_SPEC>;
#[doc = "SRAM/NOR flash control register 2"]
pub mod snctl2;
#[doc = "SNTCFG2 (rw) register accessor: SRAM/NOR flash timing configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sntcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sntcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sntcfg2`]
module"]
pub type SNTCFG2 = crate::Reg<sntcfg2::SNTCFG2_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 2"]
pub mod sntcfg2;
#[doc = "SNCTL3 (rw) register accessor: SRAM/NOR flash control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snctl3`]
module"]
pub type SNCTL3 = crate::Reg<snctl3::SNCTL3_SPEC>;
#[doc = "SRAM/NOR flash control register 3"]
pub mod snctl3;
#[doc = "SNTCFG3 (rw) register accessor: SRAM/NOR flash timing configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sntcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sntcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sntcfg3`]
module"]
pub type SNTCFG3 = crate::Reg<sntcfg3::SNTCFG3_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 3"]
pub mod sntcfg3;
#[doc = "SNWTCFG0 (rw) register accessor: SRAM/NOR flash write timing configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snwtcfg0`]
module"]
pub type SNWTCFG0 = crate::Reg<snwtcfg0::SNWTCFG0_SPEC>;
#[doc = "SRAM/NOR flash write timing configuration register 0"]
pub mod snwtcfg0;
#[doc = "SNWTCFG1 (rw) register accessor: SRAM/NOR flash write timing configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snwtcfg1`]
module"]
pub type SNWTCFG1 = crate::Reg<snwtcfg1::SNWTCFG1_SPEC>;
#[doc = "SRAM/NOR flash write timing configuration register 1"]
pub mod snwtcfg1;
#[doc = "SNWTCFG2 (rw) register accessor: SRAM/NOR flash write timing configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snwtcfg2`]
module"]
pub type SNWTCFG2 = crate::Reg<snwtcfg2::SNWTCFG2_SPEC>;
#[doc = "SRAM/NOR flash write timing configuration register 2"]
pub mod snwtcfg2;
#[doc = "SNWTCFG3 (rw) register accessor: SRAM/NOR flash write timing configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snwtcfg3`]
module"]
pub type SNWTCFG3 = crate::Reg<snwtcfg3::SNWTCFG3_SPEC>;
#[doc = "SRAM/NOR flash write timing configuration register 3"]
pub mod snwtcfg3;
#[doc = "NPCTL1 (rw) register accessor: NAND flash/PC card control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npctl1`]
module"]
pub type NPCTL1 = crate::Reg<npctl1::NPCTL1_SPEC>;
#[doc = "NAND flash/PC card control register 1"]
pub mod npctl1;
#[doc = "NPCTL2 (rw) register accessor: NAND flash/PC card control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npctl2`]
module"]
pub type NPCTL2 = crate::Reg<npctl2::NPCTL2_SPEC>;
#[doc = "NAND flash/PC card control register 2"]
pub mod npctl2;
#[doc = "NPCTL3 (rw) register accessor: NAND flash/PC card control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npctl3`]
module"]
pub type NPCTL3 = crate::Reg<npctl3::NPCTL3_SPEC>;
#[doc = "NAND flash/PC card control register 3"]
pub mod npctl3;
#[doc = "NPINTEN1 (rw) register accessor: NAND flash/PC card interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npinten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npinten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npinten1`]
module"]
pub type NPINTEN1 = crate::Reg<npinten1::NPINTEN1_SPEC>;
#[doc = "NAND flash/PC card interrupt enable register 1"]
pub mod npinten1;
#[doc = "NPINTEN2 (rw) register accessor: NAND flash/PC card interrupt enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npinten2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npinten2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npinten2`]
module"]
pub type NPINTEN2 = crate::Reg<npinten2::NPINTEN2_SPEC>;
#[doc = "NAND flash/PC card interrupt enable register 2"]
pub mod npinten2;
#[doc = "NPINTEN3 (rw) register accessor: NAND flash/PC card interrupt enable register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npinten3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npinten3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npinten3`]
module"]
pub type NPINTEN3 = crate::Reg<npinten3::NPINTEN3_SPEC>;
#[doc = "NAND flash/PC card interrupt enable register 3"]
pub mod npinten3;
#[doc = "NPCTCFG1 (rw) register accessor: NAND flash/PC card common space timing configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npctcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npctcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npctcfg1`]
module"]
pub type NPCTCFG1 = crate::Reg<npctcfg1::NPCTCFG1_SPEC>;
#[doc = "NAND flash/PC card common space timing configuration register 1"]
pub mod npctcfg1;
#[doc = "NPCTCFG2 (rw) register accessor: NAND flash/PC card common space timing configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npctcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npctcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npctcfg2`]
module"]
pub type NPCTCFG2 = crate::Reg<npctcfg2::NPCTCFG2_SPEC>;
#[doc = "NAND flash/PC card common space timing configuration register 2"]
pub mod npctcfg2;
#[doc = "NPCTCFG3 (rw) register accessor: NAND flash/PC card common space timing configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npctcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npctcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npctcfg3`]
module"]
pub type NPCTCFG3 = crate::Reg<npctcfg3::NPCTCFG3_SPEC>;
#[doc = "NAND flash/PC card common space timing configuration register 3"]
pub mod npctcfg3;
#[doc = "NPATCFG1 (rw) register accessor: NAND flash/PC card attribute space timing configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npatcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npatcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npatcfg1`]
module"]
pub type NPATCFG1 = crate::Reg<npatcfg1::NPATCFG1_SPEC>;
#[doc = "NAND flash/PC card attribute space timing configuration register 1"]
pub mod npatcfg1;
#[doc = "NPATCFG2 (rw) register accessor: NAND flash/PC card attribute space timing configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npatcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npatcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npatcfg2`]
module"]
pub type NPATCFG2 = crate::Reg<npatcfg2::NPATCFG2_SPEC>;
#[doc = "NAND flash/PC card attribute space timing configuration register 2"]
pub mod npatcfg2;
#[doc = "NPATCFG3 (rw) register accessor: NAND flash/PC card attribute space timing configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npatcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npatcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`npatcfg3`]
module"]
pub type NPATCFG3 = crate::Reg<npatcfg3::NPATCFG3_SPEC>;
#[doc = "NAND flash/PC card attribute space timing configuration register 3"]
pub mod npatcfg3;
#[doc = "PIOTCFG3 (rw) register accessor: PC card I/O space timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`piotcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`piotcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`piotcfg3`]
module"]
pub type PIOTCFG3 = crate::Reg<piotcfg3::PIOTCFG3_SPEC>;
#[doc = "PC card I/O space timing configuration register"]
pub mod piotcfg3;
#[doc = "NECC1 (r) register accessor: NAND flash ECC register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`necc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`necc1`]
module"]
pub type NECC1 = crate::Reg<necc1::NECC1_SPEC>;
#[doc = "NAND flash ECC register 1"]
pub mod necc1;
#[doc = "NECC2 (r) register accessor: NAND flash ECC register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`necc2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`necc2`]
module"]
pub type NECC2 = crate::Reg<necc2::NECC2_SPEC>;
#[doc = "NAND flash ECC register 2"]
pub mod necc2;
