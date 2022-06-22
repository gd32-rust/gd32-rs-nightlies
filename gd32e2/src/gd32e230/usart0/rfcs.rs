#[doc = "Register `RFCS` reader"]
pub struct R(crate::R<RFCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCS` writer"]
pub struct W(crate::W<RFCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCS_SPEC>;
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
impl From<crate::W<RFCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFFINT` reader - Receive FIFO full interrupt flag"]
pub type RFFINT_R = crate::BitReader<bool>;
#[doc = "Field `RFFINT` writer - Receive FIFO full interrupt flag"]
pub type RFFINT_W<'a> = crate::BitWriter<'a, u32, RFCS_SPEC, bool, 15>;
#[doc = "Field `RFCNT` reader - Receive FIFO count number"]
pub type RFCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFF` reader - Receive FIFO full flag"]
pub type RFF_R = crate::BitReader<bool>;
#[doc = "Field `RFE` reader - Receive FIFO empty flag"]
pub type RFE_R = crate::BitReader<bool>;
#[doc = "Field `RFFIE` reader - Receive FIFO full interrupt enable"]
pub type RFFIE_R = crate::BitReader<bool>;
#[doc = "Field `RFFIE` writer - Receive FIFO full interrupt enable"]
pub type RFFIE_W<'a> = crate::BitWriter<'a, u32, RFCS_SPEC, bool, 9>;
#[doc = "Field `RFEN` reader - Receive FIFO enable"]
pub type RFEN_R = crate::BitReader<bool>;
#[doc = "Field `RFEN` writer - Receive FIFO enable"]
pub type RFEN_W<'a> = crate::BitWriter<'a, u32, RFCS_SPEC, bool, 8>;
#[doc = "Field `ELNACK` reader - Early NKEN when smartcard mode is selected"]
pub type ELNACK_R = crate::BitReader<bool>;
#[doc = "Field `ELNACK` writer - Early NKEN when smartcard mode is selected"]
pub type ELNACK_W<'a> = crate::BitWriter<'a, u32, RFCS_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 15 - Receive FIFO full interrupt flag"]
    #[inline(always)]
    pub fn rffint(&self) -> RFFINT_R {
        RFFINT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Receive FIFO count number"]
    #[inline(always)]
    pub fn rfcnt(&self) -> RFCNT_R {
        RFCNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - Receive FIFO full flag"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive FIFO empty flag"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 0 - Early NKEN when smartcard mode is selected"]
    #[inline(always)]
    pub fn elnack(&self) -> ELNACK_R {
        ELNACK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Receive FIFO full interrupt flag"]
    #[inline(always)]
    pub fn rffint(&mut self) -> RFFINT_W {
        RFFINT_W::new(self)
    }
    #[doc = "Bit 9 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&mut self) -> RFFIE_W {
        RFFIE_W::new(self)
    }
    #[doc = "Bit 8 - Receive FIFO enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W::new(self)
    }
    #[doc = "Bit 0 - Early NKEN when smartcard mode is selected"]
    #[inline(always)]
    pub fn elnack(&mut self) -> ELNACK_W {
        ELNACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART receive FIFO control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcs](index.html) module"]
pub struct RFCS_SPEC;
impl crate::RegisterSpec for RFCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcs::R](R) reader structure"]
impl crate::Readable for RFCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcs::W](W) writer structure"]
impl crate::Writable for RFCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFCS to value 0x0400"]
impl crate::Resettable for RFCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
