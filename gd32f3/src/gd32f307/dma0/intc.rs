#[doc = "Register `INTC` writer"]
pub struct W(crate::W<INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTC_SPEC>;
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
impl From<crate::W<INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear global interrupt flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIFC0_AW {
    #[doc = "1: Clears the GIF flag in INTF"]
    CLEAR = 1,
}
impl From<GIFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: GIFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIFC0` writer - Clear global interrupt flag of channel 0"]
pub type GIFC0_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, GIFC0_AW, 0>;
impl<'a> GIFC0_W<'a> {
    #[doc = "Clears the GIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIFC0_AW::CLEAR)
    }
}
#[doc = "Clear bit for full transfer finish flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTFIFC0_AW {
    #[doc = "1: Clears the FDFIF flag in INTF"]
    CLEAR = 1,
}
impl From<FTFIFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: FTFIFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTFIFC0` writer - Clear bit for full transfer finish flag of channel 0"]
pub type FTFIFC0_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, FTFIFC0_AW, 1>;
impl<'a> FTFIFC0_W<'a> {
    #[doc = "Clears the FDFIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FTFIFC0_AW::CLEAR)
    }
}
#[doc = "Clear bit for half transfer finish flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTFIFC0_AW {
    #[doc = "1: Clears the HTFIF flag in INTF"]
    CLEAR = 1,
}
impl From<HTFIFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: HTFIFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTFIFC0` writer - Clear bit for half transfer finish flag of channel 0"]
pub type HTFIFC0_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, HTFIFC0_AW, 2>;
impl<'a> HTFIFC0_W<'a> {
    #[doc = "Clears the HTFIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTFIFC0_AW::CLEAR)
    }
}
#[doc = "Clear bit for error flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIFC0_AW {
    #[doc = "1: Clears the ERRIF flag in INTF"]
    CLEAR = 1,
}
impl From<ERRIFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRIFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIFC0` writer - Clear bit for error flag of channel 0"]
pub type ERRIFC0_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, ERRIFC0_AW, 3>;
impl<'a> ERRIFC0_W<'a> {
    #[doc = "Clears the ERRIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRIFC0_AW::CLEAR)
    }
}
#[doc = "Clear bit for error flag of channel 1"]
pub use ERRIFC0_AW as ERRIFC1_AW;
#[doc = "Clear bit for error flag of channel 2"]
pub use ERRIFC0_AW as ERRIFC2_AW;
#[doc = "Clear bit for error flag of channel 3"]
pub use ERRIFC0_AW as ERRIFC3_AW;
#[doc = "Clear bit for error flag of channel 4"]
pub use ERRIFC0_AW as ERRIFC4_AW;
#[doc = "Clear bit for error flag of channel 5"]
pub use ERRIFC0_AW as ERRIFC5_AW;
#[doc = "Clear bit for error flag of channel 6"]
pub use ERRIFC0_AW as ERRIFC6_AW;
#[doc = "Field `ERRIFC1` writer - Clear bit for error flag of channel 1"]
pub use ERRIFC0_W as ERRIFC1_W;
#[doc = "Field `ERRIFC2` writer - Clear bit for error flag of channel 2"]
pub use ERRIFC0_W as ERRIFC2_W;
#[doc = "Field `ERRIFC3` writer - Clear bit for error flag of channel 3"]
pub use ERRIFC0_W as ERRIFC3_W;
#[doc = "Field `ERRIFC4` writer - Clear bit for error flag of channel 4"]
pub use ERRIFC0_W as ERRIFC4_W;
#[doc = "Field `ERRIFC5` writer - Clear bit for error flag of channel 5"]
pub use ERRIFC0_W as ERRIFC5_W;
#[doc = "Field `ERRIFC6` writer - Clear bit for error flag of channel 6"]
pub use ERRIFC0_W as ERRIFC6_W;
#[doc = "Clear bit for full transfer finish flag of channel 1"]
pub use FTFIFC0_AW as FTFIFC1_AW;
#[doc = "Clear bit for full transfer finish flag of channel 2"]
pub use FTFIFC0_AW as FTFIFC2_AW;
#[doc = "Clear bit for full transfer finish flag of channel 3"]
pub use FTFIFC0_AW as FTFIFC3_AW;
#[doc = "Clear bit for full transfer finish flag of channel 4"]
pub use FTFIFC0_AW as FTFIFC4_AW;
#[doc = "Clear bit for full transfer finish flag of channel 5"]
pub use FTFIFC0_AW as FTFIFC5_AW;
#[doc = "Clear bit for full transfer finish flag of channel 6"]
pub use FTFIFC0_AW as FTFIFC6_AW;
#[doc = "Field `FTFIFC1` writer - Clear bit for full transfer finish flag of channel 1"]
pub use FTFIFC0_W as FTFIFC1_W;
#[doc = "Field `FTFIFC2` writer - Clear bit for full transfer finish flag of channel 2"]
pub use FTFIFC0_W as FTFIFC2_W;
#[doc = "Field `FTFIFC3` writer - Clear bit for full transfer finish flag of channel 3"]
pub use FTFIFC0_W as FTFIFC3_W;
#[doc = "Field `FTFIFC4` writer - Clear bit for full transfer finish flag of channel 4"]
pub use FTFIFC0_W as FTFIFC4_W;
#[doc = "Field `FTFIFC5` writer - Clear bit for full transfer finish flag of channel 5"]
pub use FTFIFC0_W as FTFIFC5_W;
#[doc = "Field `FTFIFC6` writer - Clear bit for full transfer finish flag of channel 6"]
pub use FTFIFC0_W as FTFIFC6_W;
#[doc = "Clear global interrupt flag of channel 1"]
pub use GIFC0_AW as GIFC1_AW;
#[doc = "Clear global interrupt flag of channel 2"]
pub use GIFC0_AW as GIFC2_AW;
#[doc = "Clear global interrupt flag of channel 3"]
pub use GIFC0_AW as GIFC3_AW;
#[doc = "Clear global interrupt flag of channel 4"]
pub use GIFC0_AW as GIFC4_AW;
#[doc = "Clear global interrupt flag of channel 5"]
pub use GIFC0_AW as GIFC5_AW;
#[doc = "Clear global interrupt flag of channel 6"]
pub use GIFC0_AW as GIFC6_AW;
#[doc = "Field `GIFC1` writer - Clear global interrupt flag of channel 1"]
pub use GIFC0_W as GIFC1_W;
#[doc = "Field `GIFC2` writer - Clear global interrupt flag of channel 2"]
pub use GIFC0_W as GIFC2_W;
#[doc = "Field `GIFC3` writer - Clear global interrupt flag of channel 3"]
pub use GIFC0_W as GIFC3_W;
#[doc = "Field `GIFC4` writer - Clear global interrupt flag of channel 4"]
pub use GIFC0_W as GIFC4_W;
#[doc = "Field `GIFC5` writer - Clear global interrupt flag of channel 5"]
pub use GIFC0_W as GIFC5_W;
#[doc = "Field `GIFC6` writer - Clear global interrupt flag of channel 6"]
pub use GIFC0_W as GIFC6_W;
#[doc = "Clear bit for half transfer finish flag of channel 1"]
pub use HTFIFC0_AW as HTFIFC1_AW;
#[doc = "Clear bit for half transfer finish flag of channel 2"]
pub use HTFIFC0_AW as HTFIFC2_AW;
#[doc = "Clear bit for half transfer finish flag of channel 3"]
pub use HTFIFC0_AW as HTFIFC3_AW;
#[doc = "Clear bit for half transfer finish flag of channel 4"]
pub use HTFIFC0_AW as HTFIFC4_AW;
#[doc = "Clear bit for half transfer finish flag of channel 5"]
pub use HTFIFC0_AW as HTFIFC5_AW;
#[doc = "Clear bit for half transfer finish flag of channel 6"]
pub use HTFIFC0_AW as HTFIFC6_AW;
#[doc = "Field `HTFIFC1` writer - Clear bit for half transfer finish flag of channel 1"]
pub use HTFIFC0_W as HTFIFC1_W;
#[doc = "Field `HTFIFC2` writer - Clear bit for half transfer finish flag of channel 2"]
pub use HTFIFC0_W as HTFIFC2_W;
#[doc = "Field `HTFIFC3` writer - Clear bit for half transfer finish flag of channel 3"]
pub use HTFIFC0_W as HTFIFC3_W;
#[doc = "Field `HTFIFC4` writer - Clear bit for half transfer finish flag of channel 4"]
pub use HTFIFC0_W as HTFIFC4_W;
#[doc = "Field `HTFIFC5` writer - Clear bit for half transfer finish flag of channel 5"]
pub use HTFIFC0_W as HTFIFC5_W;
#[doc = "Field `HTFIFC6` writer - Clear bit for half transfer finish flag of channel 6"]
pub use HTFIFC0_W as HTFIFC6_W;
impl W {
    #[doc = "Bit 0 - Clear global interrupt flag of channel 0"]
    #[inline(always)]
    pub fn gifc0(&mut self) -> GIFC0_W {
        GIFC0_W::new(self)
    }
    #[doc = "Bit 1 - Clear bit for full transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn ftfifc0(&mut self) -> FTFIFC0_W {
        FTFIFC0_W::new(self)
    }
    #[doc = "Bit 2 - Clear bit for half transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn htfifc0(&mut self) -> HTFIFC0_W {
        HTFIFC0_W::new(self)
    }
    #[doc = "Bit 3 - Clear bit for error flag of channel 0"]
    #[inline(always)]
    pub fn errifc0(&mut self) -> ERRIFC0_W {
        ERRIFC0_W::new(self)
    }
    #[doc = "Bit 4 - Clear global interrupt flag of channel 1"]
    #[inline(always)]
    pub fn gifc1(&mut self) -> GIFC1_W {
        GIFC1_W::new(self)
    }
    #[doc = "Bit 5 - Clear bit for full transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn ftfifc1(&mut self) -> FTFIFC1_W {
        FTFIFC1_W::new(self)
    }
    #[doc = "Bit 6 - Clear bit for half transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn htfifc1(&mut self) -> HTFIFC1_W {
        HTFIFC1_W::new(self)
    }
    #[doc = "Bit 7 - Clear bit for error flag of channel 1"]
    #[inline(always)]
    pub fn errifc1(&mut self) -> ERRIFC1_W {
        ERRIFC1_W::new(self)
    }
    #[doc = "Bit 8 - Clear global interrupt flag of channel 2"]
    #[inline(always)]
    pub fn gifc2(&mut self) -> GIFC2_W {
        GIFC2_W::new(self)
    }
    #[doc = "Bit 9 - Clear bit for full transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn ftfifc2(&mut self) -> FTFIFC2_W {
        FTFIFC2_W::new(self)
    }
    #[doc = "Bit 10 - Clear bit for half transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn htfifc2(&mut self) -> HTFIFC2_W {
        HTFIFC2_W::new(self)
    }
    #[doc = "Bit 11 - Clear bit for error flag of channel 2"]
    #[inline(always)]
    pub fn errifc2(&mut self) -> ERRIFC2_W {
        ERRIFC2_W::new(self)
    }
    #[doc = "Bit 12 - Clear global interrupt flag of channel 3"]
    #[inline(always)]
    pub fn gifc3(&mut self) -> GIFC3_W {
        GIFC3_W::new(self)
    }
    #[doc = "Bit 13 - Clear bit for full transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn ftfifc3(&mut self) -> FTFIFC3_W {
        FTFIFC3_W::new(self)
    }
    #[doc = "Bit 14 - Clear bit for half transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn htfifc3(&mut self) -> HTFIFC3_W {
        HTFIFC3_W::new(self)
    }
    #[doc = "Bit 15 - Clear bit for error flag of channel 3"]
    #[inline(always)]
    pub fn errifc3(&mut self) -> ERRIFC3_W {
        ERRIFC3_W::new(self)
    }
    #[doc = "Bit 16 - Clear global interrupt flag of channel 4"]
    #[inline(always)]
    pub fn gifc4(&mut self) -> GIFC4_W {
        GIFC4_W::new(self)
    }
    #[doc = "Bit 17 - Clear bit for full transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn ftfifc4(&mut self) -> FTFIFC4_W {
        FTFIFC4_W::new(self)
    }
    #[doc = "Bit 18 - Clear bit for half transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn htfifc4(&mut self) -> HTFIFC4_W {
        HTFIFC4_W::new(self)
    }
    #[doc = "Bit 19 - Clear bit for error flag of channel 4"]
    #[inline(always)]
    pub fn errifc4(&mut self) -> ERRIFC4_W {
        ERRIFC4_W::new(self)
    }
    #[doc = "Bit 20 - Clear global interrupt flag of channel 5"]
    #[inline(always)]
    pub fn gifc5(&mut self) -> GIFC5_W {
        GIFC5_W::new(self)
    }
    #[doc = "Bit 21 - Clear bit for full transfer finish flag of channel 5"]
    #[inline(always)]
    pub fn ftfifc5(&mut self) -> FTFIFC5_W {
        FTFIFC5_W::new(self)
    }
    #[doc = "Bit 22 - Clear bit for half transfer finish flag of channel 5"]
    #[inline(always)]
    pub fn htfifc5(&mut self) -> HTFIFC5_W {
        HTFIFC5_W::new(self)
    }
    #[doc = "Bit 23 - Clear bit for error flag of channel 5"]
    #[inline(always)]
    pub fn errifc5(&mut self) -> ERRIFC5_W {
        ERRIFC5_W::new(self)
    }
    #[doc = "Bit 24 - Clear global interrupt flag of channel 6"]
    #[inline(always)]
    pub fn gifc6(&mut self) -> GIFC6_W {
        GIFC6_W::new(self)
    }
    #[doc = "Bit 25 - Clear bit for full transfer finish flag of channel 6"]
    #[inline(always)]
    pub fn ftfifc6(&mut self) -> FTFIFC6_W {
        FTFIFC6_W::new(self)
    }
    #[doc = "Bit 26 - Clear bit for half transfer finish flag of channel 6"]
    #[inline(always)]
    pub fn htfifc6(&mut self) -> HTFIFC6_W {
        HTFIFC6_W::new(self)
    }
    #[doc = "Bit 27 - Clear bit for error flag of channel 6"]
    #[inline(always)]
    pub fn errifc6(&mut self) -> ERRIFC6_W {
        ERRIFC6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](index.html) module"]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intc::W](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
