#[doc = "Register `SAMCS` reader"]
pub type R = crate::R<SAMCS_SPEC>;
#[doc = "Register `SAMCS` writer"]
pub type W = crate::W<SAMCS_SPEC>;
#[doc = "Field `SAMEN` reader - SAM_V interface enable"]
pub type SAMEN_R = crate::BitReader;
#[doc = "Field `SAMEN` writer - SAM_V interface enable"]
pub type SAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOEN` reader - SAM_V interface timeout detect enable"]
pub type STOEN_R = crate::BitReader;
#[doc = "Field `STOEN` writer - SAM_V interface timeout detect enable"]
pub type STOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFFIE` reader - Tx frame fall interrupt enable"]
pub type TFFIE_R = crate::BitReader;
#[doc = "Field `TFFIE` writer - Tx frame fall interrupt enable"]
pub type TFFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFRIE` reader - Tx frame rise interrupt enable"]
pub type TFRIE_R = crate::BitReader;
#[doc = "Field `TFRIE` writer - Tx frame rise interrupt enable"]
pub type TFRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFFIE` reader - Rx frame fall interrupt enable"]
pub type RFFIE_R = crate::BitReader;
#[doc = "Field `RFFIE` writer - Rx frame fall interrupt enable"]
pub type RFFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFRIE` reader - Rx frame rise interrupt enable"]
pub type RFRIE_R = crate::BitReader;
#[doc = "Field `RFRIE` writer - Rx frame rise interrupt enable"]
pub type RFRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXF` reader - level of tx frame signal"]
pub type TXF_R = crate::BitReader;
#[doc = "Field `TXF` writer - level of tx frame signal"]
pub type TXF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXF` reader - level of rx frame signal"]
pub type RXF_R = crate::BitReader;
#[doc = "Field `RXF` writer - level of rx frame signal"]
pub type RXF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFF` reader - Txframe fall flag"]
pub type TFF_R = crate::BitReader;
#[doc = "Field `TFF` writer - Txframe fall flag"]
pub type TFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFR` reader - Txframe rise flag"]
pub type TFR_R = crate::BitReader;
#[doc = "Field `TFR` writer - Txframe rise flag"]
pub type TFR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFF` reader - Rxframe fall flag"]
pub type RFF_R = crate::BitReader;
#[doc = "Field `RFF` writer - Rxframe fall flag"]
pub type RFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFR` reader - Rxframe rise flag"]
pub type RFR_R = crate::BitReader;
#[doc = "Field `RFR` writer - Rxframe rise flag"]
pub type RFR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&self) -> SAMEN_R {
        SAMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&self) -> STOEN_R {
        STOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx frame fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx frame rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&self) -> TFRIE_R {
        TFRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx frame fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx frame rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&self) -> RFRIE_R {
        RFRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - level of tx frame signal"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - level of rx frame signal"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&self) -> RFR_R {
        RFR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    #[must_use]
    pub fn samen(&mut self) -> SAMEN_W<SAMCS_SPEC, 0> {
        SAMEN_W::new(self)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn stoen(&mut self) -> STOEN_W<SAMCS_SPEC, 1> {
        STOEN_W::new(self)
    }
    #[doc = "Bit 4 - Tx frame fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tffie(&mut self) -> TFFIE_W<SAMCS_SPEC, 4> {
        TFFIE_W::new(self)
    }
    #[doc = "Bit 5 - Tx frame rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfrie(&mut self) -> TFRIE_W<SAMCS_SPEC, 5> {
        TFRIE_W::new(self)
    }
    #[doc = "Bit 6 - Rx frame fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie(&mut self) -> RFFIE_W<SAMCS_SPEC, 6> {
        RFFIE_W::new(self)
    }
    #[doc = "Bit 7 - Rx frame rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfrie(&mut self) -> RFRIE_W<SAMCS_SPEC, 7> {
        RFRIE_W::new(self)
    }
    #[doc = "Bit 8 - level of tx frame signal"]
    #[inline(always)]
    #[must_use]
    pub fn txf(&mut self) -> TXF_W<SAMCS_SPEC, 8> {
        TXF_W::new(self)
    }
    #[doc = "Bit 9 - level of rx frame signal"]
    #[inline(always)]
    #[must_use]
    pub fn rxf(&mut self) -> RXF_W<SAMCS_SPEC, 9> {
        RXF_W::new(self)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    #[must_use]
    pub fn tff(&mut self) -> TFF_W<SAMCS_SPEC, 12> {
        TFF_W::new(self)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    #[must_use]
    pub fn tfr(&mut self) -> TFR_W<SAMCS_SPEC, 13> {
        TFR_W::new(self)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    #[must_use]
    pub fn rff(&mut self) -> RFF_W<SAMCS_SPEC, 14> {
        RFF_W::new(self)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfr(&mut self) -> RFR_W<SAMCS_SPEC, 15> {
        RFR_W::new(self)
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
#[doc = "SAM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMCS_SPEC;
impl crate::RegisterSpec for SAMCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samcs::R`](R) reader structure"]
impl crate::Readable for SAMCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`samcs::W`](W) writer structure"]
impl crate::Writable for SAMCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMCS to value 0"]
impl crate::Resettable for SAMCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
