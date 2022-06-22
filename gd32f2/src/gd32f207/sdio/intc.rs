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
#[doc = "Field `CCRCERRC` writer - CCRCERR flag clear bit"]
pub type CCRCERRC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 0>;
#[doc = "Field `DTCRCERRC` writer - DTCRCERR flag clear bit"]
pub type DTCRCERRC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 1>;
#[doc = "Field `CMDTMOUTC` writer - CMDTMOUT flag clear bit"]
pub type CMDTMOUTC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 2>;
#[doc = "Field `DTTMOUTC` writer - DTTMOUT flag clear bit"]
pub type DTTMOUTC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 3>;
#[doc = "Field `TXUREC` writer - TXURE flag clear bit"]
pub type TXUREC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 4>;
#[doc = "Field `RXOREC` writer - RXORE flag clear bit"]
pub type RXOREC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 5>;
#[doc = "Field `CMDRECVC` writer - CMDRECV flag clear bit"]
pub type CMDRECVC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 6>;
#[doc = "Field `CMDSENDC` writer - CMDSEND flag clear bit"]
pub type CMDSENDC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 7>;
#[doc = "Field `DTENDC` writer - DTEND flag clear bit"]
pub type DTENDC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 8>;
#[doc = "Field `STBITEC` writer - STBITE flag clear bit"]
pub type STBITEC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 9>;
#[doc = "Field `DTBLKENDC` writer - DTBLKEND flag clear bit"]
pub type DTBLKENDC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 10>;
#[doc = "Field `SDIOINTC` writer - SDIOINT flag clear bit"]
pub type SDIOINTC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 22>;
#[doc = "Field `ATAENDC` writer - ATAEND flag clear bit"]
pub type ATAENDC_W<'a> = crate::BitWriter<'a, u32, INTC_SPEC, bool, 23>;
impl W {
    #[doc = "Bit 0 - CCRCERR flag clear bit"]
    #[inline(always)]
    pub fn ccrcerrc(&mut self) -> CCRCERRC_W {
        CCRCERRC_W::new(self)
    }
    #[doc = "Bit 1 - DTCRCERR flag clear bit"]
    #[inline(always)]
    pub fn dtcrcerrc(&mut self) -> DTCRCERRC_W {
        DTCRCERRC_W::new(self)
    }
    #[doc = "Bit 2 - CMDTMOUT flag clear bit"]
    #[inline(always)]
    pub fn cmdtmoutc(&mut self) -> CMDTMOUTC_W {
        CMDTMOUTC_W::new(self)
    }
    #[doc = "Bit 3 - DTTMOUT flag clear bit"]
    #[inline(always)]
    pub fn dttmoutc(&mut self) -> DTTMOUTC_W {
        DTTMOUTC_W::new(self)
    }
    #[doc = "Bit 4 - TXURE flag clear bit"]
    #[inline(always)]
    pub fn txurec(&mut self) -> TXUREC_W {
        TXUREC_W::new(self)
    }
    #[doc = "Bit 5 - RXORE flag clear bit"]
    #[inline(always)]
    pub fn rxorec(&mut self) -> RXOREC_W {
        RXOREC_W::new(self)
    }
    #[doc = "Bit 6 - CMDRECV flag clear bit"]
    #[inline(always)]
    pub fn cmdrecvc(&mut self) -> CMDRECVC_W {
        CMDRECVC_W::new(self)
    }
    #[doc = "Bit 7 - CMDSEND flag clear bit"]
    #[inline(always)]
    pub fn cmdsendc(&mut self) -> CMDSENDC_W {
        CMDSENDC_W::new(self)
    }
    #[doc = "Bit 8 - DTEND flag clear bit"]
    #[inline(always)]
    pub fn dtendc(&mut self) -> DTENDC_W {
        DTENDC_W::new(self)
    }
    #[doc = "Bit 9 - STBITE flag clear bit"]
    #[inline(always)]
    pub fn stbitec(&mut self) -> STBITEC_W {
        STBITEC_W::new(self)
    }
    #[doc = "Bit 10 - DTBLKEND flag clear bit"]
    #[inline(always)]
    pub fn dtblkendc(&mut self) -> DTBLKENDC_W {
        DTBLKENDC_W::new(self)
    }
    #[doc = "Bit 22 - SDIOINT flag clear bit"]
    #[inline(always)]
    pub fn sdiointc(&mut self) -> SDIOINTC_W {
        SDIOINTC_W::new(self)
    }
    #[doc = "Bit 23 - ATAEND flag clear bit"]
    #[inline(always)]
    pub fn ataendc(&mut self) -> ATAENDC_W {
        ATAENDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register (SDIO_INTC)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](index.html) module"]
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
