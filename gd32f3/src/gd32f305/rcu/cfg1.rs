#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREDV0` reader - PREDV0 division factor"]
pub type PREDV0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PREDV0` writer - PREDV0 division factor"]
pub type PREDV0_W<'a> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 4, 0>;
#[doc = "Field `PREDV1` reader - PREDV1 division factor"]
pub type PREDV1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PREDV1` writer - PREDV1 division factor"]
pub type PREDV1_W<'a> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 4, 4>;
#[doc = "Field `PLL1MF` reader - The PLL1 clock multiplication factor"]
pub type PLL1MF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL1MF` writer - The PLL1 clock multiplication factor"]
pub type PLL1MF_W<'a> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `PLL2MF` reader - The PLL2 clock multiplication factor"]
pub type PLL2MF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL2MF` writer - The PLL2 clock multiplication factor"]
pub type PLL2MF_W<'a> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 4, 12>;
#[doc = "Field `PREDV0SEL` reader - PREDV0 input Clock Source Selection"]
pub type PREDV0SEL_R = crate::BitReader<bool>;
#[doc = "Field `PREDV0SEL` writer - PREDV0 input Clock Source Selection"]
pub type PREDV0SEL_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 16>;
#[doc = "Field `I2S1SEL` reader - I2S1 Clock Source Selection"]
pub type I2S1SEL_R = crate::BitReader<bool>;
#[doc = "Field `I2S1SEL` writer - I2S1 Clock Source Selection"]
pub type I2S1SEL_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 17>;
#[doc = "Field `I2S2SEL` reader - I2S2 Clock Source Selection"]
pub type I2S2SEL_R = crate::BitReader<bool>;
#[doc = "Field `I2S2SEL` writer - I2S2 Clock Source Selection"]
pub type I2S2SEL_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 18>;
#[doc = "Field `ADCPSC_3` reader - Bit 4 of ADCPSC"]
pub type ADCPSC_3_R = crate::BitReader<bool>;
#[doc = "Field `ADCPSC_3` writer - Bit 4 of ADCPSC"]
pub type ADCPSC_3_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 29>;
#[doc = "Field `PLLPRESEL` reader - PLL Clock Source Selection"]
pub type PLLPRESEL_R = crate::BitReader<bool>;
#[doc = "Field `PLLPRESEL` writer - PLL Clock Source Selection"]
pub type PLLPRESEL_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 30>;
#[doc = "Field `PLL2MF_4` reader - Bit 5 of PLL2MF"]
pub type PLL2MF_4_R = crate::BitReader<bool>;
#[doc = "Field `PLL2MF_4` writer - Bit 5 of PLL2MF"]
pub type PLL2MF_4_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:3 - PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0(&self) -> PREDV0_R {
        PREDV0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PREDV1 division factor"]
    #[inline(always)]
    pub fn predv1(&self) -> PREDV1_R {
        PREDV1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The PLL1 clock multiplication factor"]
    #[inline(always)]
    pub fn pll1mf(&self) -> PLL1MF_R {
        PLL1MF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The PLL2 clock multiplication factor"]
    #[inline(always)]
    pub fn pll2mf(&self) -> PLL2MF_R {
        PLL2MF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - PREDV0 input Clock Source Selection"]
    #[inline(always)]
    pub fn predv0sel(&self) -> PREDV0SEL_R {
        PREDV0SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2S1 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2S2 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - Bit 4 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_3(&self) -> ADCPSC_3_R {
        ADCPSC_3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllpresel(&self) -> PLLPRESEL_R {
        PLLPRESEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bit 5 of PLL2MF"]
    #[inline(always)]
    pub fn pll2mf_4(&self) -> PLL2MF_4_R {
        PLL2MF_4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0(&mut self) -> PREDV0_W {
        PREDV0_W::new(self)
    }
    #[doc = "Bits 4:7 - PREDV1 division factor"]
    #[inline(always)]
    pub fn predv1(&mut self) -> PREDV1_W {
        PREDV1_W::new(self)
    }
    #[doc = "Bits 8:11 - The PLL1 clock multiplication factor"]
    #[inline(always)]
    pub fn pll1mf(&mut self) -> PLL1MF_W {
        PLL1MF_W::new(self)
    }
    #[doc = "Bits 12:15 - The PLL2 clock multiplication factor"]
    #[inline(always)]
    pub fn pll2mf(&mut self) -> PLL2MF_W {
        PLL2MF_W::new(self)
    }
    #[doc = "Bit 16 - PREDV0 input Clock Source Selection"]
    #[inline(always)]
    pub fn predv0sel(&mut self) -> PREDV0SEL_W {
        PREDV0SEL_W::new(self)
    }
    #[doc = "Bit 17 - I2S1 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W {
        I2S1SEL_W::new(self)
    }
    #[doc = "Bit 18 - I2S2 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W {
        I2S2SEL_W::new(self)
    }
    #[doc = "Bit 29 - Bit 4 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_3(&mut self) -> ADCPSC_3_W {
        ADCPSC_3_W::new(self)
    }
    #[doc = "Bit 30 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllpresel(&mut self) -> PLLPRESEL_W {
        PLLPRESEL_W::new(self)
    }
    #[doc = "Bit 31 - Bit 5 of PLL2MF"]
    #[inline(always)]
    pub fn pll2mf_4(&mut self) -> PLL2MF_4_W {
        PLL2MF_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
