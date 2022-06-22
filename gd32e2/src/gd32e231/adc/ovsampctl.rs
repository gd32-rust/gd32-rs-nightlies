#[doc = "Register `OVSAMPCTL` reader"]
pub struct R(crate::R<OVSAMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVSAMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVSAMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVSAMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVSAMPCTL` writer"]
pub struct W(crate::W<OVSAMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVSAMPCTL_SPEC>;
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
impl From<crate::W<OVSAMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVSAMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOVS` reader - Triggered Oversampling"]
pub type TOVS_R = crate::BitReader<bool>;
#[doc = "Field `TOVS` writer - Triggered Oversampling"]
pub type TOVS_W<'a> = crate::BitWriter<'a, u32, OVSAMPCTL_SPEC, bool, 9>;
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OVSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVSS` writer - Oversampling shift"]
pub type OVSS_W<'a> = crate::FieldWriter<'a, u32, OVSAMPCTL_SPEC, u8, u8, 4, 5>;
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OVSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVSR` writer - Oversampling ratio"]
pub type OVSR_W<'a> = crate::FieldWriter<'a, u32, OVSAMPCTL_SPEC, u8, u8, 3, 2>;
#[doc = "Field `OVSEN` reader - Oversampler Enable"]
pub type OVSEN_R = crate::BitReader<bool>;
#[doc = "Field `OVSEN` writer - Oversampler Enable"]
pub type OVSEN_W<'a> = crate::BitWriter<'a, u32, OVSAMPCTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 0 - Oversampler Enable"]
    #[inline(always)]
    pub fn ovsen(&self) -> OVSEN_R {
        OVSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W {
        TOVS_W::new(self)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W::new(self)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W {
        OVSR_W::new(self)
    }
    #[doc = "Bit 0 - Oversampler Enable"]
    #[inline(always)]
    pub fn ovsen(&mut self) -> OVSEN_W {
        OVSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC oversample control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovsampctl](index.html) module"]
pub struct OVSAMPCTL_SPEC;
impl crate::RegisterSpec for OVSAMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovsampctl::R](R) reader structure"]
impl crate::Readable for OVSAMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ovsampctl::W](W) writer structure"]
impl crate::Writable for OVSAMPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVSAMPCTL to value 0"]
impl crate::Resettable for OVSAMPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
