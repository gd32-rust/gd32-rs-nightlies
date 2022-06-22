#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ONF` reader - SLCD controller on flag"]
pub type ONF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - Start of frame flag"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `UPRF` reader - Update SLCD data request flag"]
pub type UPRF_R = crate::BitReader<bool>;
#[doc = "Field `UPRF` writer - Update SLCD data request flag"]
pub type UPRF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 2>;
#[doc = "Field `UPDF` reader - Update SLCD data done flag"]
pub type UPDF_R = crate::BitReader<bool>;
#[doc = "Field `VRDYF` reader - SLCD voltage ready flag"]
pub type VRDYF_R = crate::BitReader<bool>;
#[doc = "Field `SYNF` reader - SLCD_CFG register synchronization flag"]
pub type SYNF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SLCD controller on flag"]
    #[inline(always)]
    pub fn onf(&self) -> ONF_R {
        ONF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame flag"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update SLCD data request flag"]
    #[inline(always)]
    pub fn uprf(&self) -> UPRF_R {
        UPRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update SLCD data done flag"]
    #[inline(always)]
    pub fn updf(&self) -> UPDF_R {
        UPDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLCD voltage ready flag"]
    #[inline(always)]
    pub fn vrdyf(&self) -> VRDYF_R {
        VRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SLCD_CFG register synchronization flag"]
    #[inline(always)]
    pub fn synf(&self) -> SYNF_R {
        SYNF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Update SLCD data request flag"]
    #[inline(always)]
    pub fn uprf(&mut self) -> UPRF_W {
        UPRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLCD status flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0x20"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
