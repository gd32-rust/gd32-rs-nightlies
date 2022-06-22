#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRC28MCALIB` reader - Internal 28M RC Oscillator calibration value register"]
pub type IRC28MCALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRC28MADJ` reader - Internal 28M RC Oscillator clock trim adjust value"]
pub type IRC28MADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRC28MADJ` writer - Internal 28M RC Oscillator clock trim adjust value"]
pub type IRC28MADJ_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 5, 3>;
#[doc = "Field `IRC28MSTB` reader - IRC28M Internal 28M RC Oscillator stabilization Flag"]
pub type IRC28MSTB_R = crate::BitReader<bool>;
#[doc = "Field `IRC28MEN` reader - IRC28M Internal 28M RC oscillator Enable"]
pub type IRC28MEN_R = crate::BitReader<bool>;
#[doc = "Field `IRC28MEN` writer - IRC28M Internal 28M RC oscillator Enable"]
pub type IRC28MEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 8:15 - Internal 28M RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc28mcalib(&self) -> IRC28MCALIB_R {
        IRC28MCALIB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 3:7 - Internal 28M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc28madj(&self) -> IRC28MADJ_R {
        IRC28MADJ_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - IRC28M Internal 28M RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc28mstb(&self) -> IRC28MSTB_R {
        IRC28MSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IRC28M Internal 28M RC oscillator Enable"]
    #[inline(always)]
    pub fn irc28men(&self) -> IRC28MEN_R {
        IRC28MEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:7 - Internal 28M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc28madj(&mut self) -> IRC28MADJ_W {
        IRC28MADJ_W::new(self)
    }
    #[doc = "Bit 0 - IRC28M Internal 28M RC oscillator Enable"]
    #[inline(always)]
    pub fn irc28men(&mut self) -> IRC28MEN_W {
        IRC28MEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0x80"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
