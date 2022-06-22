#[doc = "Register `ADDAPB2RST` reader"]
pub struct R(crate::R<ADDAPB2RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDAPB2RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDAPB2RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDAPB2RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDAPB2RST` writer"]
pub struct W(crate::W<ADDAPB2RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDAPB2RST_SPEC>;
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
impl From<crate::W<ADDAPB2RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDAPB2RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART5RST` reader - USART5 reset"]
pub type USART5RST_R = crate::BitReader<bool>;
#[doc = "Field `USART5RST` writer - USART5 reset"]
pub type USART5RST_W<'a> = crate::BitWriter<'a, u32, ADDAPB2RST_SPEC, bool, 24>;
#[doc = "Field `TLIRST` reader - TLI reset"]
pub type TLIRST_R = crate::BitReader<bool>;
#[doc = "Field `TLIRST` writer - TLI reset"]
pub type TLIRST_W<'a> = crate::BitWriter<'a, u32, ADDAPB2RST_SPEC, bool, 26>;
#[doc = "Field `PHRST` reader - GPIO port H reset"]
pub type PHRST_R = crate::BitReader<bool>;
#[doc = "Field `PHRST` writer - GPIO port H reset"]
pub type PHRST_W<'a> = crate::BitWriter<'a, u32, ADDAPB2RST_SPEC, bool, 30>;
#[doc = "Field `PIRST` reader - GPIO port I reset"]
pub type PIRST_R = crate::BitReader<bool>;
#[doc = "Field `PIRST` writer - GPIO port I reset"]
pub type PIRST_W<'a> = crate::BitWriter<'a, u32, ADDAPB2RST_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 24 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&self) -> USART5RST_R {
        USART5RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI reset"]
    #[inline(always)]
    pub fn tlirst(&self) -> TLIRST_R {
        TLIRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO port H reset"]
    #[inline(always)]
    pub fn phrst(&self) -> PHRST_R {
        PHRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO port I reset"]
    #[inline(always)]
    pub fn pirst(&self) -> PIRST_R {
        PIRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USART5 reset"]
    #[inline(always)]
    pub fn usart5rst(&mut self) -> USART5RST_W {
        USART5RST_W::new(self)
    }
    #[doc = "Bit 26 - TLI reset"]
    #[inline(always)]
    pub fn tlirst(&mut self) -> TLIRST_W {
        TLIRST_W::new(self)
    }
    #[doc = "Bit 30 - GPIO port H reset"]
    #[inline(always)]
    pub fn phrst(&mut self) -> PHRST_W {
        PHRST_W::new(self)
    }
    #[doc = "Bit 31 - GPIO port I reset"]
    #[inline(always)]
    pub fn pirst(&mut self) -> PIRST_W {
        PIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 additional enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addapb2rst](index.html) module"]
pub struct ADDAPB2RST_SPEC;
impl crate::RegisterSpec for ADDAPB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addapb2rst::R](R) reader structure"]
impl crate::Readable for ADDAPB2RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addapb2rst::W](W) writer structure"]
impl crate::Writable for ADDAPB2RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDAPB2RST to value 0"]
impl crate::Resettable for ADDAPB2RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
