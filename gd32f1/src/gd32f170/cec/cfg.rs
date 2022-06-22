#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFT` reader - Signal Free Time"]
pub type SFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFT` writer - Signal Free Time"]
pub type SFT_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, 0>;
#[doc = "Field `RTOL` reader - Reception bit timing tolerance"]
pub type RTOL_R = crate::BitReader<bool>;
#[doc = "Field `RTOL` writer - Reception bit timing tolerance"]
pub type RTOL_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 3>;
#[doc = "Field `RBRESTP` reader - Whether stop receive message when detected RBRE"]
pub type RBRESTP_R = crate::BitReader<bool>;
#[doc = "Field `RBRESTP` writer - Whether stop receive message when detected RBRE"]
pub type RBRESTP_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 4>;
#[doc = "Field `RBREGEN` reader - Generate Error-bit when detected RBRE in singlecast"]
pub type RBREGEN_R = crate::BitReader<bool>;
#[doc = "Field `RBREGEN` writer - Generate Error-bit when detected RBRE in singlecast"]
pub type RBREGEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 5>;
#[doc = "Field `RLBPEGEN` reader - Generate Error-bit when detected RLBPE in singlecast"]
pub type RLBPEGEN_R = crate::BitReader<bool>;
#[doc = "Field `RLBPEGEN` writer - Generate Error-bit when detected RLBPE in singlecast"]
pub type RLBPEGEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 6>;
#[doc = "Field `BCNG` reader - Do not generate Error-bit in broadcast message"]
pub type BCNG_R = crate::BitReader<bool>;
#[doc = "Field `BCNG` writer - Do not generate Error-bit in broadcast message"]
pub type BCNG_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 7>;
#[doc = "Field `SFTOPT` reader - The SFT start option"]
pub type SFTOPT_R = crate::BitReader<bool>;
#[doc = "Field `SFTOPT` writer - The SFT start option"]
pub type SFTOPT_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 8>;
#[doc = "Field `OADR` reader - Own Address"]
pub type OADR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OADR` writer - Own Address"]
pub type OADR_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u16, u16, 15, 16>;
#[doc = "Field `LMEN` reader - Listen mode enable"]
pub type LMEN_R = crate::BitReader<bool>;
#[doc = "Field `LMEN` writer - Listen mode enable"]
pub type LMEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Reception bit timing tolerance"]
    #[inline(always)]
    pub fn rtol(&self) -> RTOL_R {
        RTOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Whether stop receive message when detected RBRE"]
    #[inline(always)]
    pub fn rbrestp(&self) -> RBRESTP_R {
        RBRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate Error-bit when detected RBRE in singlecast"]
    #[inline(always)]
    pub fn rbregen(&self) -> RBREGEN_R {
        RBREGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generate Error-bit when detected RLBPE in singlecast"]
    #[inline(always)]
    pub fn rlbpegen(&self) -> RLBPEGEN_R {
        RLBPEGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Do not generate Error-bit in broadcast message"]
    #[inline(always)]
    pub fn bcng(&self) -> BCNG_R {
        BCNG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The SFT start option"]
    #[inline(always)]
    pub fn sftopt(&self) -> SFTOPT_R {
        SFTOPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Own Address"]
    #[inline(always)]
    pub fn oadr(&self) -> OADR_R {
        OADR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Listen mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LMEN_R {
        LMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W {
        SFT_W::new(self)
    }
    #[doc = "Bit 3 - Reception bit timing tolerance"]
    #[inline(always)]
    pub fn rtol(&mut self) -> RTOL_W {
        RTOL_W::new(self)
    }
    #[doc = "Bit 4 - Whether stop receive message when detected RBRE"]
    #[inline(always)]
    pub fn rbrestp(&mut self) -> RBRESTP_W {
        RBRESTP_W::new(self)
    }
    #[doc = "Bit 5 - Generate Error-bit when detected RBRE in singlecast"]
    #[inline(always)]
    pub fn rbregen(&mut self) -> RBREGEN_W {
        RBREGEN_W::new(self)
    }
    #[doc = "Bit 6 - Generate Error-bit when detected RLBPE in singlecast"]
    #[inline(always)]
    pub fn rlbpegen(&mut self) -> RLBPEGEN_W {
        RLBPEGEN_W::new(self)
    }
    #[doc = "Bit 7 - Do not generate Error-bit in broadcast message"]
    #[inline(always)]
    pub fn bcng(&mut self) -> BCNG_W {
        BCNG_W::new(self)
    }
    #[doc = "Bit 8 - The SFT start option"]
    #[inline(always)]
    pub fn sftopt(&mut self) -> SFTOPT_W {
        SFTOPT_W::new(self)
    }
    #[doc = "Bits 16:30 - Own Address"]
    #[inline(always)]
    pub fn oadr(&mut self) -> OADR_W {
        OADR_W::new(self)
    }
    #[doc = "Bit 31 - Listen mode enable"]
    #[inline(always)]
    pub fn lmen(&mut self) -> LMEN_W {
        LMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
