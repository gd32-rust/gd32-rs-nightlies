#[doc = "Register `DMAINTEN` reader"]
pub struct R(crate::R<DMAINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAINTEN` writer"]
pub struct W(crate::W<DMAINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAINTEN_SPEC>;
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
impl From<crate::W<DMAINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0IE` reader - Channel 0 capture/compare interrupt enable"]
pub type CH0IE_R = crate::BitReader<bool>;
#[doc = "Field `CH0IE` writer - Channel 0 capture/compare interrupt enable"]
pub type CH0IE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, bool, 1>;
#[doc = "Update interrupt enable"]
pub use crate::gd32f205::timer0::dmainten::UPIE_A;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub use crate::gd32f205::timer0::dmainten::UPIE_R;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a> = crate::BitWriter<'a, u32, DMAINTEN_SPEC, UPIE_A, 0>;
impl<'a> UPIE_W<'a> {
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPIE_A::DISABLED)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> CH0IE_R {
        CH0IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&mut self) -> CH0IE_W {
        CH0IE_W::new(self)
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W {
        UPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmainten](index.html) module"]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmainten::R](R) reader structure"]
impl crate::Readable for DMAINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmainten::W](W) writer structure"]
impl crate::Writable for DMAINTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DMAINTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
