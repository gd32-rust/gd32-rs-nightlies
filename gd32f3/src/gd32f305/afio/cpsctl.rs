#[doc = "Register `CPSCTL` reader"]
pub struct R(crate::R<CPSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPSCTL` writer"]
pub struct W(crate::W<CPSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPSCTL_SPEC>;
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
impl From<crate::W<CPSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPS_RDY` reader - I/O compensation cell is really or not"]
pub type CPS_RDY_R = crate::BitReader<bool>;
#[doc = "Field `CPS_RDY` writer - I/O compensation cell is really or not"]
pub type CPS_RDY_W<'a> = crate::BitWriter<'a, u32, CPSCTL_SPEC, bool, 8>;
#[doc = "Field `CPS_EN` reader - I/O compensation cell enable"]
pub type CPS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CPS_EN` writer - I/O compensation cell enable"]
pub type CPS_EN_W<'a> = crate::BitWriter<'a, u32, CPSCTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 8 - I/O compensation cell is really or not"]
    #[inline(always)]
    pub fn cps_rdy(&self) -> CPS_RDY_R {
        CPS_RDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 0 - I/O compensation cell enable"]
    #[inline(always)]
    pub fn cps_en(&self) -> CPS_EN_R {
        CPS_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - I/O compensation cell is really or not"]
    #[inline(always)]
    pub fn cps_rdy(&mut self) -> CPS_RDY_W {
        CPS_RDY_W::new(self)
    }
    #[doc = "Bit 0 - I/O compensation cell enable"]
    #[inline(always)]
    pub fn cps_en(&mut self) -> CPS_EN_W {
        CPS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO compensation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpsctl](index.html) module"]
pub struct CPSCTL_SPEC;
impl crate::RegisterSpec for CPSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpsctl::R](R) reader structure"]
impl crate::Readable for CPSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpsctl::W](W) writer structure"]
impl crate::Writable for CPSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPSCTL to value 0"]
impl crate::Resettable for CPSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
