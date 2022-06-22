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
#[doc = "Field `PLLPRESEL` reader - PLL Clock Source Selection"]
pub type PLLPRESEL_R = crate::BitReader<bool>;
#[doc = "Field `PLLPRESEL` writer - PLL Clock Source Selection"]
pub type PLLPRESEL_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 30>;
#[doc = "Field `ADCPSC_3` reader - Bit 3 of ADCPSC"]
pub type ADCPSC_3_R = crate::BitReader<bool>;
#[doc = "Field `ADCPSC_3` writer - Bit 3 of ADCPSC"]
pub type ADCPSC_3_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 30 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllpresel(&self) -> PLLPRESEL_R {
        PLLPRESEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Bit 3 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_3(&self) -> ADCPSC_3_R {
        ADCPSC_3_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllpresel(&mut self) -> PLLPRESEL_W {
        PLLPRESEL_W::new(self)
    }
    #[doc = "Bit 29 - Bit 3 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_3(&mut self) -> ADCPSC_3_W {
        ADCPSC_3_W::new(self)
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
