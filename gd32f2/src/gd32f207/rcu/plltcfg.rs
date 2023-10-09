#[doc = "Register `PLLTCFG` reader"]
pub type R = crate::R<PLLTCFG_SPEC>;
#[doc = "Register `PLLTCFG` writer"]
pub type W = crate::W<PLLTCFG_SPEC>;
#[doc = "Field `PLLTPSC` reader - PLLT prescaler selection"]
pub type PLLTPSC_R = crate::FieldReader;
#[doc = "Field `PLLTPSC` writer - PLLT prescaler selection"]
pub type PLLTPSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `PLLTMF` reader - PLLT multiply factor for VCO"]
pub type PLLTMF_R = crate::FieldReader<u16>;
#[doc = "Field `PLLTMF` writer - PLLT multiply factor for VCO"]
pub type PLLTMF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `TLIPSC` reader - TLI prescaler selection"]
pub type TLIPSC_R = crate::FieldReader;
#[doc = "Field `TLIPSC` writer - TLI prescaler selection"]
pub type TLIPSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PLLTRPSC` reader - PLLTR prescaler selection"]
pub type PLLTRPSC_R = crate::FieldReader;
#[doc = "Field `PLLTRPSC` writer - PLLTR prescaler selection"]
pub type PLLTRPSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PLLTSEL` reader - PLLT clock source select"]
pub type PLLTSEL_R = crate::BitReader;
#[doc = "Field `PLLTSEL` writer - PLLT clock source select"]
pub type PLLTSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - PLLT prescaler selection"]
    #[inline(always)]
    pub fn plltpsc(&self) -> PLLTPSC_R {
        PLLTPSC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLLT multiply factor for VCO"]
    #[inline(always)]
    pub fn plltmf(&self) -> PLLTMF_R {
        PLLTMF_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - TLI prescaler selection"]
    #[inline(always)]
    pub fn tlipsc(&self) -> TLIPSC_R {
        TLIPSC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 28:30 - PLLTR prescaler selection"]
    #[inline(always)]
    pub fn plltrpsc(&self) -> PLLTRPSC_R {
        PLLTRPSC_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - PLLT clock source select"]
    #[inline(always)]
    pub fn plltsel(&self) -> PLLTSEL_R {
        PLLTSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLLT prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn plltpsc(&mut self) -> PLLTPSC_W<PLLTCFG_SPEC, 0> {
        PLLTPSC_W::new(self)
    }
    #[doc = "Bits 6:14 - PLLT multiply factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plltmf(&mut self) -> PLLTMF_W<PLLTCFG_SPEC, 6> {
        PLLTMF_W::new(self)
    }
    #[doc = "Bits 16:17 - TLI prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn tlipsc(&mut self) -> TLIPSC_W<PLLTCFG_SPEC, 16> {
        TLIPSC_W::new(self)
    }
    #[doc = "Bits 28:30 - PLLTR prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn plltrpsc(&mut self) -> PLLTRPSC_W<PLLTCFG_SPEC, 28> {
        PLLTRPSC_W::new(self)
    }
    #[doc = "Bit 31 - PLLT clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn plltsel(&mut self) -> PLLTSEL_W<PLLTCFG_SPEC, 31> {
        PLLTSEL_W::new(self)
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
#[doc = "PLLT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plltcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plltcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLTCFG_SPEC;
impl crate::RegisterSpec for PLLTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plltcfg::R`](R) reader structure"]
impl crate::Readable for PLLTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plltcfg::W`](W) writer structure"]
impl crate::Writable for PLLTCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLTCFG to value 0x2000_3010"]
impl crate::Resettable for PLLTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_3010;
}
