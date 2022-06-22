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
#[doc = "IRC14M Internal 14M RC oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC14MEN_A {
    #[doc = "0: Clock Off"]
    OFF = 0,
    #[doc = "1: Clock On"]
    ON = 1,
}
impl From<IRC14MEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRC14MEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC14MEN` reader - IRC14M Internal 14M RC oscillator Enable"]
pub type IRC14MEN_R = crate::BitReader<IRC14MEN_A>;
impl IRC14MEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC14MEN_A {
        match self.bits {
            false => IRC14MEN_A::OFF,
            true => IRC14MEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IRC14MEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == IRC14MEN_A::ON
    }
}
#[doc = "Field `IRC14MEN` writer - IRC14M Internal 14M RC oscillator Enable"]
pub type IRC14MEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, IRC14MEN_A, 0>;
impl<'a> IRC14MEN_W<'a> {
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IRC14MEN_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(IRC14MEN_A::ON)
    }
}
#[doc = "IRC14M Internal 14M RC Oscillator stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC14MSTB_A {
    #[doc = "0: IRC14M is not stable"]
    NOTREADY = 0,
    #[doc = "1: IRC14M is stable"]
    READY = 1,
}
impl From<IRC14MSTB_A> for bool {
    #[inline(always)]
    fn from(variant: IRC14MSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC14MSTB` reader - IRC14M Internal 14M RC Oscillator stabilization Flag"]
pub type IRC14MSTB_R = crate::BitReader<IRC14MSTB_A>;
impl IRC14MSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC14MSTB_A {
        match self.bits {
            false => IRC14MSTB_A::NOTREADY,
            true => IRC14MSTB_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == IRC14MSTB_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == IRC14MSTB_A::READY
    }
}
#[doc = "Field `IRC14MADJ` reader - Internal 14M RC Oscillator clock trim adjust value"]
pub type IRC14MADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRC14MADJ` writer - Internal 14M RC Oscillator clock trim adjust value"]
pub type IRC14MADJ_W<'a> = crate::FieldWriterSafe<'a, u32, CTL1_SPEC, u8, u8, 5, 3>;
#[doc = "Field `IRC14MCALIB` reader - Internal 14M RC Oscillator calibration value register"]
pub type IRC14MCALIB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - IRC14M Internal 14M RC oscillator Enable"]
    #[inline(always)]
    pub fn irc14men(&self) -> IRC14MEN_R {
        IRC14MEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC14M Internal 14M RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc14mstb(&self) -> IRC14MSTB_R {
        IRC14MSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal 14M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc14madj(&self) -> IRC14MADJ_R {
        IRC14MADJ_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 14M RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc14mcalib(&self) -> IRC14MCALIB_R {
        IRC14MCALIB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IRC14M Internal 14M RC oscillator Enable"]
    #[inline(always)]
    pub fn irc14men(&mut self) -> IRC14MEN_W {
        IRC14MEN_W::new(self)
    }
    #[doc = "Bits 3:7 - Internal 14M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc14madj(&mut self) -> IRC14MADJ_W {
        IRC14MADJ_W::new(self)
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
