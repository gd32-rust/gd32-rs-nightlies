#[doc = "Register `RFCS` reader"]
pub type R = crate::R<RFCS_SPEC>;
#[doc = "Register `RFCS` writer"]
pub type W = crate::W<RFCS_SPEC>;
#[doc = "Field `ELNACK` reader - Early NACK when smartcard mode is selected"]
pub type ELNACK_R = crate::BitReader;
#[doc = "Field `ELNACK` writer - Early NACK when smartcard mode is selected"]
pub type ELNACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFEN` reader - Receive FIFO enable"]
pub type RFEN_R = crate::BitReader;
#[doc = "Field `RFEN` writer - Receive FIFO enable"]
pub type RFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFFIE` reader - Receive FIFO full interrupt enable"]
pub type RFFIE_R = crate::BitReader;
#[doc = "Field `RFFIE` writer - Receive FIFO full interrupt enable"]
pub type RFFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFE` reader - Receive FIFO empty flag"]
pub type RFE_R = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO full flag"]
pub type RFF_R = crate::BitReader;
#[doc = "Field `RFCNT` reader - Receive FIFO counter number"]
pub type RFCNT_R = crate::FieldReader;
#[doc = "Field `RFFINT` reader - Receive FIFO full interrupt flag"]
pub type RFFINT_R = crate::BitReader;
#[doc = "Field `RFFINT` writer - Receive FIFO full interrupt flag"]
pub type RFFINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Early NACK when smartcard mode is selected"]
    #[inline(always)]
    pub fn elnack(&self) -> ELNACK_R {
        ELNACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive FIFO empty flag"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO full flag"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Receive FIFO counter number"]
    #[inline(always)]
    pub fn rfcnt(&self) -> RFCNT_R {
        RFCNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Receive FIFO full interrupt flag"]
    #[inline(always)]
    pub fn rffint(&self) -> RFFINT_R {
        RFFINT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early NACK when smartcard mode is selected"]
    #[inline(always)]
    #[must_use]
    pub fn elnack(&mut self) -> ELNACK_W<RFCS_SPEC, 0> {
        ELNACK_W::new(self)
    }
    #[doc = "Bit 8 - Receive FIFO enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RFEN_W<RFCS_SPEC, 8> {
        RFEN_W::new(self)
    }
    #[doc = "Bit 9 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie(&mut self) -> RFFIE_W<RFCS_SPEC, 9> {
        RFFIE_W::new(self)
    }
    #[doc = "Bit 15 - Receive FIFO full interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rffint(&mut self) -> RFFINT_W<RFCS_SPEC, 15> {
        RFFINT_W::new(self)
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
#[doc = "receive FIFO control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFCS_SPEC;
impl crate::RegisterSpec for RFCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcs::R`](R) reader structure"]
impl crate::Readable for RFCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfcs::W`](W) writer structure"]
impl crate::Writable for RFCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFCS to value 0x0400"]
impl crate::Resettable for RFCS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
