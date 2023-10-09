#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `DEN0` reader - DAC0 enable"]
pub type DEN0_R = crate::BitReader<DEN0_A>;
#[doc = "DAC0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN0_A {
    #[doc = "0: DAC channel disabled"]
    DISABLED = 0,
    #[doc = "1: DAC channel enabled"]
    ENABLED = 1,
}
impl From<DEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN0_A {
        match self.bits {
            false => DEN0_A::DISABLED,
            true => DEN0_A::ENABLED,
        }
    }
    #[doc = "DAC channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEN0_A::DISABLED
    }
    #[doc = "DAC channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEN0_A::ENABLED
    }
}
#[doc = "Field `DEN0` writer - DAC0 enable"]
pub type DEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DEN0_A>;
impl<'a, REG, const O: u8> DEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEN0_A::DISABLED)
    }
    #[doc = "DAC channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEN0_A::ENABLED)
    }
}
#[doc = "Field `DBOFF0` reader - DAC0 output buffer turn off"]
pub type DBOFF0_R = crate::BitReader<DBOFF0_A>;
#[doc = "DAC0 output buffer turn off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBOFF0_A {
    #[doc = "0: DAC X output buffer enabled"]
    ENABLED = 0,
    #[doc = "1: DAC X output buffer disabled"]
    DISABLED = 1,
}
impl From<DBOFF0_A> for bool {
    #[inline(always)]
    fn from(variant: DBOFF0_A) -> Self {
        variant as u8 != 0
    }
}
impl DBOFF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBOFF0_A {
        match self.bits {
            false => DBOFF0_A::ENABLED,
            true => DBOFF0_A::DISABLED,
        }
    }
    #[doc = "DAC X output buffer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBOFF0_A::ENABLED
    }
    #[doc = "DAC X output buffer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBOFF0_A::DISABLED
    }
}
#[doc = "Field `DBOFF0` writer - DAC0 output buffer turn off"]
pub type DBOFF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DBOFF0_A>;
impl<'a, REG, const O: u8> DBOFF0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC X output buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBOFF0_A::ENABLED)
    }
    #[doc = "DAC X output buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBOFF0_A::DISABLED)
    }
}
#[doc = "Field `DTEN0` reader - DAC0 trigger enable"]
pub type DTEN0_R = crate::BitReader<DTEN0_A>;
#[doc = "DAC0 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN0_A {
    #[doc = "0: DAC trigger disabled"]
    DISABLED = 0,
    #[doc = "1: DAC trigger enabled"]
    ENABLED = 1,
}
impl From<DTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN0_A {
        match self.bits {
            false => DTEN0_A::DISABLED,
            true => DTEN0_A::ENABLED,
        }
    }
    #[doc = "DAC trigger disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN0_A::DISABLED
    }
    #[doc = "DAC trigger enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN0_A::ENABLED
    }
}
#[doc = "Field `DTEN0` writer - DAC0 trigger enable"]
pub type DTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTEN0_A>;
impl<'a, REG, const O: u8> DTEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN0_A::DISABLED)
    }
    #[doc = "DAC trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN0_A::ENABLED)
    }
}
#[doc = "Field `DTSEL0` reader - DAC0 trigger selection"]
pub type DTSEL0_R = crate::FieldReader<DTSEL0_A>;
#[doc = "DAC0 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTSEL0_A {
    #[doc = "0: Timer 5 TRGO event"]
    TIMER5_TRGO = 0,
    #[doc = "1: Timer 2 TRGO event"]
    TIMER2_TRGO = 1,
    #[doc = "3: Timer 14 TRGO event"]
    TIMER14_TRGO = 3,
    #[doc = "4: Timer 1 TRGO event"]
    TIMER1_TRGO = 4,
    #[doc = "6: External line9"]
    EXTERNAL9 = 6,
    #[doc = "7: Software trigger"]
    SOFTWARE = 7,
}
impl From<DTSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DTSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTSEL0_A {
    type Ux = u8;
}
impl DTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTSEL0_A> {
        match self.bits {
            0 => Some(DTSEL0_A::TIMER5_TRGO),
            1 => Some(DTSEL0_A::TIMER2_TRGO),
            3 => Some(DTSEL0_A::TIMER14_TRGO),
            4 => Some(DTSEL0_A::TIMER1_TRGO),
            6 => Some(DTSEL0_A::EXTERNAL9),
            7 => Some(DTSEL0_A::SOFTWARE),
            _ => None,
        }
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn is_timer5_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER5_TRGO
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_timer2_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER2_TRGO
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn is_timer14_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER14_TRGO
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn is_timer1_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER1_TRGO
    }
    #[doc = "External line9"]
    #[inline(always)]
    pub fn is_external9(&self) -> bool {
        *self == DTSEL0_A::EXTERNAL9
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == DTSEL0_A::SOFTWARE
    }
}
#[doc = "Field `DTSEL0` writer - DAC0 trigger selection"]
pub type DTSEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, DTSEL0_A>;
impl<'a, REG, const O: u8> DTSEL0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn timer5_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::TIMER5_TRGO)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn timer2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::TIMER2_TRGO)
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn timer14_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::TIMER14_TRGO)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn timer1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::TIMER1_TRGO)
    }
    #[doc = "External line9"]
    #[inline(always)]
    pub fn external9(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::EXTERNAL9)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEL0_A::SOFTWARE)
    }
}
#[doc = "Field `DDMAEN0` reader - DAC0 DMA enable"]
pub type DDMAEN0_R = crate::BitReader<DDMAEN0_A>;
#[doc = "DAC0 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDMAEN0_A {
    #[doc = "0: DAC DMA mode disabled"]
    DISABLED = 0,
    #[doc = "1: DAC DMA mode enabled"]
    ENABLED = 1,
}
impl From<DDMAEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DDMAEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl DDMAEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDMAEN0_A {
        match self.bits {
            false => DDMAEN0_A::DISABLED,
            true => DDMAEN0_A::ENABLED,
        }
    }
    #[doc = "DAC DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDMAEN0_A::DISABLED
    }
    #[doc = "DAC DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDMAEN0_A::ENABLED
    }
}
#[doc = "Field `DDMAEN0` writer - DAC0 DMA enable"]
pub type DDMAEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DDMAEN0_A>;
impl<'a, REG, const O: u8> DDMAEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDMAEN0_A::DISABLED)
    }
    #[doc = "DAC DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDMAEN0_A::ENABLED)
    }
}
#[doc = "Field `DDUDRIE0` reader - DAC0 DMA Underrun Interrupt enable"]
pub type DDUDRIE0_R = crate::BitReader<DDUDRIE0_A>;
#[doc = "DAC0 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDUDRIE0_A {
    #[doc = "0: DAC DMA Underrun Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: DAC DMA Underrun Interrupt enabled"]
    ENABLED = 1,
}
impl From<DDUDRIE0_A> for bool {
    #[inline(always)]
    fn from(variant: DDUDRIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DDUDRIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDUDRIE0_A {
        match self.bits {
            false => DDUDRIE0_A::DISABLED,
            true => DDUDRIE0_A::ENABLED,
        }
    }
    #[doc = "DAC DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDUDRIE0_A::DISABLED
    }
    #[doc = "DAC DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDUDRIE0_A::ENABLED
    }
}
#[doc = "Field `DDUDRIE0` writer - DAC0 DMA Underrun Interrupt enable"]
pub type DDUDRIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DDUDRIE0_A>;
impl<'a, REG, const O: u8> DDUDRIE0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDUDRIE0_A::DISABLED)
    }
    #[doc = "DAC DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDUDRIE0_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    pub fn den0(&self) -> DEN0_R {
        DEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    pub fn dboff0(&self) -> DBOFF0_R {
        DBOFF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    pub fn dten0(&self) -> DTEN0_R {
        DTEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    pub fn dtsel0(&self) -> DTSEL0_R {
        DTSEL0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&self) -> DDMAEN0_R {
        DDMAEN0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC0 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie0(&self) -> DDUDRIE0_R {
        DDUDRIE0_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn den0(&mut self) -> DEN0_W<CTL_SPEC, 0> {
        DEN0_W::new(self)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    #[must_use]
    pub fn dboff0(&mut self) -> DBOFF0_W<CTL_SPEC, 1> {
        DBOFF0_W::new(self)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten0(&mut self) -> DTEN0_W<CTL_SPEC, 2> {
        DTEN0_W::new(self)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtsel0(&mut self) -> DTSEL0_W<CTL_SPEC, 3> {
        DTSEL0_W::new(self)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddmaen0(&mut self) -> DDMAEN0_W<CTL_SPEC, 12> {
        DDMAEN0_W::new(self)
    }
    #[doc = "Bit 13 - DAC0 DMA Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddudrie0(&mut self) -> DDUDRIE0_W<CTL_SPEC, 13> {
        DDUDRIE0_W::new(self)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
