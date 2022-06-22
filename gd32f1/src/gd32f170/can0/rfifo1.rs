#[doc = "Register `RFIFO1` reader"]
pub struct R(crate::R<RFIFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIFO1` writer"]
pub struct W(crate::W<RFIFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIFO1_SPEC>;
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
impl From<crate::W<RFIFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO1 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFD1_A {
    #[doc = "0: Dequeuing done"]
    FINISHED = 0,
    #[doc = "1: Dequeuing in progress"]
    INPROGRESS = 1,
}
impl From<RFD1_A> for bool {
    #[inline(always)]
    fn from(variant: RFD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD1` reader - Receive FIFO1 dequeue"]
pub type RFD1_R = crate::BitReader<RFD1_A>;
impl RFD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFD1_A {
        match self.bits {
            false => RFD1_A::FINISHED,
            true => RFD1_A::INPROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == RFD1_A::FINISHED
    }
    #[doc = "Checks if the value of the field is `INPROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RFD1_A::INPROGRESS
    }
}
#[doc = "Receive FIFO1 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFD1_AW {
    #[doc = "1: Start dequeuing"]
    START = 1,
}
impl From<RFD1_AW> for bool {
    #[inline(always)]
    fn from(variant: RFD1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD1` writer - Receive FIFO1 dequeue"]
pub type RFD1_W<'a> = crate::BitWriter<'a, u32, RFIFO1_SPEC, RFD1_AW, 5>;
impl<'a> RFD1_W<'a> {
    #[doc = "Start dequeuing"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RFD1_AW::START)
    }
}
#[doc = "Receive FIFO1 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFO1_A {
    #[doc = "0: Receive FIFO is not overfull"]
    NOTOVERFULL = 0,
    #[doc = "1: Receive FIFO is overfull"]
    OVERFULL = 1,
}
impl From<RFO1_A> for bool {
    #[inline(always)]
    fn from(variant: RFO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO1` reader - Receive FIFO1 overfull"]
pub type RFO1_R = crate::BitReader<RFO1_A>;
impl RFO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFO1_A {
        match self.bits {
            false => RFO1_A::NOTOVERFULL,
            true => RFO1_A::OVERFULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTOVERFULL`"]
    #[inline(always)]
    pub fn is_not_overfull(&self) -> bool {
        *self == RFO1_A::NOTOVERFULL
    }
    #[doc = "Checks if the value of the field is `OVERFULL`"]
    #[inline(always)]
    pub fn is_overfull(&self) -> bool {
        *self == RFO1_A::OVERFULL
    }
}
#[doc = "Receive FIFO1 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFO1_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<RFO1_AW> for bool {
    #[inline(always)]
    fn from(variant: RFO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO1` writer - Receive FIFO1 overfull"]
pub type RFO1_W<'a> = crate::BitWriter<'a, u32, RFIFO1_SPEC, RFO1_AW, 4>;
impl<'a> RFO1_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RFO1_AW::CLEAR)
    }
}
#[doc = "Receive FIFO1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFF1_A {
    #[doc = "0: Receive FIFO is not full"]
    NOTFULL = 0,
    #[doc = "1: Receive FIFO is full"]
    FULL = 1,
}
impl From<RFF1_A> for bool {
    #[inline(always)]
    fn from(variant: RFF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF1` reader - Receive FIFO1 full"]
pub type RFF1_R = crate::BitReader<RFF1_A>;
impl RFF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF1_A {
        match self.bits {
            false => RFF1_A::NOTFULL,
            true => RFF1_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RFF1_A::NOTFULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RFF1_A::FULL
    }
}
#[doc = "Receive FIFO1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFF1_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<RFF1_AW> for bool {
    #[inline(always)]
    fn from(variant: RFF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF1` writer - Receive FIFO1 full"]
pub type RFF1_W<'a> = crate::BitWriter<'a, u32, RFIFO1_SPEC, RFF1_AW, 3>;
impl<'a> RFF1_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RFF1_AW::CLEAR)
    }
}
#[doc = "Field `RFL1` reader - Receive FIFO1 length"]
pub type RFL1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    pub fn rfd1(&self) -> RFD1_R {
        RFD1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    pub fn rfo1(&self) -> RFO1_R {
        RFO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    pub fn rff1(&self) -> RFF1_R {
        RFF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Receive FIFO1 length"]
    #[inline(always)]
    pub fn rfl1(&self) -> RFL1_R {
        RFL1_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    pub fn rfd1(&mut self) -> RFD1_W {
        RFD1_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    pub fn rfo1(&mut self) -> RFO1_W {
        RFO1_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    pub fn rff1(&mut self) -> RFF1_W {
        RFF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive message FIFO1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo1](index.html) module"]
pub struct RFIFO1_SPEC;
impl crate::RegisterSpec for RFIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifo1::R](R) reader structure"]
impl crate::Readable for RFIFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfifo1::W](W) writer structure"]
impl crate::Writable for RFIFO1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIFO1 to value 0"]
impl crate::Resettable for RFIFO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
