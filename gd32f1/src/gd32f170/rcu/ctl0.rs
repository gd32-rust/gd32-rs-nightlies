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
#[doc = "Internal High Speed oscillator Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC8MEN_A {
    #[doc = "0: Clock Off"]
    OFF = 0,
    #[doc = "1: Clock On"]
    ON = 1,
}
impl From<IRC8MEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRC8MEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MEN` reader - Internal High Speed oscillator Enable"]
pub type IRC8MEN_R = crate::BitReader<IRC8MEN_A>;
impl IRC8MEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC8MEN_A {
        match self.bits {
            false => IRC8MEN_A::OFF,
            true => IRC8MEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IRC8MEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == IRC8MEN_A::ON
    }
}
#[doc = "Field `IRC8MEN` writer - Internal High Speed oscillator Enable"]
pub type IRC8MEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, IRC8MEN_A, 0>;
impl<'a> IRC8MEN_W<'a> {
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IRC8MEN_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(IRC8MEN_A::ON)
    }
}
#[doc = "IRC8M High Speed Internal Oscillator stabilization Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC8MSTB_A {
    #[doc = "0: IRC8M is not stable"]
    NOTREADY = 0,
    #[doc = "1: IRC8M is stable"]
    READY = 1,
}
impl From<IRC8MSTB_A> for bool {
    #[inline(always)]
    fn from(variant: IRC8MSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC8MSTB` reader - IRC8M High Speed Internal Oscillator stabilization Flag"]
pub type IRC8MSTB_R = crate::BitReader<IRC8MSTB_A>;
impl IRC8MSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC8MSTB_A {
        match self.bits {
            false => IRC8MSTB_A::NOTREADY,
            true => IRC8MSTB_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == IRC8MSTB_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == IRC8MSTB_A::READY
    }
}
#[doc = "Field `IRC8MADJ` reader - High Speed Internal Oscillator clock trim adjust value"]
pub type IRC8MADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRC8MADJ` writer - High Speed Internal Oscillator clock trim adjust value"]
pub type IRC8MADJ_W<'a> = crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, u8, 5, 3>;
#[doc = "Field `IRC8MCALIB` reader - High Speed Internal Oscillator calibration value register"]
pub type IRC8MCALIB_R = crate::FieldReader<u8, u8>;
#[doc = "External High Speed oscillator Enable"]
pub use IRC8MEN_A as HXTALEN_A;
#[doc = "Field `HXTALEN` reader - External High Speed oscillator Enable"]
pub use IRC8MEN_R as HXTALEN_R;
#[doc = "Field `HXTALEN` writer - External High Speed oscillator Enable"]
pub use IRC8MEN_W as HXTALEN_W;
#[doc = "External crystal oscillator (HXTAL) clock stabilization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTALSTB_A {
    #[doc = "0: HXTAL is not stable"]
    NOTREADY = 0,
    #[doc = "1: HXTAL is stable"]
    READY = 1,
}
impl From<HXTALSTB_A> for bool {
    #[inline(always)]
    fn from(variant: HXTALSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HXTALSTB_R = crate::BitReader<HXTALSTB_A>;
impl HXTALSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALSTB_A {
        match self.bits {
            false => HXTALSTB_A::NOTREADY,
            true => HXTALSTB_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HXTALSTB_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HXTALSTB_A::READY
    }
}
#[doc = "External crystal oscillator (HXTAL) clock bypass mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HXTALBPS_A {
    #[doc = "0: HXTAL crystal oscillator not bypassed"]
    NOTBYPASSED = 0,
    #[doc = "1: HXTAL crystal oscillator bypassed with external clock"]
    BYPASSED = 1,
}
impl From<HXTALBPS_A> for bool {
    #[inline(always)]
    fn from(variant: HXTALBPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_R = crate::BitReader<HXTALBPS_A>;
impl HXTALBPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALBPS_A {
        match self.bits {
            false => HXTALBPS_A::NOTBYPASSED,
            true => HXTALBPS_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HXTALBPS_A::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HXTALBPS_A::BYPASSED
    }
}
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, HXTALBPS_A, 18>;
impl<'a> HXTALBPS_W<'a> {
    #[doc = "HXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HXTALBPS_A::NOTBYPASSED)
    }
    #[doc = "HXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HXTALBPS_A::BYPASSED)
    }
}
#[doc = "HXTAL Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKMEN_A {
    #[doc = "0: Clock monitor disabled"]
    OFF = 0,
    #[doc = "1: Clock monitor enabled"]
    ON = 1,
}
impl From<CKMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMEN` reader - HXTAL Clock Monitor Enable"]
pub type CKMEN_R = crate::BitReader<CKMEN_A>;
impl CKMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMEN_A {
        match self.bits {
            false => CKMEN_A::OFF,
            true => CKMEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CKMEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CKMEN_A::ON
    }
}
#[doc = "Field `CKMEN` writer - HXTAL Clock Monitor Enable"]
pub type CKMEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, CKMEN_A, 19>;
impl<'a> CKMEN_W<'a> {
    #[doc = "Clock monitor disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CKMEN_A::OFF)
    }
    #[doc = "Clock monitor enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CKMEN_A::ON)
    }
}
#[doc = "PLL enable"]
pub use IRC8MEN_A as PLLEN_A;
#[doc = "Field `PLLEN` reader - PLL enable"]
pub use IRC8MEN_R as PLLEN_R;
#[doc = "Field `PLLEN` writer - PLL enable"]
pub use IRC8MEN_W as PLLEN_W;
#[doc = "PLL Clock Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTB_A {
    #[doc = "0: PLL is not stable"]
    NOTREADY = 0,
    #[doc = "1: PLL is stable"]
    READY = 1,
}
impl From<PLLSTB_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTB` reader - PLL Clock Stabilization Flag"]
pub type PLLSTB_R = crate::BitReader<PLLSTB_A>;
impl PLLSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTB_A {
        match self.bits {
            false => PLLSTB_A::NOTREADY,
            true => PLLSTB_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PLLSTB_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PLLSTB_A::READY
    }
}
impl R {
    #[doc = "Bit 0 - Internal High Speed oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&self) -> IRC8MEN_R {
        IRC8MEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC8M High Speed Internal Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc8mstb(&self) -> IRC8MSTB_R {
        IRC8MSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - High Speed Internal Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&self) -> IRC8MADJ_R {
        IRC8MADJ_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - High Speed Internal Oscillator calibration value register"]
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
    #[doc = "Bit 0 - Internal High Speed oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&mut self) -> IRC8MEN_W {
        IRC8MEN_W::new(self)
    }
    #[doc = "Bits 3:7 - High Speed Internal Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&mut self) -> IRC8MADJ_W {
        IRC8MADJ_W::new(self)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&mut self) -> HXTALEN_W {
        HXTALEN_W::new(self)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&mut self) -> HXTALBPS_W {
        HXTALBPS_W::new(self)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&mut self) -> CKMEN_W {
        CKMEN_W::new(self)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W::new(self)
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
