#[doc = "Register `BMSTRG` reader"]
pub type R = crate::R<BMSTRG_SPEC>;
#[doc = "Register `BMSTRG` writer"]
pub type W = crate::W<BMSTRG_SPEC>;
#[doc = "Field `SWTRG` reader - Software triggers bunch mode operation"]
pub type SWTRG_R = crate::BitReader;
#[doc = "Field `SWTRG` writer - Software triggers bunch mode operation"]
pub type SWTRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTRST` reader - Master_TIMER reset event triggers bunch mode operation"]
pub type MTRST_R = crate::BitReader;
#[doc = "Field `MTRST` writer - Master_TIMER reset event triggers bunch mode operation"]
pub type MTRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTREP` reader - Master_TIMER repetition event triggers bunch mode operation"]
pub type MTREP_R = crate::BitReader;
#[doc = "Field `MTREP` writer - Master_TIMER repetition event triggers bunch mode operation"]
pub type MTREP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP0` reader - Master_TIMER compare 0 event triggers bunch mode operation"]
pub type MTCMP0_R = crate::BitReader;
#[doc = "Field `MTCMP0` writer - Master_TIMER compare 0 event triggers bunch mode operation"]
pub type MTCMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP1` reader - Master_TIMER compare 1 event triggers bunch mode operation"]
pub type MTCMP1_R = crate::BitReader;
#[doc = "Field `MTCMP1` writer - Master_TIMER compare 1 event triggers bunch mode operation"]
pub type MTCMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP2` reader - Master_TIMER compare 2 event triggers bunch mode operation"]
pub type MTCMP2_R = crate::BitReader;
#[doc = "Field `MTCMP2` writer - Master_TIMER compare 2 event triggers bunch mode operation"]
pub type MTCMP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTCMP3` reader - Master_TIMER compare 3 event triggers bunch mode operation"]
pub type MTCMP3_R = crate::BitReader;
#[doc = "Field `MTCMP3` writer - Master_TIMER compare 3 event triggers bunch mode operation"]
pub type MTCMP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0RST` reader - Slave_TIMER0 reset event triggers bunch mode operation"]
pub type ST0RST_R = crate::BitReader;
#[doc = "Field `ST0RST` writer - Slave_TIMER0 reset event triggers bunch mode operation"]
pub type ST0RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0REP` reader - Slave_TIMER0 repetition event triggers bunch mode operation"]
pub type ST0REP_R = crate::BitReader;
#[doc = "Field `ST0REP` writer - Slave_TIMER0 repetition event triggers bunch mode operation"]
pub type ST0REP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0CMP0` reader - Slave_TIMER0 compare 0 event triggers bunch mode operation"]
pub type ST0CMP0_R = crate::BitReader;
#[doc = "Field `ST0CMP0` writer - Slave_TIMER0 compare 0 event triggers bunch mode operation"]
pub type ST0CMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0CMP1` reader - Slave_TIMER0 compare 1 event triggers bunch mode operation"]
pub type ST0CMP1_R = crate::BitReader;
#[doc = "Field `ST0CMP1` writer - Slave_TIMER0 compare 1 event triggers bunch mode operation"]
pub type ST0CMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1RST` reader - Slave_TIMER1 reset event triggers bunch mode operation"]
pub type ST1RST_R = crate::BitReader;
#[doc = "Field `ST1RST` writer - Slave_TIMER1 reset event triggers bunch mode operation"]
pub type ST1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1REP` reader - Slave_TIMER1 repetition event triggers bunch mode operation"]
pub type ST1REP_R = crate::BitReader;
#[doc = "Field `ST1REP` writer - Slave_TIMER1 repetition event triggers bunch mode operation"]
pub type ST1REP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CMP0` reader - Slave_TIMER1 compare 0 event triggers bunch mode operation"]
pub type ST1CMP0_R = crate::BitReader;
#[doc = "Field `ST1CMP0` writer - Slave_TIMER1 compare 0 event triggers bunch mode operation"]
pub type ST1CMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST1CMP1` reader - Slave_TIMER1 compare 1 event triggers bunch mode operation"]
pub type ST1CMP1_R = crate::BitReader;
#[doc = "Field `ST1CMP1` writer - Slave_TIMER1 compare 1 event triggers bunch mode operation"]
pub type ST1CMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2RST` reader - Slave_TIMER2 reset event triggers bunch mode operation"]
pub type ST2RST_R = crate::BitReader;
#[doc = "Field `ST2RST` writer - Slave_TIMER2 reset event triggers bunch mode operation"]
pub type ST2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2REP` reader - Slave_TIMER1 repetition event triggers bunch mode operation"]
pub type ST2REP_R = crate::BitReader;
#[doc = "Field `ST2REP` writer - Slave_TIMER1 repetition event triggers bunch mode operation"]
pub type ST2REP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2CMP0` reader - Slave_TIMER2 compare 0 event triggers bunch mode operation"]
pub type ST2CMP0_R = crate::BitReader;
#[doc = "Field `ST2CMP0` writer - Slave_TIMER2 compare 0 event triggers bunch mode operation"]
pub type ST2CMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST2CMP1` reader - Slave_TIMER2 compare 1 event triggers bunch mode operation"]
pub type ST2CMP1_R = crate::BitReader;
#[doc = "Field `ST2CMP1` writer - Slave_TIMER2 compare 1 event triggers bunch mode operation"]
pub type ST2CMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3RST` reader - Slave_TIMER3 reset event triggers bunch mode operation"]
pub type ST3RST_R = crate::BitReader;
#[doc = "Field `ST3RST` writer - Slave_TIMER3 reset event triggers bunch mode operation"]
pub type ST3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3REP` reader - Slave_TIMER3 repetition event triggers bunch mode operation"]
pub type ST3REP_R = crate::BitReader;
#[doc = "Field `ST3REP` writer - Slave_TIMER3 repetition event triggers bunch mode operation"]
pub type ST3REP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CMP0` reader - Slave_TIMER3 compare 0 event triggers bunch mode operation"]
pub type ST3CMP0_R = crate::BitReader;
#[doc = "Field `ST3CMP0` writer - Slave_TIMER3 compare 0 event triggers bunch mode operation"]
pub type ST3CMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3CMP1` reader - Slave_TIMER3 compare 1 event triggers bunch mode operation"]
pub type ST3CMP1_R = crate::BitReader;
#[doc = "Field `ST3CMP1` writer - Slave_TIMER3 compare 1 event triggers bunch mode operation"]
pub type ST3CMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4RST` reader - Slave_TIMER4 reset event triggers bunch mode operation"]
pub type ST4RST_R = crate::BitReader;
#[doc = "Field `ST4RST` writer - Slave_TIMER4 reset event triggers bunch mode operation"]
pub type ST4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4REP` reader - Slave_TIMER4 repetition event triggers bunch mode operation"]
pub type ST4REP_R = crate::BitReader;
#[doc = "Field `ST4REP` writer - Slave_TIMER4 repetition event triggers bunch mode operation"]
pub type ST4REP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CMP0` reader - Slave_TIMER4 compare 0 event triggers bunch mode operation"]
pub type ST4CMP0_R = crate::BitReader;
#[doc = "Field `ST4CMP0` writer - Slave_TIMER4 compare 0 event triggers bunch mode operation"]
pub type ST4CMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST4CMP1` reader - Slave_TIMER4 compare 1 event triggers bunch mode operation"]
pub type ST4CMP1_R = crate::BitReader;
#[doc = "Field `ST4CMP1` writer - Slave_TIMER4 compare 1 event triggers bunch mode operation"]
pub type ST4CMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST0EXEV6` reader - Slave_TIMER0 period event following external event 6 triggers bunch mode operation"]
pub type ST0EXEV6_R = crate::BitReader;
#[doc = "Field `ST0EXEV6` writer - Slave_TIMER0 period event following external event 6 triggers bunch mode operation"]
pub type ST0EXEV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ST3EXEV7` reader - Slave_TIMER3 period event following external event 7 triggers bunch mode operation"]
pub type ST3EXEV7_R = crate::BitReader;
#[doc = "Field `ST3EXEV7` writer - Slave_TIMER3 period event following external event 7 triggers bunch mode operation"]
pub type ST3EXEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV6` reader - External event 6 triggers bunch mode operation"]
pub type EXEV6_R = crate::BitReader;
#[doc = "Field `EXEV6` writer - External event 6 triggers bunch mode operation"]
pub type EXEV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXEV7` reader - External event 7 triggers bunch mode operation"]
pub type EXEV7_R = crate::BitReader;
#[doc = "Field `EXEV7` writer - External event 7 triggers bunch mode operation"]
pub type EXEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CISGN` reader - Chip internal signal triggers bunch mode operation"]
pub type CISGN_R = crate::BitReader;
#[doc = "Field `CISGN` writer - Chip internal signal triggers bunch mode operation"]
pub type CISGN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software triggers bunch mode operation"]
    #[inline(always)]
    pub fn swtrg(&self) -> SWTRG_R {
        SWTRG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master_TIMER reset event triggers bunch mode operation"]
    #[inline(always)]
    pub fn mtrst(&self) -> MTRST_R {
        MTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master_TIMER repetition event triggers bunch mode operation"]
    #[inline(always)]
    pub fn mtrep(&self) -> MTREP_R {
        MTREP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master_TIMER compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn mtcmp0(&self) -> MTCMP0_R {
        MTCMP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master_TIMER compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn mtcmp1(&self) -> MTCMP1_R {
        MTCMP1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master_TIMER compare 2 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn mtcmp2(&self) -> MTCMP2_R {
        MTCMP2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master_TIMER compare 3 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn mtcmp3(&self) -> MTCMP3_R {
        MTCMP3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave_TIMER0 reset event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st0rst(&self) -> ST0RST_R {
        ST0RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave_TIMER0 repetition event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st0rep(&self) -> ST0REP_R {
        ST0REP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave_TIMER0 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st0cmp0(&self) -> ST0CMP0_R {
        ST0CMP0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slave_TIMER0 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st0cmp1(&self) -> ST0CMP1_R {
        ST0CMP1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave_TIMER1 reset event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st1rst(&self) -> ST1RST_R {
        ST1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave_TIMER1 repetition event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st1rep(&self) -> ST1REP_R {
        ST1REP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Slave_TIMER1 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st1cmp0(&self) -> ST1CMP0_R {
        ST1CMP0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slave_TIMER1 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st1cmp1(&self) -> ST1CMP1_R {
        ST1CMP1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave_TIMER2 reset event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st2rst(&self) -> ST2RST_R {
        ST2RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave_TIMER1 repetition event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st2rep(&self) -> ST2REP_R {
        ST2REP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave_TIMER2 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st2cmp0(&self) -> ST2CMP0_R {
        ST2CMP0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave_TIMER2 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st2cmp1(&self) -> ST2CMP1_R {
        ST2CMP1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave_TIMER3 reset event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st3rst(&self) -> ST3RST_R {
        ST3RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slave_TIMER3 repetition event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st3rep(&self) -> ST3REP_R {
        ST3REP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slave_TIMER3 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st3cmp0(&self) -> ST3CMP0_R {
        ST3CMP0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Slave_TIMER3 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st3cmp1(&self) -> ST3CMP1_R {
        ST3CMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave_TIMER4 reset event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st4rst(&self) -> ST4RST_R {
        ST4RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Slave_TIMER4 repetition event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st4rep(&self) -> ST4REP_R {
        ST4REP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Slave_TIMER4 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st4cmp0(&self) -> ST4CMP0_R {
        ST4CMP0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Slave_TIMER4 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    pub fn st4cmp1(&self) -> ST4CMP1_R {
        ST4CMP1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Slave_TIMER0 period event following external event 6 triggers bunch mode operation"]
    #[inline(always)]
    pub fn st0exev6(&self) -> ST0EXEV6_R {
        ST0EXEV6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Slave_TIMER3 period event following external event 7 triggers bunch mode operation"]
    #[inline(always)]
    pub fn st3exev7(&self) -> ST3EXEV7_R {
        ST3EXEV7_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - External event 6 triggers bunch mode operation"]
    #[inline(always)]
    pub fn exev6(&self) -> EXEV6_R {
        EXEV6_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - External event 7 triggers bunch mode operation"]
    #[inline(always)]
    pub fn exev7(&self) -> EXEV7_R {
        EXEV7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Chip internal signal triggers bunch mode operation"]
    #[inline(always)]
    pub fn cisgn(&self) -> CISGN_R {
        CISGN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn swtrg(&mut self) -> SWTRG_W<BMSTRG_SPEC, 0> {
        SWTRG_W::new(self)
    }
    #[doc = "Bit 1 - Master_TIMER reset event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn mtrst(&mut self) -> MTRST_W<BMSTRG_SPEC, 1> {
        MTRST_W::new(self)
    }
    #[doc = "Bit 2 - Master_TIMER repetition event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn mtrep(&mut self) -> MTREP_W<BMSTRG_SPEC, 2> {
        MTREP_W::new(self)
    }
    #[doc = "Bit 3 - Master_TIMER compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp0(&mut self) -> MTCMP0_W<BMSTRG_SPEC, 3> {
        MTCMP0_W::new(self)
    }
    #[doc = "Bit 4 - Master_TIMER compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp1(&mut self) -> MTCMP1_W<BMSTRG_SPEC, 4> {
        MTCMP1_W::new(self)
    }
    #[doc = "Bit 5 - Master_TIMER compare 2 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp2(&mut self) -> MTCMP2_W<BMSTRG_SPEC, 5> {
        MTCMP2_W::new(self)
    }
    #[doc = "Bit 6 - Master_TIMER compare 3 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn mtcmp3(&mut self) -> MTCMP3_W<BMSTRG_SPEC, 6> {
        MTCMP3_W::new(self)
    }
    #[doc = "Bit 7 - Slave_TIMER0 reset event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st0rst(&mut self) -> ST0RST_W<BMSTRG_SPEC, 7> {
        ST0RST_W::new(self)
    }
    #[doc = "Bit 8 - Slave_TIMER0 repetition event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st0rep(&mut self) -> ST0REP_W<BMSTRG_SPEC, 8> {
        ST0REP_W::new(self)
    }
    #[doc = "Bit 9 - Slave_TIMER0 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st0cmp0(&mut self) -> ST0CMP0_W<BMSTRG_SPEC, 9> {
        ST0CMP0_W::new(self)
    }
    #[doc = "Bit 10 - Slave_TIMER0 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st0cmp1(&mut self) -> ST0CMP1_W<BMSTRG_SPEC, 10> {
        ST0CMP1_W::new(self)
    }
    #[doc = "Bit 11 - Slave_TIMER1 reset event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st1rst(&mut self) -> ST1RST_W<BMSTRG_SPEC, 11> {
        ST1RST_W::new(self)
    }
    #[doc = "Bit 12 - Slave_TIMER1 repetition event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st1rep(&mut self) -> ST1REP_W<BMSTRG_SPEC, 12> {
        ST1REP_W::new(self)
    }
    #[doc = "Bit 13 - Slave_TIMER1 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st1cmp0(&mut self) -> ST1CMP0_W<BMSTRG_SPEC, 13> {
        ST1CMP0_W::new(self)
    }
    #[doc = "Bit 14 - Slave_TIMER1 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st1cmp1(&mut self) -> ST1CMP1_W<BMSTRG_SPEC, 14> {
        ST1CMP1_W::new(self)
    }
    #[doc = "Bit 15 - Slave_TIMER2 reset event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st2rst(&mut self) -> ST2RST_W<BMSTRG_SPEC, 15> {
        ST2RST_W::new(self)
    }
    #[doc = "Bit 16 - Slave_TIMER1 repetition event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st2rep(&mut self) -> ST2REP_W<BMSTRG_SPEC, 16> {
        ST2REP_W::new(self)
    }
    #[doc = "Bit 17 - Slave_TIMER2 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st2cmp0(&mut self) -> ST2CMP0_W<BMSTRG_SPEC, 17> {
        ST2CMP0_W::new(self)
    }
    #[doc = "Bit 18 - Slave_TIMER2 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st2cmp1(&mut self) -> ST2CMP1_W<BMSTRG_SPEC, 18> {
        ST2CMP1_W::new(self)
    }
    #[doc = "Bit 19 - Slave_TIMER3 reset event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st3rst(&mut self) -> ST3RST_W<BMSTRG_SPEC, 19> {
        ST3RST_W::new(self)
    }
    #[doc = "Bit 20 - Slave_TIMER3 repetition event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st3rep(&mut self) -> ST3REP_W<BMSTRG_SPEC, 20> {
        ST3REP_W::new(self)
    }
    #[doc = "Bit 21 - Slave_TIMER3 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st3cmp0(&mut self) -> ST3CMP0_W<BMSTRG_SPEC, 21> {
        ST3CMP0_W::new(self)
    }
    #[doc = "Bit 22 - Slave_TIMER3 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st3cmp1(&mut self) -> ST3CMP1_W<BMSTRG_SPEC, 22> {
        ST3CMP1_W::new(self)
    }
    #[doc = "Bit 23 - Slave_TIMER4 reset event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st4rst(&mut self) -> ST4RST_W<BMSTRG_SPEC, 23> {
        ST4RST_W::new(self)
    }
    #[doc = "Bit 24 - Slave_TIMER4 repetition event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st4rep(&mut self) -> ST4REP_W<BMSTRG_SPEC, 24> {
        ST4REP_W::new(self)
    }
    #[doc = "Bit 25 - Slave_TIMER4 compare 0 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st4cmp0(&mut self) -> ST4CMP0_W<BMSTRG_SPEC, 25> {
        ST4CMP0_W::new(self)
    }
    #[doc = "Bit 26 - Slave_TIMER4 compare 1 event triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st4cmp1(&mut self) -> ST4CMP1_W<BMSTRG_SPEC, 26> {
        ST4CMP1_W::new(self)
    }
    #[doc = "Bit 27 - Slave_TIMER0 period event following external event 6 triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st0exev6(&mut self) -> ST0EXEV6_W<BMSTRG_SPEC, 27> {
        ST0EXEV6_W::new(self)
    }
    #[doc = "Bit 28 - Slave_TIMER3 period event following external event 7 triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn st3exev7(&mut self) -> ST3EXEV7_W<BMSTRG_SPEC, 28> {
        ST3EXEV7_W::new(self)
    }
    #[doc = "Bit 29 - External event 6 triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn exev6(&mut self) -> EXEV6_W<BMSTRG_SPEC, 29> {
        EXEV6_W::new(self)
    }
    #[doc = "Bit 30 - External event 7 triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn exev7(&mut self) -> EXEV7_W<BMSTRG_SPEC, 30> {
        EXEV7_W::new(self)
    }
    #[doc = "Bit 31 - Chip internal signal triggers bunch mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn cisgn(&mut self) -> CISGN_W<BMSTRG_SPEC, 31> {
        CISGN_W::new(self)
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
#[doc = "SHRTIMER bunch mode start trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmstrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmstrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMSTRG_SPEC;
impl crate::RegisterSpec for BMSTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmstrg::R`](R) reader structure"]
impl crate::Readable for BMSTRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmstrg::W`](W) writer structure"]
impl crate::Writable for BMSTRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMSTRG to value 0"]
impl crate::Resettable for BMSTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
