#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `ADDM` reader - Address detection mode"]
pub type ADDM_R = crate::BitReader;
#[doc = "Field `ADDM` writer - Address detection mode"]
pub type ADDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LBLEN` reader - LIN break frame length"]
pub type LBLEN_R = crate::BitReader;
#[doc = "Field `LBLEN` writer - LIN break frame length"]
pub type LBLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLEN` reader - CK length"]
pub type CLEN_R = crate::BitReader;
#[doc = "Field `CLEN` writer - CK length"]
pub type CLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPH` reader - Clock phase"]
pub type CPH_R = crate::BitReader;
#[doc = "Field `CPH` writer - Clock phase"]
pub type CPH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPL` reader - Clock polarity"]
pub type CPL_R = crate::BitReader;
#[doc = "Field `CPL` writer - Clock polarity"]
pub type CPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKEN` reader - CK pin enable"]
pub type CKEN_R = crate::BitReader;
#[doc = "Field `CKEN` writer - CK pin enable"]
pub type CKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STB` reader - STOP bits length"]
pub type STB_R = crate::FieldReader;
#[doc = "Field `STB` writer - STOP bits length"]
pub type STB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LMEN` reader - LIN mode enable"]
pub type LMEN_R = crate::BitReader;
#[doc = "Field `LMEN` writer - LIN mode enable"]
pub type LMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRP` reader - Swap TX/RX pins"]
pub type STRP_R = crate::BitReader;
#[doc = "Field `STRP` writer - Swap TX/RX pins"]
pub type STRP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RINV` reader - RX pin level inversion"]
pub type RINV_R = crate::BitReader;
#[doc = "Field `RINV` writer - RX pin level inversion"]
pub type RINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TINV` reader - TX pin level inversion"]
pub type TINV_R = crate::BitReader;
#[doc = "Field `TINV` writer - TX pin level inversion"]
pub type TINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DINV` reader - Data bit level inversion"]
pub type DINV_R = crate::BitReader;
#[doc = "Field `DINV` writer - Data bit level inversion"]
pub type DINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSBF` reader - Most significant bit first"]
pub type MSBF_R = crate::BitReader;
#[doc = "Field `MSBF` writer - Most significant bit first"]
pub type MSBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABDEN` reader - Auto baud rate enable"]
pub type ABDEN_R = crate::BitReader;
#[doc = "Field `ABDEN` writer - Auto baud rate enable"]
pub type ABDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABDM` reader - Auto baud rate mode"]
pub type ABDM_R = crate::FieldReader;
#[doc = "Field `ABDM` writer - Auto baud rate mode"]
pub type ABDM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RTEN` reader - Receiver timeout enable"]
pub type RTEN_R = crate::BitReader;
#[doc = "Field `RTEN` writer - Receiver timeout enable"]
pub type RTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDR` reader - Address of the USART terminal"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address of the USART terminal"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 4 - Address detection mode"]
    #[inline(always)]
    pub fn addm(&self) -> ADDM_R {
        ADDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&self) -> LBLEN_R {
        LBLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CK length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&self) -> CPH_R {
        CPH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&self) -> STB_R {
        STB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LMEN_R {
        LMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn strp(&self) -> STRP_R {
        STRP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin level inversion"]
    #[inline(always)]
    pub fn rinv(&self) -> RINV_R {
        RINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin level inversion"]
    #[inline(always)]
    pub fn tinv(&self) -> TINV_R {
        TINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data bit level inversion"]
    #[inline(always)]
    pub fn dinv(&self) -> DINV_R {
        DINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abden(&self) -> ABDEN_R {
        ABDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abdm(&self) -> ABDM_R {
        ABDM_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&self) -> RTEN_R {
        RTEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART terminal"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Address detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn addm(&mut self) -> ADDM_W<CTL1_SPEC, 4> {
        ADDM_W::new(self)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    #[must_use]
    pub fn lblen(&mut self) -> LBLEN_W<CTL1_SPEC, 5> {
        LBLEN_W::new(self)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<CTL1_SPEC, 6> {
        LBDIE_W::new(self)
    }
    #[doc = "Bit 8 - CK length"]
    #[inline(always)]
    #[must_use]
    pub fn clen(&mut self) -> CLEN_W<CTL1_SPEC, 8> {
        CLEN_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cph(&mut self) -> CPH_W<CTL1_SPEC, 9> {
        CPH_W::new(self)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<CTL1_SPEC, 10> {
        CPL_W::new(self)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CKEN_W<CTL1_SPEC, 11> {
        CKEN_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> STB_W<CTL1_SPEC, 12> {
        STB_W::new(self)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmen(&mut self) -> LMEN_W<CTL1_SPEC, 14> {
        LMEN_W::new(self)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    #[must_use]
    pub fn strp(&mut self) -> STRP_W<CTL1_SPEC, 15> {
        STRP_W::new(self)
    }
    #[doc = "Bit 16 - RX pin level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rinv(&mut self) -> RINV_W<CTL1_SPEC, 16> {
        RINV_W::new(self)
    }
    #[doc = "Bit 17 - TX pin level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn tinv(&mut self) -> TINV_W<CTL1_SPEC, 17> {
        TINV_W::new(self)
    }
    #[doc = "Bit 18 - Data bit level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn dinv(&mut self) -> DINV_W<CTL1_SPEC, 18> {
        DINV_W::new(self)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<CTL1_SPEC, 19> {
        MSBF_W::new(self)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    #[must_use]
    pub fn abden(&mut self) -> ABDEN_W<CTL1_SPEC, 20> {
        ABDEN_W::new(self)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    #[must_use]
    pub fn abdm(&mut self) -> ABDM_W<CTL1_SPEC, 21> {
        ABDM_W::new(self)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rten(&mut self) -> RTEN_W<CTL1_SPEC, 23> {
        RTEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Address of the USART terminal"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<CTL1_SPEC, 24> {
        ADDR_W::new(self)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
