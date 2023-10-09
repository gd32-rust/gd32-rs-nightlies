#[doc = "Register `ST0CH1RST` reader"]
pub type R = crate::R<ST0CH1RST_SPEC>;
#[doc = "Register `ST0CH1RST` writer"]
pub type W = crate::W<ST0CH1RST_SPEC>;
#[doc = "Field `CH1RSSEV` reader - Software event generates channel 1"]
pub type CH1RSSEV_R = crate::BitReader;
#[doc = "Field `CH1RSSEV` writer - Software event generates channel 1"]
pub type CH1RSSEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSRST` reader - Slave_TIMER0 reset event generates channel 1"]
pub type CH1RSRST_R = crate::BitReader;
#[doc = "Field `CH1RSRST` writer - Slave_TIMER0 reset event generates channel 1"]
pub type CH1RSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSPER` reader - Slave_TIMER0 period event generates channel 1"]
pub type CH1RSPER_R = crate::BitReader;
#[doc = "Field `CH1RSPER` writer - Slave_TIMER0 period event generates channel 1"]
pub type CH1RSPER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSCMP0` reader - Slave_TIMER0 compare 0 event generates channel 1"]
pub type CH1RSCMP0_R = crate::BitReader;
#[doc = "Field `CH1RSCMP0` writer - Slave_TIMER0 compare 0 event generates channel 1"]
pub type CH1RSCMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSCMP1` reader - Slave_TIMER0 compare 1 event generates channel 1"]
pub type CH1RSCMP1_R = crate::BitReader;
#[doc = "Field `CH1RSCMP1` writer - Slave_TIMER0 compare 1 event generates channel 1"]
pub type CH1RSCMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSCMP2` reader - Slave_TIMER0 compare 2 event generates channel 1"]
pub type CH1RSCMP2_R = crate::BitReader;
#[doc = "Field `CH1RSCMP2` writer - Slave_TIMER0 compare 2 event generates channel 1"]
pub type CH1RSCMP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSCMP3` reader - Slave_TIMER0 compare 3 event generates channel 1"]
pub type CH1RSCMP3_R = crate::BitReader;
#[doc = "Field `CH1RSCMP3` writer - Slave_TIMER0 compare 3 event generates channel 1"]
pub type CH1RSCMP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSMTPER` reader - Master_TIMER period event generates channel 1"]
pub type CH1RSMTPER_R = crate::BitReader;
#[doc = "Field `CH1RSMTPER` writer - Master_TIMER period event generates channel 1"]
pub type CH1RSMTPER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSMTCMP0` reader - Master_TIMER compare 0 event generates channel 1"]
pub type CH1RSMTCMP0_R = crate::BitReader;
#[doc = "Field `CH1RSMTCMP0` writer - Master_TIMER compare 0 event generates channel 1"]
pub type CH1RSMTCMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSMTCMP1` reader - Master_TIMER compare 1 event generates channel 1"]
pub type CH1RSMTCMP1_R = crate::BitReader;
#[doc = "Field `CH1RSMTCMP1` writer - Master_TIMER compare 1 event generates channel 1"]
pub type CH1RSMTCMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSMTCMP2` reader - Master_TIMER compare 2 event generates channel 1"]
pub type CH1RSMTCMP2_R = crate::BitReader;
#[doc = "Field `CH1RSMTCMP2` writer - Master_TIMER compare 2 event generates channel 1"]
pub type CH1RSMTCMP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSMTCMP3` reader - Master_TIMER compare 3 event generates channel 1"]
pub type CH1RSMTCMP3_R = crate::BitReader;
#[doc = "Field `CH1RSMTCMP3` writer - Master_TIMER compare 3 event generates channel 1"]
pub type CH1RSMTCMP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV0` reader - Slave_TIMER0 interconnection event 0 generates channel 1"]
pub type CH1RSSTEV0_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV0` writer - Slave_TIMER0 interconnection event 0 generates channel 1"]
pub type CH1RSSTEV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV1` reader - Slave_TIMER0 interconnection event 1 generates channel 1"]
pub type CH1RSSTEV1_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV1` writer - Slave_TIMER0 interconnection event 1 generates channel 1"]
pub type CH1RSSTEV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV2` reader - Slave_TIMER0 interconnection event 2 generates channel 1"]
pub type CH1RSSTEV2_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV2` writer - Slave_TIMER0 interconnection event 2 generates channel 1"]
pub type CH1RSSTEV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV3` reader - Slave_TIMER0 interconnection event 3 generates channel 1"]
pub type CH1RSSTEV3_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV3` writer - Slave_TIMER0 interconnection event 3 generates channel 1"]
pub type CH1RSSTEV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV4` reader - Slave_TIMER0 interconnection event 4 generates channel 1"]
pub type CH1RSSTEV4_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV4` writer - Slave_TIMER0 interconnection event 4 generates channel 1"]
pub type CH1RSSTEV4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV5` reader - Slave_TIMER0 interconnection event 5 generates channel 1"]
pub type CH1RSSTEV5_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV5` writer - Slave_TIMER0 interconnection event 5 generates channel 1"]
pub type CH1RSSTEV5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV6` reader - Slave_TIMER0 interconnection event 6 generates channel 1"]
pub type CH1RSSTEV6_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV6` writer - Slave_TIMER0 interconnection event 6 generates channel 1"]
pub type CH1RSSTEV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV7` reader - Slave_TIMER0 interconnection event 7 generates channel 1"]
pub type CH1RSSTEV7_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV7` writer - Slave_TIMER0 interconnection event 7 generates channel 1"]
pub type CH1RSSTEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSSTEV8` reader - Slave_TIMER0 interconnection event 8 generates channel 1"]
pub type CH1RSSTEV8_R = crate::BitReader;
#[doc = "Field `CH1RSSTEV8` writer - Slave_TIMER0 interconnection event 8 generates channel 1"]
pub type CH1RSSTEV8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV0` reader - External event 0 generates channel 1"]
pub type CH1RSEXEV0_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV0` writer - External event 0 generates channel 1"]
pub type CH1RSEXEV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV1` reader - External event 1 generates channel 1"]
pub type CH1RSEXEV1_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV1` writer - External event 1 generates channel 1"]
pub type CH1RSEXEV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV2` reader - External event 2 generates channel 1"]
pub type CH1RSEXEV2_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV2` writer - External event 2 generates channel 1"]
pub type CH1RSEXEV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV3` reader - External event 3 generates channel 1"]
pub type CH1RSEXEV3_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV3` writer - External event 3 generates channel 1"]
pub type CH1RSEXEV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV4` reader - External event 4 generates channel 1"]
pub type CH1RSEXEV4_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV4` writer - External event 4 generates channel 1"]
pub type CH1RSEXEV4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV5` reader - External event 5 generates channel 1"]
pub type CH1RSEXEV5_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV5` writer - External event 5 generates channel 1"]
pub type CH1RSEXEV5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV6` reader - External event 6 generates channel 1"]
pub type CH1RSEXEV6_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV6` writer - External event 6 generates channel 1"]
pub type CH1RSEXEV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV7` reader - External event 7 generates channel 1"]
pub type CH1RSEXEV7_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV7` writer - External event 7 generates channel 1"]
pub type CH1RSEXEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV8` reader - External event 8 generates channel 1"]
pub type CH1RSEXEV8_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV8` writer - External event 8 generates channel 1"]
pub type CH1RSEXEV8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSEXEV9` reader - External event 9 generates channel 1"]
pub type CH1RSEXEV9_R = crate::BitReader;
#[doc = "Field `CH1RSEXEV9` writer - External event 9 generates channel 1"]
pub type CH1RSEXEV9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1RSUP` reader - Update event generates channel 1"]
pub type CH1RSUP_R = crate::BitReader;
#[doc = "Field `CH1RSUP` writer - Update event generates channel 1"]
pub type CH1RSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software event generates channel 1"]
    #[inline(always)]
    pub fn ch1rssev(&self) -> CH1RSSEV_R {
        CH1RSSEV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave_TIMER0 reset event generates channel 1"]
    #[inline(always)]
    pub fn ch1rsrst(&self) -> CH1RSRST_R {
        CH1RSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave_TIMER0 period event generates channel 1"]
    #[inline(always)]
    pub fn ch1rsper(&self) -> CH1RSPER_R {
        CH1RSPER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave_TIMER0 compare 0 event generates channel 1"]
    #[inline(always)]
    pub fn ch1rscmp0(&self) -> CH1RSCMP0_R {
        CH1RSCMP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave_TIMER0 compare 1 event generates channel 1"]
    #[inline(always)]
    pub fn ch1rscmp1(&self) -> CH1RSCMP1_R {
        CH1RSCMP1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave_TIMER0 compare 2 event generates channel 1"]
    #[inline(always)]
    pub fn ch1rscmp2(&self) -> CH1RSCMP2_R {
        CH1RSCMP2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave_TIMER0 compare 3 event generates channel 1"]
    #[inline(always)]
    pub fn ch1rscmp3(&self) -> CH1RSCMP3_R {
        CH1RSCMP3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master_TIMER period event generates channel 1"]
    #[inline(always)]
    pub fn ch1rsmtper(&self) -> CH1RSMTPER_R {
        CH1RSMTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master_TIMER compare 0 event generates channel 1"]
    #[inline(always)]
    pub fn ch1rsmtcmp0(&self) -> CH1RSMTCMP0_R {
        CH1RSMTCMP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master_TIMER compare 1 event generates channel 1"]
    #[inline(always)]
    pub fn ch1rsmtcmp1(&self) -> CH1RSMTCMP1_R {
        CH1RSMTCMP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master_TIMER compare 2 event generates channel 1"]
    #[inline(always)]
    pub fn ch1rsmtcmp2(&self) -> CH1RSMTCMP2_R {
        CH1RSMTCMP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master_TIMER compare 3 event generates channel 1"]
    #[inline(always)]
    pub fn ch1rsmtcmp3(&self) -> CH1RSMTCMP3_R {
        CH1RSMTCMP3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave_TIMER0 interconnection event 0 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev0(&self) -> CH1RSSTEV0_R {
        CH1RSSTEV0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Slave_TIMER0 interconnection event 1 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev1(&self) -> CH1RSSTEV1_R {
        CH1RSSTEV1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slave_TIMER0 interconnection event 2 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev2(&self) -> CH1RSSTEV2_R {
        CH1RSSTEV2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave_TIMER0 interconnection event 3 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev3(&self) -> CH1RSSTEV3_R {
        CH1RSSTEV3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave_TIMER0 interconnection event 4 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev4(&self) -> CH1RSSTEV4_R {
        CH1RSSTEV4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave_TIMER0 interconnection event 5 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev5(&self) -> CH1RSSTEV5_R {
        CH1RSSTEV5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave_TIMER0 interconnection event 6 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev6(&self) -> CH1RSSTEV6_R {
        CH1RSSTEV6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave_TIMER0 interconnection event 7 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev7(&self) -> CH1RSSTEV7_R {
        CH1RSSTEV7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slave_TIMER0 interconnection event 8 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsstev8(&self) -> CH1RSSTEV8_R {
        CH1RSSTEV8_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - External event 0 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev0(&self) -> CH1RSEXEV0_R {
        CH1RSEXEV0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - External event 1 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev1(&self) -> CH1RSEXEV1_R {
        CH1RSEXEV1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - External event 2 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev2(&self) -> CH1RSEXEV2_R {
        CH1RSEXEV2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - External event 3 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev3(&self) -> CH1RSEXEV3_R {
        CH1RSEXEV3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - External event 4 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev4(&self) -> CH1RSEXEV4_R {
        CH1RSEXEV4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External event 5 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev5(&self) -> CH1RSEXEV5_R {
        CH1RSEXEV5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - External event 6 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev6(&self) -> CH1RSEXEV6_R {
        CH1RSEXEV6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - External event 7 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev7(&self) -> CH1RSEXEV7_R {
        CH1RSEXEV7_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - External event 8 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev8(&self) -> CH1RSEXEV8_R {
        CH1RSEXEV8_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - External event 9 generates channel 1"]
    #[inline(always)]
    pub fn ch1rsexev9(&self) -> CH1RSEXEV9_R {
        CH1RSEXEV9_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update event generates channel 1"]
    #[inline(always)]
    pub fn ch1rsup(&self) -> CH1RSUP_R {
        CH1RSUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rssev(&mut self) -> CH1RSSEV_W<ST0CH1RST_SPEC, 0> {
        CH1RSSEV_W::new(self)
    }
    #[doc = "Bit 1 - Slave_TIMER0 reset event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsrst(&mut self) -> CH1RSRST_W<ST0CH1RST_SPEC, 1> {
        CH1RSRST_W::new(self)
    }
    #[doc = "Bit 2 - Slave_TIMER0 period event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsper(&mut self) -> CH1RSPER_W<ST0CH1RST_SPEC, 2> {
        CH1RSPER_W::new(self)
    }
    #[doc = "Bit 3 - Slave_TIMER0 compare 0 event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rscmp0(&mut self) -> CH1RSCMP0_W<ST0CH1RST_SPEC, 3> {
        CH1RSCMP0_W::new(self)
    }
    #[doc = "Bit 4 - Slave_TIMER0 compare 1 event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rscmp1(&mut self) -> CH1RSCMP1_W<ST0CH1RST_SPEC, 4> {
        CH1RSCMP1_W::new(self)
    }
    #[doc = "Bit 5 - Slave_TIMER0 compare 2 event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rscmp2(&mut self) -> CH1RSCMP2_W<ST0CH1RST_SPEC, 5> {
        CH1RSCMP2_W::new(self)
    }
    #[doc = "Bit 6 - Slave_TIMER0 compare 3 event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rscmp3(&mut self) -> CH1RSCMP3_W<ST0CH1RST_SPEC, 6> {
        CH1RSCMP3_W::new(self)
    }
    #[doc = "Bit 7 - Master_TIMER period event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsmtper(&mut self) -> CH1RSMTPER_W<ST0CH1RST_SPEC, 7> {
        CH1RSMTPER_W::new(self)
    }
    #[doc = "Bit 8 - Master_TIMER compare 0 event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsmtcmp0(&mut self) -> CH1RSMTCMP0_W<ST0CH1RST_SPEC, 8> {
        CH1RSMTCMP0_W::new(self)
    }
    #[doc = "Bit 9 - Master_TIMER compare 1 event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsmtcmp1(&mut self) -> CH1RSMTCMP1_W<ST0CH1RST_SPEC, 9> {
        CH1RSMTCMP1_W::new(self)
    }
    #[doc = "Bit 10 - Master_TIMER compare 2 event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsmtcmp2(&mut self) -> CH1RSMTCMP2_W<ST0CH1RST_SPEC, 10> {
        CH1RSMTCMP2_W::new(self)
    }
    #[doc = "Bit 11 - Master_TIMER compare 3 event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsmtcmp3(&mut self) -> CH1RSMTCMP3_W<ST0CH1RST_SPEC, 11> {
        CH1RSMTCMP3_W::new(self)
    }
    #[doc = "Bit 12 - Slave_TIMER0 interconnection event 0 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev0(&mut self) -> CH1RSSTEV0_W<ST0CH1RST_SPEC, 12> {
        CH1RSSTEV0_W::new(self)
    }
    #[doc = "Bit 13 - Slave_TIMER0 interconnection event 1 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev1(&mut self) -> CH1RSSTEV1_W<ST0CH1RST_SPEC, 13> {
        CH1RSSTEV1_W::new(self)
    }
    #[doc = "Bit 14 - Slave_TIMER0 interconnection event 2 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev2(&mut self) -> CH1RSSTEV2_W<ST0CH1RST_SPEC, 14> {
        CH1RSSTEV2_W::new(self)
    }
    #[doc = "Bit 15 - Slave_TIMER0 interconnection event 3 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev3(&mut self) -> CH1RSSTEV3_W<ST0CH1RST_SPEC, 15> {
        CH1RSSTEV3_W::new(self)
    }
    #[doc = "Bit 16 - Slave_TIMER0 interconnection event 4 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev4(&mut self) -> CH1RSSTEV4_W<ST0CH1RST_SPEC, 16> {
        CH1RSSTEV4_W::new(self)
    }
    #[doc = "Bit 17 - Slave_TIMER0 interconnection event 5 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev5(&mut self) -> CH1RSSTEV5_W<ST0CH1RST_SPEC, 17> {
        CH1RSSTEV5_W::new(self)
    }
    #[doc = "Bit 18 - Slave_TIMER0 interconnection event 6 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev6(&mut self) -> CH1RSSTEV6_W<ST0CH1RST_SPEC, 18> {
        CH1RSSTEV6_W::new(self)
    }
    #[doc = "Bit 19 - Slave_TIMER0 interconnection event 7 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev7(&mut self) -> CH1RSSTEV7_W<ST0CH1RST_SPEC, 19> {
        CH1RSSTEV7_W::new(self)
    }
    #[doc = "Bit 20 - Slave_TIMER0 interconnection event 8 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsstev8(&mut self) -> CH1RSSTEV8_W<ST0CH1RST_SPEC, 20> {
        CH1RSSTEV8_W::new(self)
    }
    #[doc = "Bit 21 - External event 0 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev0(&mut self) -> CH1RSEXEV0_W<ST0CH1RST_SPEC, 21> {
        CH1RSEXEV0_W::new(self)
    }
    #[doc = "Bit 22 - External event 1 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev1(&mut self) -> CH1RSEXEV1_W<ST0CH1RST_SPEC, 22> {
        CH1RSEXEV1_W::new(self)
    }
    #[doc = "Bit 23 - External event 2 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev2(&mut self) -> CH1RSEXEV2_W<ST0CH1RST_SPEC, 23> {
        CH1RSEXEV2_W::new(self)
    }
    #[doc = "Bit 24 - External event 3 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev3(&mut self) -> CH1RSEXEV3_W<ST0CH1RST_SPEC, 24> {
        CH1RSEXEV3_W::new(self)
    }
    #[doc = "Bit 25 - External event 4 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev4(&mut self) -> CH1RSEXEV4_W<ST0CH1RST_SPEC, 25> {
        CH1RSEXEV4_W::new(self)
    }
    #[doc = "Bit 26 - External event 5 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev5(&mut self) -> CH1RSEXEV5_W<ST0CH1RST_SPEC, 26> {
        CH1RSEXEV5_W::new(self)
    }
    #[doc = "Bit 27 - External event 6 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev6(&mut self) -> CH1RSEXEV6_W<ST0CH1RST_SPEC, 27> {
        CH1RSEXEV6_W::new(self)
    }
    #[doc = "Bit 28 - External event 7 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev7(&mut self) -> CH1RSEXEV7_W<ST0CH1RST_SPEC, 28> {
        CH1RSEXEV7_W::new(self)
    }
    #[doc = "Bit 29 - External event 8 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev8(&mut self) -> CH1RSEXEV8_W<ST0CH1RST_SPEC, 29> {
        CH1RSEXEV8_W::new(self)
    }
    #[doc = "Bit 30 - External event 9 generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsexev9(&mut self) -> CH1RSEXEV9_W<ST0CH1RST_SPEC, 30> {
        CH1RSEXEV9_W::new(self)
    }
    #[doc = "Bit 31 - Update event generates channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rsup(&mut self) -> CH1RSUP_W<ST0CH1RST_SPEC, 31> {
        CH1RSUP_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER0 channel 1 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0ch1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0ch1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST0CH1RST_SPEC;
impl crate::RegisterSpec for ST0CH1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0ch1rst::R`](R) reader structure"]
impl crate::Readable for ST0CH1RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st0ch1rst::W`](W) writer structure"]
impl crate::Writable for ST0CH1RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST0CH1RST to value 0"]
impl crate::Resettable for ST0CH1RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
