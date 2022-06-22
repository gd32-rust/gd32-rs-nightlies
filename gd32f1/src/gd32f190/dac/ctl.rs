#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DEN0` reader - DAC0 enable"]
pub type DEN0_R = crate::BitReader<DEN0_A>;
impl DEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN0_A {
        match self.bits {
            false => DEN0_A::DISABLED,
            true => DEN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEN0_A::ENABLED
    }
}
#[doc = "Field `DEN0` writer - DAC0 enable"]
pub type DEN0_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, DEN0_A, 0>;
impl<'a> DEN0_W<'a> {
    #[doc = "DAC channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEN0_A::DISABLED)
    }
    #[doc = "DAC channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEN0_A::ENABLED)
    }
}
#[doc = "DAC0 output buffer turn off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DBOFF0` reader - DAC0 output buffer turn off"]
pub type DBOFF0_R = crate::BitReader<DBOFF0_A>;
impl DBOFF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBOFF0_A {
        match self.bits {
            false => DBOFF0_A::ENABLED,
            true => DBOFF0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBOFF0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBOFF0_A::DISABLED
    }
}
#[doc = "Field `DBOFF0` writer - DAC0 output buffer turn off"]
pub type DBOFF0_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, DBOFF0_A, 1>;
impl<'a> DBOFF0_W<'a> {
    #[doc = "DAC X output buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBOFF0_A::ENABLED)
    }
    #[doc = "DAC X output buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBOFF0_A::DISABLED)
    }
}
#[doc = "DAC0 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DTEN0` reader - DAC0 trigger enable"]
pub type DTEN0_R = crate::BitReader<DTEN0_A>;
impl DTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN0_A {
        match self.bits {
            false => DTEN0_A::DISABLED,
            true => DTEN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN0_A::ENABLED
    }
}
#[doc = "Field `DTEN0` writer - DAC0 trigger enable"]
pub type DTEN0_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, DTEN0_A, 2>;
impl<'a> DTEN0_W<'a> {
    #[doc = "DAC trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTEN0_A::DISABLED)
    }
    #[doc = "DAC trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTEN0_A::ENABLED)
    }
}
#[doc = "DAC0 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DTSEL0` reader - DAC0 trigger selection"]
pub type DTSEL0_R = crate::FieldReader<u8, DTSEL0_A>;
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
    #[doc = "Checks if the value of the field is `TIMER5_TRGO`"]
    #[inline(always)]
    pub fn is_timer5_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER5_TRGO
    }
    #[doc = "Checks if the value of the field is `TIMER2_TRGO`"]
    #[inline(always)]
    pub fn is_timer2_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER2_TRGO
    }
    #[doc = "Checks if the value of the field is `TIMER14_TRGO`"]
    #[inline(always)]
    pub fn is_timer14_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER14_TRGO
    }
    #[doc = "Checks if the value of the field is `TIMER1_TRGO`"]
    #[inline(always)]
    pub fn is_timer1_trgo(&self) -> bool {
        *self == DTSEL0_A::TIMER1_TRGO
    }
    #[doc = "Checks if the value of the field is `EXTERNAL9`"]
    #[inline(always)]
    pub fn is_external9(&self) -> bool {
        *self == DTSEL0_A::EXTERNAL9
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == DTSEL0_A::SOFTWARE
    }
}
#[doc = "Field `DTSEL0` writer - DAC0 trigger selection"]
pub type DTSEL0_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, DTSEL0_A, 3, 3>;
impl<'a> DTSEL0_W<'a> {
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn timer5_trgo(self) -> &'a mut W {
        self.variant(DTSEL0_A::TIMER5_TRGO)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn timer2_trgo(self) -> &'a mut W {
        self.variant(DTSEL0_A::TIMER2_TRGO)
    }
    #[doc = "Timer 14 TRGO event"]
    #[inline(always)]
    pub fn timer14_trgo(self) -> &'a mut W {
        self.variant(DTSEL0_A::TIMER14_TRGO)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn timer1_trgo(self) -> &'a mut W {
        self.variant(DTSEL0_A::TIMER1_TRGO)
    }
    #[doc = "External line9"]
    #[inline(always)]
    pub fn external9(self) -> &'a mut W {
        self.variant(DTSEL0_A::EXTERNAL9)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(DTSEL0_A::SOFTWARE)
    }
}
#[doc = "DAC0 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DDMAEN0` reader - DAC0 DMA enable"]
pub type DDMAEN0_R = crate::BitReader<DDMAEN0_A>;
impl DDMAEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDMAEN0_A {
        match self.bits {
            false => DDMAEN0_A::DISABLED,
            true => DDMAEN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDMAEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDMAEN0_A::ENABLED
    }
}
#[doc = "Field `DDMAEN0` writer - DAC0 DMA enable"]
pub type DDMAEN0_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, DDMAEN0_A, 12>;
impl<'a> DDMAEN0_W<'a> {
    #[doc = "DAC DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDMAEN0_A::DISABLED)
    }
    #[doc = "DAC DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DDMAEN0_A::ENABLED)
    }
}
#[doc = "DAC0 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DDUDRIE0` reader - DAC0 DMA Underrun Interrupt enable"]
pub type DDUDRIE0_R = crate::BitReader<DDUDRIE0_A>;
impl DDUDRIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDUDRIE0_A {
        match self.bits {
            false => DDUDRIE0_A::DISABLED,
            true => DDUDRIE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDUDRIE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDUDRIE0_A::ENABLED
    }
}
#[doc = "Field `DDUDRIE0` writer - DAC0 DMA Underrun Interrupt enable"]
pub type DDUDRIE0_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, DDUDRIE0_A, 13>;
impl<'a> DDUDRIE0_W<'a> {
    #[doc = "DAC DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDUDRIE0_A::DISABLED)
    }
    #[doc = "DAC DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DDUDRIE0_A::ENABLED)
    }
}
#[doc = "DAC1 output buffer turn off"]
pub use DBOFF0_A as DBOFF1_A;
#[doc = "Field `DBOFF1` reader - DAC1 output buffer turn off"]
pub use DBOFF0_R as DBOFF1_R;
#[doc = "Field `DBOFF1` writer - DAC1 output buffer turn off"]
pub use DBOFF0_W as DBOFF1_W;
#[doc = "DAC1 DMA enable"]
pub use DDMAEN0_A as DDMAEN1_A;
#[doc = "Field `DDMAEN1` reader - DAC1 DMA enable"]
pub use DDMAEN0_R as DDMAEN1_R;
#[doc = "Field `DDMAEN1` writer - DAC1 DMA enable"]
pub use DDMAEN0_W as DDMAEN1_W;
#[doc = "DAC1 DMA Underrun Interrupt enable"]
pub use DDUDRIE0_A as DDUDRIE1_A;
#[doc = "Field `DDUDRIE1` reader - DAC1 DMA Underrun Interrupt enable"]
pub use DDUDRIE0_R as DDUDRIE1_R;
#[doc = "Field `DDUDRIE1` writer - DAC1 DMA Underrun Interrupt enable"]
pub use DDUDRIE0_W as DDUDRIE1_W;
#[doc = "DAC1 enable"]
pub use DEN0_A as DEN1_A;
#[doc = "Field `DEN1` reader - DAC1 enable"]
pub use DEN0_R as DEN1_R;
#[doc = "Field `DEN1` writer - DAC1 enable"]
pub use DEN0_W as DEN1_W;
#[doc = "DAC1 trigger enable"]
pub use DTEN0_A as DTEN1_A;
#[doc = "Field `DTEN1` reader - DAC1 trigger enable"]
pub use DTEN0_R as DTEN1_R;
#[doc = "Field `DTEN1` writer - DAC1 trigger enable"]
pub use DTEN0_W as DTEN1_W;
#[doc = "DAC1 trigger selection"]
pub use DTSEL0_A as DTSEL1_A;
#[doc = "Field `DTSEL1` reader - DAC1 trigger selection"]
pub use DTSEL0_R as DTSEL1_R;
#[doc = "Field `DTSEL1` writer - DAC1 trigger selection"]
pub use DTSEL0_W as DTSEL1_W;
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
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    pub fn den1(&self) -> DEN1_R {
        DEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    pub fn dboff1(&self) -> DBOFF1_R {
        DBOFF1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn dten1(&self) -> DTEN1_R {
        DTEN1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn dtsel1(&self) -> DTSEL1_R {
        DTSEL1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&self) -> DDMAEN1_R {
        DDMAEN1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie1(&self) -> DDUDRIE1_R {
        DDUDRIE1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 enable"]
    #[inline(always)]
    pub fn den0(&mut self) -> DEN0_W {
        DEN0_W::new(self)
    }
    #[doc = "Bit 1 - DAC0 output buffer turn off"]
    #[inline(always)]
    pub fn dboff0(&mut self) -> DBOFF0_W {
        DBOFF0_W::new(self)
    }
    #[doc = "Bit 2 - DAC0 trigger enable"]
    #[inline(always)]
    pub fn dten0(&mut self) -> DTEN0_W {
        DTEN0_W::new(self)
    }
    #[doc = "Bits 3:5 - DAC0 trigger selection"]
    #[inline(always)]
    pub fn dtsel0(&mut self) -> DTSEL0_W {
        DTSEL0_W::new(self)
    }
    #[doc = "Bit 12 - DAC0 DMA enable"]
    #[inline(always)]
    pub fn ddmaen0(&mut self) -> DDMAEN0_W {
        DDMAEN0_W::new(self)
    }
    #[doc = "Bit 13 - DAC0 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie0(&mut self) -> DDUDRIE0_W {
        DDUDRIE0_W::new(self)
    }
    #[doc = "Bit 16 - DAC1 enable"]
    #[inline(always)]
    pub fn den1(&mut self) -> DEN1_W {
        DEN1_W::new(self)
    }
    #[doc = "Bit 17 - DAC1 output buffer turn off"]
    #[inline(always)]
    pub fn dboff1(&mut self) -> DBOFF1_W {
        DBOFF1_W::new(self)
    }
    #[doc = "Bit 18 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn dten1(&mut self) -> DTEN1_W {
        DTEN1_W::new(self)
    }
    #[doc = "Bits 19:21 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn dtsel1(&mut self) -> DTSEL1_W {
        DTSEL1_W::new(self)
    }
    #[doc = "Bit 28 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn ddmaen1(&mut self) -> DDMAEN1_W {
        DDMAEN1_W::new(self)
    }
    #[doc = "Bit 29 - DAC1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ddudrie1(&mut self) -> DDUDRIE1_W {
        DDUDRIE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
