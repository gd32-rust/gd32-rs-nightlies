#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Field `EPNUM` reader - Endpoint Identifier"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint Identifier"]
pub type EPNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DIR` reader - Direction of transaction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Direction of transaction"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L1REQ` reader - LPM L1 transaction is successful"]
pub type L1REQ_R = crate::BitReader;
#[doc = "Field `L1REQ` writer - LPM L1 transaction is successful"]
pub type L1REQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESOFIF` reader - Expected start of frame interrupt flag"]
pub type ESOFIF_R = crate::BitReader;
#[doc = "Field `ESOFIF` writer - Expected start of frame interrupt flag"]
pub type ESOFIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFIF` reader - start of frame interrupt flag"]
pub type SOFIF_R = crate::BitReader;
#[doc = "Field `SOFIF` writer - start of frame interrupt flag"]
pub type SOFIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIF` reader - reset interrupt flag"]
pub type RSTIF_R = crate::BitReader;
#[doc = "Field `RSTIF` writer - reset interrupt flag"]
pub type RSTIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPSIF` reader - Suspend mode interrupt flag"]
pub type SPSIF_R = crate::BitReader;
#[doc = "Field `SPSIF` writer - Suspend mode interrupt flag"]
pub type SPSIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUPIF` reader - Wakeup interrupt flag"]
pub type WKUPIF_R = crate::BitReader;
#[doc = "Field `WKUPIF` writer - Wakeup interrupt flag"]
pub type WKUPIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIF` reader - Error interrupt flag"]
pub type ERRIF_R = crate::BitReader;
#[doc = "Field `ERRIF` writer - Error interrupt flag"]
pub type ERRIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PMOUIF` reader - Packet memory area over / underrun interrupt flag"]
pub type PMOUIF_R = crate::BitReader;
#[doc = "Field `PMOUIF` writer - Packet memory area over / underrun interrupt flag"]
pub type PMOUIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STIF` reader - Successful transfer interrupt flag"]
pub type STIF_R = crate::BitReader;
#[doc = "Field `STIF` writer - Successful transfer interrupt flag"]
pub type STIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 transaction is successful"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt flag"]
    #[inline(always)]
    pub fn esofif(&self) -> ESOFIF_R {
        ESOFIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame interrupt flag"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reset interrupt flag"]
    #[inline(always)]
    pub fn rstif(&self) -> RSTIF_R {
        RSTIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt flag"]
    #[inline(always)]
    pub fn spsif(&self) -> SPSIF_R {
        SPSIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wkupif(&self) -> WKUPIF_R {
        WKUPIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt flag"]
    #[inline(always)]
    pub fn pmouif(&self) -> PMOUIF_R {
        PMOUIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Successful transfer interrupt flag"]
    #[inline(always)]
    pub fn stif(&self) -> STIF_R {
        STIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<INTF_SPEC, 0> {
        EPNUM_W::new(self)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<INTF_SPEC, 4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 7 - LPM L1 transaction is successful"]
    #[inline(always)]
    #[must_use]
    pub fn l1req(&mut self) -> L1REQ_W<INTF_SPEC, 7> {
        L1REQ_W::new(self)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn esofif(&mut self) -> ESOFIF_W<INTF_SPEC, 8> {
        ESOFIF_W::new(self)
    }
    #[doc = "Bit 9 - start of frame interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn sofif(&mut self) -> SOFIF_W<INTF_SPEC, 9> {
        SOFIF_W::new(self)
    }
    #[doc = "Bit 10 - reset interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstif(&mut self) -> RSTIF_W<INTF_SPEC, 10> {
        RSTIF_W::new(self)
    }
    #[doc = "Bit 11 - Suspend mode interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn spsif(&mut self) -> SPSIF_W<INTF_SPEC, 11> {
        SPSIF_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn wkupif(&mut self) -> WKUPIF_W<INTF_SPEC, 12> {
        WKUPIF_W::new(self)
    }
    #[doc = "Bit 13 - Error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn errif(&mut self) -> ERRIF_W<INTF_SPEC, 13> {
        ERRIF_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn pmouif(&mut self) -> PMOUIF_W<INTF_SPEC, 14> {
        PMOUIF_W::new(self)
    }
    #[doc = "Bit 15 - Successful transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn stif(&mut self) -> STIF_W<INTF_SPEC, 15> {
        STIF_W::new(self)
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
#[doc = "interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
