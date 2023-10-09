#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `SFT` reader - Signal Free Time"]
pub type SFT_R = crate::FieldReader;
#[doc = "Field `SFT` writer - Signal Free Time"]
pub type SFT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RTOL` reader - Reception bit timing tolerance"]
pub type RTOL_R = crate::BitReader;
#[doc = "Field `RTOL` writer - Reception bit timing tolerance"]
pub type RTOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBRESTP` reader - Whether stop receive message when detected RBRE"]
pub type RBRESTP_R = crate::BitReader;
#[doc = "Field `RBRESTP` writer - Whether stop receive message when detected RBRE"]
pub type RBRESTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBREGEN` reader - Generate Error-bit when detected RBRE in singlecast"]
pub type RBREGEN_R = crate::BitReader;
#[doc = "Field `RBREGEN` writer - Generate Error-bit when detected RBRE in singlecast"]
pub type RBREGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RLBPEGEN` reader - Generate Error-bit when detected RLBPE in singlecast"]
pub type RLBPEGEN_R = crate::BitReader;
#[doc = "Field `RLBPEGEN` writer - Generate Error-bit when detected RLBPE in singlecast"]
pub type RLBPEGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BCNG` reader - Do not generate Error-bit in broadcast message"]
pub type BCNG_R = crate::BitReader;
#[doc = "Field `BCNG` writer - Do not generate Error-bit in broadcast message"]
pub type BCNG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SFTOPT` reader - The SFT start option"]
pub type SFTOPT_R = crate::BitReader;
#[doc = "Field `SFTOPT` writer - The SFT start option"]
pub type SFTOPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OADR` reader - Own Address"]
pub type OADR_R = crate::FieldReader<u16>;
#[doc = "Field `OADR` writer - Own Address"]
pub type OADR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `LMEN` reader - Listen mode enable"]
pub type LMEN_R = crate::BitReader;
#[doc = "Field `LMEN` writer - Listen mode enable"]
pub type LMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn sft(&mut self) -> SFT_W<CFG_SPEC, 0> {
        SFT_W::new(self)
    }
    #[doc = "Bit 3 - Reception bit timing tolerance"]
    #[inline(always)]
    #[must_use]
    pub fn rtol(&mut self) -> RTOL_W<CFG_SPEC, 3> {
        RTOL_W::new(self)
    }
    #[doc = "Bit 4 - Whether stop receive message when detected RBRE"]
    #[inline(always)]
    #[must_use]
    pub fn rbrestp(&mut self) -> RBRESTP_W<CFG_SPEC, 4> {
        RBRESTP_W::new(self)
    }
    #[doc = "Bit 5 - Generate Error-bit when detected RBRE in singlecast"]
    #[inline(always)]
    #[must_use]
    pub fn rbregen(&mut self) -> RBREGEN_W<CFG_SPEC, 5> {
        RBREGEN_W::new(self)
    }
    #[doc = "Bit 6 - Generate Error-bit when detected RLBPE in singlecast"]
    #[inline(always)]
    #[must_use]
    pub fn rlbpegen(&mut self) -> RLBPEGEN_W<CFG_SPEC, 6> {
        RLBPEGEN_W::new(self)
    }
    #[doc = "Bit 7 - Do not generate Error-bit in broadcast message"]
    #[inline(always)]
    #[must_use]
    pub fn bcng(&mut self) -> BCNG_W<CFG_SPEC, 7> {
        BCNG_W::new(self)
    }
    #[doc = "Bit 8 - The SFT start option"]
    #[inline(always)]
    #[must_use]
    pub fn sftopt(&mut self) -> SFTOPT_W<CFG_SPEC, 8> {
        SFTOPT_W::new(self)
    }
    #[doc = "Bits 16:30 - Own Address"]
    #[inline(always)]
    #[must_use]
    pub fn oadr(&mut self) -> OADR_W<CFG_SPEC, 16> {
        OADR_W::new(self)
    }
    #[doc = "Bit 31 - Listen mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmen(&mut self) -> LMEN_W<CFG_SPEC, 31> {
        LMEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
