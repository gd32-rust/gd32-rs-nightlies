#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System configuration register 0"]
    pub cfg0: crate::Reg<cfg0::CFG0_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: crate::Reg<extiss0::EXTISS0_SPEC>,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: crate::Reg<extiss1::EXTISS1_SPEC>,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: crate::Reg<extiss2::EXTISS2_SPEC>,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: crate::Reg<extiss3::EXTISS3_SPEC>,
    #[doc = "0x18 - System configuration register 2"]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
    _reserved6: [u8; 0xe4],
    #[doc = "0x100 - IRQ Latency register"]
    pub cpu_irq_lat: crate::Reg<cpu_irq_lat::CPU_IRQ_LAT_SPEC>,
}
#[doc = "CFG0 register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "System configuration register 0"]
pub mod cfg0;
#[doc = "EXTISS0 register accessor: an alias for `Reg<EXTISS0_SPEC>`"]
pub type EXTISS0 = crate::Reg<extiss0::EXTISS0_SPEC>;
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTISS1 register accessor: an alias for `Reg<EXTISS1_SPEC>`"]
pub type EXTISS1 = crate::Reg<extiss1::EXTISS1_SPEC>;
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTISS2 register accessor: an alias for `Reg<EXTISS2_SPEC>`"]
pub type EXTISS2 = crate::Reg<extiss2::EXTISS2_SPEC>;
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTISS3 register accessor: an alias for `Reg<EXTISS3_SPEC>`"]
pub type EXTISS3 = crate::Reg<extiss3::EXTISS3_SPEC>;
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "System configuration register 2"]
pub mod cfg2;
#[doc = "CPU_IRQ_LAT register accessor: an alias for `Reg<CPU_IRQ_LAT_SPEC>`"]
pub type CPU_IRQ_LAT = crate::Reg<cpu_irq_lat::CPU_IRQ_LAT_SPEC>;
#[doc = "IRQ Latency register"]
pub mod cpu_irq_lat;
