#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    #[doc = "0x300 - IVREF control register"]
    pub ivref_ctl: IVREF_CTL,
}
#[doc = "IVREF_CTL (rw) register accessor: IVREF control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivref_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivref_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ivref_ctl`]
module"]
pub type IVREF_CTL = crate::Reg<ivref_ctl::IVREF_CTL_SPEC>;
#[doc = "IVREF control register"]
pub mod ivref_ctl;
