#[doc = "Register `INTC` writer"]
pub struct W(crate::W<INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Global interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type GIFC0_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, GIFC0_AW, 0>;
impl<'a> GIFC0_W<'a> {
    #[doc = "Clears the GIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIFC0_AW::CLEAR)
    }
}
#[doc = "Channel 1 Global interrupt flag clear"]
pub use GIFC0_AW as GIFC1_AW;
#[doc = "Channel 2 Global interrupt flag clear"]
pub use GIFC0_AW as GIFC2_AW;
#[doc = "Channel 3 Global interrupt flag clear"]
pub use GIFC0_AW as GIFC3_AW;
#[doc = "Channel 4 Global interrupt flag clear"]
pub use GIFC0_AW as GIFC4_AW;
#[doc = "Field `GIFC1` writer - Channel 1 Global interrupt flag clear"]
pub use GIFC0_W as GIFC1_W;
#[doc = "Field `GIFC2` writer - Channel 2 Global interrupt flag clear"]
pub use GIFC0_W as GIFC2_W;
#[doc = "Field `GIFC3` writer - Channel 3 Global interrupt flag clear"]
pub use GIFC0_W as GIFC3_W;
#[doc = "Field `GIFC4` writer - Channel 4 Global interrupt flag clear"]
pub use GIFC0_W as GIFC4_W;
#[doc = "Channel 0 Full Transfer Finish clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type FTFIFC0_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, FTFIFC0_AW, 1>;
impl<'a> FTFIFC0_W<'a> {
    #[doc = "Clears the FDFIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FTFIFC0_AW::CLEAR)
    }
}
#[doc = "Channel 1 Full Transfer Finish clear"]
pub use FTFIFC0_AW as FTFIFC1_AW;
#[doc = "Channel 2 Full Transfer Finish clear"]
pub use FTFIFC0_AW as FTFIFC2_AW;
#[doc = "Channel 3 Full Transfer Finish clear"]
pub use FTFIFC0_AW as FTFIFC3_AW;
#[doc = "Channel 4 Full Transfer Finish clear"]
pub use FTFIFC0_AW as FTFIFC4_AW;
#[doc = "Field `FTFIFC1` writer - Channel 1 Full Transfer Finish clear"]
pub use FTFIFC0_W as FTFIFC1_W;
#[doc = "Field `FTFIFC2` writer - Channel 2 Full Transfer Finish clear"]
pub use FTFIFC0_W as FTFIFC2_W;
#[doc = "Field `FTFIFC3` writer - Channel 3 Full Transfer Finish clear"]
pub use FTFIFC0_W as FTFIFC3_W;
#[doc = "Field `FTFIFC4` writer - Channel 4 Full Transfer Finish clear"]
pub use FTFIFC0_W as FTFIFC4_W;
#[doc = "Channel 0 Half Transfer clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type HTFIFC0_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, HTFIFC0_AW, 2>;
impl<'a> HTFIFC0_W<'a> {
    #[doc = "Clears the HTFIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTFIFC0_AW::CLEAR)
    }
}
#[doc = "Channel 1 Half Transfer clear"]
pub use HTFIFC0_AW as HTFIFC1_AW;
#[doc = "Channel 2 Half Transfer clear"]
pub use HTFIFC0_AW as HTFIFC2_AW;
#[doc = "Channel 3 Half Transfer clear"]
pub use HTFIFC0_AW as HTFIFC3_AW;
#[doc = "Channel 4 Half Transfer clear"]
pub use HTFIFC0_AW as HTFIFC4_AW;
#[doc = "Field `HTFIFC1` writer - Channel 1 Half Transfer clear"]
pub use HTFIFC0_W as HTFIFC1_W;
#[doc = "Field `HTFIFC2` writer - Channel 2 Half Transfer clear"]
pub use HTFIFC0_W as HTFIFC2_W;
#[doc = "Field `HTFIFC3` writer - Channel 3 Half Transfer clear"]
pub use HTFIFC0_W as HTFIFC3_W;
#[doc = "Field `HTFIFC4` writer - Channel 4 Half Transfer clear"]
pub use HTFIFC0_W as HTFIFC4_W;
#[doc = "Channel 0 Error clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type ERRIFC0_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, ERRIFC0_AW, 3>;
impl<'a> ERRIFC0_W<'a> {
    #[doc = "Clears the ERRIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRIFC0_AW::CLEAR)
    }
}
#[doc = "Channel 1 Error clear"]
pub use ERRIFC0_AW as ERRIFC1_AW;
#[doc = "Channel 2 Error clear"]
pub use ERRIFC0_AW as ERRIFC2_AW;
#[doc = "Channel 3 Error clear"]
pub use ERRIFC0_AW as ERRIFC3_AW;
#[doc = "Channel 4 Error clear"]
pub use ERRIFC0_AW as ERRIFC4_AW;
#[doc = "Field `ERRIFC1` writer - Channel 1 Error clear"]
pub use ERRIFC0_W as ERRIFC1_W;
#[doc = "Field `ERRIFC2` writer - Channel 2 Error clear"]
pub use ERRIFC0_W as ERRIFC2_W;
#[doc = "Field `ERRIFC3` writer - Channel 3 Error clear"]
pub use ERRIFC0_W as ERRIFC3_W;
#[doc = "Field `ERRIFC4` writer - Channel 4 Error clear"]
pub use ERRIFC0_W as ERRIFC4_W;
impl W {
    #[doc = "Bit 0 - Channel 0 Global interrupt flag clear"]
    #[inline(always)]
    pub fn gifc0(&mut self) -> GIFC0_W {
        GIFC0_W::new(self)
    }
    #[doc = "Bit 4 - Channel 1 Global interrupt flag clear"]
    #[inline(always)]
    pub fn gifc1(&mut self) -> GIFC1_W {
        GIFC1_W::new(self)
    }
    #[doc = "Bit 8 - Channel 2 Global interrupt flag clear"]
    #[inline(always)]
    pub fn gifc2(&mut self) -> GIFC2_W {
        GIFC2_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 Global interrupt flag clear"]
    #[inline(always)]
    pub fn gifc3(&mut self) -> GIFC3_W {
        GIFC3_W::new(self)
    }
    #[doc = "Bit 16 - Channel 4 Global interrupt flag clear"]
    #[inline(always)]
    pub fn gifc4(&mut self) -> GIFC4_W {
        GIFC4_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 Full Transfer Finish clear"]
    #[inline(always)]
    pub fn ftfifc0(&mut self) -> FTFIFC0_W {
        FTFIFC0_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 Full Transfer Finish clear"]
    #[inline(always)]
    pub fn ftfifc1(&mut self) -> FTFIFC1_W {
        FTFIFC1_W::new(self)
    }
    #[doc = "Bit 9 - Channel 2 Full Transfer Finish clear"]
    #[inline(always)]
    pub fn ftfifc2(&mut self) -> FTFIFC2_W {
        FTFIFC2_W::new(self)
    }
    #[doc = "Bit 13 - Channel 3 Full Transfer Finish clear"]
    #[inline(always)]
    pub fn ftfifc3(&mut self) -> FTFIFC3_W {
        FTFIFC3_W::new(self)
    }
    #[doc = "Bit 17 - Channel 4 Full Transfer Finish clear"]
    #[inline(always)]
    pub fn ftfifc4(&mut self) -> FTFIFC4_W {
        FTFIFC4_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 Half Transfer clear"]
    #[inline(always)]
    pub fn htfifc0(&mut self) -> HTFIFC0_W {
        HTFIFC0_W::new(self)
    }
    #[doc = "Bit 6 - Channel 1 Half Transfer clear"]
    #[inline(always)]
    pub fn htfifc1(&mut self) -> HTFIFC1_W {
        HTFIFC1_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 Half Transfer clear"]
    #[inline(always)]
    pub fn htfifc2(&mut self) -> HTFIFC2_W {
        HTFIFC2_W::new(self)
    }
    #[doc = "Bit 14 - Channel 3 Half Transfer clear"]
    #[inline(always)]
    pub fn htfifc3(&mut self) -> HTFIFC3_W {
        HTFIFC3_W::new(self)
    }
    #[doc = "Bit 18 - Channel 4 Half Transfer clear"]
    #[inline(always)]
    pub fn htfifc4(&mut self) -> HTFIFC4_W {
        HTFIFC4_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 Error clear"]
    #[inline(always)]
    pub fn errifc0(&mut self) -> ERRIFC0_W {
        ERRIFC0_W::new(self)
    }
    #[doc = "Bit 7 - Channel 1 Error clear"]
    #[inline(always)]
    pub fn errifc1(&mut self) -> ERRIFC1_W {
        ERRIFC1_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 Error clear"]
    #[inline(always)]
    pub fn errifc2(&mut self) -> ERRIFC2_W {
        ERRIFC2_W::new(self)
    }
    #[doc = "Bit 15 - Channel 3 Error clear"]
    #[inline(always)]
    pub fn errifc3(&mut self) -> ERRIFC3_W {
        ERRIFC3_W::new(self)
    }
    #[doc = "Bit 19 - Channel 4 Error clear"]
    #[inline(always)]
    pub fn errifc4(&mut self) -> ERRIFC4_W {
        ERRIFC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA interrupt flag clear register (DMA_INTC)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](index.html) module"]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intc::W](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
