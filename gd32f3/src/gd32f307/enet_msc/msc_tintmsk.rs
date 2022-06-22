#[doc = "Register `MSC_TINTMSK` reader"]
pub struct R(crate::R<MSC_TINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_TINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_TINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_TINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSC_TINTMSK` writer"]
pub struct W(crate::W<MSC_TINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSC_TINTMSK_SPEC>;
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
impl From<crate::W<MSC_TINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSC_TINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TGFSCIM` reader - Transmitted good frames single collision interrupt mask"]
pub type TGFSCIM_R = crate::BitReader<bool>;
#[doc = "Field `TGFSCIM` writer - Transmitted good frames single collision interrupt mask"]
pub type TGFSCIM_W<'a> = crate::BitWriter<'a, u32, MSC_TINTMSK_SPEC, bool, 14>;
#[doc = "Field `TGFMSCIM` reader - Transmitted good frames more single interrupt collision mask"]
pub type TGFMSCIM_R = crate::BitReader<bool>;
#[doc = "Field `TGFMSCIM` writer - Transmitted good frames more single interrupt collision mask"]
pub type TGFMSCIM_W<'a> = crate::BitWriter<'a, u32, MSC_TINTMSK_SPEC, bool, 15>;
#[doc = "Field `TGFIM` reader - Transmitted good frames interrupt mask"]
pub type TGFIM_R = crate::BitReader<bool>;
#[doc = "Field `TGFIM` writer - Transmitted good frames interrupt mask"]
pub type TGFIM_W<'a> = crate::BitWriter<'a, u32, MSC_TINTMSK_SPEC, bool, 21>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision interrupt mask"]
    #[inline(always)]
    pub fn tgfscim(&self) -> TGFSCIM_R {
        TGFSCIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single interrupt collision mask"]
    #[inline(always)]
    pub fn tgfmscim(&self) -> TGFMSCIM_R {
        TGFMSCIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames interrupt mask"]
    #[inline(always)]
    pub fn tgfim(&self) -> TGFIM_R {
        TGFIM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision interrupt mask"]
    #[inline(always)]
    pub fn tgfscim(&mut self) -> TGFSCIM_W {
        TGFSCIM_W::new(self)
    }
    #[doc = "Bit 15 - Transmitted good frames more single interrupt collision mask"]
    #[inline(always)]
    pub fn tgfmscim(&mut self) -> TGFMSCIM_W {
        TGFMSCIM_W::new(self)
    }
    #[doc = "Bit 21 - Transmitted good frames interrupt mask"]
    #[inline(always)]
    pub fn tgfim(&mut self) -> TGFIM_W {
        TGFIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_tintmsk](index.html) module"]
pub struct MSC_TINTMSK_SPEC;
impl crate::RegisterSpec for MSC_TINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_tintmsk::R](R) reader structure"]
impl crate::Readable for MSC_TINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msc_tintmsk::W](W) writer structure"]
impl crate::Writable for MSC_TINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSC_TINTMSK to value 0"]
impl crate::Resettable for MSC_TINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
