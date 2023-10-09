#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `IRC8MEN` reader - Internal 8MHz RC oscillator Enable"]
pub type IRC8MEN_R = crate::BitReader<IRC8MEN_A>;
#[doc = "Internal 8MHz RC oscillator Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IRC8MEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC8MEN_A {
        match self.bits {
            false => IRC8MEN_A::OFF,
            true => IRC8MEN_A::ON,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IRC8MEN_A::OFF
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == IRC8MEN_A::ON
    }
}
#[doc = "Field `IRC8MEN` writer - Internal 8MHz RC oscillator Enable"]
pub type IRC8MEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRC8MEN_A>;
impl<'a, REG, const O: u8> IRC8MEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(IRC8MEN_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(IRC8MEN_A::ON)
    }
}
#[doc = "Field `IRC8MSTB` reader - IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
pub type IRC8MSTB_R = crate::BitReader<IRC8MSTBR_A>;
#[doc = "IRC8M Internal 8MHz RC Oscillator stabilization Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRC8MSTBR_A {
    #[doc = "0: IRC8M is not stable"]
    NOT_READY = 0,
    #[doc = "1: IRC8M is stable"]
    READY = 1,
}
impl From<IRC8MSTBR_A> for bool {
    #[inline(always)]
    fn from(variant: IRC8MSTBR_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC8MSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC8MSTBR_A {
        match self.bits {
            false => IRC8MSTBR_A::NOT_READY,
            true => IRC8MSTBR_A::READY,
        }
    }
    #[doc = "IRC8M is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == IRC8MSTBR_A::NOT_READY
    }
    #[doc = "IRC8M is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == IRC8MSTBR_A::READY
    }
}
#[doc = "Field `IRC8MADJ` reader - Internal 8MHz RC Oscillator clock trim adjust value"]
pub type IRC8MADJ_R = crate::FieldReader;
#[doc = "Field `IRC8MADJ` writer - Internal 8MHz RC Oscillator clock trim adjust value"]
pub type IRC8MADJ_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 5, O>;
#[doc = "Field `IRC8MCALIB` reader - Internal 8MHz RC Oscillator calibration value register"]
pub type IRC8MCALIB_R = crate::FieldReader;
#[doc = "Field `HXTALEN` reader - External High Speed oscillator Enable"]
pub use IRC8MEN_R as HXTALEN_R;
#[doc = "Field `HXTALEN` writer - External High Speed oscillator Enable"]
pub use IRC8MEN_W as HXTALEN_W;
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HXTALSTB_R = crate::BitReader<HXTALSTBR_A>;
#[doc = "External crystal oscillator (HXTAL) clock stabilization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HXTALSTBR_A {
    #[doc = "0: HXTAL is not stable"]
    NOT_READY = 0,
    #[doc = "1: HXTAL is stable"]
    READY = 1,
}
impl From<HXTALSTBR_A> for bool {
    #[inline(always)]
    fn from(variant: HXTALSTBR_A) -> Self {
        variant as u8 != 0
    }
}
impl HXTALSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALSTBR_A {
        match self.bits {
            false => HXTALSTBR_A::NOT_READY,
            true => HXTALSTBR_A::READY,
        }
    }
    #[doc = "HXTAL is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HXTALSTBR_A::NOT_READY
    }
    #[doc = "HXTAL is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HXTALSTBR_A::READY
    }
}
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_R = crate::BitReader<HXTALBPS_A>;
#[doc = "External crystal oscillator (HXTAL) clock bypass mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HXTALBPS_A {
    #[doc = "0: HXTAL crystal oscillator not bypassed"]
    NOT_BYPASSED = 0,
    #[doc = "1: HXTAL crystal oscillator bypassed with external clock"]
    BYPASSED = 1,
}
impl From<HXTALBPS_A> for bool {
    #[inline(always)]
    fn from(variant: HXTALBPS_A) -> Self {
        variant as u8 != 0
    }
}
impl HXTALBPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HXTALBPS_A {
        match self.bits {
            false => HXTALBPS_A::NOT_BYPASSED,
            true => HXTALBPS_A::BYPASSED,
        }
    }
    #[doc = "HXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HXTALBPS_A::NOT_BYPASSED
    }
    #[doc = "HXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HXTALBPS_A::BYPASSED
    }
}
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HXTALBPS_A>;
impl<'a, REG, const O: u8> HXTALBPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALBPS_A::NOT_BYPASSED)
    }
    #[doc = "HXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HXTALBPS_A::BYPASSED)
    }
}
#[doc = "Field `CKMEN` reader - HXTAL Clock Monitor Enable"]
pub type CKMEN_R = crate::BitReader<CKMEN_A>;
#[doc = "HXTAL Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CKMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMEN_A {
        match self.bits {
            false => CKMEN_A::OFF,
            true => CKMEN_A::ON,
        }
    }
    #[doc = "Clock monitor disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CKMEN_A::OFF
    }
    #[doc = "Clock monitor enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CKMEN_A::ON
    }
}
#[doc = "Field `CKMEN` writer - HXTAL Clock Monitor Enable"]
pub type CKMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CKMEN_A>;
impl<'a, REG, const O: u8> CKMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock monitor disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CKMEN_A::OFF)
    }
    #[doc = "Clock monitor enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CKMEN_A::ON)
    }
}
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PLLEN_R = crate::BitReader<PLLEN_A>;
#[doc = "PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLEN_A {
    #[doc = "0: Clock Off"]
    OFF = 0,
    #[doc = "1: Clock On"]
    ON = 1,
}
impl From<PLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLEN_A {
        match self.bits {
            false => PLLEN_A::OFF,
            true => PLLEN_A::ON,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PLLEN_A::OFF
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PLLEN_A::ON
    }
}
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PLLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PLLEN_A>;
impl<'a, REG, const O: u8> PLLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PLLEN_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(PLLEN_A::ON)
    }
}
#[doc = "Field `PLLSTB` reader - PLL Clock Stabilization Flag"]
pub type PLLSTB_R = crate::BitReader<PLLSTBR_A>;
#[doc = "PLL Clock Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTBR_A {
    #[doc = "0: PLL is not stable"]
    NOT_READY = 0,
    #[doc = "1: PLL is stable"]
    READY = 1,
}
impl From<PLLSTBR_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBR_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTBR_A {
        match self.bits {
            false => PLLSTBR_A::NOT_READY,
            true => PLLSTBR_A::READY,
        }
    }
    #[doc = "PLL is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PLLSTBR_A::NOT_READY
    }
    #[doc = "PLL is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PLLSTBR_A::READY
    }
}
#[doc = "Field `PLL1EN` reader - PLL1 enable"]
pub use PLLEN_R as PLL1EN_R;
#[doc = "Field `PLL2EN` reader - PLL2 enable"]
pub use PLLEN_R as PLL2EN_R;
#[doc = "Field `PLL1EN` writer - PLL1 enable"]
pub use PLLEN_W as PLL1EN_W;
#[doc = "Field `PLL2EN` writer - PLL2 enable"]
pub use PLLEN_W as PLL2EN_W;
#[doc = "Field `PLL1STB` reader - PLL1 Clock Stabilization Flag"]
pub use PLLSTB_R as PLL1STB_R;
#[doc = "Field `PLL2STB` reader - PLL2 Clock Stabilization Flag"]
pub use PLLSTB_R as PLL2STB_R;
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
    #[doc = "Bit 26 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1en(&self) -> PLL1EN_R {
        PLL1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL1 Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pll1stb(&self) -> PLL1STB_R {
        PLL1STB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2en(&self) -> PLL2EN_R {
        PLL2EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLL2 Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pll2stb(&self) -> PLL2STB_R {
        PLL2STB_R::new(((self.bits >> 29) & 1) != 0)
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
    #[doc = "Bit 26 - PLL1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1en(&mut self) -> PLL1EN_W<CTL0_SPEC, 26> {
        PLL1EN_W::new(self)
    }
    #[doc = "Bit 28 - PLL2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2en(&mut self) -> PLL2EN_W<CTL0_SPEC, 28> {
        PLL2EN_W::new(self)
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
