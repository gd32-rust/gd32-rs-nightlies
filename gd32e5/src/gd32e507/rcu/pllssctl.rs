#[doc = "Register `PLLSSCTL` reader"]
pub type R = crate::R<PLLSSCTL_SPEC>;
#[doc = "Register `PLLSSCTL` writer"]
pub type W = crate::W<PLLSSCTL_SPEC>;
#[doc = "Field `MODCNT` reader - Configure PLL spread spectrum modulation profile amplitude and frequency"]
pub type MODCNT_R = crate::FieldReader<u16>;
#[doc = "Field `MODCNT` writer - Configure PLL spread spectrum modulation profile amplitude and frequency"]
pub type MODCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
#[doc = "Field `MODSTEP` reader - Configure PLL spread spectrum modulation profile amplitude and frequency"]
pub type MODSTEP_R = crate::FieldReader<u16>;
#[doc = "Field `MODSTEP` writer - Configure PLL spread spectrum modulation profile amplitude and frequency"]
pub type MODSTEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `SS_TYPE` reader - PLL spread spectrum modulation type select"]
pub type SS_TYPE_R = crate::BitReader;
#[doc = "Field `SS_TYPE` writer - PLL spread spectrum modulation type select"]
pub type SS_TYPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSCGON` reader - PLL spread spectrum modulation enable"]
pub type SSCGON_R = crate::BitReader;
#[doc = "Field `SSCGON` writer - PLL spread spectrum modulation enable"]
pub type SSCGON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:12 - Configure PLL spread spectrum modulation profile amplitude and frequency"]
    #[inline(always)]
    pub fn modcnt(&self) -> MODCNT_R {
        MODCNT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:27 - Configure PLL spread spectrum modulation profile amplitude and frequency"]
    #[inline(always)]
    pub fn modstep(&self) -> MODSTEP_R {
        MODSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bit 30 - PLL spread spectrum modulation type select"]
    #[inline(always)]
    pub fn ss_type(&self) -> SS_TYPE_R {
        SS_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgon(&self) -> SSCGON_R {
        SSCGON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Configure PLL spread spectrum modulation profile amplitude and frequency"]
    #[inline(always)]
    #[must_use]
    pub fn modcnt(&mut self) -> MODCNT_W<PLLSSCTL_SPEC, 0> {
        MODCNT_W::new(self)
    }
    #[doc = "Bits 13:27 - Configure PLL spread spectrum modulation profile amplitude and frequency"]
    #[inline(always)]
    #[must_use]
    pub fn modstep(&mut self) -> MODSTEP_W<PLLSSCTL_SPEC, 13> {
        MODSTEP_W::new(self)
    }
    #[doc = "Bit 30 - PLL spread spectrum modulation type select"]
    #[inline(always)]
    #[must_use]
    pub fn ss_type(&mut self) -> SS_TYPE_W<PLLSSCTL_SPEC, 30> {
        SS_TYPE_W::new(self)
    }
    #[doc = "Bit 31 - PLL spread spectrum modulation enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscgon(&mut self) -> SSCGON_W<PLLSSCTL_SPEC, 31> {
        SSCGON_W::new(self)
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
#[doc = "PLL clock spread spectrum control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllssctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllssctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLSSCTL_SPEC;
impl crate::RegisterSpec for PLLSSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllssctl::R`](R) reader structure"]
impl crate::Readable for PLLSSCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllssctl::W`](W) writer structure"]
impl crate::Writable for PLLSSCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLSSCTL to value 0"]
impl crate::Resettable for PLLSSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
