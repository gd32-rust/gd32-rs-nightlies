#[doc = "Register `CTL3` reader"]
pub type R = crate::R<CTL3_SPEC>;
#[doc = "Register `CTL3` writer"]
pub type W = crate::W<CTL3_SPEC>;
#[doc = "Field `RTEN` reader - Receiver timeout enable"]
pub type RTEN_R = crate::BitReader;
#[doc = "Field `RTEN` writer - Receiver timeout enable"]
pub type RTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCRTNUM` reader - Smartcard auto-retry number"]
pub type SCRTNUM_R = crate::FieldReader;
#[doc = "Field `SCRTNUM` writer - Smartcard auto-retry number"]
pub type SCRTNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RTIE` reader - Interrupt enable bit of receive timeout event"]
pub type RTIE_R = crate::BitReader;
#[doc = "Field `RTIE` writer - Interrupt enable bit of receive timeout event"]
pub type RTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EBIE` reader - Interrupt enable bit of end of block event"]
pub type EBIE_R = crate::BitReader;
#[doc = "Field `EBIE` writer - Interrupt enable bit of end of block event"]
pub type EBIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl R {
    #[doc = "Bit 0 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&self) -> RTEN_R {
        RTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Smartcard auto-retry number"]
    #[inline(always)]
    pub fn scrtnum(&self) -> SCRTNUM_R {
        SCRTNUM_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Interrupt enable bit of receive timeout event"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable bit of end of block event"]
    #[inline(always)]
    pub fn ebie(&self) -> EBIE_R {
        EBIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - RX pin level inversion"]
    #[inline(always)]
    pub fn rinv(&self) -> RINV_R {
        RINV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX pin level inversion"]
    #[inline(always)]
    pub fn tinv(&self) -> TINV_R {
        TINV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data bit level inversion"]
    #[inline(always)]
    pub fn dinv(&self) -> DINV_R {
        DINV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rten(&mut self) -> RTEN_W<CTL3_SPEC, 0> {
        RTEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Smartcard auto-retry number"]
    #[inline(always)]
    #[must_use]
    pub fn scrtnum(&mut self) -> SCRTNUM_W<CTL3_SPEC, 1> {
        SCRTNUM_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable bit of receive timeout event"]
    #[inline(always)]
    #[must_use]
    pub fn rtie(&mut self) -> RTIE_W<CTL3_SPEC, 4> {
        RTIE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable bit of end of block event"]
    #[inline(always)]
    #[must_use]
    pub fn ebie(&mut self) -> EBIE_W<CTL3_SPEC, 5> {
        EBIE_W::new(self)
    }
    #[doc = "Bit 8 - RX pin level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rinv(&mut self) -> RINV_W<CTL3_SPEC, 8> {
        RINV_W::new(self)
    }
    #[doc = "Bit 9 - TX pin level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn tinv(&mut self) -> TINV_W<CTL3_SPEC, 9> {
        TINV_W::new(self)
    }
    #[doc = "Bit 10 - Data bit level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn dinv(&mut self) -> DINV_W<CTL3_SPEC, 10> {
        DINV_W::new(self)
    }
    #[doc = "Bit 11 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<CTL3_SPEC, 11> {
        MSBF_W::new(self)
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
#[doc = "Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL3_SPEC;
impl crate::RegisterSpec for CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl3::R`](R) reader structure"]
impl crate::Readable for CTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl3::W`](W) writer structure"]
impl crate::Writable for CTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL3 to value 0"]
impl crate::Resettable for CTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
