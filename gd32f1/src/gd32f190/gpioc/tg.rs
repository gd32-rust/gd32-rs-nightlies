#[doc = "Register `TG` writer"]
pub type W = crate::W<TG_SPEC>;
#[doc = "Port toggle bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TG0W_AW {
    #[doc = "1: Toggles the corresponding OCTLx bit"]
    TOGGLE = 1,
}
impl From<TG0W_AW> for bool {
    #[inline(always)]
    fn from(variant: TG0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG0` writer - Port toggle bit 0"]
pub type TG0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TG0W_AW>;
impl<'a, REG, const O: u8> TG0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Toggles the corresponding OCTLx bit"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(TG0W_AW::TOGGLE)
    }
}
#[doc = "Field `TG1` writer - Port toggle bit 1"]
pub use TG0_W as TG1_W;
#[doc = "Field `TG2` writer - Port toggle bit 2"]
pub use TG0_W as TG2_W;
#[doc = "Field `TG3` writer - Port toggle bit 3"]
pub use TG0_W as TG3_W;
#[doc = "Field `TG4` writer - Port toggle bit 4"]
pub use TG0_W as TG4_W;
#[doc = "Field `TG5` writer - Port toggle bit 5"]
pub use TG0_W as TG5_W;
#[doc = "Field `TG6` writer - Port toggle bit 6"]
pub use TG0_W as TG6_W;
#[doc = "Field `TG7` writer - Port toggle bit 7"]
pub use TG0_W as TG7_W;
#[doc = "Field `TG8` writer - Port toggle bit 8"]
pub use TG0_W as TG8_W;
#[doc = "Field `TG9` writer - Port toggle bit 9"]
pub use TG0_W as TG9_W;
#[doc = "Field `TG10` writer - Port toggle bit 10"]
pub use TG0_W as TG10_W;
#[doc = "Field `TG11` writer - Port toggle bit 11"]
pub use TG0_W as TG11_W;
#[doc = "Field `TG12` writer - Port toggle bit 12"]
pub use TG0_W as TG12_W;
#[doc = "Field `TG13` writer - Port toggle bit 13"]
pub use TG0_W as TG13_W;
#[doc = "Field `TG14` writer - Port toggle bit 14"]
pub use TG0_W as TG14_W;
#[doc = "Field `TG15` writer - Port toggle bit 15"]
pub use TG0_W as TG15_W;
impl W {
    #[doc = "Bit 0 - Port toggle bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tg0(&mut self) -> TG0_W<TG_SPEC, 0> {
        TG0_W::new(self)
    }
    #[doc = "Bit 1 - Port toggle bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tg1(&mut self) -> TG1_W<TG_SPEC, 1> {
        TG1_W::new(self)
    }
    #[doc = "Bit 2 - Port toggle bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tg2(&mut self) -> TG2_W<TG_SPEC, 2> {
        TG2_W::new(self)
    }
    #[doc = "Bit 3 - Port toggle bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tg3(&mut self) -> TG3_W<TG_SPEC, 3> {
        TG3_W::new(self)
    }
    #[doc = "Bit 4 - Port toggle bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tg4(&mut self) -> TG4_W<TG_SPEC, 4> {
        TG4_W::new(self)
    }
    #[doc = "Bit 5 - Port toggle bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tg5(&mut self) -> TG5_W<TG_SPEC, 5> {
        TG5_W::new(self)
    }
    #[doc = "Bit 6 - Port toggle bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tg6(&mut self) -> TG6_W<TG_SPEC, 6> {
        TG6_W::new(self)
    }
    #[doc = "Bit 7 - Port toggle bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tg7(&mut self) -> TG7_W<TG_SPEC, 7> {
        TG7_W::new(self)
    }
    #[doc = "Bit 8 - Port toggle bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tg8(&mut self) -> TG8_W<TG_SPEC, 8> {
        TG8_W::new(self)
    }
    #[doc = "Bit 9 - Port toggle bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tg9(&mut self) -> TG9_W<TG_SPEC, 9> {
        TG9_W::new(self)
    }
    #[doc = "Bit 10 - Port toggle bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tg10(&mut self) -> TG10_W<TG_SPEC, 10> {
        TG10_W::new(self)
    }
    #[doc = "Bit 11 - Port toggle bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tg11(&mut self) -> TG11_W<TG_SPEC, 11> {
        TG11_W::new(self)
    }
    #[doc = "Bit 12 - Port toggle bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tg12(&mut self) -> TG12_W<TG_SPEC, 12> {
        TG12_W::new(self)
    }
    #[doc = "Bit 13 - Port toggle bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn tg13(&mut self) -> TG13_W<TG_SPEC, 13> {
        TG13_W::new(self)
    }
    #[doc = "Bit 14 - Port toggle bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tg14(&mut self) -> TG14_W<TG_SPEC, 14> {
        TG14_W::new(self)
    }
    #[doc = "Bit 15 - Port toggle bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tg15(&mut self) -> TG15_W<TG_SPEC, 15> {
        TG15_W::new(self)
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
#[doc = "Port bit toggle register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TG_SPEC;
impl crate::RegisterSpec for TG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tg::W`](W) writer structure"]
impl crate::Writable for TG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TG to value 0"]
impl crate::Resettable for TG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
