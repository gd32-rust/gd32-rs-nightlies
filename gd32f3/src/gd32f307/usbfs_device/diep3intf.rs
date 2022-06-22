#[doc = "Register `DIEP3INTF` reader"]
pub struct R(crate::R<DIEP3INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP3INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP3INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP3INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP3INTF` writer"]
pub struct W(crate::W<DIEP3INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP3INTF_SPEC>;
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
impl From<crate::W<DIEP3INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP3INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub type TXFE_R = crate::BitReader<bool>;
#[doc = "Field `IEPNE` reader - IN endpoint NAK effective"]
pub type IEPNE_R = crate::BitReader<bool>;
#[doc = "Field `IEPNE` writer - IN endpoint NAK effective"]
pub type IEPNE_W<'a> = crate::BitWriter<'a, u32, DIEP3INTF_SPEC, bool, 6>;
#[doc = "Field `EPTXFUD` reader - Endpoint Tx FIFO underrun"]
pub type EPTXFUD_R = crate::BitReader<bool>;
#[doc = "Field `EPTXFUD` writer - Endpoint Tx FIFO underrun"]
pub type EPTXFUD_W<'a> = crate::BitWriter<'a, u32, DIEP3INTF_SPEC, bool, 4>;
#[doc = "Field `CITO` reader - Control in timeout interrupt"]
pub type CITO_R = crate::BitReader<bool>;
#[doc = "Field `CITO` writer - Control in timeout interrupt"]
pub type CITO_W<'a> = crate::BitWriter<'a, u32, DIEP3INTF_SPEC, bool, 3>;
#[doc = "Field `EPDIS` reader - Endpoint finished"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPDIS` writer - Endpoint finished"]
pub type EPDIS_W<'a> = crate::BitWriter<'a, u32, DIEP3INTF_SPEC, bool, 1>;
#[doc = "Field `TF` reader - Transfer finished"]
pub type TF_R = crate::BitReader<bool>;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TF_W<'a> = crate::BitWriter<'a, u32, DIEP3INTF_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn iepne(&self) -> IEPNE_R {
        IEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun"]
    #[inline(always)]
    pub fn eptxfud(&self) -> EPTXFUD_R {
        EPTXFUD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Control in timeout interrupt"]
    #[inline(always)]
    pub fn cito(&self) -> CITO_R {
        CITO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint finished"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn iepne(&mut self) -> IEPNE_W {
        IEPNE_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun"]
    #[inline(always)]
    pub fn eptxfud(&mut self) -> EPTXFUD_W {
        EPTXFUD_W::new(self)
    }
    #[doc = "Bit 3 - Control in timeout interrupt"]
    #[inline(always)]
    pub fn cito(&mut self) -> CITO_W {
        CITO_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint finished"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W {
        EPDIS_W::new(self)
    }
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&mut self) -> TF_W {
        TF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3intf](index.html) module"]
pub struct DIEP3INTF_SPEC;
impl crate::RegisterSpec for DIEP3INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep3intf::R](R) reader structure"]
impl crate::Readable for DIEP3INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep3intf::W](W) writer structure"]
impl crate::Writable for DIEP3INTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEP3INTF to value 0x80"]
impl crate::Resettable for DIEP3INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
