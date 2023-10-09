#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Channel 0 Global interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIFC0_AW {
    #[doc = "1: Clears the GIF flag in INTF"]
    CLEAR = 1,
}
impl From<GIFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: GIFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIFC0` writer - Channel 0 Global interrupt flag clear"]
pub type GIFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GIFC0_AW>;
impl<'a, REG, const O: u8> GIFC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the GIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(GIFC0_AW::CLEAR)
    }
}
#[doc = "Channel 0 Full Transfer Finish clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTFIFC0_AW {
    #[doc = "1: Clears the FDFIF flag in INTF"]
    CLEAR = 1,
}
impl From<FTFIFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: FTFIFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTFIFC0` writer - Channel 0 Full Transfer Finish clear"]
pub type FTFIFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FTFIFC0_AW>;
impl<'a, REG, const O: u8> FTFIFC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the FDFIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FTFIFC0_AW::CLEAR)
    }
}
#[doc = "Channel 0 Half Transfer clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTFIFC0_AW {
    #[doc = "1: Clears the HTFIF flag in INTF"]
    CLEAR = 1,
}
impl From<HTFIFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: HTFIFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTFIFC0` writer - Channel 0 Half Transfer clear"]
pub type HTFIFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HTFIFC0_AW>;
impl<'a, REG, const O: u8> HTFIFC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the HTFIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HTFIFC0_AW::CLEAR)
    }
}
#[doc = "Channel 0 Error clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIFC0_AW {
    #[doc = "1: Clears the ERRIF flag in INTF"]
    CLEAR = 1,
}
impl From<ERRIFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRIFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIFC0` writer - Channel 0 Error clear"]
pub type ERRIFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIFC0_AW>;
impl<'a, REG, const O: u8> ERRIFC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ERRIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIFC0_AW::CLEAR)
    }
}
#[doc = "Field `ERRIFC1` writer - Channel 1 Error clear"]
pub use ERRIFC0_W as ERRIFC1_W;
#[doc = "Field `ERRIFC2` writer - Channel 2 Error clear"]
pub use ERRIFC0_W as ERRIFC2_W;
#[doc = "Field `ERRIFC3` writer - Channel 3 Error clear"]
pub use ERRIFC0_W as ERRIFC3_W;
#[doc = "Field `ERRIFC4` writer - Channel 4 Error clear"]
pub use ERRIFC0_W as ERRIFC4_W;
#[doc = "Field `FTFIFC1` writer - Channel 1 Full Transfer Finish clear"]
pub use FTFIFC0_W as FTFIFC1_W;
#[doc = "Field `FTFIFC2` writer - Channel 2 Full Transfer Finish clear"]
pub use FTFIFC0_W as FTFIFC2_W;
#[doc = "Field `FTFIFC3` writer - Channel 3 Full Transfer Finish clear"]
pub use FTFIFC0_W as FTFIFC3_W;
#[doc = "Field `FTFIFC4` writer - Channel 4 Full Transfer Finish clear"]
pub use FTFIFC0_W as FTFIFC4_W;
#[doc = "Field `GIFC1` writer - Channel 1 Global interrupt flag clear"]
pub use GIFC0_W as GIFC1_W;
#[doc = "Field `GIFC2` writer - Channel 2 Global interrupt flag clear"]
pub use GIFC0_W as GIFC2_W;
#[doc = "Field `GIFC3` writer - Channel 3 Global interrupt flag clear"]
pub use GIFC0_W as GIFC3_W;
#[doc = "Field `GIFC4` writer - Channel 4 Global interrupt flag clear"]
pub use GIFC0_W as GIFC4_W;
#[doc = "Field `HTFIFC1` writer - Channel 1 Half Transfer clear"]
pub use HTFIFC0_W as HTFIFC1_W;
#[doc = "Field `HTFIFC2` writer - Channel 2 Half Transfer clear"]
pub use HTFIFC0_W as HTFIFC2_W;
#[doc = "Field `HTFIFC3` writer - Channel 3 Half Transfer clear"]
pub use HTFIFC0_W as HTFIFC3_W;
#[doc = "Field `HTFIFC4` writer - Channel 4 Half Transfer clear"]
pub use HTFIFC0_W as HTFIFC4_W;
impl W {
    #[doc = "Bit 0 - Channel 0 Global interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn gifc0(&mut self) -> GIFC0_W<INTC_SPEC, 0> {
        GIFC0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 Full Transfer Finish clear"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc0(&mut self) -> FTFIFC0_W<INTC_SPEC, 1> {
        FTFIFC0_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc0(&mut self) -> HTFIFC0_W<INTC_SPEC, 2> {
        HTFIFC0_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn errifc0(&mut self) -> ERRIFC0_W<INTC_SPEC, 3> {
        ERRIFC0_W::new(self)
    }
    #[doc = "Bit 4 - Channel 1 Global interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn gifc1(&mut self) -> GIFC1_W<INTC_SPEC, 4> {
        GIFC1_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 Full Transfer Finish clear"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc1(&mut self) -> FTFIFC1_W<INTC_SPEC, 5> {
        FTFIFC1_W::new(self)
    }
    #[doc = "Bit 6 - Channel 1 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc1(&mut self) -> HTFIFC1_W<INTC_SPEC, 6> {
        HTFIFC1_W::new(self)
    }
    #[doc = "Bit 7 - Channel 1 Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn errifc1(&mut self) -> ERRIFC1_W<INTC_SPEC, 7> {
        ERRIFC1_W::new(self)
    }
    #[doc = "Bit 8 - Channel 2 Global interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn gifc2(&mut self) -> GIFC2_W<INTC_SPEC, 8> {
        GIFC2_W::new(self)
    }
    #[doc = "Bit 9 - Channel 2 Full Transfer Finish clear"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc2(&mut self) -> FTFIFC2_W<INTC_SPEC, 9> {
        FTFIFC2_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc2(&mut self) -> HTFIFC2_W<INTC_SPEC, 10> {
        HTFIFC2_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn errifc2(&mut self) -> ERRIFC2_W<INTC_SPEC, 11> {
        ERRIFC2_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 Global interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn gifc3(&mut self) -> GIFC3_W<INTC_SPEC, 12> {
        GIFC3_W::new(self)
    }
    #[doc = "Bit 13 - Channel 3 Full Transfer Finish clear"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc3(&mut self) -> FTFIFC3_W<INTC_SPEC, 13> {
        FTFIFC3_W::new(self)
    }
    #[doc = "Bit 14 - Channel 3 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc3(&mut self) -> HTFIFC3_W<INTC_SPEC, 14> {
        HTFIFC3_W::new(self)
    }
    #[doc = "Bit 15 - Channel 3 Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn errifc3(&mut self) -> ERRIFC3_W<INTC_SPEC, 15> {
        ERRIFC3_W::new(self)
    }
    #[doc = "Bit 16 - Channel 4 Global interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn gifc4(&mut self) -> GIFC4_W<INTC_SPEC, 16> {
        GIFC4_W::new(self)
    }
    #[doc = "Bit 17 - Channel 4 Full Transfer Finish clear"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc4(&mut self) -> FTFIFC4_W<INTC_SPEC, 17> {
        FTFIFC4_W::new(self)
    }
    #[doc = "Bit 18 - Channel 4 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc4(&mut self) -> HTFIFC4_W<INTC_SPEC, 18> {
        HTFIFC4_W::new(self)
    }
    #[doc = "Bit 19 - Channel 4 Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn errifc4(&mut self) -> ERRIFC4_W<INTC_SPEC, 19> {
        ERRIFC4_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA interrupt flag clear register (DMA_INTC)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
