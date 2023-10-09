#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - CMP1 control and status register"]
    pub cmp1_cs: CMP1_CS,
    _reserved1: [u8; 0x04],
    #[doc = "0x28 - CMP3 control and status register"]
    pub cmp3_cs: CMP3_CS,
    _reserved2: [u8; 0x04],
    #[doc = "0x30 - CMP5 control and status register"]
    pub cmp5_cs: CMP5_CS,
}
#[doc = "CMP1_CS (rw) register accessor: CMP1 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp1_cs`]
module"]
pub type CMP1_CS = crate::Reg<cmp1_cs::CMP1_CS_SPEC>;
#[doc = "CMP1 control and status register"]
pub mod cmp1_cs;
#[doc = "CMP3_CS (rw) register accessor: CMP3 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp3_cs`]
module"]
pub type CMP3_CS = crate::Reg<cmp3_cs::CMP3_CS_SPEC>;
#[doc = "CMP3 control and status register"]
pub mod cmp3_cs;
#[doc = "CMP5_CS (rw) register accessor: CMP5 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp5_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp5_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp5_cs`]
module"]
pub type CMP5_CS = crate::Reg<cmp5_cs::CMP5_CS_SPEC>;
#[doc = "CMP5 control and status register"]
pub mod cmp5_cs;
