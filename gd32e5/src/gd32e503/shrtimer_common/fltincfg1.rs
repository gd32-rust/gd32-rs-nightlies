#[doc = "Register `FLTINCFG1` reader"]
pub type R = crate::R<FLTINCFG1_SPEC>;
#[doc = "Register `FLTINCFG1` writer"]
pub type W = crate::W<FLTINCFG1_SPEC>;
#[doc = "Field `FLT4INEN` reader - Fault 4 input enable"]
pub type FLT4INEN_R = crate::BitReader;
#[doc = "Field `FLT4INEN` writer - Fault 4 input enable"]
pub type FLT4INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT4INP` reader - Fault 4 input polarity"]
pub type FLT4INP_R = crate::BitReader;
#[doc = "Field `FLT4INP` writer - Fault 4 input polarity"]
pub type FLT4INP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT4INSRC` reader - Fault 4 input source"]
pub type FLT4INSRC_R = crate::BitReader;
#[doc = "Field `FLT4INSRC` writer - Fault 4 input source"]
pub type FLT4INSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT4INFC` reader - Fault 4 input filter control"]
pub type FLT4INFC_R = crate::FieldReader;
#[doc = "Field `FLT4INFC` writer - Fault 4 input filter control"]
pub type FLT4INFC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FLT4INPROT` reader - Protect fault 4 input configuration"]
pub type FLT4INPROT_R = crate::BitReader;
#[doc = "Field `FLT4INPROT` writer - Protect fault 4 input configuration"]
pub type FLT4INPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLTFDIV` reader - Fault input digital filter clock division"]
pub type FLTFDIV_R = crate::FieldReader;
#[doc = "Field `FLTFDIV` writer - Fault input digital filter clock division"]
pub type FLTFDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - Fault 4 input enable"]
    #[inline(always)]
    pub fn flt4inen(&self) -> FLT4INEN_R {
        FLT4INEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 4 input polarity"]
    #[inline(always)]
    pub fn flt4inp(&self) -> FLT4INP_R {
        FLT4INP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 4 input source"]
    #[inline(always)]
    pub fn flt4insrc(&self) -> FLT4INSRC_R {
        FLT4INSRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Fault 4 input filter control"]
    #[inline(always)]
    pub fn flt4infc(&self) -> FLT4INFC_R {
        FLT4INFC_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Protect fault 4 input configuration"]
    #[inline(always)]
    pub fn flt4inprot(&self) -> FLT4INPROT_R {
        FLT4INPROT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Fault input digital filter clock division"]
    #[inline(always)]
    pub fn fltfdiv(&self) -> FLTFDIV_R {
        FLTFDIV_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 4 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4inen(&mut self) -> FLT4INEN_W<FLTINCFG1_SPEC, 0> {
        FLT4INEN_W::new(self)
    }
    #[doc = "Bit 1 - Fault 4 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt4inp(&mut self) -> FLT4INP_W<FLTINCFG1_SPEC, 1> {
        FLT4INP_W::new(self)
    }
    #[doc = "Bit 2 - Fault 4 input source"]
    #[inline(always)]
    #[must_use]
    pub fn flt4insrc(&mut self) -> FLT4INSRC_W<FLTINCFG1_SPEC, 2> {
        FLT4INSRC_W::new(self)
    }
    #[doc = "Bits 3:6 - Fault 4 input filter control"]
    #[inline(always)]
    #[must_use]
    pub fn flt4infc(&mut self) -> FLT4INFC_W<FLTINCFG1_SPEC, 3> {
        FLT4INFC_W::new(self)
    }
    #[doc = "Bit 7 - Protect fault 4 input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn flt4inprot(&mut self) -> FLT4INPROT_W<FLTINCFG1_SPEC, 7> {
        FLT4INPROT_W::new(self)
    }
    #[doc = "Bits 24:25 - Fault input digital filter clock division"]
    #[inline(always)]
    #[must_use]
    pub fn fltfdiv(&mut self) -> FLTFDIV_W<FLTINCFG1_SPEC, 24> {
        FLTFDIV_W::new(self)
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
#[doc = "SHRTIMER fault input configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltincfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltincfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTINCFG1_SPEC;
impl crate::RegisterSpec for FLTINCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltincfg1::R`](R) reader structure"]
impl crate::Readable for FLTINCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fltincfg1::W`](W) writer structure"]
impl crate::Writable for FLTINCFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTINCFG1 to value 0"]
impl crate::Resettable for FLTINCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
