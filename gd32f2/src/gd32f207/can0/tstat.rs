#[doc = "Register `TSTAT` reader"]
pub type R = crate::R<TSTAT_SPEC>;
#[doc = "Register `TSTAT` writer"]
pub type W = crate::W<TSTAT_SPEC>;
#[doc = "Field `MTF0` reader - Mailbox 0 transmit finished"]
pub type MTF0_R = crate::BitReader<MTF0R_A>;
#[doc = "Mailbox 0 transmit finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTF0R_A {
    #[doc = "0: Mailbox transmission still in progress"]
    IN_PROGRESS = 0,
    #[doc = "1: Mailbox transmission finished"]
    FINISHED = 1,
}
impl From<MTF0R_A> for bool {
    #[inline(always)]
    fn from(variant: MTF0R_A) -> Self {
        variant as u8 != 0
    }
}
impl MTF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTF0R_A {
        match self.bits {
            false => MTF0R_A::IN_PROGRESS,
            true => MTF0R_A::FINISHED,
        }
    }
    #[doc = "Mailbox transmission still in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == MTF0R_A::IN_PROGRESS
    }
    #[doc = "Mailbox transmission finished"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == MTF0R_A::FINISHED
    }
}
#[doc = "Mailbox 0 transmit finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTF0W_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<MTF0W_AW> for bool {
    #[inline(always)]
    fn from(variant: MTF0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTF0` writer - Mailbox 0 transmit finished"]
pub type MTF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTF0W_AW>;
impl<'a, REG, const O: u8> MTF0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MTF0W_AW::CLEAR)
    }
}
#[doc = "Field `MTFNERR0` reader - Mailbox 0 transmit finished and no error"]
pub type MTFNERR0_R = crate::BitReader<MTFNERR0R_A>;
#[doc = "Mailbox 0 transmit finished and no error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTFNERR0R_A {
    #[doc = "0: Mailbox transmission finished with an error"]
    FINISHED_WITH_ERROR = 0,
    #[doc = "1: Mailbox transmission finished with no error"]
    FINISHED_NO_ERROR = 1,
}
impl From<MTFNERR0R_A> for bool {
    #[inline(always)]
    fn from(variant: MTFNERR0R_A) -> Self {
        variant as u8 != 0
    }
}
impl MTFNERR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTFNERR0R_A {
        match self.bits {
            false => MTFNERR0R_A::FINISHED_WITH_ERROR,
            true => MTFNERR0R_A::FINISHED_NO_ERROR,
        }
    }
    #[doc = "Mailbox transmission finished with an error"]
    #[inline(always)]
    pub fn is_finished_with_error(&self) -> bool {
        *self == MTFNERR0R_A::FINISHED_WITH_ERROR
    }
    #[doc = "Mailbox transmission finished with no error"]
    #[inline(always)]
    pub fn is_finished_no_error(&self) -> bool {
        *self == MTFNERR0R_A::FINISHED_NO_ERROR
    }
}
#[doc = "Mailbox 0 transmit finished and no error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTFNERR0W_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<MTFNERR0W_AW> for bool {
    #[inline(always)]
    fn from(variant: MTFNERR0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTFNERR0` writer - Mailbox 0 transmit finished and no error"]
pub type MTFNERR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTFNERR0W_AW>;
impl<'a, REG, const O: u8> MTFNERR0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MTFNERR0W_AW::CLEAR)
    }
}
#[doc = "Field `MAL0` reader - Mailbox 0 arbitration lost"]
pub type MAL0_R = crate::BitReader<MAL0R_A>;
#[doc = "Mailbox 0 arbitration lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAL0R_A {
    #[doc = "0: Arbitration was not lost"]
    NO_ARBITRATION_LOST = 0,
    #[doc = "1: Arbitration lost"]
    ARBITRATION_LOST = 1,
}
impl From<MAL0R_A> for bool {
    #[inline(always)]
    fn from(variant: MAL0R_A) -> Self {
        variant as u8 != 0
    }
}
impl MAL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAL0R_A {
        match self.bits {
            false => MAL0R_A::NO_ARBITRATION_LOST,
            true => MAL0R_A::ARBITRATION_LOST,
        }
    }
    #[doc = "Arbitration was not lost"]
    #[inline(always)]
    pub fn is_no_arbitration_lost(&self) -> bool {
        *self == MAL0R_A::NO_ARBITRATION_LOST
    }
    #[doc = "Arbitration lost"]
    #[inline(always)]
    pub fn is_arbitration_lost(&self) -> bool {
        *self == MAL0R_A::ARBITRATION_LOST
    }
}
#[doc = "Mailbox 0 arbitration lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAL0W_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<MAL0W_AW> for bool {
    #[inline(always)]
    fn from(variant: MAL0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAL0` writer - Mailbox 0 arbitration lost"]
pub type MAL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MAL0W_AW>;
impl<'a, REG, const O: u8> MAL0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MAL0W_AW::CLEAR)
    }
}
#[doc = "Field `MTE0` reader - Mailbox 0 transmit error"]
pub type MTE0_R = crate::BitReader<MTE0R_A>;
#[doc = "Mailbox 0 transmit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTE0R_A {
    #[doc = "0: There was no error"]
    NO_ERROR = 0,
    #[doc = "1: An error was detected"]
    ERROR = 1,
}
impl From<MTE0R_A> for bool {
    #[inline(always)]
    fn from(variant: MTE0R_A) -> Self {
        variant as u8 != 0
    }
}
impl MTE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTE0R_A {
        match self.bits {
            false => MTE0R_A::NO_ERROR,
            true => MTE0R_A::ERROR,
        }
    }
    #[doc = "There was no error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MTE0R_A::NO_ERROR
    }
    #[doc = "An error was detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MTE0R_A::ERROR
    }
}
#[doc = "Mailbox 0 transmit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTE0W_AW {
    #[doc = "1: Clears error flag"]
    CLEAR = 1,
}
impl From<MTE0W_AW> for bool {
    #[inline(always)]
    fn from(variant: MTE0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTE0` writer - Mailbox 0 transmit error"]
pub type MTE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MTE0W_AW>;
impl<'a, REG, const O: u8> MTE0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MTE0W_AW::CLEAR)
    }
}
#[doc = "Field `MST0` reader - Mailbox 0 stop transmitting"]
pub type MST0_R = crate::BitReader<MST0R_A>;
#[doc = "Mailbox 0 stop transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST0R_A {
    #[doc = "0: Mailbox is not stopped, or is empty"]
    NOT_STOP = 0,
    #[doc = "1: Stop mailbox transmitting"]
    STOP = 1,
}
impl From<MST0R_A> for bool {
    #[inline(always)]
    fn from(variant: MST0R_A) -> Self {
        variant as u8 != 0
    }
}
impl MST0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST0R_A {
        match self.bits {
            false => MST0R_A::NOT_STOP,
            true => MST0R_A::STOP,
        }
    }
    #[doc = "Mailbox is not stopped, or is empty"]
    #[inline(always)]
    pub fn is_not_stop(&self) -> bool {
        *self == MST0R_A::NOT_STOP
    }
    #[doc = "Stop mailbox transmitting"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MST0R_A::STOP
    }
}
#[doc = "Mailbox 0 stop transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST0W_AW {
    #[doc = "1: Stop mailbox transmitting"]
    STOP = 1,
}
impl From<MST0W_AW> for bool {
    #[inline(always)]
    fn from(variant: MST0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST0` writer - Mailbox 0 stop transmitting"]
pub type MST0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MST0W_AW>;
impl<'a, REG, const O: u8> MST0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop mailbox transmitting"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(MST0W_AW::STOP)
    }
}
#[doc = "Field `MAL1` reader - Mailbox 1 arbitration lost"]
pub use MAL0_R as MAL1_R;
#[doc = "Field `MAL2` reader - Mailbox 2 arbitration lost"]
pub use MAL0_R as MAL2_R;
#[doc = "Field `MAL1` writer - Mailbox 1 arbitration lost"]
pub use MAL0_W as MAL1_W;
#[doc = "Field `MAL2` writer - Mailbox 2 arbitration lost"]
pub use MAL0_W as MAL2_W;
#[doc = "Field `MST1` reader - Mailbox 1 stop transmitting"]
pub use MST0_R as MST1_R;
#[doc = "Field `MST2` reader - Mailbox 2 stop transmitting"]
pub use MST0_R as MST2_R;
#[doc = "Field `MST1` writer - Mailbox 1 stop transmitting"]
pub use MST0_W as MST1_W;
#[doc = "Field `MST2` writer - Mailbox 2 stop transmitting"]
pub use MST0_W as MST2_W;
#[doc = "Field `MTE1` reader - Mailbox 1 transmit error"]
pub use MTE0_R as MTE1_R;
#[doc = "Field `MTE2` reader - Mailbox 2 transmit error"]
pub use MTE0_R as MTE2_R;
#[doc = "Field `MTE1` writer - Mailbox 1 transmit error"]
pub use MTE0_W as MTE1_W;
#[doc = "Field `MTE2` writer - Mailbox 2 transmit error"]
pub use MTE0_W as MTE2_W;
#[doc = "Field `MTF1` reader - Mailbox 1 transmit finished"]
pub use MTF0_R as MTF1_R;
#[doc = "Field `MTF2` reader - Mailbox 2 transmit finished"]
pub use MTF0_R as MTF2_R;
#[doc = "Field `MTF1` writer - Mailbox 1 transmit finished"]
pub use MTF0_W as MTF1_W;
#[doc = "Field `MTF2` writer - Mailbox 2 transmit finished"]
pub use MTF0_W as MTF2_W;
#[doc = "Field `MTFNERR1` reader - Mailbox 1 transmit finished and no error"]
pub use MTFNERR0_R as MTFNERR1_R;
#[doc = "Field `MTFNERR2` reader - Mailbox 2 transmit finished and no error"]
pub use MTFNERR0_R as MTFNERR2_R;
#[doc = "Field `MTFNERR1` writer - Mailbox 1 transmit finished and no error"]
pub use MTFNERR0_W as MTFNERR1_W;
#[doc = "Field `MTFNERR2` writer - Mailbox 2 transmit finished and no error"]
pub use MTFNERR0_W as MTFNERR2_W;
#[doc = "Field `NUM` reader - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
pub type NUM_R = crate::FieldReader;
#[doc = "Field `TME0` reader - Transmit mailbox 0 empty"]
pub type TME0_R = crate::BitReader<TME0_A>;
#[doc = "Transmit mailbox 0 empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TME0_A {
    #[doc = "0: Transmit mailbox not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: Transmit mailbox is empty"]
    EMPTY = 1,
}
impl From<TME0_A> for bool {
    #[inline(always)]
    fn from(variant: TME0_A) -> Self {
        variant as u8 != 0
    }
}
impl TME0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TME0_A {
        match self.bits {
            false => TME0_A::NOT_EMPTY,
            true => TME0_A::EMPTY,
        }
    }
    #[doc = "Transmit mailbox not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TME0_A::NOT_EMPTY
    }
    #[doc = "Transmit mailbox is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TME0_A::EMPTY
    }
}
#[doc = "Field `TME1` reader - Transmit mailbox 1 empty"]
pub use TME0_R as TME1_R;
#[doc = "Field `TME2` reader - Transmit mailbox 2 empty"]
pub use TME0_R as TME2_R;
#[doc = "Field `TMLS0` reader - Transmit mailbox 0 last sending in transmit FIFO"]
pub type TMLS0_R = crate::BitReader<TMLS0_A>;
#[doc = "Transmit mailbox 0 last sending in transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMLS0_A {
    #[doc = "0: The mailbox doesn't have the last sending order"]
    NOT_LAST = 0,
    #[doc = "1: The mailbox has the last sending order with at least two frames pending"]
    LAST = 1,
}
impl From<TMLS0_A> for bool {
    #[inline(always)]
    fn from(variant: TMLS0_A) -> Self {
        variant as u8 != 0
    }
}
impl TMLS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMLS0_A {
        match self.bits {
            false => TMLS0_A::NOT_LAST,
            true => TMLS0_A::LAST,
        }
    }
    #[doc = "The mailbox doesn't have the last sending order"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == TMLS0_A::NOT_LAST
    }
    #[doc = "The mailbox has the last sending order with at least two frames pending"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == TMLS0_A::LAST
    }
}
#[doc = "Field `TMLS1` reader - Transmit mailbox 1 last sending in transmit FIFO"]
pub use TMLS0_R as TMLS1_R;
#[doc = "Field `TMLS2` reader - Transmit mailbox 2 last sending in transmit FIFO"]
pub use TMLS0_R as TMLS2_R;
impl R {
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    pub fn mtf0(&self) -> MTF0_R {
        MTF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr0(&self) -> MTFNERR0_R {
        MTFNERR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    pub fn mal0(&self) -> MAL0_R {
        MAL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    pub fn mte0(&self) -> MTE0_R {
        MTE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    pub fn mst0(&self) -> MST0_R {
        MST0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    pub fn mtf1(&self) -> MTF1_R {
        MTF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr1(&self) -> MTFNERR1_R {
        MTFNERR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    pub fn mal1(&self) -> MAL1_R {
        MAL1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    pub fn mte1(&self) -> MTE1_R {
        MTE1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    pub fn mst1(&self) -> MST1_R {
        MST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    pub fn mtf2(&self) -> MTF2_R {
        MTF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr2(&self) -> MTFNERR2_R {
        MTFNERR2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    pub fn mal2(&self) -> MAL2_R {
        MAL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    pub fn mte2(&self) -> MTE2_R {
        MTE2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    pub fn mst2(&self) -> MST2_R {
        MST2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty"]
    #[inline(always)]
    pub fn tme0(&self) -> TME0_R {
        TME0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty"]
    #[inline(always)]
    pub fn tme1(&self) -> TME1_R {
        TME1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty"]
    #[inline(always)]
    pub fn tme2(&self) -> TME2_R {
        TME2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit mailbox 0 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls0(&self) -> TMLS0_R {
        TMLS0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit mailbox 1 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls1(&self) -> TMLS1_R {
        TMLS1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit mailbox 2 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls2(&self) -> TMLS2_R {
        TMLS2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf0(&mut self) -> MTF0_W<TSTAT_SPEC, 0> {
        MTF0_W::new(self)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr0(&mut self) -> MTFNERR0_W<TSTAT_SPEC, 1> {
        MTFNERR0_W::new(self)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal0(&mut self) -> MAL0_W<TSTAT_SPEC, 2> {
        MAL0_W::new(self)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte0(&mut self) -> MTE0_W<TSTAT_SPEC, 3> {
        MTE0_W::new(self)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst0(&mut self) -> MST0_W<TSTAT_SPEC, 7> {
        MST0_W::new(self)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf1(&mut self) -> MTF1_W<TSTAT_SPEC, 8> {
        MTF1_W::new(self)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr1(&mut self) -> MTFNERR1_W<TSTAT_SPEC, 9> {
        MTFNERR1_W::new(self)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal1(&mut self) -> MAL1_W<TSTAT_SPEC, 10> {
        MAL1_W::new(self)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte1(&mut self) -> MTE1_W<TSTAT_SPEC, 11> {
        MTE1_W::new(self)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst1(&mut self) -> MST1_W<TSTAT_SPEC, 15> {
        MST1_W::new(self)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf2(&mut self) -> MTF2_W<TSTAT_SPEC, 16> {
        MTF2_W::new(self)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr2(&mut self) -> MTFNERR2_W<TSTAT_SPEC, 17> {
        MTFNERR2_W::new(self)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal2(&mut self) -> MAL2_W<TSTAT_SPEC, 18> {
        MAL2_W::new(self)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte2(&mut self) -> MTE2_W<TSTAT_SPEC, 19> {
        MTE2_W::new(self)
    }
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst2(&mut self) -> MST2_W<TSTAT_SPEC, 23> {
        MST2_W::new(self)
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
#[doc = "Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSTAT_SPEC;
impl crate::RegisterSpec for TSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstat::R`](R) reader structure"]
impl crate::Readable for TSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tstat::W`](W) writer structure"]
impl crate::Writable for TSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSTAT to value 0x1c00_0000"]
impl crate::Resettable for TSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c00_0000;
}
