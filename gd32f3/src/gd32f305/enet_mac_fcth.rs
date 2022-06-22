#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC flow control threshold register"]
    pub mac_fcth: crate::Reg<mac_fcth::MAC_FCTH_SPEC>,
}
#[doc = "MAC_FCTH register accessor: an alias for `Reg<MAC_FCTH_SPEC>`"]
pub type MAC_FCTH = crate::Reg<mac_fcth::MAC_FCTH_SPEC>;
#[doc = "Ethernet MAC flow control threshold register"]
pub mod mac_fcth;
