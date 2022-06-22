#[doc = "Register `TG` writer"]
pub struct W(crate::W<TG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TG_SPEC>;
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
impl From<crate::W<TG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port toggle bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TG0_AW {
    #[doc = "1: Toggles the corresponding OCTLx bit"]
    TOGGLE = 1,
}
impl From<TG0_AW> for bool {
    #[inline(always)]
    fn from(variant: TG0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG0` writer - Port toggle bit"]
pub type TG0_W<'a> = crate::BitWriter<'a, u32, TG_SPEC, TG0_AW, 0>;
impl<'a> TG0_W<'a> {
    #[doc = "Toggles the corresponding OCTLx bit"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(TG0_AW::TOGGLE)
    }
}
#[doc = "Port toggle bit"]
pub use TG0_AW as TG1_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG2_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG3_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG4_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG5_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG6_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG7_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG8_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG9_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG10_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG11_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG12_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG13_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG14_AW;
#[doc = "Port toggle bit"]
pub use TG0_AW as TG15_AW;
#[doc = "Field `TG1` writer - Port toggle bit"]
pub use TG0_W as TG1_W;
#[doc = "Field `TG2` writer - Port toggle bit"]
pub use TG0_W as TG2_W;
#[doc = "Field `TG3` writer - Port toggle bit"]
pub use TG0_W as TG3_W;
#[doc = "Field `TG4` writer - Port toggle bit"]
pub use TG0_W as TG4_W;
#[doc = "Field `TG5` writer - Port toggle bit"]
pub use TG0_W as TG5_W;
#[doc = "Field `TG6` writer - Port toggle bit"]
pub use TG0_W as TG6_W;
#[doc = "Field `TG7` writer - Port toggle bit"]
pub use TG0_W as TG7_W;
#[doc = "Field `TG8` writer - Port toggle bit"]
pub use TG0_W as TG8_W;
#[doc = "Field `TG9` writer - Port toggle bit"]
pub use TG0_W as TG9_W;
#[doc = "Field `TG10` writer - Port toggle bit"]
pub use TG0_W as TG10_W;
#[doc = "Field `TG11` writer - Port toggle bit"]
pub use TG0_W as TG11_W;
#[doc = "Field `TG12` writer - Port toggle bit"]
pub use TG0_W as TG12_W;
#[doc = "Field `TG13` writer - Port toggle bit"]
pub use TG0_W as TG13_W;
#[doc = "Field `TG14` writer - Port toggle bit"]
pub use TG0_W as TG14_W;
#[doc = "Field `TG15` writer - Port toggle bit"]
pub use TG0_W as TG15_W;
impl W {
    #[doc = "Bit 0 - Port toggle bit"]
    #[inline(always)]
    pub fn tg0(&mut self) -> TG0_W {
        TG0_W::new(self)
    }
    #[doc = "Bit 1 - Port toggle bit"]
    #[inline(always)]
    pub fn tg1(&mut self) -> TG1_W {
        TG1_W::new(self)
    }
    #[doc = "Bit 2 - Port toggle bit"]
    #[inline(always)]
    pub fn tg2(&mut self) -> TG2_W {
        TG2_W::new(self)
    }
    #[doc = "Bit 3 - Port toggle bit"]
    #[inline(always)]
    pub fn tg3(&mut self) -> TG3_W {
        TG3_W::new(self)
    }
    #[doc = "Bit 4 - Port toggle bit"]
    #[inline(always)]
    pub fn tg4(&mut self) -> TG4_W {
        TG4_W::new(self)
    }
    #[doc = "Bit 5 - Port toggle bit"]
    #[inline(always)]
    pub fn tg5(&mut self) -> TG5_W {
        TG5_W::new(self)
    }
    #[doc = "Bit 6 - Port toggle bit"]
    #[inline(always)]
    pub fn tg6(&mut self) -> TG6_W {
        TG6_W::new(self)
    }
    #[doc = "Bit 7 - Port toggle bit"]
    #[inline(always)]
    pub fn tg7(&mut self) -> TG7_W {
        TG7_W::new(self)
    }
    #[doc = "Bit 8 - Port toggle bit"]
    #[inline(always)]
    pub fn tg8(&mut self) -> TG8_W {
        TG8_W::new(self)
    }
    #[doc = "Bit 9 - Port toggle bit"]
    #[inline(always)]
    pub fn tg9(&mut self) -> TG9_W {
        TG9_W::new(self)
    }
    #[doc = "Bit 10 - Port toggle bit"]
    #[inline(always)]
    pub fn tg10(&mut self) -> TG10_W {
        TG10_W::new(self)
    }
    #[doc = "Bit 11 - Port toggle bit"]
    #[inline(always)]
    pub fn tg11(&mut self) -> TG11_W {
        TG11_W::new(self)
    }
    #[doc = "Bit 12 - Port toggle bit"]
    #[inline(always)]
    pub fn tg12(&mut self) -> TG12_W {
        TG12_W::new(self)
    }
    #[doc = "Bit 13 - Port toggle bit"]
    #[inline(always)]
    pub fn tg13(&mut self) -> TG13_W {
        TG13_W::new(self)
    }
    #[doc = "Bit 14 - Port toggle bit"]
    #[inline(always)]
    pub fn tg14(&mut self) -> TG14_W {
        TG14_W::new(self)
    }
    #[doc = "Bit 15 - Port toggle bit"]
    #[inline(always)]
    pub fn tg15(&mut self) -> TG15_W {
        TG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port bit toggle register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tg](index.html) module"]
pub struct TG_SPEC;
impl crate::RegisterSpec for TG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tg::W](W) writer structure"]
impl crate::Writable for TG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TG to value 0"]
impl crate::Resettable for TG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
