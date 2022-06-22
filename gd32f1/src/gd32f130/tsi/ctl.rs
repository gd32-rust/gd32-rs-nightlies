#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDT` reader - Charge State Duration Time"]
pub type CDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CDT` writer - Charge State Duration Time"]
pub type CDT_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, 28>;
#[doc = "Field `CTDT` reader - Charge Transfer State Duration Time"]
pub type CTDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTDT` writer - Charge Transfer State Duration Time"]
pub type CTDT_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, 24>;
#[doc = "Field `ECDT` reader - Extend Charge State Maximum Duration Time"]
pub type ECDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECDT` writer - Extend Charge State Maximum Duration Time"]
pub type ECDT_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 7, 17>;
#[doc = "Field `ECEN` reader - Extend Charge State Enable"]
pub type ECEN_R = crate::BitReader<bool>;
#[doc = "Field `ECEN` writer - Extend Charge State Enable"]
pub type ECEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 16>;
#[doc = "Field `ECDIV` reader - ECCLK clock division factor"]
pub type ECDIV_R = crate::BitReader<bool>;
#[doc = "Field `ECDIV` writer - ECCLK clock division factor"]
pub type ECDIV_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 15>;
#[doc = "Field `CTCDIV` reader - CTCLK clock division factor"]
pub type CTCDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTCDIV` writer - CTCLK clock division factor"]
pub type CTCDIV_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, 12>;
#[doc = "Field `MCN` reader - Max cycle number of a sequence"]
pub type MCN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCN` writer - Max cycle number of a sequence"]
pub type MCN_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, 5>;
#[doc = "Field `PINMOD` reader - Pin mode"]
pub type PINMOD_R = crate::BitReader<bool>;
#[doc = "Field `PINMOD` writer - Pin mode"]
pub type PINMOD_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 4>;
#[doc = "Field `EGSEL` reader - Edge selection"]
pub type EGSEL_R = crate::BitReader<bool>;
#[doc = "Field `EGSEL` writer - Edge selection"]
pub type EGSEL_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 3>;
#[doc = "Field `TRGMOD` reader - Trigger mode selection"]
pub type TRGMOD_R = crate::BitReader<bool>;
#[doc = "Field `TRGMOD` writer - Trigger mode selection"]
pub type TRGMOD_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
#[doc = "Field `TSIS` reader - TSI start"]
pub type TSIS_R = crate::BitReader<bool>;
#[doc = "Field `TSIS` writer - TSI start"]
pub type TSIS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `TSIEN` reader - TSI enable"]
pub type TSIEN_R = crate::BitReader<bool>;
#[doc = "Field `TSIEN` writer - TSI enable"]
pub type TSIEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 28:31 - Charge State Duration Time"]
    #[inline(always)]
    pub fn cdt(&self) -> CDT_R {
        CDT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Charge Transfer State Duration Time"]
    #[inline(always)]
    pub fn ctdt(&self) -> CTDT_R {
        CTDT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 17:23 - Extend Charge State Maximum Duration Time"]
    #[inline(always)]
    pub fn ecdt(&self) -> ECDT_R {
        ECDT_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Extend Charge State Enable"]
    #[inline(always)]
    pub fn ecen(&self) -> ECEN_R {
        ECEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - ECCLK clock division factor"]
    #[inline(always)]
    pub fn ecdiv(&self) -> ECDIV_R {
        ECDIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - CTCLK clock division factor"]
    #[inline(always)]
    pub fn ctcdiv(&self) -> CTCDIV_R {
        CTCDIV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Max cycle number of a sequence"]
    #[inline(always)]
    pub fn mcn(&self) -> MCN_R {
        MCN_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 4 - Pin mode"]
    #[inline(always)]
    pub fn pinmod(&self) -> PINMOD_R {
        PINMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge selection"]
    #[inline(always)]
    pub fn egsel(&self) -> EGSEL_R {
        EGSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger mode selection"]
    #[inline(always)]
    pub fn trgmod(&self) -> TRGMOD_R {
        TRGMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - TSI start"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TSI enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - Charge State Duration Time"]
    #[inline(always)]
    pub fn cdt(&mut self) -> CDT_W {
        CDT_W::new(self)
    }
    #[doc = "Bits 24:27 - Charge Transfer State Duration Time"]
    #[inline(always)]
    pub fn ctdt(&mut self) -> CTDT_W {
        CTDT_W::new(self)
    }
    #[doc = "Bits 17:23 - Extend Charge State Maximum Duration Time"]
    #[inline(always)]
    pub fn ecdt(&mut self) -> ECDT_W {
        ECDT_W::new(self)
    }
    #[doc = "Bit 16 - Extend Charge State Enable"]
    #[inline(always)]
    pub fn ecen(&mut self) -> ECEN_W {
        ECEN_W::new(self)
    }
    #[doc = "Bit 15 - ECCLK clock division factor"]
    #[inline(always)]
    pub fn ecdiv(&mut self) -> ECDIV_W {
        ECDIV_W::new(self)
    }
    #[doc = "Bits 12:14 - CTCLK clock division factor"]
    #[inline(always)]
    pub fn ctcdiv(&mut self) -> CTCDIV_W {
        CTCDIV_W::new(self)
    }
    #[doc = "Bits 5:7 - Max cycle number of a sequence"]
    #[inline(always)]
    pub fn mcn(&mut self) -> MCN_W {
        MCN_W::new(self)
    }
    #[doc = "Bit 4 - Pin mode"]
    #[inline(always)]
    pub fn pinmod(&mut self) -> PINMOD_W {
        PINMOD_W::new(self)
    }
    #[doc = "Bit 3 - Edge selection"]
    #[inline(always)]
    pub fn egsel(&mut self) -> EGSEL_W {
        EGSEL_W::new(self)
    }
    #[doc = "Bit 2 - Trigger mode selection"]
    #[inline(always)]
    pub fn trgmod(&mut self) -> TRGMOD_W {
        TRGMOD_W::new(self)
    }
    #[doc = "Bit 1 - TSI start"]
    #[inline(always)]
    pub fn tsis(&mut self) -> TSIS_W {
        TSIS_W::new(self)
    }
    #[doc = "Bit 0 - TSI enable"]
    #[inline(always)]
    pub fn tsien(&mut self) -> TSIEN_W {
        TSIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
