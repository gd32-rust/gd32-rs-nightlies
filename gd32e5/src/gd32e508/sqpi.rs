#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SQPI Initial Register"]
    pub init: INIT,
    #[doc = "0x04 - SQPI Read Command Register"]
    pub rcmd: RCMD,
    #[doc = "0x08 - Write Command Register"]
    pub wcmd: WCMD,
    #[doc = "0x0c - ID Low Register"]
    pub idl: IDL,
    #[doc = "0x10 - ID High Register"]
    pub idh: IDH,
}
#[doc = "INIT (rw) register accessor: SQPI Initial Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`init`]
module"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "SQPI Initial Register"]
pub mod init;
#[doc = "RCMD (rw) register accessor: SQPI Read Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rcmd`]
module"]
pub type RCMD = crate::Reg<rcmd::RCMD_SPEC>;
#[doc = "SQPI Read Command Register"]
pub mod rcmd;
#[doc = "WCMD (rw) register accessor: Write Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wcmd`]
module"]
pub type WCMD = crate::Reg<wcmd::WCMD_SPEC>;
#[doc = "Write Command Register"]
pub mod wcmd;
#[doc = "IDL (r) register accessor: ID Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idl`]
module"]
pub type IDL = crate::Reg<idl::IDL_SPEC>;
#[doc = "ID Low Register"]
pub mod idl;
#[doc = "IDH (r) register accessor: ID High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idh`]
module"]
pub type IDH = crate::Reg<idh::IDH_SPEC>;
#[doc = "ID High Register"]
pub mod idh;
