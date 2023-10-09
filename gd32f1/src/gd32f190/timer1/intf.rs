#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Channel 0 Capture/Compare interrupt flag"]
pub use crate::gd32f190::timer0::intf::CH0IF_A;
#[doc = "Field `CH0IF` reader - Channel 0 Capture/Compare interrupt flag"]
pub use crate::gd32f190::timer0::intf::CH0IF_R;
#[doc = "Field `CH1IF` reader - Channel 1 Capture/Compare interrupt enable"]
pub use crate::gd32f190::timer0::intf::CH0IF_R as CH1IF_R;
#[doc = "Field `CH2IF` reader - Channel 2 Capture/Compare interrupt enable"]
pub use crate::gd32f190::timer0::intf::CH0IF_R as CH2IF_R;
#[doc = "Field `CH3IF` reader - Channel 3 Capture/Compare interrupt enable"]
pub use crate::gd32f190::timer0::intf::CH0IF_R as CH3IF_R;
#[doc = "Field `CH0IF` writer - Channel 0 Capture/Compare interrupt flag"]
pub use crate::gd32f190::timer0::intf::CH0IF_W;
#[doc = "Field `CH1IF` writer - Channel 1 Capture/Compare interrupt enable"]
pub use crate::gd32f190::timer0::intf::CH0IF_W as CH1IF_W;
#[doc = "Field `CH2IF` writer - Channel 2 Capture/Compare interrupt enable"]
pub use crate::gd32f190::timer0::intf::CH0IF_W as CH2IF_W;
#[doc = "Field `CH3IF` writer - Channel 3 Capture/Compare interrupt enable"]
pub use crate::gd32f190::timer0::intf::CH0IF_W as CH3IF_W;
#[doc = "Update interrupt flag"]
pub use crate::gd32f190::timer0::intf::UPIF_A;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub use crate::gd32f190::timer0::intf::UPIF_R;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub use crate::gd32f190::timer0::intf::UPIF_W;
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
#[doc = "Channel 0 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_A;
#[doc = "Field `CH0OF` reader - Channel 0 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_R;
#[doc = "Field `CH1OF` reader - Channel 1 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_R as CH1OF_R;
#[doc = "Field `CH2OF` reader - Channel 2 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_R as CH2OF_R;
#[doc = "Field `CH3OF` reader - Channel 3 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_R as CH3OF_R;
#[doc = "Field `CH0OF` writer - Channel 0 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_W;
#[doc = "Field `CH1OF` writer - Channel 1 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_W as CH1OF_W;
#[doc = "Field `CH2OF` writer - Channel 2 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_W as CH2OF_W;
#[doc = "Field `CH3OF` writer - Channel 3 Capture overflow flag"]
pub use crate::gd32f190::timer0::intf::CH0OF_W as CH3OF_W;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Capture/Compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> CH0IF_R {
        CH0IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Capture/Compare interrupt enable"]
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
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TRGIF_R {
        TRGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 Capture overflow flag"]
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
    #[doc = "Bit 1 - Channel 0 Capture/Compare interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0if(&mut self) -> CH0IF_W<INTF_SPEC, 1> {
        CH0IF_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 Capture/Compare interrupt enable"]
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
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgif(&mut self) -> TRGIF_W<INTF_SPEC, 6> {
        TRGIF_W::new(self)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<INTF_SPEC, 9> {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 10 - Channel 1 Capture overflow flag"]
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
#[doc = "interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
