#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `TSIEN` reader - TSI enable"]
pub type TSIEN_R = crate::BitReader;
#[doc = "Field `TSIEN` writer - TSI enable"]
pub type TSIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSIS` reader - TSI start"]
pub type TSIS_R = crate::BitReader;
#[doc = "Field `TSIS` writer - TSI start"]
pub type TSIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGMOD` reader - Trigger mode selection"]
pub type TRGMOD_R = crate::BitReader;
#[doc = "Field `TRGMOD` writer - Trigger mode selection"]
pub type TRGMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EGSEL` reader - Edge selection"]
pub type EGSEL_R = crate::BitReader;
#[doc = "Field `EGSEL` writer - Edge selection"]
pub type EGSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINMOD` reader - Pin mode"]
pub type PINMOD_R = crate::BitReader;
#[doc = "Field `PINMOD` writer - Pin mode"]
pub type PINMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCN` reader - Max cycle number of a sequence"]
pub type MCN_R = crate::FieldReader;
#[doc = "Field `MCN` writer - Max cycle number of a sequence"]
pub type MCN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CTCDIV` reader - CTCLK clock division factor"]
pub type CTCDIV_R = crate::FieldReader;
#[doc = "Field `CTCDIV` writer - CTCLK clock division factor"]
pub type CTCDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ECDIV` reader - ECCLK clock division factor"]
pub type ECDIV_R = crate::BitReader;
#[doc = "Field `ECDIV` writer - ECCLK clock division factor"]
pub type ECDIV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECEN` reader - Extend Charge State Enable"]
pub type ECEN_R = crate::BitReader;
#[doc = "Field `ECEN` writer - Extend Charge State Enable"]
pub type ECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECDT` reader - Extend Charge State Maximum Duration Time"]
pub type ECDT_R = crate::FieldReader;
#[doc = "Field `ECDT` writer - Extend Charge State Maximum Duration Time"]
pub type ECDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `CTDT` reader - Charge Transfer State Duration Time"]
pub type CTDT_R = crate::FieldReader;
#[doc = "Field `CTDT` writer - Charge Transfer State Duration Time"]
pub type CTDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CDT` reader - Charge State Duration Time"]
pub type CDT_R = crate::FieldReader;
#[doc = "Field `CDT` writer - Charge State Duration Time"]
pub type CDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - TSI enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSI start"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger mode selection"]
    #[inline(always)]
    pub fn trgmod(&self) -> TRGMOD_R {
        TRGMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge selection"]
    #[inline(always)]
    pub fn egsel(&self) -> EGSEL_R {
        EGSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin mode"]
    #[inline(always)]
    pub fn pinmod(&self) -> PINMOD_R {
        PINMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max cycle number of a sequence"]
    #[inline(always)]
    pub fn mcn(&self) -> MCN_R {
        MCN_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 12:14 - CTCLK clock division factor"]
    #[inline(always)]
    pub fn ctcdiv(&self) -> CTCDIV_R {
        CTCDIV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - ECCLK clock division factor"]
    #[inline(always)]
    pub fn ecdiv(&self) -> ECDIV_R {
        ECDIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Extend Charge State Enable"]
    #[inline(always)]
    pub fn ecen(&self) -> ECEN_R {
        ECEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Extend Charge State Maximum Duration Time"]
    #[inline(always)]
    pub fn ecdt(&self) -> ECDT_R {
        ECDT_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Charge Transfer State Duration Time"]
    #[inline(always)]
    pub fn ctdt(&self) -> CTDT_R {
        CTDT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Charge State Duration Time"]
    #[inline(always)]
    pub fn cdt(&self) -> CDT_R {
        CDT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TSIEN_W<CTL_SPEC, 0> {
        TSIEN_W::new(self)
    }
    #[doc = "Bit 1 - TSI start"]
    #[inline(always)]
    #[must_use]
    pub fn tsis(&mut self) -> TSIS_W<CTL_SPEC, 1> {
        TSIS_W::new(self)
    }
    #[doc = "Bit 2 - Trigger mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgmod(&mut self) -> TRGMOD_W<CTL_SPEC, 2> {
        TRGMOD_W::new(self)
    }
    #[doc = "Bit 3 - Edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn egsel(&mut self) -> EGSEL_W<CTL_SPEC, 3> {
        EGSEL_W::new(self)
    }
    #[doc = "Bit 4 - Pin mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinmod(&mut self) -> PINMOD_W<CTL_SPEC, 4> {
        PINMOD_W::new(self)
    }
    #[doc = "Bits 5:7 - Max cycle number of a sequence"]
    #[inline(always)]
    #[must_use]
    pub fn mcn(&mut self) -> MCN_W<CTL_SPEC, 5> {
        MCN_W::new(self)
    }
    #[doc = "Bits 12:14 - CTCLK clock division factor"]
    #[inline(always)]
    #[must_use]
    pub fn ctcdiv(&mut self) -> CTCDIV_W<CTL_SPEC, 12> {
        CTCDIV_W::new(self)
    }
    #[doc = "Bit 15 - ECCLK clock division factor"]
    #[inline(always)]
    #[must_use]
    pub fn ecdiv(&mut self) -> ECDIV_W<CTL_SPEC, 15> {
        ECDIV_W::new(self)
    }
    #[doc = "Bit 16 - Extend Charge State Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecen(&mut self) -> ECEN_W<CTL_SPEC, 16> {
        ECEN_W::new(self)
    }
    #[doc = "Bits 17:23 - Extend Charge State Maximum Duration Time"]
    #[inline(always)]
    #[must_use]
    pub fn ecdt(&mut self) -> ECDT_W<CTL_SPEC, 17> {
        ECDT_W::new(self)
    }
    #[doc = "Bits 24:27 - Charge Transfer State Duration Time"]
    #[inline(always)]
    #[must_use]
    pub fn ctdt(&mut self) -> CTDT_W<CTL_SPEC, 24> {
        CTDT_W::new(self)
    }
    #[doc = "Bits 28:31 - Charge State Duration Time"]
    #[inline(always)]
    #[must_use]
    pub fn cdt(&mut self) -> CDT_W<CTL_SPEC, 28> {
        CDT_W::new(self)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
