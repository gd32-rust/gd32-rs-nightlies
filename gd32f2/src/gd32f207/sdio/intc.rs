#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Field `CCRCERRC` writer - CCRCERR flag clear bit"]
pub type CCRCERRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTCRCERRC` writer - DTCRCERR flag clear bit"]
pub type DTCRCERRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDTMOUTC` writer - CMDTMOUT flag clear bit"]
pub type CMDTMOUTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTTMOUTC` writer - DTTMOUT flag clear bit"]
pub type DTTMOUTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUREC` writer - TXURE flag clear bit"]
pub type TXUREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOREC` writer - RXORE flag clear bit"]
pub type RXOREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDRECVC` writer - CMDRECV flag clear bit"]
pub type CMDRECVC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDSENDC` writer - CMDSEND flag clear bit"]
pub type CMDSENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTENDC` writer - DTEND flag clear bit"]
pub type DTENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STBITEC` writer - STBITE flag clear bit"]
pub type STBITEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTBLKENDC` writer - DTBLKEND flag clear bit"]
pub type DTBLKENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOINTC` writer - SDIOINT flag clear bit"]
pub type SDIOINTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ATAENDC` writer - ATAEND flag clear bit"]
pub type ATAENDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - CCRCERR flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcerrc(&mut self) -> CCRCERRC_W<INTC_SPEC, 0> {
        CCRCERRC_W::new(self)
    }
    #[doc = "Bit 1 - DTCRCERR flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtcrcerrc(&mut self) -> DTCRCERRC_W<INTC_SPEC, 1> {
        DTCRCERRC_W::new(self)
    }
    #[doc = "Bit 2 - CMDTMOUT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtmoutc(&mut self) -> CMDTMOUTC_W<INTC_SPEC, 2> {
        CMDTMOUTC_W::new(self)
    }
    #[doc = "Bit 3 - DTTMOUT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dttmoutc(&mut self) -> DTTMOUTC_W<INTC_SPEC, 3> {
        DTTMOUTC_W::new(self)
    }
    #[doc = "Bit 4 - TXURE flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txurec(&mut self) -> TXUREC_W<INTC_SPEC, 4> {
        TXUREC_W::new(self)
    }
    #[doc = "Bit 5 - RXORE flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxorec(&mut self) -> RXOREC_W<INTC_SPEC, 5> {
        RXOREC_W::new(self)
    }
    #[doc = "Bit 6 - CMDRECV flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrecvc(&mut self) -> CMDRECVC_W<INTC_SPEC, 6> {
        CMDRECVC_W::new(self)
    }
    #[doc = "Bit 7 - CMDSEND flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsendc(&mut self) -> CMDSENDC_W<INTC_SPEC, 7> {
        CMDSENDC_W::new(self)
    }
    #[doc = "Bit 8 - DTEND flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtendc(&mut self) -> DTENDC_W<INTC_SPEC, 8> {
        DTENDC_W::new(self)
    }
    #[doc = "Bit 9 - STBITE flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn stbitec(&mut self) -> STBITEC_W<INTC_SPEC, 9> {
        STBITEC_W::new(self)
    }
    #[doc = "Bit 10 - DTBLKEND flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtblkendc(&mut self) -> DTBLKENDC_W<INTC_SPEC, 10> {
        DTBLKENDC_W::new(self)
    }
    #[doc = "Bit 22 - SDIOINT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn sdiointc(&mut self) -> SDIOINTC_W<INTC_SPEC, 22> {
        SDIOINTC_W::new(self)
    }
    #[doc = "Bit 23 - ATAEND flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ataendc(&mut self) -> ATAENDC_W<INTC_SPEC, 23> {
        ATAENDC_W::new(self)
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
#[doc = "Interrupt clear register (SDIO_INTC)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
