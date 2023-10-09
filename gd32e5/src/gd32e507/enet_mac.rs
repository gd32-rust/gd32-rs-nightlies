#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register (MAC_CFG)"]
    pub mac_cfg: MAC_CFG,
    #[doc = "0x04 - Ethernet MAC frame filter register (MAC_FRMF)"]
    pub mac_frmf: MAC_FRMF,
    #[doc = "0x08 - Ethernet MAC hash list high register"]
    pub mac_hlh: MAC_HLH,
    #[doc = "0x0c - Ethernet MAC hash list low register"]
    pub mac_hll: MAC_HLL,
    #[doc = "0x10 - Ethernet MAC PHY control register (MAC_PHY_CTL)"]
    pub mac_phy_ctl: MAC_PHY_CTL,
    #[doc = "0x14 - Ethernet MAC MII data register (MAC_PHY_DATA)"]
    pub mac_phy_data: MAC_PHY_DATA,
    #[doc = "0x18 - Ethernet MAC flow control register (MAC_FCTL)"]
    pub mac_fctl: MAC_FCTL,
    #[doc = "0x1c - Ethernet MAC VLAN tag register (MAC_VLT)"]
    pub mac_vlt: MAC_VLT,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register (MAC_RWFF)"]
    pub mac_rwff: MAC_RWFF,
    #[doc = "0x2c - Ethernet MAC wakeup management register (MAC_WUM)"]
    pub mac_wum: MAC_WUM,
    _reserved10: [u8; 0x04],
    #[doc = "0x34 - Ethernet MAC debug register (MAC_DBG)"]
    pub mac_dbg: MAC_DBG,
    #[doc = "0x38 - Ethernet MAC interrupt flag register (MAC_INTF)"]
    pub mac_intf: MAC_INTF,
    #[doc = "0x3c - Ethernet MAC interrupt mask register (MAC_INTMSK)"]
    pub mac_intmsk: MAC_INTMSK,
    #[doc = "0x40 - Ethernet MAC address 0 high register (MAC_ADDR0H)"]
    pub mac_addr0h: MAC_ADDR0H,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub mac_addr0l: MAC_ADDR0L,
    #[doc = "0x48 - Ethernet MAC address 1 high register (MAC_ADDR1H)"]
    pub mac_addr1h: MAC_ADDR1H,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub mac_addr1l: MAC_ADDR1L,
    #[doc = "0x50 - Ethernet MAC address 2 high register (MAC_ADDR2H)"]
    pub mac_addr2h: MAC_ADDR2H,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub mac_addr2l: MAC_ADDR2L,
    #[doc = "0x58 - Ethernet MAC address 3 high register (MAC_ADDR3H)"]
    pub mac_addr3h: MAC_ADDR3H,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub mac_addr3l: MAC_ADDR3L,
}
#[doc = "MAC_CFG (rw) register accessor: Ethernet MAC configuration register (MAC_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_cfg`]
module"]
pub type MAC_CFG = crate::Reg<mac_cfg::MAC_CFG_SPEC>;
#[doc = "Ethernet MAC configuration register (MAC_CFG)"]
pub mod mac_cfg;
#[doc = "MAC_FRMF (rw) register accessor: Ethernet MAC frame filter register (MAC_FRMF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_frmf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_frmf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_frmf`]
module"]
pub type MAC_FRMF = crate::Reg<mac_frmf::MAC_FRMF_SPEC>;
#[doc = "Ethernet MAC frame filter register (MAC_FRMF)"]
pub mod mac_frmf;
#[doc = "MAC_HLH (rw) register accessor: Ethernet MAC hash list high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_hlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_hlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_hlh`]
module"]
pub type MAC_HLH = crate::Reg<mac_hlh::MAC_HLH_SPEC>;
#[doc = "Ethernet MAC hash list high register"]
pub mod mac_hlh;
#[doc = "MAC_HLL (rw) register accessor: Ethernet MAC hash list low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_hll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_hll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_hll`]
module"]
pub type MAC_HLL = crate::Reg<mac_hll::MAC_HLL_SPEC>;
#[doc = "Ethernet MAC hash list low register"]
pub mod mac_hll;
#[doc = "MAC_PHY_CTL (rw) register accessor: Ethernet MAC PHY control register (MAC_PHY_CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_phy_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_phy_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_phy_ctl`]
module"]
pub type MAC_PHY_CTL = crate::Reg<mac_phy_ctl::MAC_PHY_CTL_SPEC>;
#[doc = "Ethernet MAC PHY control register (MAC_PHY_CTL)"]
pub mod mac_phy_ctl;
#[doc = "MAC_PHY_DATA (rw) register accessor: Ethernet MAC MII data register (MAC_PHY_DATA)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_phy_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_phy_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_phy_data`]
module"]
pub type MAC_PHY_DATA = crate::Reg<mac_phy_data::MAC_PHY_DATA_SPEC>;
#[doc = "Ethernet MAC MII data register (MAC_PHY_DATA)"]
pub mod mac_phy_data;
#[doc = "MAC_FCTL (rw) register accessor: Ethernet MAC flow control register (MAC_FCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_fctl`]
module"]
pub type MAC_FCTL = crate::Reg<mac_fctl::MAC_FCTL_SPEC>;
#[doc = "Ethernet MAC flow control register (MAC_FCTL)"]
pub mod mac_fctl;
#[doc = "MAC_VLT (rw) register accessor: Ethernet MAC VLAN tag register (MAC_VLT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_vlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_vlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_vlt`]
module"]
pub type MAC_VLT = crate::Reg<mac_vlt::MAC_VLT_SPEC>;
#[doc = "Ethernet MAC VLAN tag register (MAC_VLT)"]
pub mod mac_vlt;
#[doc = "MAC_RWFF (rw) register accessor: Ethernet MAC remote wakeup frame filter register (MAC_RWFF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_rwff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_rwff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_rwff`]
module"]
pub type MAC_RWFF = crate::Reg<mac_rwff::MAC_RWFF_SPEC>;
#[doc = "Ethernet MAC remote wakeup frame filter register (MAC_RWFF)"]
pub mod mac_rwff;
#[doc = "MAC_WUM (rw) register accessor: Ethernet MAC wakeup management register (MAC_WUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_wum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_wum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_wum`]
module"]
pub type MAC_WUM = crate::Reg<mac_wum::MAC_WUM_SPEC>;
#[doc = "Ethernet MAC wakeup management register (MAC_WUM)"]
pub mod mac_wum;
#[doc = "MAC_DBG (r) register accessor: Ethernet MAC debug register (MAC_DBG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_dbg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_dbg`]
module"]
pub type MAC_DBG = crate::Reg<mac_dbg::MAC_DBG_SPEC>;
#[doc = "Ethernet MAC debug register (MAC_DBG)"]
pub mod mac_dbg;
#[doc = "MAC_INTF (r) register accessor: Ethernet MAC interrupt flag register (MAC_INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_intf`]
module"]
pub type MAC_INTF = crate::Reg<mac_intf::MAC_INTF_SPEC>;
#[doc = "Ethernet MAC interrupt flag register (MAC_INTF)"]
pub mod mac_intf;
#[doc = "MAC_INTMSK (rw) register accessor: Ethernet MAC interrupt mask register (MAC_INTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_intmsk`]
module"]
pub type MAC_INTMSK = crate::Reg<mac_intmsk::MAC_INTMSK_SPEC>;
#[doc = "Ethernet MAC interrupt mask register (MAC_INTMSK)"]
pub mod mac_intmsk;
#[doc = "MAC_ADDR0H (rw) register accessor: Ethernet MAC address 0 high register (MAC_ADDR0H)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr0h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr0h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_addr0h`]
module"]
pub type MAC_ADDR0H = crate::Reg<mac_addr0h::MAC_ADDR0H_SPEC>;
#[doc = "Ethernet MAC address 0 high register (MAC_ADDR0H)"]
pub mod mac_addr0h;
#[doc = "MAC_ADDR0L (rw) register accessor: Ethernet MAC address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr0l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr0l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_addr0l`]
module"]
pub type MAC_ADDR0L = crate::Reg<mac_addr0l::MAC_ADDR0L_SPEC>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod mac_addr0l;
#[doc = "MAC_ADDR1H (rw) register accessor: Ethernet MAC address 1 high register (MAC_ADDR1H)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_addr1h`]
module"]
pub type MAC_ADDR1H = crate::Reg<mac_addr1h::MAC_ADDR1H_SPEC>;
#[doc = "Ethernet MAC address 1 high register (MAC_ADDR1H)"]
pub mod mac_addr1h;
#[doc = "MAC_ADDR1L (rw) register accessor: Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_addr1l`]
module"]
pub type MAC_ADDR1L = crate::Reg<mac_addr1l::MAC_ADDR1L_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod mac_addr1l;
#[doc = "MAC_ADDR2H (rw) register accessor: Ethernet MAC address 2 high register (MAC_ADDR2H)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_addr2h`]
module"]
pub type MAC_ADDR2H = crate::Reg<mac_addr2h::MAC_ADDR2H_SPEC>;
#[doc = "Ethernet MAC address 2 high register (MAC_ADDR2H)"]
pub mod mac_addr2h;
#[doc = "MAC_ADDR2L (rw) register accessor: Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_addr2l`]
module"]
pub type MAC_ADDR2L = crate::Reg<mac_addr2l::MAC_ADDR2L_SPEC>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod mac_addr2l;
#[doc = "MAC_ADDR3H (rw) register accessor: Ethernet MAC address 3 high register (MAC_ADDR3H)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr3h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr3h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_addr3h`]
module"]
pub type MAC_ADDR3H = crate::Reg<mac_addr3h::MAC_ADDR3H_SPEC>;
#[doc = "Ethernet MAC address 3 high register (MAC_ADDR3H)"]
pub mod mac_addr3h;
#[doc = "MAC_ADDR3L (rw) register accessor: Ethernet MAC address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr3l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr3l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_addr3l`]
module"]
pub type MAC_ADDR3L = crate::Reg<mac_addr3l::MAC_ADDR3L_SPEC>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod mac_addr3l;
