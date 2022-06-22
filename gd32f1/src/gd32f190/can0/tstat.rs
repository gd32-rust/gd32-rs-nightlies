#[doc = "Register `TSTAT` reader"]
pub struct R(crate::R<TSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSTAT` writer"]
pub struct W(crate::W<TSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSTAT_SPEC>;
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
impl From<crate::W<TSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit mailbox 2 last sending in transmit FIFO"]
pub use TMLS0_A as TMLS2_A;
#[doc = "Transmit mailbox 1 last sending in transmit FIFO"]
pub use TMLS0_A as TMLS1_A;
#[doc = "Field `TMLS2` reader - Transmit mailbox 2 last sending in transmit FIFO"]
pub use TMLS0_R as TMLS2_R;
#[doc = "Field `TMLS1` reader - Transmit mailbox 1 last sending in transmit FIFO"]
pub use TMLS0_R as TMLS1_R;
#[doc = "Transmit mailbox 0 last sending in transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMLS0_A {
    #[doc = "0: The mailbox doesn't have the last sending order"]
    NOTLAST = 0,
    #[doc = "1: The mailbox has the last sending order with at least two frames pending"]
    LAST = 1,
}
impl From<TMLS0_A> for bool {
    #[inline(always)]
    fn from(variant: TMLS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMLS0` reader - Transmit mailbox 0 last sending in transmit FIFO"]
pub type TMLS0_R = crate::BitReader<TMLS0_A>;
impl TMLS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMLS0_A {
        match self.bits {
            false => TMLS0_A::NOTLAST,
            true => TMLS0_A::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLAST`"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == TMLS0_A::NOTLAST
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == TMLS0_A::LAST
    }
}
#[doc = "Transmit mailbox 2 empty"]
pub use TME0_A as TME2_A;
#[doc = "Transmit mailbox 1 empty"]
pub use TME0_A as TME1_A;
#[doc = "Field `TME2` reader - Transmit mailbox 2 empty"]
pub use TME0_R as TME2_R;
#[doc = "Field `TME1` reader - Transmit mailbox 1 empty"]
pub use TME0_R as TME1_R;
#[doc = "Transmit mailbox 0 empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TME0_A {
    #[doc = "0: Transmit mailbox not empty"]
    NOTEMPTY = 0,
    #[doc = "1: Transmit mailbox is empty"]
    EMPTY = 1,
}
impl From<TME0_A> for bool {
    #[inline(always)]
    fn from(variant: TME0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TME0` reader - Transmit mailbox 0 empty"]
pub type TME0_R = crate::BitReader<TME0_A>;
impl TME0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TME0_A {
        match self.bits {
            false => TME0_A::NOTEMPTY,
            true => TME0_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TME0_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TME0_A::EMPTY
    }
}
#[doc = "Field `NUM` reader - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
pub type NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Mailbox 2 arbitration lost"]
pub use MAL0_A as MAL2_A;
#[doc = "Mailbox 1 arbitration lost"]
pub use MAL0_A as MAL1_A;
#[doc = "Mailbox 2 arbitration lost"]
pub use MAL0_AW as MAL2_AW;
#[doc = "Mailbox 1 arbitration lost"]
pub use MAL0_AW as MAL1_AW;
#[doc = "Field `MAL2` reader - Mailbox 2 arbitration lost"]
pub use MAL0_R as MAL2_R;
#[doc = "Field `MAL1` reader - Mailbox 1 arbitration lost"]
pub use MAL0_R as MAL1_R;
#[doc = "Field `MAL2` writer - Mailbox 2 arbitration lost"]
pub use MAL0_W as MAL2_W;
#[doc = "Field `MAL1` writer - Mailbox 1 arbitration lost"]
pub use MAL0_W as MAL1_W;
#[doc = "Mailbox 2 stop transmitting"]
pub use MST0_A as MST2_A;
#[doc = "Mailbox 1 stop transmitting"]
pub use MST0_A as MST1_A;
#[doc = "Mailbox 2 stop transmitting"]
pub use MST0_AW as MST2_AW;
#[doc = "Mailbox 1 stop transmitting"]
pub use MST0_AW as MST1_AW;
#[doc = "Field `MST2` reader - Mailbox 2 stop transmitting"]
pub use MST0_R as MST2_R;
#[doc = "Field `MST1` reader - Mailbox 1 stop transmitting"]
pub use MST0_R as MST1_R;
#[doc = "Field `MST2` writer - Mailbox 2 stop transmitting"]
pub use MST0_W as MST2_W;
#[doc = "Field `MST1` writer - Mailbox 1 stop transmitting"]
pub use MST0_W as MST1_W;
#[doc = "Mailbox 2 transmit error"]
pub use MTE0_A as MTE2_A;
#[doc = "Mailbox 1 transmit error"]
pub use MTE0_A as MTE1_A;
#[doc = "Mailbox 2 transmit error"]
pub use MTE0_AW as MTE2_AW;
#[doc = "Mailbox 1 transmit error"]
pub use MTE0_AW as MTE1_AW;
#[doc = "Field `MTE2` reader - Mailbox 2 transmit error"]
pub use MTE0_R as MTE2_R;
#[doc = "Field `MTE1` reader - Mailbox 1 transmit error"]
pub use MTE0_R as MTE1_R;
#[doc = "Field `MTE2` writer - Mailbox 2 transmit error"]
pub use MTE0_W as MTE2_W;
#[doc = "Field `MTE1` writer - Mailbox 1 transmit error"]
pub use MTE0_W as MTE1_W;
#[doc = "Mailbox 2 transmit finished"]
pub use MTF0_A as MTF2_A;
#[doc = "Mailbox 1 transmit finished"]
pub use MTF0_A as MTF1_A;
#[doc = "Mailbox 2 transmit finished"]
pub use MTF0_AW as MTF2_AW;
#[doc = "Mailbox 1 transmit finished"]
pub use MTF0_AW as MTF1_AW;
#[doc = "Field `MTF2` reader - Mailbox 2 transmit finished"]
pub use MTF0_R as MTF2_R;
#[doc = "Field `MTF1` reader - Mailbox 1 transmit finished"]
pub use MTF0_R as MTF1_R;
#[doc = "Field `MTF2` writer - Mailbox 2 transmit finished"]
pub use MTF0_W as MTF2_W;
#[doc = "Field `MTF1` writer - Mailbox 1 transmit finished"]
pub use MTF0_W as MTF1_W;
#[doc = "Mailbox 2 transmit finished and no error"]
pub use MTFNERR0_A as MTFNERR2_A;
#[doc = "Mailbox 1 transmit finished and no error"]
pub use MTFNERR0_A as MTFNERR1_A;
#[doc = "Mailbox 2 transmit finished and no error"]
pub use MTFNERR0_AW as MTFNERR2_AW;
#[doc = "Mailbox 1 transmit finished and no error"]
pub use MTFNERR0_AW as MTFNERR1_AW;
#[doc = "Field `MTFNERR2` reader - Mailbox 2 transmit finished and no error"]
pub use MTFNERR0_R as MTFNERR2_R;
#[doc = "Field `MTFNERR1` reader - Mailbox 1 transmit finished and no error"]
pub use MTFNERR0_R as MTFNERR1_R;
#[doc = "Field `MTFNERR2` writer - Mailbox 2 transmit finished and no error"]
pub use MTFNERR0_W as MTFNERR2_W;
#[doc = "Field `MTFNERR1` writer - Mailbox 1 transmit finished and no error"]
pub use MTFNERR0_W as MTFNERR1_W;
#[doc = "Mailbox 0 stop transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST0_A {
    #[doc = "0: Mailbox is not stopped, or is empty"]
    NOTSTOP = 0,
    #[doc = "1: Stop mailbox transmitting"]
    STOP = 1,
}
impl From<MST0_A> for bool {
    #[inline(always)]
    fn from(variant: MST0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST0` reader - Mailbox 0 stop transmitting"]
pub type MST0_R = crate::BitReader<MST0_A>;
impl MST0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST0_A {
        match self.bits {
            false => MST0_A::NOTSTOP,
            true => MST0_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTOP`"]
    #[inline(always)]
    pub fn is_not_stop(&self) -> bool {
        *self == MST0_A::NOTSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MST0_A::STOP
    }
}
#[doc = "Mailbox 0 stop transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST0_AW {
    #[doc = "1: Stop mailbox transmitting"]
    STOP = 1,
}
impl From<MST0_AW> for bool {
    #[inline(always)]
    fn from(variant: MST0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST0` writer - Mailbox 0 stop transmitting"]
pub type MST0_W<'a> = crate::BitWriter<'a, u32, TSTAT_SPEC, MST0_AW, 7>;
impl<'a> MST0_W<'a> {
    #[doc = "Stop mailbox transmitting"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MST0_AW::STOP)
    }
}
#[doc = "Mailbox 0 transmit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTE0_A {
    #[doc = "0: There was no error"]
    NOERROR = 0,
    #[doc = "1: An error was detected"]
    ERROR = 1,
}
impl From<MTE0_A> for bool {
    #[inline(always)]
    fn from(variant: MTE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTE0` reader - Mailbox 0 transmit error"]
pub type MTE0_R = crate::BitReader<MTE0_A>;
impl MTE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTE0_A {
        match self.bits {
            false => MTE0_A::NOERROR,
            true => MTE0_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MTE0_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MTE0_A::ERROR
    }
}
#[doc = "Mailbox 0 transmit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTE0_AW {
    #[doc = "1: Clears error flag"]
    CLEAR = 1,
}
impl From<MTE0_AW> for bool {
    #[inline(always)]
    fn from(variant: MTE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTE0` writer - Mailbox 0 transmit error"]
pub type MTE0_W<'a> = crate::BitWriter<'a, u32, TSTAT_SPEC, MTE0_AW, 3>;
impl<'a> MTE0_W<'a> {
    #[doc = "Clears error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MTE0_AW::CLEAR)
    }
}
#[doc = "Mailbox 0 arbitration lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAL0_A {
    #[doc = "0: Arbitration was not lost"]
    NOARBITRATIONLOST = 0,
    #[doc = "1: Arbitration lost"]
    ARBITRATIONLOST = 1,
}
impl From<MAL0_A> for bool {
    #[inline(always)]
    fn from(variant: MAL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAL0` reader - Mailbox 0 arbitration lost"]
pub type MAL0_R = crate::BitReader<MAL0_A>;
impl MAL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAL0_A {
        match self.bits {
            false => MAL0_A::NOARBITRATIONLOST,
            true => MAL0_A::ARBITRATIONLOST,
        }
    }
    #[doc = "Checks if the value of the field is `NOARBITRATIONLOST`"]
    #[inline(always)]
    pub fn is_no_arbitration_lost(&self) -> bool {
        *self == MAL0_A::NOARBITRATIONLOST
    }
    #[doc = "Checks if the value of the field is `ARBITRATIONLOST`"]
    #[inline(always)]
    pub fn is_arbitration_lost(&self) -> bool {
        *self == MAL0_A::ARBITRATIONLOST
    }
}
#[doc = "Mailbox 0 arbitration lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAL0_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<MAL0_AW> for bool {
    #[inline(always)]
    fn from(variant: MAL0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAL0` writer - Mailbox 0 arbitration lost"]
pub type MAL0_W<'a> = crate::BitWriter<'a, u32, TSTAT_SPEC, MAL0_AW, 2>;
impl<'a> MAL0_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MAL0_AW::CLEAR)
    }
}
#[doc = "Mailbox 0 transmit finished and no error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTFNERR0_A {
    #[doc = "0: Mailbox transmission finished with an error"]
    FINISHEDWITHERROR = 0,
    #[doc = "1: Mailbox transmission finished with no error"]
    FINISHEDNOERROR = 1,
}
impl From<MTFNERR0_A> for bool {
    #[inline(always)]
    fn from(variant: MTFNERR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTFNERR0` reader - Mailbox 0 transmit finished and no error"]
pub type MTFNERR0_R = crate::BitReader<MTFNERR0_A>;
impl MTFNERR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTFNERR0_A {
        match self.bits {
            false => MTFNERR0_A::FINISHEDWITHERROR,
            true => MTFNERR0_A::FINISHEDNOERROR,
        }
    }
    #[doc = "Checks if the value of the field is `FINISHEDWITHERROR`"]
    #[inline(always)]
    pub fn is_finished_with_error(&self) -> bool {
        *self == MTFNERR0_A::FINISHEDWITHERROR
    }
    #[doc = "Checks if the value of the field is `FINISHEDNOERROR`"]
    #[inline(always)]
    pub fn is_finished_no_error(&self) -> bool {
        *self == MTFNERR0_A::FINISHEDNOERROR
    }
}
#[doc = "Mailbox 0 transmit finished and no error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTFNERR0_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<MTFNERR0_AW> for bool {
    #[inline(always)]
    fn from(variant: MTFNERR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTFNERR0` writer - Mailbox 0 transmit finished and no error"]
pub type MTFNERR0_W<'a> = crate::BitWriter<'a, u32, TSTAT_SPEC, MTFNERR0_AW, 1>;
impl<'a> MTFNERR0_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MTFNERR0_AW::CLEAR)
    }
}
#[doc = "Mailbox 0 transmit finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTF0_A {
    #[doc = "0: Mailbox transmission still in progress"]
    INPROGRESS = 0,
    #[doc = "1: Mailbox transmission finished"]
    FINISHED = 1,
}
impl From<MTF0_A> for bool {
    #[inline(always)]
    fn from(variant: MTF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTF0` reader - Mailbox 0 transmit finished"]
pub type MTF0_R = crate::BitReader<MTF0_A>;
impl MTF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTF0_A {
        match self.bits {
            false => MTF0_A::INPROGRESS,
            true => MTF0_A::FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `INPROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == MTF0_A::INPROGRESS
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == MTF0_A::FINISHED
    }
}
#[doc = "Mailbox 0 transmit finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTF0_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<MTF0_AW> for bool {
    #[inline(always)]
    fn from(variant: MTF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTF0` writer - Mailbox 0 transmit finished"]
pub type MTF0_W<'a> = crate::BitWriter<'a, u32, TSTAT_SPEC, MTF0_AW, 0>;
impl<'a> MTF0_W<'a> {
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MTF0_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 31 - Transmit mailbox 2 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls2(&self) -> TMLS2_R {
        TMLS2_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit mailbox 1 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls1(&self) -> TMLS1_R {
        TMLS1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit mailbox 0 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls0(&self) -> TMLS0_R {
        TMLS0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty"]
    #[inline(always)]
    pub fn tme2(&self) -> TME2_R {
        TME2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty"]
    #[inline(always)]
    pub fn tme1(&self) -> TME1_R {
        TME1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty"]
    #[inline(always)]
    pub fn tme0(&self) -> TME0_R {
        TME0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 24:25 - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    pub fn mst2(&self) -> MST2_R {
        MST2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    pub fn mte2(&self) -> MTE2_R {
        MTE2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    pub fn mal2(&self) -> MAL2_R {
        MAL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr2(&self) -> MTFNERR2_R {
        MTFNERR2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    pub fn mtf2(&self) -> MTF2_R {
        MTF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    pub fn mst1(&self) -> MST1_R {
        MST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    pub fn mte1(&self) -> MTE1_R {
        MTE1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    pub fn mal1(&self) -> MAL1_R {
        MAL1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr1(&self) -> MTFNERR1_R {
        MTFNERR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    pub fn mtf1(&self) -> MTF1_R {
        MTF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    pub fn mst0(&self) -> MST0_R {
        MST0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    pub fn mte0(&self) -> MTE0_R {
        MTE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    pub fn mal0(&self) -> MAL0_R {
        MAL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr0(&self) -> MTFNERR0_R {
        MTFNERR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    pub fn mtf0(&self) -> MTF0_R {
        MTF0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    pub fn mst2(&mut self) -> MST2_W {
        MST2_W::new(self)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    pub fn mte2(&mut self) -> MTE2_W {
        MTE2_W::new(self)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    pub fn mal2(&mut self) -> MAL2_W {
        MAL2_W::new(self)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr2(&mut self) -> MTFNERR2_W {
        MTFNERR2_W::new(self)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    pub fn mtf2(&mut self) -> MTF2_W {
        MTF2_W::new(self)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    pub fn mst1(&mut self) -> MST1_W {
        MST1_W::new(self)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    pub fn mte1(&mut self) -> MTE1_W {
        MTE1_W::new(self)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    pub fn mal1(&mut self) -> MAL1_W {
        MAL1_W::new(self)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr1(&mut self) -> MTFNERR1_W {
        MTFNERR1_W::new(self)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    pub fn mtf1(&mut self) -> MTF1_W {
        MTF1_W::new(self)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    pub fn mst0(&mut self) -> MST0_W {
        MST0_W::new(self)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    pub fn mte0(&mut self) -> MTE0_W {
        MTE0_W::new(self)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    pub fn mal0(&mut self) -> MAL0_W {
        MAL0_W::new(self)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr0(&mut self) -> MTFNERR0_W {
        MTFNERR0_W::new(self)
    }
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    pub fn mtf0(&mut self) -> MTF0_W {
        MTF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstat](index.html) module"]
pub struct TSTAT_SPEC;
impl crate::RegisterSpec for TSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tstat::R](R) reader structure"]
impl crate::Readable for TSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tstat::W](W) writer structure"]
impl crate::Writable for TSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSTAT to value 0x1c00_0000"]
impl crate::Resettable for TSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1c00_0000
    }
}
