#[doc = "Register `RFIFO1` reader"]
pub type R = crate::R<RFIFO1_SPEC>;
#[doc = "Register `RFIFO1` writer"]
pub type W = crate::W<RFIFO1_SPEC>;
#[doc = "Field `RFL1` reader - Receive FIFO1 length"]
pub type RFL1_R = crate::FieldReader;
#[doc = "Field `RFF1` reader - Receive FIFO1 full"]
pub type RFF1_R = crate::BitReader<RFF1R_A>;
#[doc = "Receive FIFO1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFF1R_A {
    #[doc = "0: Receive FIFO is not full"]
    NOT_FULL = 0,
    #[doc = "1: Receive FIFO is full"]
    FULL = 1,
}
impl From<RFF1R_A> for bool {
    #[inline(always)]
    fn from(variant: RFF1R_A) -> Self {
        variant as u8 != 0
    }
}
impl RFF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF1R_A {
        match self.bits {
            false => RFF1R_A::NOT_FULL,
            true => RFF1R_A::FULL,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RFF1R_A::NOT_FULL
    }
    #[doc = "Receive FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RFF1R_A::FULL
    }
}
#[doc = "Receive FIFO1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFF1W_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<RFF1W_AW> for bool {
    #[inline(always)]
    fn from(variant: RFF1W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF1` writer - Receive FIFO1 full"]
pub type RFF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFF1W_AW>;
impl<'a, REG, const O: u8> RFF1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RFF1W_AW::CLEAR)
    }
}
#[doc = "Field `RFO1` reader - Receive FIFO1 overfull"]
pub type RFO1_R = crate::BitReader<RFO1R_A>;
#[doc = "Receive FIFO1 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFO1R_A {
    #[doc = "0: Receive FIFO is not overfull"]
    NOT_OVERFULL = 0,
    #[doc = "1: Receive FIFO is overfull"]
    OVERFULL = 1,
}
impl From<RFO1R_A> for bool {
    #[inline(always)]
    fn from(variant: RFO1R_A) -> Self {
        variant as u8 != 0
    }
}
impl RFO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFO1R_A {
        match self.bits {
            false => RFO1R_A::NOT_OVERFULL,
            true => RFO1R_A::OVERFULL,
        }
    }
    #[doc = "Receive FIFO is not overfull"]
    #[inline(always)]
    pub fn is_not_overfull(&self) -> bool {
        *self == RFO1R_A::NOT_OVERFULL
    }
    #[doc = "Receive FIFO is overfull"]
    #[inline(always)]
    pub fn is_overfull(&self) -> bool {
        *self == RFO1R_A::OVERFULL
    }
}
#[doc = "Receive FIFO1 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFO1W_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<RFO1W_AW> for bool {
    #[inline(always)]
    fn from(variant: RFO1W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO1` writer - Receive FIFO1 overfull"]
pub type RFO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFO1W_AW>;
impl<'a, REG, const O: u8> RFO1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RFO1W_AW::CLEAR)
    }
}
#[doc = "Field `RFD1` reader - Receive FIFO1 dequeue"]
pub type RFD1_R = crate::BitReader<RFD1R_A>;
#[doc = "Receive FIFO1 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFD1R_A {
    #[doc = "0: Dequeuing done"]
    FINISHED = 0,
    #[doc = "1: Dequeuing in progress"]
    IN_PROGRESS = 1,
}
impl From<RFD1R_A> for bool {
    #[inline(always)]
    fn from(variant: RFD1R_A) -> Self {
        variant as u8 != 0
    }
}
impl RFD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFD1R_A {
        match self.bits {
            false => RFD1R_A::FINISHED,
            true => RFD1R_A::IN_PROGRESS,
        }
    }
    #[doc = "Dequeuing done"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == RFD1R_A::FINISHED
    }
    #[doc = "Dequeuing in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RFD1R_A::IN_PROGRESS
    }
}
#[doc = "Receive FIFO1 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFD1W_AW {
    #[doc = "1: Start dequeuing"]
    START = 1,
}
impl From<RFD1W_AW> for bool {
    #[inline(always)]
    fn from(variant: RFD1W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD1` writer - Receive FIFO1 dequeue"]
pub type RFD1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFD1W_AW>;
impl<'a, REG, const O: u8> RFD1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start dequeuing"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RFD1W_AW::START)
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive FIFO1 length"]
    #[inline(always)]
    pub fn rfl1(&self) -> RFL1_R {
        RFL1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    pub fn rff1(&self) -> RFF1_R {
        RFF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    pub fn rfo1(&self) -> RFO1_R {
        RFO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    pub fn rfd1(&self) -> RFD1_R {
        RFD1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    #[must_use]
    pub fn rff1(&mut self) -> RFF1_W<RFIFO1_SPEC, 3> {
        RFF1_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    #[must_use]
    pub fn rfo1(&mut self) -> RFO1_W<RFIFO1_SPEC, 4> {
        RFO1_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    #[must_use]
    pub fn rfd1(&mut self) -> RFD1_W<RFIFO1_SPEC, 5> {
        RFD1_W::new(self)
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
#[doc = "Receive message FIFO1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFO1_SPEC;
impl crate::RegisterSpec for RFIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo1::R`](R) reader structure"]
impl crate::Readable for RFIFO1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfifo1::W`](W) writer structure"]
impl crate::Writable for RFIFO1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIFO1 to value 0"]
impl crate::Resettable for RFIFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
