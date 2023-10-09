#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `IRC14MEN` reader - IRC14M Internal 14M RC oscillator Enable"]
pub type IRC14MEN_R = crate::BitReader<IRC14MEN_A>;
#[doc = "IRC14M Internal 14M RC oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IRC14MEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC14MEN_A {
        match self.bits {
            false => IRC14MEN_A::OFF,
            true => IRC14MEN_A::ON,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IRC14MEN_A::OFF
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == IRC14MEN_A::ON
    }
}
#[doc = "Field `IRC14MEN` writer - IRC14M Internal 14M RC oscillator Enable"]
pub type IRC14MEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRC14MEN_A>;
impl<'a, REG, const O: u8> IRC14MEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(IRC14MEN_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(IRC14MEN_A::ON)
    }
}
#[doc = "Field `IRC14MSTB` reader - IRC14M Internal 14M RC Oscillator stabilization Flag"]
pub type IRC14MSTB_R = crate::BitReader<IRC14MSTBR_A>;
#[doc = "IRC14M Internal 14M RC Oscillator stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC14MSTBR_A {
    #[doc = "0: IRC14M is not stable"]
    NOT_READY = 0,
    #[doc = "1: IRC14M is stable"]
    READY = 1,
}
impl From<IRC14MSTBR_A> for bool {
    #[inline(always)]
    fn from(variant: IRC14MSTBR_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC14MSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC14MSTBR_A {
        match self.bits {
            false => IRC14MSTBR_A::NOT_READY,
            true => IRC14MSTBR_A::READY,
        }
    }
    #[doc = "IRC14M is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == IRC14MSTBR_A::NOT_READY
    }
    #[doc = "IRC14M is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == IRC14MSTBR_A::READY
    }
}
#[doc = "Field `IRC14MADJ` reader - Internal 14M RC Oscillator clock trim adjust value"]
pub type IRC14MADJ_R = crate::FieldReader;
#[doc = "Field `IRC14MADJ` writer - Internal 14M RC Oscillator clock trim adjust value"]
pub type IRC14MADJ_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `IRC14MCALIB` reader - Internal 14M RC Oscillator calibration value register"]
pub type IRC14MCALIB_R = crate::FieldReader;
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
    #[must_use]
    pub fn irc14men(&mut self) -> IRC14MEN_W<CTL1_SPEC, 0> {
        IRC14MEN_W::new(self)
    }
    #[doc = "Bits 3:7 - Internal 14M RC Oscillator clock trim adjust value"]
    #[inline(always)]
    #[must_use]
    pub fn irc14madj(&mut self) -> IRC14MADJ_W<CTL1_SPEC, 3> {
        IRC14MADJ_W::new(self)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x80"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
