#[doc = "Register `RFIFO0` reader"]
pub struct R(crate::R<RFIFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIFO0` writer"]
pub struct W(crate::W<RFIFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIFO0_SPEC>;
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
impl From<crate::W<RFIFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO 0 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFD0_A {
    #[doc = "0: Dequeuing done"]
    FINISHED = 0,
    #[doc = "1: Dequeuing in progress"]
    INPROGRESS = 1,
}
impl From<RFD0_A> for bool {
    #[inline(always)]
    fn from(variant: RFD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD0` reader - Receive FIFO 0 dequeue"]
pub type RFD0_R = crate::BitReader<RFD0_A>;
impl RFD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFD0_A {
        match self.bits {
            false => RFD0_A::FINISHED,
            true => RFD0_A::INPROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == RFD0_A::FINISHED
    }
    #[doc = "Checks if the value of the field is `INPROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RFD0_A::INPROGRESS
    }
}
#[doc = "Receive FIFO 0 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFD0_AW {
    #[doc = "1: Start dequeuing"]
    START = 1,
}
impl From<RFD0_AW> for bool {
    #[inline(always)]
    fn from(variant: RFD0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD0` writer - Receive FIFO 0 dequeue"]
pub type RFD0_W<'a> = crate::BitWriter<'a, u32, RFIFO0_SPEC, RFD0_AW, 5>;
impl<'a> RFD0_W<'a> {
    #[doc = "Start dequeuing"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RFD0_AW::START)
    }
}
#[doc = "Receive FIFO 0 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFO0_A {
    #[doc = "0: Receive FIFO is not overfull"]
    NOTOVERFULL = 0,
    #[doc = "1: Receive FIFO is overfull"]
    OVERFULL = 1,
}
impl From<RFO0_A> for bool {
    #[inline(always)]
    fn from(variant: RFO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO0` reader - Receive FIFO 0 overfull"]
pub type RFO0_R = crate::BitReader<RFO0_A>;
impl RFO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFO0_A {
        match self.bits {
            false => RFO0_A::NOTOVERFULL,
            true => RFO0_A::OVERFULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTOVERFULL`"]
    #[inline(always)]
    pub fn is_not_overfull(&self) -> bool {
        *self == RFO0_A::NOTOVERFULL
    }
    #[doc = "Checks if the value of the field is `OVERFULL`"]
    #[inline(always)]
    pub fn is_overfull(&self) -> bool {
        *self == RFO0_A::OVERFULL
    }
}
#[doc = "Receive FIFO 0 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFO0_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<RFO0_AW> for bool {
    #[inline(always)]
    fn from(variant: RFO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO0` writer - Receive FIFO 0 overfull"]
pub type RFO0_W<'a> = crate::BitWriter<'a, u32, RFIFO0_SPEC, RFO0_AW, 4>;
impl<'a> RFO0_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RFO0_AW::CLEAR)
    }
}
#[doc = "Receive FIFO 0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFF0_A {
    #[doc = "0: Receive FIFO is not full"]
    NOTFULL = 0,
    #[doc = "1: Receive FIFO is full"]
    FULL = 1,
}
impl From<RFF0_A> for bool {
    #[inline(always)]
    fn from(variant: RFF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF0` reader - Receive FIFO 0 full"]
pub type RFF0_R = crate::BitReader<RFF0_A>;
impl RFF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF0_A {
        match self.bits {
            false => RFF0_A::NOTFULL,
            true => RFF0_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RFF0_A::NOTFULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RFF0_A::FULL
    }
}
#[doc = "Receive FIFO 0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFF0_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<RFF0_AW> for bool {
    #[inline(always)]
    fn from(variant: RFF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF0` writer - Receive FIFO 0 full"]
pub type RFF0_W<'a> = crate::BitWriter<'a, u32, RFIFO0_SPEC, RFF0_AW, 3>;
impl<'a> RFF0_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RFF0_AW::CLEAR)
    }
}
#[doc = "Field `RFL0` reader - Receive FIFO 0 length"]
pub type RFL0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 5 - Receive FIFO 0 dequeue"]
    #[inline(always)]
    pub fn rfd0(&self) -> RFD0_R {
        RFD0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO 0 overfull"]
    #[inline(always)]
    pub fn rfo0(&self) -> RFO0_R {
        RFO0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO 0 full"]
    #[inline(always)]
    pub fn rff0(&self) -> RFF0_R {
        RFF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Receive FIFO 0 length"]
    #[inline(always)]
    pub fn rfl0(&self) -> RFL0_R {
        RFL0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Receive FIFO 0 dequeue"]
    #[inline(always)]
    pub fn rfd0(&mut self) -> RFD0_W {
        RFD0_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO 0 overfull"]
    #[inline(always)]
    pub fn rfo0(&mut self) -> RFO0_W {
        RFO0_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO 0 full"]
    #[inline(always)]
    pub fn rff0(&mut self) -> RFF0_W {
        RFF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive message FIFO0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo0](index.html) module"]
pub struct RFIFO0_SPEC;
impl crate::RegisterSpec for RFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifo0::R](R) reader structure"]
impl crate::Readable for RFIFO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfifo0::W](W) writer structure"]
impl crate::Writable for RFIFO0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIFO0 to value 0"]
impl crate::Resettable for RFIFO0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
