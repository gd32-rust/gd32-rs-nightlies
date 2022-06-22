#[doc = "Register `INTF` reader"]
pub struct R(crate::R<INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTF` writer"]
pub struct W(crate::W<INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTF_SPEC>;
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
impl From<crate::W<INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 3 Capture overflow flag"]
pub use CH0OF_A as CH3OF_A;
#[doc = "Channel 2 Capture overflow flag"]
pub use CH0OF_A as CH2OF_A;
#[doc = "Channel 2 Capture overflow flag"]
pub use CH0OF_A as CH1OF_A;
#[doc = "Field `CH3OF` reader - Channel 3 Capture overflow flag"]
pub use CH0OF_R as CH3OF_R;
#[doc = "Field `CH2OF` reader - Channel 2 Capture overflow flag"]
pub use CH0OF_R as CH2OF_R;
#[doc = "Field `CH1OF` reader - Channel 2 Capture overflow flag"]
pub use CH0OF_R as CH1OF_R;
#[doc = "Field `CH3OF` writer - Channel 3 Capture overflow flag"]
pub use CH0OF_W as CH3OF_W;
#[doc = "Field `CH2OF` writer - Channel 2 Capture overflow flag"]
pub use CH0OF_W as CH2OF_W;
#[doc = "Field `CH1OF` writer - Channel 2 Capture overflow flag"]
pub use CH0OF_W as CH1OF_W;
#[doc = "Channel 0 Capture overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OF_A {
    #[doc = "0: No over capture occurred"]
    CLEAR = 0,
    #[doc = "1: A capture event occured while CHnIF was already set"]
    OVERCAPTURE = 1,
}
impl From<CH0OF_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OF` reader - Channel 0 Capture overflow flag"]
pub type CH0OF_R = crate::BitReader<CH0OF_A>;
impl CH0OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OF_A {
        match self.bits {
            false => CH0OF_A::CLEAR,
            true => CH0OF_A::OVERCAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CH0OF_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `OVERCAPTURE`"]
    #[inline(always)]
    pub fn is_over_capture(&self) -> bool {
        *self == CH0OF_A::OVERCAPTURE
    }
}
#[doc = "Field `CH0OF` writer - Channel 0 Capture overflow flag"]
pub type CH0OF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, CH0OF_A, 9>;
impl<'a> CH0OF_W<'a> {
    #[doc = "No over capture occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH0OF_A::CLEAR)
    }
    #[doc = "A capture event occured while CHnIF was already set"]
    #[inline(always)]
    pub fn over_capture(self) -> &'a mut W {
        self.variant(CH0OF_A::OVERCAPTURE)
    }
}
#[doc = "Break interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKIF_A {
    #[doc = "0: No active level break detected"]
    CLEAR = 0,
    #[doc = "1: Active level detected"]
    BREAK = 1,
}
impl From<BRKIF_A> for bool {
    #[inline(always)]
    fn from(variant: BRKIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKIF` reader - Break interrupt flag"]
pub type BRKIF_R = crate::BitReader<BRKIF_A>;
impl BRKIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKIF_A {
        match self.bits {
            false => BRKIF_A::CLEAR,
            true => BRKIF_A::BREAK,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BRKIF_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `BREAK`"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == BRKIF_A::BREAK
    }
}
#[doc = "Field `BRKIF` writer - Break interrupt flag"]
pub type BRKIF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, BRKIF_A, 7>;
impl<'a> BRKIF_W<'a> {
    #[doc = "No active level break detected"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BRKIF_A::CLEAR)
    }
    #[doc = "Active level detected"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(BRKIF_A::BREAK)
    }
}
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGIF_A {
    #[doc = "0: No trigger event occured"]
    CLEAR = 0,
    #[doc = "1: Trigger event occurred"]
    TRIGGERED = 1,
}
impl From<TRGIF_A> for bool {
    #[inline(always)]
    fn from(variant: TRGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TRGIF_R = crate::BitReader<TRGIF_A>;
impl TRGIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGIF_A {
        match self.bits {
            false => TRGIF_A::CLEAR,
            true => TRGIF_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TRGIF_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TRGIF_A::TRIGGERED
    }
}
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TRGIF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, TRGIF_A, 6>;
impl<'a> TRGIF_W<'a> {
    #[doc = "No trigger event occured"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TRGIF_A::CLEAR)
    }
    #[doc = "Trigger event occurred"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut W {
        self.variant(TRGIF_A::TRIGGERED)
    }
}
#[doc = "Channel commutation interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTIF_A {
    #[doc = "0: No channel commutation event occured"]
    CLEAR = 0,
    #[doc = "1: Channel commutation event occurred"]
    COMMUTATION = 1,
}
impl From<CMTIF_A> for bool {
    #[inline(always)]
    fn from(variant: CMTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTIF` reader - Channel commutation interrupt flag"]
pub type CMTIF_R = crate::BitReader<CMTIF_A>;
impl CMTIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTIF_A {
        match self.bits {
            false => CMTIF_A::CLEAR,
            true => CMTIF_A::COMMUTATION,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMTIF_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `COMMUTATION`"]
    #[inline(always)]
    pub fn is_commutation(&self) -> bool {
        *self == CMTIF_A::COMMUTATION
    }
}
#[doc = "Field `CMTIF` writer - Channel commutation interrupt flag"]
pub type CMTIF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, CMTIF_A, 5>;
impl<'a> CMTIF_W<'a> {
    #[doc = "No channel commutation event occured"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMTIF_A::CLEAR)
    }
    #[doc = "Channel commutation event occurred"]
    #[inline(always)]
    pub fn commutation(self) -> &'a mut W {
        self.variant(CMTIF_A::COMMUTATION)
    }
}
#[doc = "Channel 3 Capture/Compare interrupt enable"]
pub use CH0IF_A as CH3IF_A;
#[doc = "Channel 2 Capture/Compare interrupt enable"]
pub use CH0IF_A as CH2IF_A;
#[doc = "Channel 1s Capture/Compare interrupt enable"]
pub use CH0IF_A as CH1IF_A;
#[doc = "Field `CH3IF` reader - Channel 3 Capture/Compare interrupt enable"]
pub use CH0IF_R as CH3IF_R;
#[doc = "Field `CH2IF` reader - Channel 2 Capture/Compare interrupt enable"]
pub use CH0IF_R as CH2IF_R;
#[doc = "Field `CH1IF` reader - Channel 1s Capture/Compare interrupt enable"]
pub use CH0IF_R as CH1IF_R;
#[doc = "Field `CH3IF` writer - Channel 3 Capture/Compare interrupt enable"]
pub use CH0IF_W as CH3IF_W;
#[doc = "Field `CH2IF` writer - Channel 2 Capture/Compare interrupt enable"]
pub use CH0IF_W as CH2IF_W;
#[doc = "Field `CH1IF` writer - Channel 1s Capture/Compare interrupt enable"]
pub use CH0IF_W as CH1IF_W;
#[doc = "Channel 0s Capture/Compare interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0IF_A {
    #[doc = "0: No capture or compare interrupt occurred"]
    CLEAR = 0,
    #[doc = "1: A capture or compare event occurred"]
    CAPTURECOMPARE = 1,
}
impl From<CH0IF_A> for bool {
    #[inline(always)]
    fn from(variant: CH0IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0IF` reader - Channel 0s Capture/Compare interrupt flag"]
pub type CH0IF_R = crate::BitReader<CH0IF_A>;
impl CH0IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0IF_A {
        match self.bits {
            false => CH0IF_A::CLEAR,
            true => CH0IF_A::CAPTURECOMPARE,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CH0IF_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `CAPTURECOMPARE`"]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == CH0IF_A::CAPTURECOMPARE
    }
}
#[doc = "Field `CH0IF` writer - Channel 0s Capture/Compare interrupt flag"]
pub type CH0IF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, CH0IF_A, 1>;
impl<'a> CH0IF_W<'a> {
    #[doc = "No capture or compare interrupt occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH0IF_A::CLEAR)
    }
    #[doc = "A capture or compare event occurred"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut W {
        self.variant(CH0IF_A::CAPTURECOMPARE)
    }
}
#[doc = "Update interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPIF_A {
    #[doc = "0: No update interrupt occurred"]
    CLEAR = 0,
    #[doc = "1: Update interrupt pending."]
    UPDATEPENDING = 1,
}
impl From<UPIF_A> for bool {
    #[inline(always)]
    fn from(variant: UPIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UPIF_R = crate::BitReader<UPIF_A>;
impl UPIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPIF_A {
        match self.bits {
            false => UPIF_A::CLEAR,
            true => UPIF_A::UPDATEPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UPIF_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `UPDATEPENDING`"]
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UPIF_A::UPDATEPENDING
    }
}
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub type UPIF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, UPIF_A, 0>;
impl<'a> UPIF_W<'a> {
    #[doc = "No update interrupt occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UPIF_A::CLEAR)
    }
    #[doc = "Update interrupt pending."]
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut W {
        self.variant(UPIF_A::UPDATEPENDING)
    }
}
impl R {
    #[doc = "Bit 12 - Channel 3 Capture overflow flag"]
    #[inline(always)]
    pub fn ch3of(&self) -> CH3OF_R {
        CH3OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 Capture overflow flag"]
    #[inline(always)]
    pub fn ch2of(&self) -> CH2OF_R {
        CH2OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Capture overflow flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&self) -> BRKIF_R {
        BRKIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TRGIF_R {
        TRGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&self) -> CMTIF_R {
        CMTIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch3if(&self) -> CH3IF_R {
        CH3IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch2if(&self) -> CH2IF_R {
        CH2IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1s Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch1if(&self) -> CH1IF_R {
        CH1IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0s Capture/Compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> CH0IF_R {
        CH0IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Channel 3 Capture overflow flag"]
    #[inline(always)]
    pub fn ch3of(&mut self) -> CH3OF_W {
        CH3OF_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 Capture overflow flag"]
    #[inline(always)]
    pub fn ch2of(&mut self) -> CH2OF_W {
        CH2OF_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 Capture overflow flag"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> CH1OF_W {
        CH1OF_W::new(self)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&mut self) -> BRKIF_W {
        BRKIF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&mut self) -> TRGIF_W {
        TRGIF_W::new(self)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&mut self) -> CMTIF_W {
        CMTIF_W::new(self)
    }
    #[doc = "Bit 4 - Channel 3 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch3if(&mut self) -> CH3IF_W {
        CH3IF_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch2if(&mut self) -> CH2IF_W {
        CH2IF_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1s Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch1if(&mut self) -> CH1IF_W {
        CH1IF_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0s Capture/Compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&mut self) -> CH0IF_W {
        CH0IF_W::new(self)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&mut self) -> UPIF_W {
        UPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intf::W](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
