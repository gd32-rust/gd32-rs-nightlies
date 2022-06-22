#[doc = "Register `MSC_RINTMSK` reader"]
pub struct R(crate::R<MSC_RINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_RINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_RINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_RINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSC_RINTMSK` writer"]
pub struct W(crate::W<MSC_RINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSC_RINTMSK_SPEC>;
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
impl From<crate::W<MSC_RINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSC_RINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFCEIM` reader - Received frame CRC error interrupt mask"]
pub type RFCEIM_R = crate::BitReader<bool>;
#[doc = "Field `RFCEIM` writer - Received frame CRC error interrupt mask"]
pub type RFCEIM_W<'a> = crate::BitWriter<'a, u32, MSC_RINTMSK_SPEC, bool, 5>;
#[doc = "Field `RFAEIM` reader - Received frames alignment error interrupt mask"]
pub type RFAEIM_R = crate::BitReader<bool>;
#[doc = "Field `RFAEIM` writer - Received frames alignment error interrupt mask"]
pub type RFAEIM_W<'a> = crate::BitWriter<'a, u32, MSC_RINTMSK_SPEC, bool, 6>;
#[doc = "Field `RGUFIM` reader - Received good unicast frames interrupt mask"]
pub type RGUFIM_R = crate::BitReader<bool>;
#[doc = "Field `RGUFIM` writer - Received good unicast frames interrupt mask"]
pub type RGUFIM_W<'a> = crate::BitWriter<'a, u32, MSC_RINTMSK_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 5 - Received frame CRC error interrupt mask"]
    #[inline(always)]
    pub fn rfceim(&self) -> RFCEIM_R {
        RFCEIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error interrupt mask"]
    #[inline(always)]
    pub fn rfaeim(&self) -> RFAEIM_R {
        RFAEIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames interrupt mask"]
    #[inline(always)]
    pub fn rgufim(&self) -> RGUFIM_R {
        RGUFIM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error interrupt mask"]
    #[inline(always)]
    pub fn rfceim(&mut self) -> RFCEIM_W {
        RFCEIM_W::new(self)
    }
    #[doc = "Bit 6 - Received frames alignment error interrupt mask"]
    #[inline(always)]
    pub fn rfaeim(&mut self) -> RFAEIM_W {
        RFAEIM_W::new(self)
    }
    #[doc = "Bit 17 - Received good unicast frames interrupt mask"]
    #[inline(always)]
    pub fn rgufim(&mut self) -> RGUFIM_W {
        RGUFIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MSC receive interrupt mask register (MSC_RINTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc_rintmsk](index.html) module"]
pub struct MSC_RINTMSK_SPEC;
impl crate::RegisterSpec for MSC_RINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msc_rintmsk::R](R) reader structure"]
impl crate::Readable for MSC_RINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msc_rintmsk::W](W) writer structure"]
impl crate::Writable for MSC_RINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSC_RINTMSK to value 0"]
impl crate::Resettable for MSC_RINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
