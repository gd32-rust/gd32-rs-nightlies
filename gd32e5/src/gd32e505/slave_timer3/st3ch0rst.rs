#[doc = "Register `ST3CH0RST` reader"]
pub type R = crate::R<ST3CH0RST_SPEC>;
#[doc = "Register `ST3CH0RST` writer"]
pub type W = crate::W<ST3CH0RST_SPEC>;
#[doc = "Field `CH0RSSEV` reader - Software event generates channel 0"]
pub type CH0RSSEV_R = crate::BitReader;
#[doc = "Field `CH0RSSEV` writer - Software event generates channel 0"]
pub type CH0RSSEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSRST` reader - Slave_TIMERx reset event generates channel 0"]
pub type CH0RSRST_R = crate::BitReader;
#[doc = "Field `CH0RSRST` writer - Slave_TIMERx reset event generates channel 0"]
pub type CH0RSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSPER` reader - Slave_TIMERx period event generates channel 0"]
pub type CH0RSPER_R = crate::BitReader;
#[doc = "Field `CH0RSPER` writer - Slave_TIMERx period event generates channel 0"]
pub type CH0RSPER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSCMP0` reader - Slave_TIMERx compare 0 event generates channel 0"]
pub type CH0RSCMP0_R = crate::BitReader;
#[doc = "Field `CH0RSCMP0` writer - Slave_TIMERx compare 0 event generates channel 0"]
pub type CH0RSCMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSCMP1` reader - Slave_TIMERx compare 1 event generates channel 0"]
pub type CH0RSCMP1_R = crate::BitReader;
#[doc = "Field `CH0RSCMP1` writer - Slave_TIMERx compare 1 event generates channel 0"]
pub type CH0RSCMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSCMP2` reader - Slave_TIMER1 compare 2 event generates channel 0"]
pub type CH0RSCMP2_R = crate::BitReader;
#[doc = "Field `CH0RSCMP2` writer - Slave_TIMER1 compare 2 event generates channel 0"]
pub type CH0RSCMP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSCMP3` reader - Slave_TIMERx compare 3 event generates channel 0"]
pub type CH0RSCMP3_R = crate::BitReader;
#[doc = "Field `CH0RSCMP3` writer - Slave_TIMERx compare 3 event generates channel 0"]
pub type CH0RSCMP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSMTPER` reader - Master_TIMER period event generates channel 0"]
pub type CH0RSMTPER_R = crate::BitReader;
#[doc = "Field `CH0RSMTPER` writer - Master_TIMER period event generates channel 0"]
pub type CH0RSMTPER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSMTCMP0` reader - Master_TIMER compare 0 event generates channel 0"]
pub type CH0RSMTCMP0_R = crate::BitReader;
#[doc = "Field `CH0RSMTCMP0` writer - Master_TIMER compare 0 event generates channel 0"]
pub type CH0RSMTCMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSMTCMP1` reader - Master_TIMER compare 1 event generates channel 0"]
pub type CH0RSMTCMP1_R = crate::BitReader;
#[doc = "Field `CH0RSMTCMP1` writer - Master_TIMER compare 1 event generates channel 0"]
pub type CH0RSMTCMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSMTCMP2` reader - Master_TIMER compare 2 event generates channel 0"]
pub type CH0RSMTCMP2_R = crate::BitReader;
#[doc = "Field `CH0RSMTCMP2` writer - Master_TIMER compare 2 event generates channel 0"]
pub type CH0RSMTCMP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSMTCMP3` reader - Master_TIMER compare 3 event generates channel 0"]
pub type CH0RSMTCMP3_R = crate::BitReader;
#[doc = "Field `CH0RSMTCMP3` writer - Master_TIMER compare 3 event generates channel 0"]
pub type CH0RSMTCMP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV0` reader - Slave_TIMER1 interconnection event 0 generates channel 0"]
pub type CH0RSSTEV0_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV0` writer - Slave_TIMER1 interconnection event 0 generates channel 0"]
pub type CH0RSSTEV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV1` reader - Slave_TIMERx interconnection event 1 generates channel 0"]
pub type CH0RSSTEV1_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV1` writer - Slave_TIMERx interconnection event 1 generates channel 0"]
pub type CH0RSSTEV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV2` reader - Slave_TIMERx interconnection event 2 generates channel 0"]
pub type CH0RSSTEV2_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV2` writer - Slave_TIMERx interconnection event 2 generates channel 0"]
pub type CH0RSSTEV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV3` reader - Slave_TIMERx interconnection event 3 generates channel 0"]
pub type CH0RSSTEV3_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV3` writer - Slave_TIMERx interconnection event 3 generates channel 0"]
pub type CH0RSSTEV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV4` reader - Slave_TIMER1 interconnection event 4 generates channel 0"]
pub type CH0RSSTEV4_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV4` writer - Slave_TIMER1 interconnection event 4 generates channel 0"]
pub type CH0RSSTEV4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV5` reader - Slave_TIMERx interconnection event 5 generates channel 0"]
pub type CH0RSSTEV5_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV5` writer - Slave_TIMERx interconnection event 5 generates channel 0"]
pub type CH0RSSTEV5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV6` reader - Slave_TIMERx interconnection event 6 generates channel 0"]
pub type CH0RSSTEV6_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV6` writer - Slave_TIMERx interconnection event 6 generates channel 0"]
pub type CH0RSSTEV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV7` reader - Slave_TIMERx interconnection event 7 generates channel 0"]
pub type CH0RSSTEV7_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV7` writer - Slave_TIMERx interconnection event 7 generates channel 0"]
pub type CH0RSSTEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSSTEV8` reader - Slave_TIMERx interconnection event 8 generates channel 0"]
pub type CH0RSSTEV8_R = crate::BitReader;
#[doc = "Field `CH0RSSTEV8` writer - Slave_TIMERx interconnection event 8 generates channel 0"]
pub type CH0RSSTEV8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV0` reader - External event 0 generates channel 0"]
pub type CH0RSEXEV0_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV0` writer - External event 0 generates channel 0"]
pub type CH0RSEXEV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV1` reader - External event 1 generates channel 0"]
pub type CH0RSEXEV1_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV1` writer - External event 1 generates channel 0"]
pub type CH0RSEXEV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV2` reader - External event 2 generates channel 0"]
pub type CH0RSEXEV2_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV2` writer - External event 2 generates channel 0"]
pub type CH0RSEXEV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV3` reader - External event 3 generates channel 0"]
pub type CH0RSEXEV3_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV3` writer - External event 3 generates channel 0"]
pub type CH0RSEXEV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV4` reader - External event 4 generates channel 0"]
pub type CH0RSEXEV4_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV4` writer - External event 4 generates channel 0"]
pub type CH0RSEXEV4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV5` reader - External event 5 generates channel 0"]
pub type CH0RSEXEV5_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV5` writer - External event 5 generates channel 0"]
pub type CH0RSEXEV5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV6` reader - External event 6 generates channel 0"]
pub type CH0RSEXEV6_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV6` writer - External event 6 generates channel 0"]
pub type CH0RSEXEV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV7` reader - External event 7 generates channel 0"]
pub type CH0RSEXEV7_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV7` writer - External event 7 generates channel 0"]
pub type CH0RSEXEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV8` reader - External event 8 generates channel 0"]
pub type CH0RSEXEV8_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV8` writer - External event 8 generates channel 0"]
pub type CH0RSEXEV8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSEXEV9` reader - External event 9 generates channel 0"]
pub type CH0RSEXEV9_R = crate::BitReader;
#[doc = "Field `CH0RSEXEV9` writer - External event 9 generates channel 0"]
pub type CH0RSEXEV9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0RSUP` reader - Update event generates channel 0"]
pub type CH0RSUP_R = crate::BitReader;
#[doc = "Field `CH0RSUP` writer - Update event generates channel 0"]
pub type CH0RSUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software event generates channel 0"]
    #[inline(always)]
    pub fn ch0rssev(&self) -> CH0RSSEV_R {
        CH0RSSEV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave_TIMERx reset event generates channel 0"]
    #[inline(always)]
    pub fn ch0rsrst(&self) -> CH0RSRST_R {
        CH0RSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave_TIMERx period event generates channel 0"]
    #[inline(always)]
    pub fn ch0rsper(&self) -> CH0RSPER_R {
        CH0RSPER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave_TIMERx compare 0 event generates channel 0"]
    #[inline(always)]
    pub fn ch0rscmp0(&self) -> CH0RSCMP0_R {
        CH0RSCMP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave_TIMERx compare 1 event generates channel 0"]
    #[inline(always)]
    pub fn ch0rscmp1(&self) -> CH0RSCMP1_R {
        CH0RSCMP1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave_TIMER1 compare 2 event generates channel 0"]
    #[inline(always)]
    pub fn ch0rscmp2(&self) -> CH0RSCMP2_R {
        CH0RSCMP2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave_TIMERx compare 3 event generates channel 0"]
    #[inline(always)]
    pub fn ch0rscmp3(&self) -> CH0RSCMP3_R {
        CH0RSCMP3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master_TIMER period event generates channel 0"]
    #[inline(always)]
    pub fn ch0rsmtper(&self) -> CH0RSMTPER_R {
        CH0RSMTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master_TIMER compare 0 event generates channel 0"]
    #[inline(always)]
    pub fn ch0rsmtcmp0(&self) -> CH0RSMTCMP0_R {
        CH0RSMTCMP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master_TIMER compare 1 event generates channel 0"]
    #[inline(always)]
    pub fn ch0rsmtcmp1(&self) -> CH0RSMTCMP1_R {
        CH0RSMTCMP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master_TIMER compare 2 event generates channel 0"]
    #[inline(always)]
    pub fn ch0rsmtcmp2(&self) -> CH0RSMTCMP2_R {
        CH0RSMTCMP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master_TIMER compare 3 event generates channel 0"]
    #[inline(always)]
    pub fn ch0rsmtcmp3(&self) -> CH0RSMTCMP3_R {
        CH0RSMTCMP3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave_TIMER1 interconnection event 0 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev0(&self) -> CH0RSSTEV0_R {
        CH0RSSTEV0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Slave_TIMERx interconnection event 1 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev1(&self) -> CH0RSSTEV1_R {
        CH0RSSTEV1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slave_TIMERx interconnection event 2 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev2(&self) -> CH0RSSTEV2_R {
        CH0RSSTEV2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave_TIMERx interconnection event 3 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev3(&self) -> CH0RSSTEV3_R {
        CH0RSSTEV3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave_TIMER1 interconnection event 4 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev4(&self) -> CH0RSSTEV4_R {
        CH0RSSTEV4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave_TIMERx interconnection event 5 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev5(&self) -> CH0RSSTEV5_R {
        CH0RSSTEV5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave_TIMERx interconnection event 6 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev6(&self) -> CH0RSSTEV6_R {
        CH0RSSTEV6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave_TIMERx interconnection event 7 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev7(&self) -> CH0RSSTEV7_R {
        CH0RSSTEV7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Slave_TIMERx interconnection event 8 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsstev8(&self) -> CH0RSSTEV8_R {
        CH0RSSTEV8_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - External event 0 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev0(&self) -> CH0RSEXEV0_R {
        CH0RSEXEV0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - External event 1 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev1(&self) -> CH0RSEXEV1_R {
        CH0RSEXEV1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - External event 2 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev2(&self) -> CH0RSEXEV2_R {
        CH0RSEXEV2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - External event 3 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev3(&self) -> CH0RSEXEV3_R {
        CH0RSEXEV3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - External event 4 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev4(&self) -> CH0RSEXEV4_R {
        CH0RSEXEV4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External event 5 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev5(&self) -> CH0RSEXEV5_R {
        CH0RSEXEV5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - External event 6 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev6(&self) -> CH0RSEXEV6_R {
        CH0RSEXEV6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - External event 7 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev7(&self) -> CH0RSEXEV7_R {
        CH0RSEXEV7_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - External event 8 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev8(&self) -> CH0RSEXEV8_R {
        CH0RSEXEV8_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - External event 9 generates channel 0"]
    #[inline(always)]
    pub fn ch0rsexev9(&self) -> CH0RSEXEV9_R {
        CH0RSEXEV9_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update event generates channel 0"]
    #[inline(always)]
    pub fn ch0rsup(&self) -> CH0RSUP_R {
        CH0RSUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rssev(&mut self) -> CH0RSSEV_W<ST3CH0RST_SPEC, 0> {
        CH0RSSEV_W::new(self)
    }
    #[doc = "Bit 1 - Slave_TIMERx reset event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsrst(&mut self) -> CH0RSRST_W<ST3CH0RST_SPEC, 1> {
        CH0RSRST_W::new(self)
    }
    #[doc = "Bit 2 - Slave_TIMERx period event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsper(&mut self) -> CH0RSPER_W<ST3CH0RST_SPEC, 2> {
        CH0RSPER_W::new(self)
    }
    #[doc = "Bit 3 - Slave_TIMERx compare 0 event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rscmp0(&mut self) -> CH0RSCMP0_W<ST3CH0RST_SPEC, 3> {
        CH0RSCMP0_W::new(self)
    }
    #[doc = "Bit 4 - Slave_TIMERx compare 1 event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rscmp1(&mut self) -> CH0RSCMP1_W<ST3CH0RST_SPEC, 4> {
        CH0RSCMP1_W::new(self)
    }
    #[doc = "Bit 5 - Slave_TIMER1 compare 2 event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rscmp2(&mut self) -> CH0RSCMP2_W<ST3CH0RST_SPEC, 5> {
        CH0RSCMP2_W::new(self)
    }
    #[doc = "Bit 6 - Slave_TIMERx compare 3 event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rscmp3(&mut self) -> CH0RSCMP3_W<ST3CH0RST_SPEC, 6> {
        CH0RSCMP3_W::new(self)
    }
    #[doc = "Bit 7 - Master_TIMER period event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsmtper(&mut self) -> CH0RSMTPER_W<ST3CH0RST_SPEC, 7> {
        CH0RSMTPER_W::new(self)
    }
    #[doc = "Bit 8 - Master_TIMER compare 0 event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsmtcmp0(&mut self) -> CH0RSMTCMP0_W<ST3CH0RST_SPEC, 8> {
        CH0RSMTCMP0_W::new(self)
    }
    #[doc = "Bit 9 - Master_TIMER compare 1 event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsmtcmp1(&mut self) -> CH0RSMTCMP1_W<ST3CH0RST_SPEC, 9> {
        CH0RSMTCMP1_W::new(self)
    }
    #[doc = "Bit 10 - Master_TIMER compare 2 event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsmtcmp2(&mut self) -> CH0RSMTCMP2_W<ST3CH0RST_SPEC, 10> {
        CH0RSMTCMP2_W::new(self)
    }
    #[doc = "Bit 11 - Master_TIMER compare 3 event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsmtcmp3(&mut self) -> CH0RSMTCMP3_W<ST3CH0RST_SPEC, 11> {
        CH0RSMTCMP3_W::new(self)
    }
    #[doc = "Bit 12 - Slave_TIMER1 interconnection event 0 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev0(&mut self) -> CH0RSSTEV0_W<ST3CH0RST_SPEC, 12> {
        CH0RSSTEV0_W::new(self)
    }
    #[doc = "Bit 13 - Slave_TIMERx interconnection event 1 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev1(&mut self) -> CH0RSSTEV1_W<ST3CH0RST_SPEC, 13> {
        CH0RSSTEV1_W::new(self)
    }
    #[doc = "Bit 14 - Slave_TIMERx interconnection event 2 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev2(&mut self) -> CH0RSSTEV2_W<ST3CH0RST_SPEC, 14> {
        CH0RSSTEV2_W::new(self)
    }
    #[doc = "Bit 15 - Slave_TIMERx interconnection event 3 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev3(&mut self) -> CH0RSSTEV3_W<ST3CH0RST_SPEC, 15> {
        CH0RSSTEV3_W::new(self)
    }
    #[doc = "Bit 16 - Slave_TIMER1 interconnection event 4 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev4(&mut self) -> CH0RSSTEV4_W<ST3CH0RST_SPEC, 16> {
        CH0RSSTEV4_W::new(self)
    }
    #[doc = "Bit 17 - Slave_TIMERx interconnection event 5 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev5(&mut self) -> CH0RSSTEV5_W<ST3CH0RST_SPEC, 17> {
        CH0RSSTEV5_W::new(self)
    }
    #[doc = "Bit 18 - Slave_TIMERx interconnection event 6 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev6(&mut self) -> CH0RSSTEV6_W<ST3CH0RST_SPEC, 18> {
        CH0RSSTEV6_W::new(self)
    }
    #[doc = "Bit 19 - Slave_TIMERx interconnection event 7 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev7(&mut self) -> CH0RSSTEV7_W<ST3CH0RST_SPEC, 19> {
        CH0RSSTEV7_W::new(self)
    }
    #[doc = "Bit 20 - Slave_TIMERx interconnection event 8 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsstev8(&mut self) -> CH0RSSTEV8_W<ST3CH0RST_SPEC, 20> {
        CH0RSSTEV8_W::new(self)
    }
    #[doc = "Bit 21 - External event 0 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev0(&mut self) -> CH0RSEXEV0_W<ST3CH0RST_SPEC, 21> {
        CH0RSEXEV0_W::new(self)
    }
    #[doc = "Bit 22 - External event 1 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev1(&mut self) -> CH0RSEXEV1_W<ST3CH0RST_SPEC, 22> {
        CH0RSEXEV1_W::new(self)
    }
    #[doc = "Bit 23 - External event 2 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev2(&mut self) -> CH0RSEXEV2_W<ST3CH0RST_SPEC, 23> {
        CH0RSEXEV2_W::new(self)
    }
    #[doc = "Bit 24 - External event 3 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev3(&mut self) -> CH0RSEXEV3_W<ST3CH0RST_SPEC, 24> {
        CH0RSEXEV3_W::new(self)
    }
    #[doc = "Bit 25 - External event 4 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev4(&mut self) -> CH0RSEXEV4_W<ST3CH0RST_SPEC, 25> {
        CH0RSEXEV4_W::new(self)
    }
    #[doc = "Bit 26 - External event 5 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev5(&mut self) -> CH0RSEXEV5_W<ST3CH0RST_SPEC, 26> {
        CH0RSEXEV5_W::new(self)
    }
    #[doc = "Bit 27 - External event 6 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev6(&mut self) -> CH0RSEXEV6_W<ST3CH0RST_SPEC, 27> {
        CH0RSEXEV6_W::new(self)
    }
    #[doc = "Bit 28 - External event 7 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev7(&mut self) -> CH0RSEXEV7_W<ST3CH0RST_SPEC, 28> {
        CH0RSEXEV7_W::new(self)
    }
    #[doc = "Bit 29 - External event 8 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev8(&mut self) -> CH0RSEXEV8_W<ST3CH0RST_SPEC, 29> {
        CH0RSEXEV8_W::new(self)
    }
    #[doc = "Bit 30 - External event 9 generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsexev9(&mut self) -> CH0RSEXEV9_W<ST3CH0RST_SPEC, 30> {
        CH0RSEXEV9_W::new(self)
    }
    #[doc = "Bit 31 - Update event generates channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rsup(&mut self) -> CH0RSUP_W<ST3CH0RST_SPEC, 31> {
        CH0RSUP_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMERx channel 0 reset request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3ch0rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3ch0rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST3CH0RST_SPEC;
impl crate::RegisterSpec for ST3CH0RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st3ch0rst::R`](R) reader structure"]
impl crate::Readable for ST3CH0RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st3ch0rst::W`](W) writer structure"]
impl crate::Writable for ST3CH0RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST3CH0RST to value 0"]
impl crate::Resettable for ST3CH0RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
