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
#[doc = "Field `LCRIE` reader - Layer Configuration Reloaded Interrupt Enable"]
pub type LCRIE_R = crate::BitReader<bool>;
#[doc = "Field `LCRIE` writer - Layer Configuration Reloaded Interrupt Enable"]
pub type LCRIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
#[doc = "Field `TEIE` reader - Transaction Error Interrupt Enable"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - Transaction Error Interrupt Enable"]
pub type TEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `FEIE` reader - FIFO Error Interrupt Enable"]
pub type FEIE_R = crate::BitReader<bool>;
#[doc = "Field `FEIE` writer - FIFO Error Interrupt Enable"]
pub type FEIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `LMIE` reader - Line Mark Interrupt Enable"]
pub type LMIE_R = crate::BitReader<bool>;
#[doc = "Field `LMIE` writer - Line Mark Interrupt Enable"]
pub type LMIE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 3 - Layer Configuration Reloaded Interrupt Enable"]
    #[inline(always)]
    pub fn lcrie(&self) -> LCRIE_R {
        LCRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Line Mark Interrupt Enable"]
    #[inline(always)]
    pub fn lmie(&self) -> LMIE_R {
        LMIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Layer Configuration Reloaded Interrupt Enable"]
    #[inline(always)]
    pub fn lcrie(&mut self) -> LCRIE_W {
        LCRIE_W::new(self)
    }
    #[doc = "Bit 2 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W::new(self)
    }
    #[doc = "Bit 0 - Line Mark Interrupt Enable"]
    #[inline(always)]
    pub fn lmie(&mut self) -> LMIE_W {
        LMIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
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
