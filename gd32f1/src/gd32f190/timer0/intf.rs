#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UPIF_R = crate::BitReader<UPIF_A>;
#[doc = "Update interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPIF_A {
    #[doc = "0: No update interrupt occurred"]
    CLEAR = 0,
    #[doc = "1: Update interrupt pending."]
    UPDATE_PENDING = 1,
}
impl From<UPIF_A> for bool {
    #[inline(always)]
    fn from(variant: UPIF_A) -> Self {
        variant as u8 != 0
    }
}
impl UPIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPIF_A {
        match self.bits {
            false => UPIF_A::CLEAR,
            true => UPIF_A::UPDATE_PENDING,
        }
    }
    #[doc = "No update interrupt occurred"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UPIF_A::CLEAR
    }
    #[doc = "Update interrupt pending."]
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UPIF_A::UPDATE_PENDING
    }
}
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub type UPIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UPIF_A>;
impl<'a, REG, const O: u8> UPIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No update interrupt occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UPIF_A::CLEAR)
    }
    #[doc = "Update interrupt pending."]
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut crate::W<REG> {
        self.variant(UPIF_A::UPDATE_PENDING)
    }
}
#[doc = "Field `CH0IF` reader - Channel 0s Capture/Compare interrupt flag"]
pub type CH0IF_R = crate::BitReader<CH0IF_A>;
#[doc = "Channel 0s Capture/Compare interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0IF_A {
    #[doc = "0: No capture or compare interrupt occurred"]
    CLEAR = 0,
    #[doc = "1: A capture or compare event occurred"]
    CAPTURE_COMPARE = 1,
}
impl From<CH0IF_A> for bool {
    #[inline(always)]
    fn from(variant: CH0IF_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0IF_A {
        match self.bits {
            false => CH0IF_A::CLEAR,
            true => CH0IF_A::CAPTURE_COMPARE,
        }
    }
    #[doc = "No capture or compare interrupt occurred"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CH0IF_A::CLEAR
    }
    #[doc = "A capture or compare event occurred"]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == CH0IF_A::CAPTURE_COMPARE
    }
}
#[doc = "Field `CH0IF` writer - Channel 0s Capture/Compare interrupt flag"]
pub type CH0IF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0IF_A>;
impl<'a, REG, const O: u8> CH0IF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No capture or compare interrupt occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CH0IF_A::CLEAR)
    }
    #[doc = "A capture or compare event occurred"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut crate::W<REG> {
        self.variant(CH0IF_A::CAPTURE_COMPARE)
    }
}
#[doc = "Field `CH1IF` reader - Channel 1s Capture/Compare interrupt enable"]
pub use CH0IF_R as CH1IF_R;
#[doc = "Field `CH2IF` reader - Channel 2 Capture/Compare interrupt enable"]
pub use CH0IF_R as CH2IF_R;
#[doc = "Field `CH3IF` reader - Channel 3 Capture/Compare interrupt enable"]
pub use CH0IF_R as CH3IF_R;
#[doc = "Field `CH1IF` writer - Channel 1s Capture/Compare interrupt enable"]
pub use CH0IF_W as CH1IF_W;
#[doc = "Field `CH2IF` writer - Channel 2 Capture/Compare interrupt enable"]
pub use CH0IF_W as CH2IF_W;
#[doc = "Field `CH3IF` writer - Channel 3 Capture/Compare interrupt enable"]
pub use CH0IF_W as CH3IF_W;
#[doc = "Field `CMTIF` reader - Channel commutation interrupt flag"]
pub type CMTIF_R = crate::BitReader<CMTIF_A>;
#[doc = "Channel commutation interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CMTIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTIF_A {
        match self.bits {
            false => CMTIF_A::CLEAR,
            true => CMTIF_A::COMMUTATION,
        }
    }
    #[doc = "No channel commutation event occured"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMTIF_A::CLEAR
    }
    #[doc = "Channel commutation event occurred"]
    #[inline(always)]
    pub fn is_commutation(&self) -> bool {
        *self == CMTIF_A::COMMUTATION
    }
}
#[doc = "Field `CMTIF` writer - Channel commutation interrupt flag"]
pub type CMTIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMTIF_A>;
impl<'a, REG, const O: u8> CMTIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel commutation event occured"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMTIF_A::CLEAR)
    }
    #[doc = "Channel commutation event occurred"]
    #[inline(always)]
    pub fn commutation(self) -> &'a mut crate::W<REG> {
        self.variant(CMTIF_A::COMMUTATION)
    }
}
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TRGIF_R = crate::BitReader<TRGIF_A>;
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TRGIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGIF_A {
        match self.bits {
            false => TRGIF_A::CLEAR,
            true => TRGIF_A::TRIGGERED,
        }
    }
    #[doc = "No trigger event occured"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TRGIF_A::CLEAR
    }
    #[doc = "Trigger event occurred"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TRGIF_A::TRIGGERED
    }
}
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TRGIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRGIF_A>;
impl<'a, REG, const O: u8> TRGIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger event occured"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TRGIF_A::CLEAR)
    }
    #[doc = "Trigger event occurred"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(TRGIF_A::TRIGGERED)
    }
}
#[doc = "Field `BRKIF` reader - Break interrupt flag"]
pub type BRKIF_R = crate::BitReader<BRKIF_A>;
#[doc = "Break interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BRKIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRKIF_A {
        match self.bits {
            false => BRKIF_A::CLEAR,
            true => BRKIF_A::BREAK,
        }
    }
    #[doc = "No active level break detected"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BRKIF_A::CLEAR
    }
    #[doc = "Active level detected"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == BRKIF_A::BREAK
    }
}
#[doc = "Field `BRKIF` writer - Break interrupt flag"]
pub type BRKIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRKIF_A>;
impl<'a, REG, const O: u8> BRKIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active level break detected"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BRKIF_A::CLEAR)
    }
    #[doc = "Active level detected"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(BRKIF_A::BREAK)
    }
}
#[doc = "Field `CH0OF` reader - Channel 0 Capture overflow flag"]
pub type CH0OF_R = crate::BitReader<CH0OF_A>;
#[doc = "Channel 0 Capture overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0OF_A {
    #[doc = "0: No over capture occurred"]
    CLEAR = 0,
    #[doc = "1: A capture event occured while CHnIF was already set"]
    OVER_CAPTURE = 1,
}
impl From<CH0OF_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OF_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OF_A {
        match self.bits {
            false => CH0OF_A::CLEAR,
            true => CH0OF_A::OVER_CAPTURE,
        }
    }
    #[doc = "No over capture occurred"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CH0OF_A::CLEAR
    }
    #[doc = "A capture event occured while CHnIF was already set"]
    #[inline(always)]
    pub fn is_over_capture(&self) -> bool {
        *self == CH0OF_A::OVER_CAPTURE
    }
}
#[doc = "Field `CH0OF` writer - Channel 0 Capture overflow flag"]
pub type CH0OF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0OF_A>;
impl<'a, REG, const O: u8> CH0OF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No over capture occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CH0OF_A::CLEAR)
    }
    #[doc = "A capture event occured while CHnIF was already set"]
    #[inline(always)]
    pub fn over_capture(self) -> &'a mut crate::W<REG> {
        self.variant(CH0OF_A::OVER_CAPTURE)
    }
}
#[doc = "Field `CH1OF` reader - Channel 2 Capture overflow flag"]
pub use CH0OF_R as CH1OF_R;
#[doc = "Field `CH2OF` reader - Channel 2 Capture overflow flag"]
pub use CH0OF_R as CH2OF_R;
#[doc = "Field `CH3OF` reader - Channel 3 Capture overflow flag"]
pub use CH0OF_R as CH3OF_R;
#[doc = "Field `CH1OF` writer - Channel 2 Capture overflow flag"]
pub use CH0OF_W as CH1OF_W;
#[doc = "Field `CH2OF` writer - Channel 2 Capture overflow flag"]
pub use CH0OF_W as CH2OF_W;
#[doc = "Field `CH3OF` writer - Channel 3 Capture overflow flag"]
pub use CH0OF_W as CH3OF_W;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0s Capture/Compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> CH0IF_R {
        CH0IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1s Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch1if(&self) -> CH1IF_R {
        CH1IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch2if(&self) -> CH2IF_R {
        CH2IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 Capture/Compare interrupt enable"]
    #[inline(always)]
    pub fn ch3if(&self) -> CH3IF_R {
        CH3IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&self) -> CMTIF_R {
        CMTIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TRGIF_R {
        TRGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&self) -> BRKIF_R {
        BRKIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Capture overflow flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 Capture overflow flag"]
    #[inline(always)]
    pub fn ch2of(&self) -> CH2OF_R {
        CH2OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 Capture overflow flag"]
    #[inline(always)]
    pub fn ch3of(&self) -> CH3OF_R {
        CH3OF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn upif(&mut self) -> UPIF_W<INTF_SPEC, 0> {
        UPIF_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0s Capture/Compare interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0if(&mut self) -> CH0IF_W<INTF_SPEC, 1> {
        CH0IF_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1s Capture/Compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1if(&mut self) -> CH1IF_W<INTF_SPEC, 2> {
        CH1IF_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 Capture/Compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2if(&mut self) -> CH2IF_W<INTF_SPEC, 3> {
        CH2IF_W::new(self)
    }
    #[doc = "Bit 4 - Channel 3 Capture/Compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3if(&mut self) -> CH3IF_W<INTF_SPEC, 4> {
        CH3IF_W::new(self)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmtif(&mut self) -> CMTIF_W<INTF_SPEC, 5> {
        CMTIF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgif(&mut self) -> TRGIF_W<INTF_SPEC, 6> {
        TRGIF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn brkif(&mut self) -> BRKIF_W<INTF_SPEC, 7> {
        BRKIF_W::new(self)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<INTF_SPEC, 9> {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 Capture overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> CH1OF_W<INTF_SPEC, 10> {
        CH1OF_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 Capture overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2of(&mut self) -> CH2OF_W<INTF_SPEC, 11> {
        CH2OF_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 Capture overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3of(&mut self) -> CH3OF_W<INTF_SPEC, 12> {
        CH3OF_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
