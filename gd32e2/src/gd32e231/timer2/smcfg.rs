#[doc = "Register `SMCFG` reader"]
pub struct R(crate::R<SMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCFG` writer"]
pub struct W(crate::W<SMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCFG_SPEC>;
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
impl From<crate::W<SMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader<bool>;
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a> = crate::BitWriter<'a, u32, SMCFG_SPEC, bool, 15>;
#[doc = "Field `SMC1` reader - External clock enable"]
pub type SMC1_R = crate::BitReader<bool>;
#[doc = "Field `SMC1` writer - External clock enable"]
pub type SMC1_W<'a> = crate::BitWriter<'a, u32, SMCFG_SPEC, bool, 14>;
#[doc = "Field `ETPSC` reader - External trigger prescaler"]
pub type ETPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETPSC` writer - External trigger prescaler"]
pub type ETPSC_W<'a> = crate::FieldWriter<'a, u32, SMCFG_SPEC, u8, u8, 2, 12>;
#[doc = "Field `ETFC` reader - External trigger filter"]
pub type ETFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETFC` writer - External trigger filter"]
pub type ETFC_W<'a> = crate::FieldWriter<'a, u32, SMCFG_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader<bool>;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a> = crate::BitWriter<'a, u32, SMCFG_SPEC, bool, 7>;
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TRGS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TRGS_W<'a> = crate::FieldWriter<'a, u32, SMCFG_SPEC, u8, u8, 3, 4>;
#[doc = "Field `OCRC` reader - OCREF clear source selection"]
pub type OCRC_R = crate::BitReader<bool>;
#[doc = "Field `OCRC` writer - OCREF clear source selection"]
pub type OCRC_W<'a> = crate::BitWriter<'a, u32, SMCFG_SPEC, bool, 3>;
#[doc = "Field `SMC` reader - Slave mode selection"]
pub type SMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMC` writer - Slave mode selection"]
pub type SMC_W<'a> = crate::FieldWriter<'a, u32, SMCFG_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn smc1(&self) -> SMC1_R {
        SMC1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&self) -> ETPSC_R {
        ETPSC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etfc(&self) -> ETFC_R {
        ETFC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TRGS_R {
        TRGS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear source selection"]
    #[inline(always)]
    pub fn ocrc(&self) -> OCRC_R {
        OCRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W::new(self)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn smc1(&mut self) -> SMC1_W {
        SMC1_W::new(self)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&mut self) -> ETPSC_W {
        ETPSC_W::new(self)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etfc(&mut self) -> ETFC_W {
        ETFC_W::new(self)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&mut self) -> TRGS_W {
        TRGS_W::new(self)
    }
    #[doc = "Bit 3 - OCREF clear source selection"]
    #[inline(always)]
    pub fn ocrc(&mut self) -> OCRC_W {
        OCRC_W::new(self)
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&mut self) -> SMC_W {
        SMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcfg](index.html) module"]
pub struct SMCFG_SPEC;
impl crate::RegisterSpec for SMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smcfg::R](R) reader structure"]
impl crate::Readable for SMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcfg::W](W) writer structure"]
impl crate::Writable for SMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
