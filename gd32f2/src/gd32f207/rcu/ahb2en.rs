#[doc = "Register `AHB2EN` reader"]
pub struct R(crate::R<AHB2EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2EN` writer"]
pub struct W(crate::W<AHB2EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2EN_SPEC>;
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
impl From<crate::W<AHB2EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCIEN` reader - DCI clock enable"]
pub type DCIEN_R = crate::BitReader<bool>;
#[doc = "Field `DCIEN` writer - DCI clock enable"]
pub type DCIEN_W<'a> = crate::BitWriter<'a, u32, AHB2EN_SPEC, bool, 0>;
#[doc = "Field `CAUEN` reader - CAU clock enable"]
pub type CAUEN_R = crate::BitReader<bool>;
#[doc = "Field `CAUEN` writer - CAU clock enable"]
pub type CAUEN_W<'a> = crate::BitWriter<'a, u32, AHB2EN_SPEC, bool, 4>;
#[doc = "Field `HAUEN` reader - HAU clock enable"]
pub type HAUEN_R = crate::BitReader<bool>;
#[doc = "Field `HAUEN` writer - HAU clock enable"]
pub type HAUEN_W<'a> = crate::BitWriter<'a, u32, AHB2EN_SPEC, bool, 5>;
#[doc = "Field `TRNGEN` reader - TRNG clock enable"]
pub type TRNGEN_R = crate::BitReader<bool>;
#[doc = "Field `TRNGEN` writer - TRNG clock enable"]
pub type TRNGEN_W<'a> = crate::BitWriter<'a, u32, AHB2EN_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    pub fn dcien(&self) -> DCIEN_R {
        DCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CAU clock enable"]
    #[inline(always)]
    pub fn cauen(&self) -> CAUEN_R {
        CAUEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HAU clock enable"]
    #[inline(always)]
    pub fn hauen(&self) -> HAUEN_R {
        HAUEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    pub fn trngen(&self) -> TRNGEN_R {
        TRNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    pub fn dcien(&mut self) -> DCIEN_W {
        DCIEN_W::new(self)
    }
    #[doc = "Bit 4 - CAU clock enable"]
    #[inline(always)]
    pub fn cauen(&mut self) -> CAUEN_W {
        CAUEN_W::new(self)
    }
    #[doc = "Bit 5 - HAU clock enable"]
    #[inline(always)]
    pub fn hauen(&mut self) -> HAUEN_W {
        HAUEN_W::new(self)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    pub fn trngen(&mut self) -> TRNGEN_W {
        TRNGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2en](index.html) module"]
pub struct AHB2EN_SPEC;
impl crate::RegisterSpec for AHB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2en::R](R) reader structure"]
impl crate::Readable for AHB2EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2en::W](W) writer structure"]
impl crate::Writable for AHB2EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2EN to value 0"]
impl crate::Resettable for AHB2EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
