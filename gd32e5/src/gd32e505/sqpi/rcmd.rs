#[doc = "Register `RCMD` reader"]
pub type R = crate::R<RCMD_SPEC>;
#[doc = "Register `RCMD` writer"]
pub type W = crate::W<RCMD_SPEC>;
#[doc = "Field `SQPI_RCMD` reader - SQPI read command for AHB read transfer"]
pub type SQPI_RCMD_R = crate::FieldReader<u16>;
#[doc = "Field `SQPI_RCMD` writer - SQPI read command for AHB read transfer"]
pub type SQPI_RCMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `SQPI_RWAITCYCLE` reader - SQPI read command waitcycle number after address phase"]
pub type SQPI_RWAITCYCLE_R = crate::FieldReader;
#[doc = "Field `SQPI_RWAITCYCLE` writer - SQPI read command waitcycle number after address phase"]
pub type SQPI_RWAITCYCLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SQPI_RMODE` reader - SQPI controller read command mode"]
pub type SQPI_RMODE_R = crate::FieldReader;
#[doc = "Field `SQPI_RMODE` writer - SQPI controller read command mode"]
pub type SQPI_RMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SQPI_RID` reader - Send read ID command"]
pub type SQPI_RID_R = crate::BitReader;
#[doc = "Field `SQPI_RID` writer - Send read ID command"]
pub type SQPI_RID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - SQPI read command for AHB read transfer"]
    #[inline(always)]
    pub fn sqpi_rcmd(&self) -> SQPI_RCMD_R {
        SQPI_RCMD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - SQPI read command waitcycle number after address phase"]
    #[inline(always)]
    pub fn sqpi_rwaitcycle(&self) -> SQPI_RWAITCYCLE_R {
        SQPI_RWAITCYCLE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - SQPI controller read command mode"]
    #[inline(always)]
    pub fn sqpi_rmode(&self) -> SQPI_RMODE_R {
        SQPI_RMODE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 31 - Send read ID command"]
    #[inline(always)]
    pub fn sqpi_rid(&self) -> SQPI_RID_R {
        SQPI_RID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - SQPI read command for AHB read transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_rcmd(&mut self) -> SQPI_RCMD_W<RCMD_SPEC, 0> {
        SQPI_RCMD_W::new(self)
    }
    #[doc = "Bits 16:19 - SQPI read command waitcycle number after address phase"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_rwaitcycle(&mut self) -> SQPI_RWAITCYCLE_W<RCMD_SPEC, 16> {
        SQPI_RWAITCYCLE_W::new(self)
    }
    #[doc = "Bits 20:22 - SQPI controller read command mode"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_rmode(&mut self) -> SQPI_RMODE_W<RCMD_SPEC, 20> {
        SQPI_RMODE_W::new(self)
    }
    #[doc = "Bit 31 - Send read ID command"]
    #[inline(always)]
    #[must_use]
    pub fn sqpi_rid(&mut self) -> SQPI_RID_W<RCMD_SPEC, 31> {
        SQPI_RID_W::new(self)
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
#[doc = "SQPI Read Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCMD_SPEC;
impl crate::RegisterSpec for RCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcmd::R`](R) reader structure"]
impl crate::Readable for RCMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcmd::W`](W) writer structure"]
impl crate::Writable for RCMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCMD to value 0x0010_0000"]
impl crate::Resettable for RCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}
