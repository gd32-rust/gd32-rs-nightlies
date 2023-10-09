#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Clear global interrupt flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type GIFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GIFC0_AW>;
impl<'a, REG, const O: u8> GIFC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the GIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(GIFC0_AW::CLEAR)
    }
}
#[doc = "Clear bit for full transfer finish flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type FTFIFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FTFIFC0_AW>;
impl<'a, REG, const O: u8> FTFIFC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the FDFIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FTFIFC0_AW::CLEAR)
    }
}
#[doc = "Clear bit for half transfer finish flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type HTFIFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HTFIFC0_AW>;
impl<'a, REG, const O: u8> HTFIFC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the HTFIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HTFIFC0_AW::CLEAR)
    }
}
#[doc = "Clear bit for error flag of channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type ERRIFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIFC0_AW>;
impl<'a, REG, const O: u8> ERRIFC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ERRIF flag in INTF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIFC0_AW::CLEAR)
    }
}
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
    #[must_use]
    pub fn gifc0(&mut self) -> GIFC0_W<INTC_SPEC, 0> {
        GIFC0_W::new(self)
    }
    #[doc = "Bit 1 - Clear bit for full transfer finish flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc0(&mut self) -> FTFIFC0_W<INTC_SPEC, 1> {
        FTFIFC0_W::new(self)
    }
    #[doc = "Bit 2 - Clear bit for half transfer finish flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc0(&mut self) -> HTFIFC0_W<INTC_SPEC, 2> {
        HTFIFC0_W::new(self)
    }
    #[doc = "Bit 3 - Clear bit for error flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn errifc0(&mut self) -> ERRIFC0_W<INTC_SPEC, 3> {
        ERRIFC0_W::new(self)
    }
    #[doc = "Bit 4 - Clear global interrupt flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn gifc1(&mut self) -> GIFC1_W<INTC_SPEC, 4> {
        GIFC1_W::new(self)
    }
    #[doc = "Bit 5 - Clear bit for full transfer finish flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc1(&mut self) -> FTFIFC1_W<INTC_SPEC, 5> {
        FTFIFC1_W::new(self)
    }
    #[doc = "Bit 6 - Clear bit for half transfer finish flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc1(&mut self) -> HTFIFC1_W<INTC_SPEC, 6> {
        HTFIFC1_W::new(self)
    }
    #[doc = "Bit 7 - Clear bit for error flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn errifc1(&mut self) -> ERRIFC1_W<INTC_SPEC, 7> {
        ERRIFC1_W::new(self)
    }
    #[doc = "Bit 8 - Clear global interrupt flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn gifc2(&mut self) -> GIFC2_W<INTC_SPEC, 8> {
        GIFC2_W::new(self)
    }
    #[doc = "Bit 9 - Clear bit for full transfer finish flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc2(&mut self) -> FTFIFC2_W<INTC_SPEC, 9> {
        FTFIFC2_W::new(self)
    }
    #[doc = "Bit 10 - Clear bit for half transfer finish flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc2(&mut self) -> HTFIFC2_W<INTC_SPEC, 10> {
        HTFIFC2_W::new(self)
    }
    #[doc = "Bit 11 - Clear bit for error flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn errifc2(&mut self) -> ERRIFC2_W<INTC_SPEC, 11> {
        ERRIFC2_W::new(self)
    }
    #[doc = "Bit 12 - Clear global interrupt flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn gifc3(&mut self) -> GIFC3_W<INTC_SPEC, 12> {
        GIFC3_W::new(self)
    }
    #[doc = "Bit 13 - Clear bit for full transfer finish flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc3(&mut self) -> FTFIFC3_W<INTC_SPEC, 13> {
        FTFIFC3_W::new(self)
    }
    #[doc = "Bit 14 - Clear bit for half transfer finish flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc3(&mut self) -> HTFIFC3_W<INTC_SPEC, 14> {
        HTFIFC3_W::new(self)
    }
    #[doc = "Bit 15 - Clear bit for error flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn errifc3(&mut self) -> ERRIFC3_W<INTC_SPEC, 15> {
        ERRIFC3_W::new(self)
    }
    #[doc = "Bit 16 - Clear global interrupt flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn gifc4(&mut self) -> GIFC4_W<INTC_SPEC, 16> {
        GIFC4_W::new(self)
    }
    #[doc = "Bit 17 - Clear bit for full transfer finish flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc4(&mut self) -> FTFIFC4_W<INTC_SPEC, 17> {
        FTFIFC4_W::new(self)
    }
    #[doc = "Bit 18 - Clear bit for half transfer finish flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc4(&mut self) -> HTFIFC4_W<INTC_SPEC, 18> {
        HTFIFC4_W::new(self)
    }
    #[doc = "Bit 19 - Clear bit for error flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn errifc4(&mut self) -> ERRIFC4_W<INTC_SPEC, 19> {
        ERRIFC4_W::new(self)
    }
    #[doc = "Bit 20 - Clear global interrupt flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn gifc5(&mut self) -> GIFC5_W<INTC_SPEC, 20> {
        GIFC5_W::new(self)
    }
    #[doc = "Bit 21 - Clear bit for full transfer finish flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc5(&mut self) -> FTFIFC5_W<INTC_SPEC, 21> {
        FTFIFC5_W::new(self)
    }
    #[doc = "Bit 22 - Clear bit for half transfer finish flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc5(&mut self) -> HTFIFC5_W<INTC_SPEC, 22> {
        HTFIFC5_W::new(self)
    }
    #[doc = "Bit 23 - Clear bit for error flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn errifc5(&mut self) -> ERRIFC5_W<INTC_SPEC, 23> {
        ERRIFC5_W::new(self)
    }
    #[doc = "Bit 24 - Clear global interrupt flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn gifc6(&mut self) -> GIFC6_W<INTC_SPEC, 24> {
        GIFC6_W::new(self)
    }
    #[doc = "Bit 25 - Clear bit for full transfer finish flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc6(&mut self) -> FTFIFC6_W<INTC_SPEC, 25> {
        FTFIFC6_W::new(self)
    }
    #[doc = "Bit 26 - Clear bit for half transfer finish flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc6(&mut self) -> HTFIFC6_W<INTC_SPEC, 26> {
        HTFIFC6_W::new(self)
    }
    #[doc = "Bit 27 - Clear bit for error flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn errifc6(&mut self) -> ERRIFC6_W<INTC_SPEC, 27> {
        ERRIFC6_W::new(self)
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
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
