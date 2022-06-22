#[doc = "Register `LPMCS` reader"]
pub struct R(crate::R<LPMCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMCS` writer"]
pub struct W(crate::W<LPMCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMCS_SPEC>;
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
impl From<crate::W<LPMCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLSTAT` reader - bLinkState value"]
pub type BLSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLSTAT` writer - bLinkState value"]
pub type BLSTAT_W<'a> = crate::FieldWriter<'a, u32, LPMCS_SPEC, u8, u8, 4, 4>;
#[doc = "Field `REMWK` reader - bRemoteWake value"]
pub type REMWK_R = crate::BitReader<bool>;
#[doc = "Field `REMWK` writer - bRemoteWake value"]
pub type REMWK_W<'a> = crate::BitWriter<'a, u32, LPMCS_SPEC, bool, 3>;
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable"]
pub type LPMACK_R = crate::BitReader<bool>;
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable"]
pub type LPMACK_W<'a> = crate::BitWriter<'a, u32, LPMCS_SPEC, bool, 1>;
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub type LPMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub type LPMEN_W<'a> = crate::BitWriter<'a, u32, LPMCS_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 4:7 - bLinkState value"]
    #[inline(always)]
    pub fn blstat(&self) -> BLSTAT_R {
        BLSTAT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwk(&self) -> REMWK_R {
        REMWK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - bLinkState value"]
    #[inline(always)]
    pub fn blstat(&mut self) -> BLSTAT_W {
        BLSTAT_W::new(self)
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwk(&mut self) -> REMWK_W {
        REMWK_W::new(self)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W {
        LPMACK_W::new(self)
    }
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W {
        LPMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LPM control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcs](index.html) module"]
pub struct LPMCS_SPEC;
impl crate::RegisterSpec for LPMCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmcs::R](R) reader structure"]
impl crate::Readable for LPMCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmcs::W](W) writer structure"]
impl crate::Writable for LPMCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMCS to value 0"]
impl crate::Resettable for LPMCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
