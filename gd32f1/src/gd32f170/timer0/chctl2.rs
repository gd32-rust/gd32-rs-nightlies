#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<CHCTL2_SPEC>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<CHCTL2_SPEC>;
#[doc = "Field `CH0EN` reader - Channel 0 enable"]
pub type CH0EN_R = crate::BitReader<CH0EN_A>;
#[doc = "Channel 0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0EN_A {
    #[doc = "0: Channel output is disabled"]
    DISABLED = 0,
    #[doc = "1: Channel output is enabled"]
    ENABLED = 1,
}
impl From<CH0EN_A> for bool {
    #[inline(always)]
    fn from(variant: CH0EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0EN_A {
        match self.bits {
            false => CH0EN_A::DISABLED,
            true => CH0EN_A::ENABLED,
        }
    }
    #[doc = "Channel output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0EN_A::DISABLED
    }
    #[doc = "Channel output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0EN_A::ENABLED
    }
}
#[doc = "Field `CH0EN` writer - Channel 0 enable"]
pub type CH0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0EN_A>;
impl<'a, REG, const O: u8> CH0EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0EN_A::DISABLED)
    }
    #[doc = "Channel output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CH0EN_A::ENABLED)
    }
}
#[doc = "Field `CH0P` reader - Channel 0 polarity"]
pub type CH0P_R = crate::BitReader<CH0P_A>;
#[doc = "Channel 0 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0P_A {
    #[doc = "0: Active high"]
    NOT_INVERTED = 0,
    #[doc = "1: Active low"]
    INVERTED = 1,
}
impl From<CH0P_A> for bool {
    #[inline(always)]
    fn from(variant: CH0P_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0P_A {
        match self.bits {
            false => CH0P_A::NOT_INVERTED,
            true => CH0P_A::INVERTED,
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == CH0P_A::NOT_INVERTED
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CH0P_A::INVERTED
    }
}
#[doc = "Field `CH0P` writer - Channel 0 polarity"]
pub type CH0P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0P_A>;
impl<'a, REG, const O: u8> CH0P_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CH0P_A::NOT_INVERTED)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CH0P_A::INVERTED)
    }
}
#[doc = "Field `CH0NEN` reader - Channel 0 complementary output enable"]
pub use CH0EN_R as CH0NEN_R;
#[doc = "Field `CH1EN` reader - Channel 1 enable"]
pub use CH0EN_R as CH1EN_R;
#[doc = "Field `CH1NEN` reader - Channel 1 complementary output enable"]
pub use CH0EN_R as CH1NEN_R;
#[doc = "Field `CH2EN` reader - Channel 2 enable"]
pub use CH0EN_R as CH2EN_R;
#[doc = "Field `CH2NEN` reader - Channel 2 complementary output enable"]
pub use CH0EN_R as CH2NEN_R;
#[doc = "Field `CH3EN` reader - Channel 3 enable"]
pub use CH0EN_R as CH3EN_R;
#[doc = "Field `CH0NEN` writer - Channel 0 complementary output enable"]
pub use CH0EN_W as CH0NEN_W;
#[doc = "Field `CH1EN` writer - Channel 1 enable"]
pub use CH0EN_W as CH1EN_W;
#[doc = "Field `CH1NEN` writer - Channel 1 complementary output enable"]
pub use CH0EN_W as CH1NEN_W;
#[doc = "Field `CH2EN` writer - Channel 2 enable"]
pub use CH0EN_W as CH2EN_W;
#[doc = "Field `CH2NEN` writer - Channel 2 complementary output enable"]
pub use CH0EN_W as CH2NEN_W;
#[doc = "Field `CH3EN` writer - Channel 3 enable"]
pub use CH0EN_W as CH3EN_W;
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub use CH0P_R as CH0NP_R;
#[doc = "Field `CH1P` reader - Channel 1 polarity"]
pub use CH0P_R as CH1P_R;
#[doc = "Field `CH1NP` reader - Channel 1 complementary output polarity"]
pub use CH0P_R as CH1NP_R;
#[doc = "Field `CH2P` reader - Channel 2 polarity"]
pub use CH0P_R as CH2P_R;
#[doc = "Field `CH2NP` reader - Channel 2 complementary output polarity"]
pub use CH0P_R as CH2NP_R;
#[doc = "Field `CH3P` reader - Channel 3 polarity"]
pub use CH0P_R as CH3P_R;
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub use CH0P_W as CH0NP_W;
#[doc = "Field `CH1P` writer - Channel 1 polarity"]
pub use CH0P_W as CH1P_W;
#[doc = "Field `CH1NP` writer - Channel 1 complementary output polarity"]
pub use CH0P_W as CH1NP_W;
#[doc = "Field `CH2P` writer - Channel 2 polarity"]
pub use CH0P_W as CH2P_W;
#[doc = "Field `CH2NP` writer - Channel 2 complementary output polarity"]
pub use CH0P_W as CH2NP_W;
#[doc = "Field `CH3P` writer - Channel 3 polarity"]
pub use CH0P_W as CH3P_W;
impl R {
    #[doc = "Bit 0 - Channel 0 enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&self) -> CH0NEN_R {
        CH0NEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 1 complementary output enable"]
    #[inline(always)]
    pub fn ch1nen(&self) -> CH1NEN_R {
        CH1NEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 complementary output polarity"]
    #[inline(always)]
    pub fn ch1np(&self) -> CH1NP_R {
        CH1NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 2 enable"]
    #[inline(always)]
    pub fn ch2en(&self) -> CH2EN_R {
        CH2EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 polarity"]
    #[inline(always)]
    pub fn ch2p(&self) -> CH2P_R {
        CH2P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 complementary output enable"]
    #[inline(always)]
    pub fn ch2nen(&self) -> CH2NEN_R {
        CH2NEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 complementary output polarity"]
    #[inline(always)]
    pub fn ch2np(&self) -> CH2NP_R {
        CH2NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 enable"]
    #[inline(always)]
    pub fn ch3en(&self) -> CH3EN_R {
        CH3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 polarity"]
    #[inline(always)]
    pub fn ch3p(&self) -> CH3P_R {
        CH3P_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0en(&mut self) -> CH0EN_W<CHCTL2_SPEC, 0> {
        CH0EN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> CH0P_W<CHCTL2_SPEC, 1> {
        CH0P_W::new(self)
    }
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0nen(&mut self) -> CH0NEN_W<CHCTL2_SPEC, 2> {
        CH0NEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0np(&mut self) -> CH0NP_W<CHCTL2_SPEC, 3> {
        CH0NP_W::new(self)
    }
    #[doc = "Bit 4 - Channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1en(&mut self) -> CH1EN_W<CHCTL2_SPEC, 4> {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> CH1P_W<CHCTL2_SPEC, 5> {
        CH1P_W::new(self)
    }
    #[doc = "Bit 6 - Channel 1 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1nen(&mut self) -> CH1NEN_W<CHCTL2_SPEC, 6> {
        CH1NEN_W::new(self)
    }
    #[doc = "Bit 7 - Channel 1 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1np(&mut self) -> CH1NP_W<CHCTL2_SPEC, 7> {
        CH1NP_W::new(self)
    }
    #[doc = "Bit 8 - Channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2en(&mut self) -> CH2EN_W<CHCTL2_SPEC, 8> {
        CH2EN_W::new(self)
    }
    #[doc = "Bit 9 - Channel 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch2p(&mut self) -> CH2P_W<CHCTL2_SPEC, 9> {
        CH2P_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2nen(&mut self) -> CH2NEN_W<CHCTL2_SPEC, 10> {
        CH2NEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch2np(&mut self) -> CH2NP_W<CHCTL2_SPEC, 11> {
        CH2NP_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3en(&mut self) -> CH3EN_W<CHCTL2_SPEC, 12> {
        CH3EN_W::new(self)
    }
    #[doc = "Bit 13 - Channel 3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch3p(&mut self) -> CH3P_W<CHCTL2_SPEC, 13> {
        CH3P_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL2_SPEC;
impl crate::RegisterSpec for CHCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chctl2::R`](R) reader structure"]
impl crate::Readable for CHCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl2::W`](W) writer structure"]
impl crate::Writable for CHCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for CHCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
