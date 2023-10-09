#[doc = "Register `RFIFO0` reader"]
pub type R = crate::R<RFIFO0_SPEC>;
#[doc = "Register `RFIFO0` writer"]
pub type W = crate::W<RFIFO0_SPEC>;
#[doc = "Field `RFL0` reader - Receive FIFO0 length"]
pub type RFL0_R = crate::FieldReader;
#[doc = "Field `RFF0` reader - Receive FIFO0 full"]
pub type RFF0_R = crate::BitReader<RFF0R_A>;
#[doc = "Receive FIFO0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFF0R_A {
    #[doc = "0: Receive FIFO is not full"]
    NOT_FULL = 0,
    #[doc = "1: Receive FIFO is full"]
    FULL = 1,
}
impl From<RFF0R_A> for bool {
    #[inline(always)]
    fn from(variant: RFF0R_A) -> Self {
        variant as u8 != 0
    }
}
impl RFF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF0R_A {
        match self.bits {
            false => RFF0R_A::NOT_FULL,
            true => RFF0R_A::FULL,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RFF0R_A::NOT_FULL
    }
    #[doc = "Receive FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RFF0R_A::FULL
    }
}
#[doc = "Receive FIFO0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFF0W_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<RFF0W_AW> for bool {
    #[inline(always)]
    fn from(variant: RFF0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF0` writer - Receive FIFO0 full"]
pub type RFF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFF0W_AW>;
impl<'a, REG, const O: u8> RFF0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RFF0W_AW::CLEAR)
    }
}
#[doc = "Field `RFO0` reader - Receive FIFO0 overfull"]
pub type RFO0_R = crate::BitReader<RFO0R_A>;
#[doc = "Receive FIFO0 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFO0R_A {
    #[doc = "0: Receive FIFO is not overfull"]
    NOT_OVERFULL = 0,
    #[doc = "1: Receive FIFO is overfull"]
    OVERFULL = 1,
}
impl From<RFO0R_A> for bool {
    #[inline(always)]
    fn from(variant: RFO0R_A) -> Self {
        variant as u8 != 0
    }
}
impl RFO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFO0R_A {
        match self.bits {
            false => RFO0R_A::NOT_OVERFULL,
            true => RFO0R_A::OVERFULL,
        }
    }
    #[doc = "Receive FIFO is not overfull"]
    #[inline(always)]
    pub fn is_not_overfull(&self) -> bool {
        *self == RFO0R_A::NOT_OVERFULL
    }
    #[doc = "Receive FIFO is overfull"]
    #[inline(always)]
    pub fn is_overfull(&self) -> bool {
        *self == RFO0R_A::OVERFULL
    }
}
#[doc = "Receive FIFO0 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFO0W_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<RFO0W_AW> for bool {
    #[inline(always)]
    fn from(variant: RFO0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO0` writer - Receive FIFO0 overfull"]
pub type RFO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFO0W_AW>;
impl<'a, REG, const O: u8> RFO0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RFO0W_AW::CLEAR)
    }
}
#[doc = "Field `RFD0` reader - Receive FIFO0 dequeue"]
pub type RFD0_R = crate::BitReader<RFD0R_A>;
#[doc = "Receive FIFO0 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFD0R_A {
    #[doc = "0: Dequeuing done"]
    FINISHED = 0,
    #[doc = "1: Dequeuing in progress"]
    IN_PROGRESS = 1,
}
impl From<RFD0R_A> for bool {
    #[inline(always)]
    fn from(variant: RFD0R_A) -> Self {
        variant as u8 != 0
    }
}
impl RFD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFD0R_A {
        match self.bits {
            false => RFD0R_A::FINISHED,
            true => RFD0R_A::IN_PROGRESS,
        }
    }
    #[doc = "Dequeuing done"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == RFD0R_A::FINISHED
    }
    #[doc = "Dequeuing in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RFD0R_A::IN_PROGRESS
    }
}
#[doc = "Receive FIFO0 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFD0W_AW {
    #[doc = "1: Start dequeuing"]
    START = 1,
}
impl From<RFD0W_AW> for bool {
    #[inline(always)]
    fn from(variant: RFD0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD0` writer - Receive FIFO0 dequeue"]
pub type RFD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFD0W_AW>;
impl<'a, REG, const O: u8> RFD0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start dequeuing"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RFD0W_AW::START)
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive FIFO0 length"]
    #[inline(always)]
    pub fn rfl0(&self) -> RFL0_R {
        RFL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO0 full"]
    #[inline(always)]
    pub fn rff0(&self) -> RFF0_R {
        RFF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    pub fn rfo0(&self) -> RFO0_R {
        RFO0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    pub fn rfd0(&self) -> RFD0_R {
        RFD0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO0 full"]
    #[inline(always)]
    #[must_use]
    pub fn rff0(&mut self) -> RFF0_W<RFIFO0_SPEC, 3> {
        RFF0_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    #[must_use]
    pub fn rfo0(&mut self) -> RFO0_W<RFIFO0_SPEC, 4> {
        RFO0_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    #[must_use]
    pub fn rfd0(&mut self) -> RFD0_W<RFIFO0_SPEC, 5> {
        RFD0_W::new(self)
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
#[doc = "Receive message FIFO0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFO0_SPEC;
impl crate::RegisterSpec for RFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo0::R`](R) reader structure"]
impl crate::Readable for RFIFO0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfifo0::W`](W) writer structure"]
impl crate::Writable for RFIFO0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIFO0 to value 0"]
impl crate::Resettable for RFIFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
