#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt flag register (DMA_INTF)"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_INTC)"]
    pub intc: crate::Reg<intc::INTC_SPEC>,
    #[doc = "0x08 - DMA channel configuration register (DMA_CH0CTL0)"]
    pub ch0ctl: crate::Reg<ch0ctl::CH0CTL_SPEC>,
    #[doc = "0x0c - DMA channel 0 counter register"]
    pub ch0cnt: crate::Reg<ch0cnt::CH0CNT_SPEC>,
    #[doc = "0x10 - DMA channel 0 peripheral base address register"]
    pub ch0paddr: crate::Reg<ch0paddr::CH0PADDR_SPEC>,
    #[doc = "0x14 - DMA channel 0 memory base address register"]
    pub ch0maddr: crate::Reg<ch0maddr::CH0MADDR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA channel configuration register (DMA_CH1CTL0)"]
    pub ch1ctl: crate::Reg<ch1ctl::CH1CTL_SPEC>,
    #[doc = "0x20 - DMA channel 1 counter register"]
    pub ch1cnt: crate::Reg<ch1cnt::CH1CNT_SPEC>,
    #[doc = "0x24 - DMA channel 1 peripheral base address register"]
    pub ch1paddr: crate::Reg<ch1paddr::CH1PADDR_SPEC>,
    #[doc = "0x28 - DMA channel 1 memory base address register"]
    pub ch1maddr: crate::Reg<ch1maddr::CH1MADDR_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - DMA channel configuration register (DMA_CH2CTL0)"]
    pub ch2ctl: crate::Reg<ch2ctl::CH2CTL_SPEC>,
    #[doc = "0x34 - DMA channel 2 counter register"]
    pub ch2cnt: crate::Reg<ch2cnt::CH2CNT_SPEC>,
    #[doc = "0x38 - DMA channel 2 peripheral base address register"]
    pub ch2paddr: crate::Reg<ch2paddr::CH2PADDR_SPEC>,
    #[doc = "0x3c - DMA channel 2 memory base address register"]
    pub ch2maddr: crate::Reg<ch2maddr::CH2MADDR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x44 - DMA channel configuration register (DMA_CH3CTL0)"]
    pub ch3ctl: crate::Reg<ch3ctl::CH3CTL_SPEC>,
    #[doc = "0x48 - DMA channel 3 counter register"]
    pub ch3cnt: crate::Reg<ch3cnt::CH3CNT_SPEC>,
    #[doc = "0x4c - DMA channel 3 peripheral base address register"]
    pub ch3paddr: crate::Reg<ch3paddr::CH3PADDR_SPEC>,
    #[doc = "0x50 - DMA channel 3 memory base address register"]
    pub ch3maddr: crate::Reg<ch3maddr::CH3MADDR_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - DMA channel configuration register (DMA_CH4CTL0)"]
    pub ch4ctl: crate::Reg<ch4ctl::CH4CTL_SPEC>,
    #[doc = "0x5c - DMA channel 4 counter register"]
    pub ch4cnt: crate::Reg<ch4cnt::CH4CNT_SPEC>,
    #[doc = "0x60 - DMA channel 4 peripheral base address register"]
    pub ch4paddr: crate::Reg<ch4paddr::CH4PADDR_SPEC>,
    #[doc = "0x64 - DMA channel 4 memory base address register"]
    pub ch4maddr: crate::Reg<ch4maddr::CH4MADDR_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x6c - DMA channel configuration register (DMA_CH5CTL0)"]
    pub ch5ctl: crate::Reg<ch5ctl::CH5CTL_SPEC>,
    #[doc = "0x70 - DMA channel 5 counter register"]
    pub ch5cnt: crate::Reg<ch5cnt::CH5CNT_SPEC>,
    #[doc = "0x74 - DMA channel 5 peripheral base address register"]
    pub ch5paddr: crate::Reg<ch5paddr::CH5PADDR_SPEC>,
    #[doc = "0x78 - DMA channel 5 memory base address register"]
    pub ch5maddr: crate::Reg<ch5maddr::CH5MADDR_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x80 - DMA channel configuration register (DMA_CH6CTL0)"]
    pub ch6ctl: crate::Reg<ch6ctl::CH6CTL_SPEC>,
    #[doc = "0x84 - DMA channel 6 counter register"]
    pub ch6cnt: crate::Reg<ch6cnt::CH6CNT_SPEC>,
    #[doc = "0x88 - DMA channel 6 peripheral base address register"]
    pub ch6paddr: crate::Reg<ch6paddr::CH6PADDR_SPEC>,
    #[doc = "0x8c - DMA channel 6 memory base address register"]
    pub ch6maddr: crate::Reg<ch6maddr::CH6MADDR_SPEC>,
}
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "DMA interrupt flag register (DMA_INTF)"]
pub mod intf;
#[doc = "INTC register accessor: an alias for `Reg<INTC_SPEC>`"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_INTC)"]
pub mod intc;
#[doc = "CH0CTL register accessor: an alias for `Reg<CH0CTL_SPEC>`"]
pub type CH0CTL = crate::Reg<ch0ctl::CH0CTL_SPEC>;
#[doc = "DMA channel configuration register (DMA_CH0CTL0)"]
pub mod ch0ctl;
#[doc = "CH0CNT register accessor: an alias for `Reg<CH0CNT_SPEC>`"]
pub type CH0CNT = crate::Reg<ch0cnt::CH0CNT_SPEC>;
#[doc = "DMA channel 0 counter register"]
pub mod ch0cnt;
#[doc = "CH0PADDR register accessor: an alias for `Reg<CH0PADDR_SPEC>`"]
pub type CH0PADDR = crate::Reg<ch0paddr::CH0PADDR_SPEC>;
#[doc = "DMA channel 0 peripheral base address register"]
pub mod ch0paddr;
#[doc = "CH0MADDR register accessor: an alias for `Reg<CH0MADDR_SPEC>`"]
pub type CH0MADDR = crate::Reg<ch0maddr::CH0MADDR_SPEC>;
#[doc = "DMA channel 0 memory base address register"]
pub mod ch0maddr;
#[doc = "CH1CTL register accessor: an alias for `Reg<CH1CTL_SPEC>`"]
pub type CH1CTL = crate::Reg<ch1ctl::CH1CTL_SPEC>;
#[doc = "DMA channel configuration register (DMA_CH1CTL0)"]
pub mod ch1ctl;
#[doc = "CH1CNT register accessor: an alias for `Reg<CH1CNT_SPEC>`"]
pub type CH1CNT = crate::Reg<ch1cnt::CH1CNT_SPEC>;
#[doc = "DMA channel 1 counter register"]
pub mod ch1cnt;
#[doc = "CH1PADDR register accessor: an alias for `Reg<CH1PADDR_SPEC>`"]
pub type CH1PADDR = crate::Reg<ch1paddr::CH1PADDR_SPEC>;
#[doc = "DMA channel 1 peripheral base address register"]
pub mod ch1paddr;
#[doc = "CH1MADDR register accessor: an alias for `Reg<CH1MADDR_SPEC>`"]
pub type CH1MADDR = crate::Reg<ch1maddr::CH1MADDR_SPEC>;
#[doc = "DMA channel 1 memory base address register"]
pub mod ch1maddr;
#[doc = "CH2CTL register accessor: an alias for `Reg<CH2CTL_SPEC>`"]
pub type CH2CTL = crate::Reg<ch2ctl::CH2CTL_SPEC>;
#[doc = "DMA channel configuration register (DMA_CH2CTL0)"]
pub mod ch2ctl;
#[doc = "CH2CNT register accessor: an alias for `Reg<CH2CNT_SPEC>`"]
pub type CH2CNT = crate::Reg<ch2cnt::CH2CNT_SPEC>;
#[doc = "DMA channel 2 counter register"]
pub mod ch2cnt;
#[doc = "CH2PADDR register accessor: an alias for `Reg<CH2PADDR_SPEC>`"]
pub type CH2PADDR = crate::Reg<ch2paddr::CH2PADDR_SPEC>;
#[doc = "DMA channel 2 peripheral base address register"]
pub mod ch2paddr;
#[doc = "CH2MADDR register accessor: an alias for `Reg<CH2MADDR_SPEC>`"]
pub type CH2MADDR = crate::Reg<ch2maddr::CH2MADDR_SPEC>;
#[doc = "DMA channel 2 memory base address register"]
pub mod ch2maddr;
#[doc = "CH3CTL register accessor: an alias for `Reg<CH3CTL_SPEC>`"]
pub type CH3CTL = crate::Reg<ch3ctl::CH3CTL_SPEC>;
#[doc = "DMA channel configuration register (DMA_CH3CTL0)"]
pub mod ch3ctl;
#[doc = "CH3CNT register accessor: an alias for `Reg<CH3CNT_SPEC>`"]
pub type CH3CNT = crate::Reg<ch3cnt::CH3CNT_SPEC>;
#[doc = "DMA channel 3 counter register"]
pub mod ch3cnt;
#[doc = "CH3PADDR register accessor: an alias for `Reg<CH3PADDR_SPEC>`"]
pub type CH3PADDR = crate::Reg<ch3paddr::CH3PADDR_SPEC>;
#[doc = "DMA channel 3 peripheral base address register"]
pub mod ch3paddr;
#[doc = "CH3MADDR register accessor: an alias for `Reg<CH3MADDR_SPEC>`"]
pub type CH3MADDR = crate::Reg<ch3maddr::CH3MADDR_SPEC>;
#[doc = "DMA channel 3 memory base address register"]
pub mod ch3maddr;
#[doc = "CH4CTL register accessor: an alias for `Reg<CH4CTL_SPEC>`"]
pub type CH4CTL = crate::Reg<ch4ctl::CH4CTL_SPEC>;
#[doc = "DMA channel configuration register (DMA_CH4CTL0)"]
pub mod ch4ctl;
#[doc = "CH4CNT register accessor: an alias for `Reg<CH4CNT_SPEC>`"]
pub type CH4CNT = crate::Reg<ch4cnt::CH4CNT_SPEC>;
#[doc = "DMA channel 4 counter register"]
pub mod ch4cnt;
#[doc = "CH4PADDR register accessor: an alias for `Reg<CH4PADDR_SPEC>`"]
pub type CH4PADDR = crate::Reg<ch4paddr::CH4PADDR_SPEC>;
#[doc = "DMA channel 4 peripheral base address register"]
pub mod ch4paddr;
#[doc = "CH4MADDR register accessor: an alias for `Reg<CH4MADDR_SPEC>`"]
pub type CH4MADDR = crate::Reg<ch4maddr::CH4MADDR_SPEC>;
#[doc = "DMA channel 4 memory base address register"]
pub mod ch4maddr;
#[doc = "CH5CTL register accessor: an alias for `Reg<CH5CTL_SPEC>`"]
pub type CH5CTL = crate::Reg<ch5ctl::CH5CTL_SPEC>;
#[doc = "DMA channel configuration register (DMA_CH5CTL0)"]
pub mod ch5ctl;
#[doc = "CH5CNT register accessor: an alias for `Reg<CH5CNT_SPEC>`"]
pub type CH5CNT = crate::Reg<ch5cnt::CH5CNT_SPEC>;
#[doc = "DMA channel 5 counter register"]
pub mod ch5cnt;
#[doc = "CH5PADDR register accessor: an alias for `Reg<CH5PADDR_SPEC>`"]
pub type CH5PADDR = crate::Reg<ch5paddr::CH5PADDR_SPEC>;
#[doc = "DMA channel 5 peripheral base address register"]
pub mod ch5paddr;
#[doc = "CH5MADDR register accessor: an alias for `Reg<CH5MADDR_SPEC>`"]
pub type CH5MADDR = crate::Reg<ch5maddr::CH5MADDR_SPEC>;
#[doc = "DMA channel 5 memory base address register"]
pub mod ch5maddr;
#[doc = "CH6CTL register accessor: an alias for `Reg<CH6CTL_SPEC>`"]
pub type CH6CTL = crate::Reg<ch6ctl::CH6CTL_SPEC>;
#[doc = "DMA channel configuration register (DMA_CH6CTL0)"]
pub mod ch6ctl;
#[doc = "CH6CNT register accessor: an alias for `Reg<CH6CNT_SPEC>`"]
pub type CH6CNT = crate::Reg<ch6cnt::CH6CNT_SPEC>;
#[doc = "DMA channel 6 counter register"]
pub mod ch6cnt;
#[doc = "CH6PADDR register accessor: an alias for `Reg<CH6PADDR_SPEC>`"]
pub type CH6PADDR = crate::Reg<ch6paddr::CH6PADDR_SPEC>;
#[doc = "DMA channel 6 peripheral base address register"]
pub mod ch6paddr;
#[doc = "CH6MADDR register accessor: an alias for `Reg<CH6MADDR_SPEC>`"]
pub type CH6MADDR = crate::Reg<ch6maddr::CH6MADDR_SPEC>;
#[doc = "DMA channel 6 memory base address register"]
pub mod ch6maddr;
