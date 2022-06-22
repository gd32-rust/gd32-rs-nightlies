#[doc = "Register `RSTSCK` reader"]
pub struct R(crate::R<RSTSCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTSCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTSCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTSCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTSCK` writer"]
pub struct W(crate::W<RSTSCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTSCK_SPEC>;
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
impl From<crate::W<RSTSCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTSCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub type LPRSTF_R = crate::BitReader<bool>;
#[doc = "Field `LPRSTF` writer - Low-power reset flag"]
pub type LPRSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 31>;
#[doc = "Field `WWDGTRSTF` reader - Window watchdog timer reset flag"]
pub type WWDGTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WWDGTRSTF` writer - Window watchdog timer reset flag"]
pub type WWDGTRSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 30>;
#[doc = "Field `FWDGTRSTF` reader - Free Watchdog timer reset flag"]
pub type FWDGTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `FWDGTRSTF` writer - Free Watchdog timer reset flag"]
pub type FWDGTRSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 29>;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub type SWRSTF_R = crate::BitReader<bool>;
#[doc = "Field `SWRSTF` writer - Software reset flag"]
pub type SWRSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 28>;
#[doc = "Field `PORRSTF` reader - Power reset flag"]
pub type PORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PORRSTF` writer - Power reset flag"]
pub type PORRSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 27>;
#[doc = "Field `EPRSTF` reader - External PIN reset flag"]
pub type EPRSTF_R = crate::BitReader<bool>;
#[doc = "Field `EPRSTF` writer - External PIN reset flag"]
pub type EPRSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 26>;
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OBLRSTF_R = crate::BitReader<bool>;
#[doc = "Field `OBLRSTF` writer - Option byte loader reset flag"]
pub type OBLRSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 25>;
#[doc = "Field `RSTFC` reader - Reset flag clear"]
pub type RSTFC_R = crate::BitReader<bool>;
#[doc = "Field `RSTFC` writer - Reset flag clear"]
pub type RSTFC_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 24>;
#[doc = "Field `V12RSTF` reader - V12 domain Power reset flag"]
pub type V12RSTF_R = crate::BitReader<bool>;
#[doc = "Field `V12RSTF` writer - V12 domain Power reset flag"]
pub type V12RSTF_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 23>;
#[doc = "Field `IRC40KSTB` reader - IRC40K stabilization"]
pub type IRC40KSTB_R = crate::BitReader<bool>;
#[doc = "Field `IRC40KEN` reader - IRC40K enable"]
pub type IRC40KEN_R = crate::BitReader<bool>;
#[doc = "Field `IRC40KEN` writer - IRC40K enable"]
pub type IRC40KEN_W<'a> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LPRSTF_R {
        LPRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdgtrstf(&self) -> WWDGTRSTF_R {
        WWDGTRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Free Watchdog timer reset flag"]
    #[inline(always)]
    pub fn fwdgtrstf(&self) -> FWDGTRSTF_R {
        FWDGTRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SWRSTF_R {
        SWRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - Power reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - External PIN reset flag"]
    #[inline(always)]
    pub fn eprstf(&self) -> EPRSTF_R {
        EPRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&self) -> RSTFC_R {
        RSTFC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - V12 domain Power reset flag"]
    #[inline(always)]
    pub fn v12rstf(&self) -> V12RSTF_R {
        V12RSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 1 - IRC40K stabilization"]
    #[inline(always)]
    pub fn irc40kstb(&self) -> IRC40KSTB_R {
        IRC40KSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    pub fn irc40ken(&self) -> IRC40KEN_R {
        IRC40KEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&mut self) -> LPRSTF_W {
        LPRSTF_W::new(self)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdgtrstf(&mut self) -> WWDGTRSTF_W {
        WWDGTRSTF_W::new(self)
    }
    #[doc = "Bit 29 - Free Watchdog timer reset flag"]
    #[inline(always)]
    pub fn fwdgtrstf(&mut self) -> FWDGTRSTF_W {
        FWDGTRSTF_W::new(self)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&mut self) -> SWRSTF_W {
        SWRSTF_W::new(self)
    }
    #[doc = "Bit 27 - Power reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 26 - External PIN reset flag"]
    #[inline(always)]
    pub fn eprstf(&mut self) -> EPRSTF_W {
        EPRSTF_W::new(self)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&mut self) -> OBLRSTF_W {
        OBLRSTF_W::new(self)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&mut self) -> RSTFC_W {
        RSTFC_W::new(self)
    }
    #[doc = "Bit 23 - V12 domain Power reset flag"]
    #[inline(always)]
    pub fn v12rstf(&mut self) -> V12RSTF_W {
        V12RSTF_W::new(self)
    }
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    pub fn irc40ken(&mut self) -> IRC40KEN_W {
        IRC40KEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset source /clock register (RCU_RSTSCK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsck](index.html) module"]
pub struct RSTSCK_SPEC;
impl crate::RegisterSpec for RSTSCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstsck::R](R) reader structure"]
impl crate::Readable for RSTSCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstsck::W](W) writer structure"]
impl crate::Writable for RSTSCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTSCK to value 0x0c00_0000"]
impl crate::Resettable for RSTSCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0000
    }
}
