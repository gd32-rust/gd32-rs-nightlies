#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `IRC8MEN` reader - Internal 8MHz RC oscillator Enable"]
pub type IRC8MEN_R = crate::BitReader;
#[doc = "Field `IRC8MEN` writer - Internal 8MHz RC oscillator Enable"]
pub type IRC8MEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IRC8MSTB` reader - IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
pub type IRC8MSTB_R = crate::BitReader;
#[doc = "Field `IRC8MADJ` reader - Internal 8MHz RC Oscillator clock trim adjust value"]
pub type IRC8MADJ_R = crate::FieldReader;
#[doc = "Field `IRC8MADJ` writer - Internal 8MHz RC Oscillator clock trim adjust value"]
pub type IRC8MADJ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `IRC8MCALIB` reader - Internal 8MHz RC Oscillator calibration value register"]
pub type IRC8MCALIB_R = crate::FieldReader;
#[doc = "Field `HXTALEN` reader - External High Speed oscillator Enable"]
pub type HXTALEN_R = crate::BitReader;
#[doc = "Field `HXTALEN` writer - External High Speed oscillator Enable"]
pub type HXTALEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HXTALSTB_R = crate::BitReader;
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_R = crate::BitReader;
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKMEN` reader - HXTAL Clock Monitor Enable"]
pub type CKMEN_R = crate::BitReader;
#[doc = "Field `CKMEN` writer - HXTAL Clock Monitor Enable"]
pub type CKMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PLLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLSTB` reader - PLL Clock Stabilization Flag"]
pub type PLLSTB_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Internal 8MHz RC oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&self) -> IRC8MEN_R {
        IRC8MEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc8mstb(&self) -> IRC8MSTB_R {
        IRC8MSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal 8MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&self) -> IRC8MADJ_R {
        IRC8MADJ_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 8MHz RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc8mcalib(&self) -> IRC8MCALIB_R {
        IRC8MCALIB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&self) -> HXTALEN_R {
        HXTALEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External crystal oscillator (HXTAL) clock stabilization flag"]
    #[inline(always)]
    pub fn hxtalstb(&self) -> HXTALSTB_R {
        HXTALSTB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&self) -> HXTALBPS_R {
        HXTALBPS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&self) -> CKMEN_R {
        CKMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllstb(&self) -> PLLSTB_R {
        PLLSTB_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal 8MHz RC oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc8men(&mut self) -> IRC8MEN_W<CTL0_SPEC, 0> {
        IRC8MEN_W::new(self)
    }
    #[doc = "Bits 3:7 - Internal 8MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    #[must_use]
    pub fn irc8madj(&mut self) -> IRC8MADJ_W<CTL0_SPEC, 3> {
        IRC8MADJ_W::new(self)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalen(&mut self) -> HXTALEN_W<CTL0_SPEC, 16> {
        HXTALEN_W::new(self)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalbps(&mut self) -> HXTALBPS_W<CTL0_SPEC, 18> {
        HXTALBPS_W::new(self)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckmen(&mut self) -> CKMEN_W<CTL0_SPEC, 19> {
        CKMEN_W::new(self)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<CTL0_SPEC, 24> {
        PLLEN_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x83"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x83;
}
