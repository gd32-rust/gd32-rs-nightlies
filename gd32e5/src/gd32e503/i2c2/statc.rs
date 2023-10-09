#[doc = "Register `STATC` writer"]
pub type W = crate::W<STATC_SPEC>;
#[doc = "Field `ADDSENDC` writer - ADDSEND flag clear"]
pub type ADDSENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NACKC` writer - Not Acknowledge flag clear"]
pub type NACKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STPDETC` writer - STPDET flag clear"]
pub type STPDETC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BERRC` writer - Bus error flag clear"]
pub type BERRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOSTARBC` writer - Arbitration Lost flag clear"]
pub type LOSTARBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUERRC` writer - Overrun/Underrun flag clear"]
pub type OUERRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECERRC` writer - PEC error flag clear"]
pub type PECERRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMEOUTC` writer - TIMEOUT flag clear"]
pub type TIMEOUTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBALTC` writer - SMBus Alert flag clear"]
pub type SMBALTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 3 - ADDSEND flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn addsendc(&mut self) -> ADDSENDC_W<STATC_SPEC, 3> {
        ADDSENDC_W::new(self)
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn nackc(&mut self) -> NACKC_W<STATC_SPEC, 4> {
        NACKC_W::new(self)
    }
    #[doc = "Bit 5 - STPDET flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn stpdetc(&mut self) -> STPDETC_W<STATC_SPEC, 5> {
        STPDETC_W::new(self)
    }
    #[doc = "Bit 8 - Bus error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn berrc(&mut self) -> BERRC_W<STATC_SPEC, 8> {
        BERRC_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration Lost flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn lostarbc(&mut self) -> LOSTARBC_W<STATC_SPEC, 9> {
        LOSTARBC_W::new(self)
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ouerrc(&mut self) -> OUERRC_W<STATC_SPEC, 10> {
        OUERRC_W::new(self)
    }
    #[doc = "Bit 11 - PEC error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn pecerrc(&mut self) -> PECERRC_W<STATC_SPEC, 11> {
        PECERRC_W::new(self)
    }
    #[doc = "Bit 12 - TIMEOUT flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutc(&mut self) -> TIMEOUTC_W<STATC_SPEC, 12> {
        TIMEOUTC_W::new(self)
    }
    #[doc = "Bit 13 - SMBus Alert flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn smbaltc(&mut self) -> SMBALTC_W<STATC_SPEC, 13> {
        SMBALTC_W::new(self)
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
#[doc = "Status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATC_SPEC;
impl crate::RegisterSpec for STATC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`statc::W`](W) writer structure"]
impl crate::Writable for STATC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATC to value 0"]
impl crate::Resettable for STATC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
