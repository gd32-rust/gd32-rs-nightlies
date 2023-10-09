#[doc = "Register `ST2CNTRST` reader"]
pub type R = crate::R<ST2CNTRST_SPEC>;
#[doc = "Register `ST2CNTRST` writer"]
pub type W = crate::W<ST2CNTRST_SPEC>;
#[doc = "Field `UPRST` reader - Slave_TIMER2 update event resets counter"]
pub type UPRST_R = crate::BitReader;
#[doc = "Field `UPRST` writer - Slave_TIMER2 update event resets counter"]
pub type UPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1RST` reader - Slave_TIMER2 compare 1 event resets counter"]
pub type CMP1RST_R = crate::BitReader;
#[doc = "Field `CMP1RST` writer - Slave_TIMER2 compare 1 event resets counter"]
pub type CMP1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP3RST` reader - Slave_TIMER2 compare 3 event resets counter"]
pub type CMP3RST_R = crate::BitReader;
#[doc = "Field `CMP3RST` writer - Slave_TIMER2 compare 3 event resets counter"]
pub type CMP3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTPERRST` reader - Master_TIMER period event resets counter"]
pub type MTPERRST_R = crate::BitReader;
#[doc = "Field `MTPERRST` writer - Master_TIMER period event resets counter"]
pub type MTPERRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP0RST` reader - Master_TIMER compare 0 event resets counter"]
pub type MTCMP0RST_R = crate::BitReader;
#[doc = "Field `MTCMP0RST` writer - Master_TIMER compare 0 event resets counter"]
pub type MTCMP0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP1RST` reader - Master_TIMER compare 1 event resets counter"]
pub type MTCMP1RST_R = crate::BitReader;
#[doc = "Field `MTCMP1RST` writer - Master_TIMER compare 1 event resets counter"]
pub type MTCMP1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP2RST` reader - Master_TIMER compare 2 event resets counter"]
pub type MTCMP2RST_R = crate::BitReader;
#[doc = "Field `MTCMP2RST` writer - Master_TIMER compare 2 event resets counter"]
pub type MTCMP2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP3RST` reader - Master_TIMER compare 3 event resets counter"]
pub type MTCMP3RST_R = crate::BitReader;
#[doc = "Field `MTCMP3RST` writer - Master_TIMER compare 3 event resets counter"]
pub type MTCMP3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV0RST` reader - External event 0 resets counter"]
pub type EXEV0RST_R = crate::BitReader;
#[doc = "Field `EXEV0RST` writer - External event 0 resets counter"]
pub type EXEV0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV1RST` reader - External event 1 resets counter"]
pub type EXEV1RST_R = crate::BitReader;
#[doc = "Field `EXEV1RST` writer - External event 1 resets counter"]
pub type EXEV1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV2RST` reader - External event 2 resets counter"]
pub type EXEV2RST_R = crate::BitReader;
#[doc = "Field `EXEV2RST` writer - External event 2 resets counter"]
pub type EXEV2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV3RST` reader - External event 3 resets counter"]
pub type EXEV3RST_R = crate::BitReader;
#[doc = "Field `EXEV3RST` writer - External event 3 resets counter"]
pub type EXEV3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV4RST` reader - External event 4 resets counter"]
pub type EXEV4RST_R = crate::BitReader;
#[doc = "Field `EXEV4RST` writer - External event 4 resets counter"]
pub type EXEV4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV5RST` reader - External event 5 resets counter"]
pub type EXEV5RST_R = crate::BitReader;
#[doc = "Field `EXEV5RST` writer - External event 5 resets counter"]
pub type EXEV5RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV6RST` reader - External event 6 resets counter"]
pub type EXEV6RST_R = crate::BitReader;
#[doc = "Field `EXEV6RST` writer - External event 6 resets counter"]
pub type EXEV6RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV7RST` reader - External event 7 resets counter"]
pub type EXEV7RST_R = crate::BitReader;
#[doc = "Field `EXEV7RST` writer - External event 7 resets counter"]
pub type EXEV7RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV8RST` reader - External event 8 resets counter"]
pub type EXEV8RST_R = crate::BitReader;
#[doc = "Field `EXEV8RST` writer - External event 8 resets counter"]
pub type EXEV8RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV9RST` reader - External event 9 resets counter"]
pub type EXEV9RST_R = crate::BitReader;
#[doc = "Field `EXEV9RST` writer - External event 9 resets counter"]
pub type EXEV9RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0CMP0RST` reader - Slave_TIMER0 compare 0 event resets counter"]
pub type ST0CMP0RST_R = crate::BitReader;
#[doc = "Field `ST0CMP0RST` writer - Slave_TIMER0 compare 0 event resets counter"]
pub type ST0CMP0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0CMP1RST` reader - Slave_TIMER0 compare 1 event resets counter"]
pub type ST0CMP1RST_R = crate::BitReader;
#[doc = "Field `ST0CMP1RST` writer - Slave_TIMER0 compare 1 event resets counter"]
pub type ST0CMP1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0CMP3RST` reader - Slave_TIMER0 compare 3 event resets counter"]
pub type ST0CMP3RST_R = crate::BitReader;
#[doc = "Field `ST0CMP3RST` writer - Slave_TIMER0 compare 3 event resets counter"]
pub type ST0CMP3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CMP0RST` reader - Slave_TIMER1 compare 0 event resets counter"]
pub type ST1CMP0RST_R = crate::BitReader;
#[doc = "Field `ST1CMP0RST` writer - Slave_TIMER1 compare 0 event resets counter"]
pub type ST1CMP0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CMP1RST` reader - Slave_TIMER1 compare 1 event resets counter"]
pub type ST1CMP1RST_R = crate::BitReader;
#[doc = "Field `ST1CMP1RST` writer - Slave_TIMER1 compare 1 event resets counter"]
pub type ST1CMP1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CMP3RST` reader - Slave_TIMER1 compare 3 event resets counter"]
pub type ST1CMP3RST_R = crate::BitReader;
#[doc = "Field `ST1CMP3RST` writer - Slave_TIMER1 compare 3 event resets counter"]
pub type ST1CMP3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CMP0RST` reader - Slave_TIMER3 compare 0 event resets counter"]
pub type ST3CMP0RST_R = crate::BitReader;
#[doc = "Field `ST3CMP0RST` writer - Slave_TIMER3 compare 0 event resets counter"]
pub type ST3CMP0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CMP1RST` reader - Slave_TIMER3 compare 1 event resets counter"]
pub type ST3CMP1RST_R = crate::BitReader;
#[doc = "Field `ST3CMP1RST` writer - Slave_TIMER3 compare 1 event resets counter"]
pub type ST3CMP1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CMP3RST` reader - Slave_TIMER3 compare 3 event resets counter"]
pub type ST3CMP3RST_R = crate::BitReader;
#[doc = "Field `ST3CMP3RST` writer - Slave_TIMER3 compare 3 event resets counter"]
pub type ST3CMP3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CMP0RST` reader - Slave_TIMER4 compare 0 event resets counter"]
pub type ST4CMP0RST_R = crate::BitReader;
#[doc = "Field `ST4CMP0RST` writer - Slave_TIMER4 compare 0 event resets counter"]
pub type ST4CMP0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CMP1RST` reader - Slave_TIMER4 compare 1 event resets counter"]
pub type ST4CMP1RST_R = crate::BitReader;
#[doc = "Field `ST4CMP1RST` writer - Slave_TIMER4 compare 1 event resets counter"]
pub type ST4CMP1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CMP3RST` reader - Slave_TIMER4 compare 3 event resets counter"]
pub type ST4CMP3RST_R = crate::BitReader;
#[doc = "Field `ST4CMP3RST` writer - Slave_TIMER4 compare 3 event resets counter"]
pub type ST4CMP3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Slave_TIMER2 update event resets counter"]
    #[inline(always)]
    pub fn uprst(&self) -> UPRST_R {
        UPRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave_TIMER2 compare 1 event resets counter"]
    #[inline(always)]
    pub fn cmp1rst(&self) -> CMP1RST_R {
        CMP1RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave_TIMER2 compare 3 event resets counter"]
    #[inline(always)]
    pub fn cmp3rst(&self) -> CMP3RST_R {
        CMP3RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master_TIMER period event resets counter"]
    #[inline(always)]
    pub fn mtperrst(&self) -> MTPERRST_R {
        MTPERRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master_TIMER compare 0 event resets counter"]
    #[inline(always)]
    pub fn mtcmp0rst(&self) -> MTCMP0RST_R {
        MTCMP0RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master_TIMER compare 1 event resets counter"]
    #[inline(always)]
    pub fn mtcmp1rst(&self) -> MTCMP1RST_R {
        MTCMP1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master_TIMER compare 2 event resets counter"]
    #[inline(always)]
    pub fn mtcmp2rst(&self) -> MTCMP2RST_R {
        MTCMP2RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master_TIMER compare 3 event resets counter"]
    #[inline(always)]
    pub fn mtcmp3rst(&self) -> MTCMP3RST_R {
        MTCMP3RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External event 0 resets counter"]
    #[inline(always)]
    pub fn exev0rst(&self) -> EXEV0RST_R {
        EXEV0RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External event 1 resets counter"]
    #[inline(always)]
    pub fn exev1rst(&self) -> EXEV1RST_R {
        EXEV1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External event 2 resets counter"]
    #[inline(always)]
    pub fn exev2rst(&self) -> EXEV2RST_R {
        EXEV2RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External event 3 resets counter"]
    #[inline(always)]
    pub fn exev3rst(&self) -> EXEV3RST_R {
        EXEV3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External event 4 resets counter"]
    #[inline(always)]
    pub fn exev4rst(&self) -> EXEV4RST_R {
        EXEV4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External event 5 resets counter"]
    #[inline(always)]
    pub fn exev5rst(&self) -> EXEV5RST_R {
        EXEV5RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External event 6 resets counter"]
    #[inline(always)]
    pub fn exev6rst(&self) -> EXEV6RST_R {
        EXEV6RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - External event 7 resets counter"]
    #[inline(always)]
    pub fn exev7rst(&self) -> EXEV7RST_R {
        EXEV7RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External event 8 resets counter"]
    #[inline(always)]
    pub fn exev8rst(&self) -> EXEV8RST_R {
        EXEV8RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External event 9 resets counter"]
    #[inline(always)]
    pub fn exev9rst(&self) -> EXEV9RST_R {
        EXEV9RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave_TIMER0 compare 0 event resets counter"]
    #[inline(always)]
    pub fn st0cmp0rst(&self) -> ST0CMP0RST_R {
        ST0CMP0RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slave_TIMER0 compare 1 event resets counter"]
    #[inline(always)]
    pub fn st0cmp1rst(&self) -> ST0CMP1RST_R {
        ST0CMP1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slave_TIMER0 compare 3 event resets counter"]
    #[inline(always)]
    pub fn st0cmp3rst(&self) -> ST0CMP3RST_R {
        ST0CMP3RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Slave_TIMER1 compare 0 event resets counter"]
    #[inline(always)]
    pub fn st1cmp0rst(&self) -> ST1CMP0RST_R {
        ST1CMP0RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave_TIMER1 compare 1 event resets counter"]
    #[inline(always)]
    pub fn st1cmp1rst(&self) -> ST1CMP1RST_R {
        ST1CMP1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Slave_TIMER1 compare 3 event resets counter"]
    #[inline(always)]
    pub fn st1cmp3rst(&self) -> ST1CMP3RST_R {
        ST1CMP3RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Slave_TIMER3 compare 0 event resets counter"]
    #[inline(always)]
    pub fn st3cmp0rst(&self) -> ST3CMP0RST_R {
        ST3CMP0RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Slave_TIMER3 compare 1 event resets counter"]
    #[inline(always)]
    pub fn st3cmp1rst(&self) -> ST3CMP1RST_R {
        ST3CMP1RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Slave_TIMER3 compare 3 event resets counter"]
    #[inline(always)]
    pub fn st3cmp3rst(&self) -> ST3CMP3RST_R {
        ST3CMP3RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Slave_TIMER4 compare 0 event resets counter"]
    #[inline(always)]
    pub fn st4cmp0rst(&self) -> ST4CMP0RST_R {
        ST4CMP0RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Slave_TIMER4 compare 1 event resets counter"]
    #[inline(always)]
    pub fn st4cmp1rst(&self) -> ST4CMP1RST_R {
        ST4CMP1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave_TIMER4 compare 3 event resets counter"]
    #[inline(always)]
    pub fn st4cmp3rst(&self) -> ST4CMP3RST_R {
        ST4CMP3RST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Slave_TIMER2 update event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn uprst(&mut self) -> UPRST_W<ST2CNTRST_SPEC, 1> {
        UPRST_W::new(self)
    }
    #[doc = "Bit 2 - Slave_TIMER2 compare 1 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1rst(&mut self) -> CMP1RST_W<ST2CNTRST_SPEC, 2> {
        CMP1RST_W::new(self)
    }
    #[doc = "Bit 3 - Slave_TIMER2 compare 3 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3rst(&mut self) -> CMP3RST_W<ST2CNTRST_SPEC, 3> {
        CMP3RST_W::new(self)
    }
    #[doc = "Bit 4 - Master_TIMER period event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn mtperrst(&mut self) -> MTPERRST_W<ST2CNTRST_SPEC, 4> {
        MTPERRST_W::new(self)
    }
    #[doc = "Bit 5 - Master_TIMER compare 0 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp0rst(&mut self) -> MTCMP0RST_W<ST2CNTRST_SPEC, 5> {
        MTCMP0RST_W::new(self)
    }
    #[doc = "Bit 6 - Master_TIMER compare 1 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp1rst(&mut self) -> MTCMP1RST_W<ST2CNTRST_SPEC, 6> {
        MTCMP1RST_W::new(self)
    }
    #[doc = "Bit 7 - Master_TIMER compare 2 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp2rst(&mut self) -> MTCMP2RST_W<ST2CNTRST_SPEC, 7> {
        MTCMP2RST_W::new(self)
    }
    #[doc = "Bit 8 - Master_TIMER compare 3 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp3rst(&mut self) -> MTCMP3RST_W<ST2CNTRST_SPEC, 8> {
        MTCMP3RST_W::new(self)
    }
    #[doc = "Bit 9 - External event 0 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev0rst(&mut self) -> EXEV0RST_W<ST2CNTRST_SPEC, 9> {
        EXEV0RST_W::new(self)
    }
    #[doc = "Bit 10 - External event 1 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev1rst(&mut self) -> EXEV1RST_W<ST2CNTRST_SPEC, 10> {
        EXEV1RST_W::new(self)
    }
    #[doc = "Bit 11 - External event 2 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev2rst(&mut self) -> EXEV2RST_W<ST2CNTRST_SPEC, 11> {
        EXEV2RST_W::new(self)
    }
    #[doc = "Bit 12 - External event 3 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev3rst(&mut self) -> EXEV3RST_W<ST2CNTRST_SPEC, 12> {
        EXEV3RST_W::new(self)
    }
    #[doc = "Bit 13 - External event 4 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev4rst(&mut self) -> EXEV4RST_W<ST2CNTRST_SPEC, 13> {
        EXEV4RST_W::new(self)
    }
    #[doc = "Bit 14 - External event 5 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev5rst(&mut self) -> EXEV5RST_W<ST2CNTRST_SPEC, 14> {
        EXEV5RST_W::new(self)
    }
    #[doc = "Bit 15 - External event 6 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev6rst(&mut self) -> EXEV6RST_W<ST2CNTRST_SPEC, 15> {
        EXEV6RST_W::new(self)
    }
    #[doc = "Bit 16 - External event 7 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev7rst(&mut self) -> EXEV7RST_W<ST2CNTRST_SPEC, 16> {
        EXEV7RST_W::new(self)
    }
    #[doc = "Bit 17 - External event 8 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev8rst(&mut self) -> EXEV8RST_W<ST2CNTRST_SPEC, 17> {
        EXEV8RST_W::new(self)
    }
    #[doc = "Bit 18 - External event 9 resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn exev9rst(&mut self) -> EXEV9RST_W<ST2CNTRST_SPEC, 18> {
        EXEV9RST_W::new(self)
    }
    #[doc = "Bit 19 - Slave_TIMER0 compare 0 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st0cmp0rst(&mut self) -> ST0CMP0RST_W<ST2CNTRST_SPEC, 19> {
        ST0CMP0RST_W::new(self)
    }
    #[doc = "Bit 20 - Slave_TIMER0 compare 1 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st0cmp1rst(&mut self) -> ST0CMP1RST_W<ST2CNTRST_SPEC, 20> {
        ST0CMP1RST_W::new(self)
    }
    #[doc = "Bit 21 - Slave_TIMER0 compare 3 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st0cmp3rst(&mut self) -> ST0CMP3RST_W<ST2CNTRST_SPEC, 21> {
        ST0CMP3RST_W::new(self)
    }
    #[doc = "Bit 22 - Slave_TIMER1 compare 0 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st1cmp0rst(&mut self) -> ST1CMP0RST_W<ST2CNTRST_SPEC, 22> {
        ST1CMP0RST_W::new(self)
    }
    #[doc = "Bit 23 - Slave_TIMER1 compare 1 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st1cmp1rst(&mut self) -> ST1CMP1RST_W<ST2CNTRST_SPEC, 23> {
        ST1CMP1RST_W::new(self)
    }
    #[doc = "Bit 24 - Slave_TIMER1 compare 3 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st1cmp3rst(&mut self) -> ST1CMP3RST_W<ST2CNTRST_SPEC, 24> {
        ST1CMP3RST_W::new(self)
    }
    #[doc = "Bit 25 - Slave_TIMER3 compare 0 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st3cmp0rst(&mut self) -> ST3CMP0RST_W<ST2CNTRST_SPEC, 25> {
        ST3CMP0RST_W::new(self)
    }
    #[doc = "Bit 26 - Slave_TIMER3 compare 1 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st3cmp1rst(&mut self) -> ST3CMP1RST_W<ST2CNTRST_SPEC, 26> {
        ST3CMP1RST_W::new(self)
    }
    #[doc = "Bit 27 - Slave_TIMER3 compare 3 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st3cmp3rst(&mut self) -> ST3CMP3RST_W<ST2CNTRST_SPEC, 27> {
        ST3CMP3RST_W::new(self)
    }
    #[doc = "Bit 28 - Slave_TIMER4 compare 0 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st4cmp0rst(&mut self) -> ST4CMP0RST_W<ST2CNTRST_SPEC, 28> {
        ST4CMP0RST_W::new(self)
    }
    #[doc = "Bit 29 - Slave_TIMER4 compare 1 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st4cmp1rst(&mut self) -> ST4CMP1RST_W<ST2CNTRST_SPEC, 29> {
        ST4CMP1RST_W::new(self)
    }
    #[doc = "Bit 30 - Slave_TIMER4 compare 3 event resets counter"]
    #[inline(always)]
    #[must_use]
    pub fn st4cmp3rst(&mut self) -> ST4CMP3RST_W<ST2CNTRST_SPEC, 30> {
        ST4CMP3RST_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx counter reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cntrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cntrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST2CNTRST_SPEC;
impl crate::RegisterSpec for ST2CNTRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2cntrst::R`](R) reader structure"]
impl crate::Readable for ST2CNTRST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st2cntrst::W`](W) writer structure"]
impl crate::Writable for ST2CNTRST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST2CNTRST to value 0"]
impl crate::Resettable for ST2CNTRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
