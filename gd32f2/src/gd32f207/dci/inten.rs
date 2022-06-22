#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELIE` reader - End of line interrupt Enable"]
pub type ELIE_R = crate::BitReader<bool>;
#[doc = "Field `ELIE` writer - End of line interrupt Enable"]
pub type ELIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 4>;
#[doc = "Field `VSIE` reader - Vsync Interrupt Enable"]
pub type VSIE_R = crate::BitReader<bool>;
#[doc = "Field `VSIE` writer - Vsync Interrupt Enable"]
pub type VSIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
#[doc = "Field `ESEIE` reader - Embedded Synchronous Error Interrupt Enable"]
pub type ESEIE_R = crate::BitReader<bool>;
#[doc = "Field `ESEIE` writer - Embedded Synchronous Error Interrupt Enable"]
pub type ESEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `OVRIE` reader - FIFO Overrun interrupt Enable"]
pub type OVRIE_R = crate::BitReader<bool>;
#[doc = "Field `OVRIE` writer - FIFO Overrun interrupt Enable"]
pub type OVRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `EFIE` reader - End of Frame Interrupt Enable"]
pub type EFIE_R = crate::BitReader<bool>;
#[doc = "Field `EFIE` writer - End of Frame Interrupt Enable"]
pub type EFIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 4 - End of line interrupt Enable"]
    #[inline(always)]
    pub fn elie(&self) -> ELIE_R {
        ELIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Vsync Interrupt Enable"]
    #[inline(always)]
    pub fn vsie(&self) -> VSIE_R {
        VSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Interrupt Enable"]
    #[inline(always)]
    pub fn eseie(&self) -> ESEIE_R {
        ESEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overrun interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - End of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn efie(&self) -> EFIE_R {
        EFIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - End of line interrupt Enable"]
    #[inline(always)]
    pub fn elie(&mut self) -> ELIE_W {
        ELIE_W::new(self)
    }
    #[doc = "Bit 3 - Vsync Interrupt Enable"]
    #[inline(always)]
    pub fn vsie(&mut self) -> VSIE_W {
        VSIE_W::new(self)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Interrupt Enable"]
    #[inline(always)]
    pub fn eseie(&mut self) -> ESEIE_W {
        ESEIE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Overrun interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 0 - End of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn efie(&mut self) -> EFIE_W {
        EFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCI inrerrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
