#[doc = "Register `ST1INTC` writer"]
pub type W = crate::W<ST1INTC_SPEC>;
#[doc = "Field `CMP0IFC` writer - Clear compare 0 interrupt flag"]
pub type CMP0IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1IFC` writer - Clear compare 1 interrupt flag"]
pub type CMP1IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP2IFC` writer - Clear compare 2 interrupt flag"]
pub type CMP2IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP3IFC` writer - Clear compare 3 interrupt flag"]
pub type CMP3IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REPIFC` writer - Clear repetition interrupt flag"]
pub type REPIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPIFC` writer - Clear update interrupt flag"]
pub type UPIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0IFC` writer - Clear capture 0 interrupt flag"]
pub type CAP0IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1IFC` writer - Clear capture 1 interrupt flag"]
pub type CAP1IFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0OAIFC` writer - Clear channel 0 output active interrupt flag"]
pub type CH0OAIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0ONAIFC` writer - Clear channel 0 output inactive interrupt flag"]
pub type CH0ONAIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1OAIFC` writer - Clear channel 1 output active interrupt flag"]
pub type CH1OAIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1ONAIFC` writer - Clear channel 1 output inactive interrupt flag"]
pub type CH1ONAIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIFC` writer - Clear counter reset interrupt flag"]
pub type RSTIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLYIIFC` writer - Clear delayed IDLE mode entry interrupt flag"]
pub type DLYIIFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear compare 0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0ifc(&mut self) -> CMP0IFC_W<ST1INTC_SPEC, 0> {
        CMP0IFC_W::new(self)
    }
    #[doc = "Bit 1 - Clear compare 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ifc(&mut self) -> CMP1IFC_W<ST1INTC_SPEC, 1> {
        CMP1IFC_W::new(self)
    }
    #[doc = "Bit 2 - Clear compare 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ifc(&mut self) -> CMP2IFC_W<ST1INTC_SPEC, 2> {
        CMP2IFC_W::new(self)
    }
    #[doc = "Bit 3 - Clear compare 3 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3ifc(&mut self) -> CMP3IFC_W<ST1INTC_SPEC, 3> {
        CMP3IFC_W::new(self)
    }
    #[doc = "Bit 4 - Clear repetition interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn repifc(&mut self) -> REPIFC_W<ST1INTC_SPEC, 4> {
        REPIFC_W::new(self)
    }
    #[doc = "Bit 6 - Clear update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn upifc(&mut self) -> UPIFC_W<ST1INTC_SPEC, 6> {
        UPIFC_W::new(self)
    }
    #[doc = "Bit 7 - Clear capture 0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cap0ifc(&mut self) -> CAP0IFC_W<ST1INTC_SPEC, 7> {
        CAP0IFC_W::new(self)
    }
    #[doc = "Bit 8 - Clear capture 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cap1ifc(&mut self) -> CAP1IFC_W<ST1INTC_SPEC, 8> {
        CAP1IFC_W::new(self)
    }
    #[doc = "Bit 9 - Clear channel 0 output active interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0oaifc(&mut self) -> CH0OAIFC_W<ST1INTC_SPEC, 9> {
        CH0OAIFC_W::new(self)
    }
    #[doc = "Bit 10 - Clear channel 0 output inactive interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0onaifc(&mut self) -> CH0ONAIFC_W<ST1INTC_SPEC, 10> {
        CH0ONAIFC_W::new(self)
    }
    #[doc = "Bit 11 - Clear channel 1 output active interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1oaifc(&mut self) -> CH1OAIFC_W<ST1INTC_SPEC, 11> {
        CH1OAIFC_W::new(self)
    }
    #[doc = "Bit 12 - Clear channel 1 output inactive interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1onaifc(&mut self) -> CH1ONAIFC_W<ST1INTC_SPEC, 12> {
        CH1ONAIFC_W::new(self)
    }
    #[doc = "Bit 13 - Clear counter reset interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstifc(&mut self) -> RSTIFC_W<ST1INTC_SPEC, 13> {
        RSTIFC_W::new(self)
    }
    #[doc = "Bit 14 - Clear delayed IDLE mode entry interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlyiifc(&mut self) -> DLYIIFC_W<ST1INTC_SPEC, 14> {
        DLYIIFC_W::new(self)
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
#[doc = "SHRTIMER Slave_TIMER1 interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st1intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST1INTC_SPEC;
impl crate::RegisterSpec for ST1INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`st1intc::W`](W) writer structure"]
impl crate::Writable for ST1INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1INTC to value 0"]
impl crate::Resettable for ST1INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
