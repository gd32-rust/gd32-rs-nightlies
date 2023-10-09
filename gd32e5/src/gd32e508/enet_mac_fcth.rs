#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC flow control threshold register"]
    pub mac_fcth: MAC_FCTH,
}
#[doc = "MAC_FCTH (rw) register accessor: Ethernet MAC flow control threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_fcth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_fcth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_fcth`]
module"]
pub type MAC_FCTH = crate::Reg<mac_fcth::MAC_FCTH_SPEC>;
#[doc = "Ethernet MAC flow control threshold register"]
pub mod mac_fcth;
