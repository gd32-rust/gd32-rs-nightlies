#[doc = "Register `ADDAPB2EN` reader"]
pub struct R(crate::R<ADDAPB2EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDAPB2EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDAPB2EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDAPB2EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDAPB2EN` writer"]
pub struct W(crate::W<ADDAPB2EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDAPB2EN_SPEC>;
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
impl From<crate::W<ADDAPB2EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDAPB2EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART5EN` reader - USART5 clock enable"]
pub type USART5EN_R = crate::BitReader<bool>;
#[doc = "Field `USART5EN` writer - USART5 clock enable"]
pub type USART5EN_W<'a> = crate::BitWriter<'a, u32, ADDAPB2EN_SPEC, bool, 24>;
#[doc = "Field `TLIEN` reader - TLI clock enable"]
pub type TLIEN_R = crate::BitReader<bool>;
#[doc = "Field `TLIEN` writer - TLI clock enable"]
pub type TLIEN_W<'a> = crate::BitWriter<'a, u32, ADDAPB2EN_SPEC, bool, 26>;
#[doc = "Field `PHEN` reader - GPIO port H clock enable"]
pub type PHEN_R = crate::BitReader<bool>;
#[doc = "Field `PHEN` writer - GPIO port H clock enable"]
pub type PHEN_W<'a> = crate::BitWriter<'a, u32, ADDAPB2EN_SPEC, bool, 30>;
#[doc = "Field `PIEN` reader - GPIO port I clock enable"]
pub type PIEN_R = crate::BitReader<bool>;
#[doc = "Field `PIEN` writer - GPIO port I clock enable"]
pub type PIEN_W<'a> = crate::BitWriter<'a, u32, ADDAPB2EN_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 24 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI clock enable"]
    #[inline(always)]
    pub fn tlien(&self) -> TLIEN_R {
        TLIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO port H clock enable"]
    #[inline(always)]
    pub fn phen(&self) -> PHEN_R {
        PHEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO port I clock enable"]
    #[inline(always)]
    pub fn pien(&self) -> PIEN_R {
        PIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5en(&mut self) -> USART5EN_W {
        USART5EN_W::new(self)
    }
    #[doc = "Bit 26 - TLI clock enable"]
    #[inline(always)]
    pub fn tlien(&mut self) -> TLIEN_W {
        TLIEN_W::new(self)
    }
    #[doc = "Bit 30 - GPIO port H clock enable"]
    #[inline(always)]
    pub fn phen(&mut self) -> PHEN_W {
        PHEN_W::new(self)
    }
    #[doc = "Bit 31 - GPIO port I clock enable"]
    #[inline(always)]
    pub fn pien(&mut self) -> PIEN_W {
        PIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 additional enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addapb2en](index.html) module"]
pub struct ADDAPB2EN_SPEC;
impl crate::RegisterSpec for ADDAPB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addapb2en::R](R) reader structure"]
impl crate::Readable for ADDAPB2EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addapb2en::W](W) writer structure"]
impl crate::Writable for ADDAPB2EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDAPB2EN to value 0"]
impl crate::Resettable for ADDAPB2EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
