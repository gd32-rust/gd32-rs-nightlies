#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register (MAC_CFG)"]
    pub mac_cfg: crate::Reg<mac_cfg::MAC_CFG_SPEC>,
    #[doc = "0x04 - Ethernet MAC frame filter register (MAC_FRMF)"]
    pub mac_frmf: crate::Reg<mac_frmf::MAC_FRMF_SPEC>,
    #[doc = "0x08 - Ethernet MAC hash list high register"]
    pub mac_hlh: crate::Reg<mac_hlh::MAC_HLH_SPEC>,
    #[doc = "0x0c - Ethernet MAC hash list low register"]
    pub mac_hll: crate::Reg<mac_hll::MAC_HLL_SPEC>,
    #[doc = "0x10 - Ethernet MAC PHY control register (MAC_PHY_CTL)"]
    pub mac_phy_ctl: crate::Reg<mac_phy_ctl::MAC_PHY_CTL_SPEC>,
    #[doc = "0x14 - Ethernet MAC MII data register (MAC_PHY_DATA)"]
    pub mac_phy_data: crate::Reg<mac_phy_data::MAC_PHY_DATA_SPEC>,
    #[doc = "0x18 - Ethernet MAC flow control register (MAC_FCTL)"]
    pub mac_fctl: crate::Reg<mac_fctl::MAC_FCTL_SPEC>,
    #[doc = "0x1c - Ethernet MAC VLAN tag register (MAC_VLT)"]
    pub mac_vlt: crate::Reg<mac_vlt::MAC_VLT_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register (MAC_RWFF)"]
    pub mac_rwff: crate::Reg<mac_rwff::MAC_RWFF_SPEC>,
    #[doc = "0x2c - Ethernet MAC wakeup management register (MAC_WUM)"]
    pub mac_wum: crate::Reg<mac_wum::MAC_WUM_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x38 - Ethernet MAC interrupt flag register (MAC_INTF)"]
    pub mac_intf: crate::Reg<mac_intf::MAC_INTF_SPEC>,
    #[doc = "0x3c - Ethernet MAC interrupt mask register (MAC_INTMSK)"]
    pub mac_intmsk: crate::Reg<mac_intmsk::MAC_INTMSK_SPEC>,
    #[doc = "0x40 - Ethernet MAC address 0 high register (MAC_ADDR0H)"]
    pub mac_addr0h: crate::Reg<mac_addr0h::MAC_ADDR0H_SPEC>,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub mac_addr0l: crate::Reg<mac_addr0l::MAC_ADDR0L_SPEC>,
    #[doc = "0x48 - Ethernet MAC address 1 high register (MAC_ADDR1H)"]
    pub mac_addr1h: crate::Reg<mac_addr1h::MAC_ADDR1H_SPEC>,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub mac_addr1l: crate::Reg<mac_addr1l::MAC_ADDR1L_SPEC>,
    #[doc = "0x50 - Ethernet MAC address 2 high register (MAC_ADDR2H)"]
    pub mac_addr2h: crate::Reg<mac_addr2h::MAC_ADDR2H_SPEC>,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub mac_addr2l: crate::Reg<mac_addr2l::MAC_ADDR2L_SPEC>,
    #[doc = "0x58 - Ethernet MAC address 3 high register (MAC_ADDR3H)"]
    pub mac_addr3h: crate::Reg<mac_addr3h::MAC_ADDR3H_SPEC>,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub mac_addr3l: crate::Reg<mac_addr3l::MAC_ADDR3L_SPEC>,
    _reserved20: [u8; 0x1020],
    #[doc = "0x1080 - Ethernet MAC flow control threshold register"]
    pub mac_fcth: crate::Reg<mac_fcth::MAC_FCTH_SPEC>,
}
#[doc = "MAC_CFG register accessor: an alias for `Reg<MAC_CFG_SPEC>`"]
pub type MAC_CFG = crate::Reg<mac_cfg::MAC_CFG_SPEC>;
#[doc = "Ethernet MAC configuration register (MAC_CFG)"]
pub mod mac_cfg;
#[doc = "MAC_FRMF register accessor: an alias for `Reg<MAC_FRMF_SPEC>`"]
pub type MAC_FRMF = crate::Reg<mac_frmf::MAC_FRMF_SPEC>;
#[doc = "Ethernet MAC frame filter register (MAC_FRMF)"]
pub mod mac_frmf;
#[doc = "MAC_HLH register accessor: an alias for `Reg<MAC_HLH_SPEC>`"]
pub type MAC_HLH = crate::Reg<mac_hlh::MAC_HLH_SPEC>;
#[doc = "Ethernet MAC hash list high register"]
pub mod mac_hlh;
#[doc = "MAC_HLL register accessor: an alias for `Reg<MAC_HLL_SPEC>`"]
pub type MAC_HLL = crate::Reg<mac_hll::MAC_HLL_SPEC>;
#[doc = "Ethernet MAC hash list low register"]
pub mod mac_hll;
#[doc = "MAC_PHY_CTL register accessor: an alias for `Reg<MAC_PHY_CTL_SPEC>`"]
pub type MAC_PHY_CTL = crate::Reg<mac_phy_ctl::MAC_PHY_CTL_SPEC>;
#[doc = "Ethernet MAC PHY control register (MAC_PHY_CTL)"]
pub mod mac_phy_ctl;
#[doc = "MAC_PHY_DATA register accessor: an alias for `Reg<MAC_PHY_DATA_SPEC>`"]
pub type MAC_PHY_DATA = crate::Reg<mac_phy_data::MAC_PHY_DATA_SPEC>;
#[doc = "Ethernet MAC MII data register (MAC_PHY_DATA)"]
pub mod mac_phy_data;
#[doc = "MAC_FCTL register accessor: an alias for `Reg<MAC_FCTL_SPEC>`"]
pub type MAC_FCTL = crate::Reg<mac_fctl::MAC_FCTL_SPEC>;
#[doc = "Ethernet MAC flow control register (MAC_FCTL)"]
pub mod mac_fctl;
#[doc = "MAC_FCTH register accessor: an alias for `Reg<MAC_FCTH_SPEC>`"]
pub type MAC_FCTH = crate::Reg<mac_fcth::MAC_FCTH_SPEC>;
#[doc = "Ethernet MAC flow control threshold register"]
pub mod mac_fcth;
#[doc = "MAC_VLT register accessor: an alias for `Reg<MAC_VLT_SPEC>`"]
pub type MAC_VLT = crate::Reg<mac_vlt::MAC_VLT_SPEC>;
#[doc = "Ethernet MAC VLAN tag register (MAC_VLT)"]
pub mod mac_vlt;
#[doc = "MAC_RWFF register accessor: an alias for `Reg<MAC_RWFF_SPEC>`"]
pub type MAC_RWFF = crate::Reg<mac_rwff::MAC_RWFF_SPEC>;
#[doc = "Ethernet MAC remote wakeup frame filter register (MAC_RWFF)"]
pub mod mac_rwff;
#[doc = "MAC_WUM register accessor: an alias for `Reg<MAC_WUM_SPEC>`"]
pub type MAC_WUM = crate::Reg<mac_wum::MAC_WUM_SPEC>;
#[doc = "Ethernet MAC wakeup management register (MAC_WUM)"]
pub mod mac_wum;
#[doc = "MAC_INTF register accessor: an alias for `Reg<MAC_INTF_SPEC>`"]
pub type MAC_INTF = crate::Reg<mac_intf::MAC_INTF_SPEC>;
#[doc = "Ethernet MAC interrupt flag register (MAC_INTF)"]
pub mod mac_intf;
#[doc = "MAC_INTMSK register accessor: an alias for `Reg<MAC_INTMSK_SPEC>`"]
pub type MAC_INTMSK = crate::Reg<mac_intmsk::MAC_INTMSK_SPEC>;
#[doc = "Ethernet MAC interrupt mask register (MAC_INTMSK)"]
pub mod mac_intmsk;
#[doc = "MAC_ADDR0H register accessor: an alias for `Reg<MAC_ADDR0H_SPEC>`"]
pub type MAC_ADDR0H = crate::Reg<mac_addr0h::MAC_ADDR0H_SPEC>;
#[doc = "Ethernet MAC address 0 high register (MAC_ADDR0H)"]
pub mod mac_addr0h;
#[doc = "MAC_ADDR0L register accessor: an alias for `Reg<MAC_ADDR0L_SPEC>`"]
pub type MAC_ADDR0L = crate::Reg<mac_addr0l::MAC_ADDR0L_SPEC>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod mac_addr0l;
#[doc = "MAC_ADDR1H register accessor: an alias for `Reg<MAC_ADDR1H_SPEC>`"]
pub type MAC_ADDR1H = crate::Reg<mac_addr1h::MAC_ADDR1H_SPEC>;
#[doc = "Ethernet MAC address 1 high register (MAC_ADDR1H)"]
pub mod mac_addr1h;
#[doc = "MAC_ADDR1L register accessor: an alias for `Reg<MAC_ADDR1L_SPEC>`"]
pub type MAC_ADDR1L = crate::Reg<mac_addr1l::MAC_ADDR1L_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod mac_addr1l;
#[doc = "MAC_ADDR2H register accessor: an alias for `Reg<MAC_ADDR2H_SPEC>`"]
pub type MAC_ADDR2H = crate::Reg<mac_addr2h::MAC_ADDR2H_SPEC>;
#[doc = "Ethernet MAC address 2 high register (MAC_ADDR2H)"]
pub mod mac_addr2h;
#[doc = "MAC_ADDR2L register accessor: an alias for `Reg<MAC_ADDR2L_SPEC>`"]
pub type MAC_ADDR2L = crate::Reg<mac_addr2l::MAC_ADDR2L_SPEC>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod mac_addr2l;
#[doc = "MAC_ADDR3H register accessor: an alias for `Reg<MAC_ADDR3H_SPEC>`"]
pub type MAC_ADDR3H = crate::Reg<mac_addr3h::MAC_ADDR3H_SPEC>;
#[doc = "Ethernet MAC address 3 high register (MAC_ADDR3H)"]
pub mod mac_addr3h;
#[doc = "MAC_ADDR3L register accessor: an alias for `Reg<MAC_ADDR3L_SPEC>`"]
pub type MAC_ADDR3L = crate::Reg<mac_addr3l::MAC_ADDR3L_SPEC>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod mac_addr3l;
