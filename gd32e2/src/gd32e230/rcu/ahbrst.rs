#[doc = "Register `AHBRST` reader"]
pub struct R(crate::R<AHBRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBRST` writer"]
pub struct W(crate::W<AHBRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRST_SPEC>;
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
impl From<crate::W<AHBRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFRST` reader - GPIO port F reset"]
pub type PFRST_R = crate::BitReader<bool>;
#[doc = "Field `PFRST` writer - GPIO port F reset"]
pub type PFRST_W<'a> = crate::BitWriter<'a, u32, AHBRST_SPEC, bool, 22>;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub type PCRST_R = crate::BitReader<bool>;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub type PCRST_W<'a> = crate::BitWriter<'a, u32, AHBRST_SPEC, bool, 19>;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub type PBRST_R = crate::BitReader<bool>;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub type PBRST_W<'a> = crate::BitWriter<'a, u32, AHBRST_SPEC, bool, 18>;
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type PARST_R = crate::BitReader<bool>;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type PARST_W<'a> = crate::BitWriter<'a, u32, AHBRST_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 22 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&self) -> PFRST_R {
        PFRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&mut self) -> PFRST_W {
        PFRST_W::new(self)
    }
    #[doc = "Bit 19 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&mut self) -> PCRST_W {
        PCRST_W::new(self)
    }
    #[doc = "Bit 18 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&mut self) -> PBRST_W {
        PBRST_W::new(self)
    }
    #[doc = "Bit 17 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&mut self) -> PARST_W {
        PARST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrst](index.html) module"]
pub struct AHBRST_SPEC;
impl crate::RegisterSpec for AHBRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbrst::R](R) reader structure"]
impl crate::Readable for AHBRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbrst::W](W) writer structure"]
impl crate::Writable for AHBRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AHBRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
