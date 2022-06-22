#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSTB` reader - PLL Clock Stabilization Flag"]
pub type PLLSTB_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PLLEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PLLEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 24>;
#[doc = "Field `CKMEN` reader - HXTAL Clock Monitor Enable"]
pub type CKMEN_R = crate::BitReader<bool>;
#[doc = "Field `CKMEN` writer - HXTAL Clock Monitor Enable"]
pub type CKMEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 19>;
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_R = crate::BitReader<bool>;
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 18>;
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HXTALSTB_R = crate::BitReader<bool>;
#[doc = "Field `HXTALEN` reader - External High Speed oscillator Enable"]
pub type HXTALEN_R = crate::BitReader<bool>;
#[doc = "Field `HXTALEN` writer - External High Speed oscillator Enable"]
pub type HXTALEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 16>;
#[doc = "Field `IRC8MCALIB` reader - High Speed Internal Oscillator calibration value register"]
pub type IRC8MCALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRC8MADJ` reader - High Speed Internal Oscillator clock trim adjust value"]
pub type IRC8MADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRC8MADJ` writer - High Speed Internal Oscillator clock trim adjust value"]
pub type IRC8MADJ_W<'a> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 5, 3>;
#[doc = "Field `IRC8MSTB` reader - IRC8M High Speed Internal Oscillator stabilization Flag"]
pub type IRC8MSTB_R = crate::BitReader<bool>;
#[doc = "Field `IRC8MEN` reader - Internal High Speed oscillator Enable"]
pub type IRC8MEN_R = crate::BitReader<bool>;
#[doc = "Field `IRC8MEN` writer - Internal High Speed oscillator Enable"]
pub type IRC8MEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 25 - PLL Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllstb(&self) -> PLLSTB_R {
        PLLSTB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&self) -> CKMEN_R {
        CKMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&self) -> HXTALBPS_R {
        HXTALBPS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - External crystal oscillator (HXTAL) clock stabilization flag"]
    #[inline(always)]
    pub fn hxtalstb(&self) -> HXTALSTB_R {
        HXTALSTB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&self) -> HXTALEN_R {
        HXTALEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:15 - High Speed Internal Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc8mcalib(&self) -> IRC8MCALIB_R {
        IRC8MCALIB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 3:7 - High Speed Internal Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&self) -> IRC8MADJ_R {
        IRC8MADJ_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - IRC8M High Speed Internal Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc8mstb(&self) -> IRC8MSTB_R {
        IRC8MSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Internal High Speed oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&self) -> IRC8MEN_R {
        IRC8MEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W::new(self)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&mut self) -> CKMEN_W {
        CKMEN_W::new(self)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&mut self) -> HXTALBPS_W {
        HXTALBPS_W::new(self)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&mut self) -> HXTALEN_W {
        HXTALEN_W::new(self)
    }
    #[doc = "Bits 3:7 - High Speed Internal Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&mut self) -> IRC8MADJ_W {
        IRC8MADJ_W::new(self)
    }
    #[doc = "Bit 0 - Internal High Speed oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&mut self) -> IRC8MEN_W {
        IRC8MEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0x83"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x83
    }
}
