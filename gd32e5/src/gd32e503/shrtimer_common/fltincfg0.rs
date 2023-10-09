#[doc = "Register `FLTINCFG0` reader"]
pub type R = crate::R<FLTINCFG0_SPEC>;
#[doc = "Register `FLTINCFG0` writer"]
pub type W = crate::W<FLTINCFG0_SPEC>;
#[doc = "Field `FLT0INEN` reader - Fault 0 input enable"]
pub type FLT0INEN_R = crate::BitReader;
#[doc = "Field `FLT0INEN` writer - Fault 0 input enable"]
pub type FLT0INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT0INP` reader - Fault 0 input polarity"]
pub type FLT0INP_R = crate::BitReader;
#[doc = "Field `FLT0INP` writer - Fault 0 input polarity"]
pub type FLT0INP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT0INSRC` reader - Fault 0 input source"]
pub type FLT0INSRC_R = crate::BitReader;
#[doc = "Field `FLT0INSRC` writer - Fault 0 input source"]
pub type FLT0INSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT0INFC` reader - Fault 0 input filter control"]
pub type FLT0INFC_R = crate::FieldReader;
#[doc = "Field `FLT0INFC` writer - Fault 0 input filter control"]
pub type FLT0INFC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FLT0INPROT` reader - Protect fault 0 input configuration"]
pub type FLT0INPROT_R = crate::BitReader;
#[doc = "Field `FLT0INPROT` writer - Protect fault 0 input configuration"]
pub type FLT0INPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT1INEN` reader - Fault 1 input enable"]
pub type FLT1INEN_R = crate::BitReader;
#[doc = "Field `FLT1INEN` writer - Fault 1 input enable"]
pub type FLT1INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT1INP` reader - Fault 1 input polarity"]
pub type FLT1INP_R = crate::BitReader;
#[doc = "Field `FLT1INP` writer - Fault 1 input polarity"]
pub type FLT1INP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT1INSRC` reader - Fault 2 input source"]
pub type FLT1INSRC_R = crate::BitReader;
#[doc = "Field `FLT1INSRC` writer - Fault 2 input source"]
pub type FLT1INSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT1INFC` reader - Fault 1 input filter control"]
pub type FLT1INFC_R = crate::FieldReader;
#[doc = "Field `FLT1INFC` writer - Fault 1 input filter control"]
pub type FLT1INFC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FLT1INPROT` reader - Protect fault 1 input configuration"]
pub type FLT1INPROT_R = crate::BitReader;
#[doc = "Field `FLT1INPROT` writer - Protect fault 1 input configuration"]
pub type FLT1INPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT2INEN` reader - Fault 2 input enable"]
pub type FLT2INEN_R = crate::BitReader;
#[doc = "Field `FLT2INEN` writer - Fault 2 input enable"]
pub type FLT2INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT2INP` reader - Fault 2 input polarity"]
pub type FLT2INP_R = crate::BitReader;
#[doc = "Field `FLT2INP` writer - Fault 2 input polarity"]
pub type FLT2INP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT2INSRC` reader - Fault 2 input source"]
pub type FLT2INSRC_R = crate::BitReader;
#[doc = "Field `FLT2INSRC` writer - Fault 2 input source"]
pub type FLT2INSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT2INFC` reader - Fault 2 input filter control"]
pub type FLT2INFC_R = crate::FieldReader;
#[doc = "Field `FLT2INFC` writer - Fault 2 input filter control"]
pub type FLT2INFC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FLT2INPROT` reader - Protect fault 2 input configuration"]
pub type FLT2INPROT_R = crate::BitReader;
#[doc = "Field `FLT2INPROT` writer - Protect fault 2 input configuration"]
pub type FLT2INPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT3INEN` reader - Fault 3 input enable"]
pub type FLT3INEN_R = crate::BitReader;
#[doc = "Field `FLT3INEN` writer - Fault 3 input enable"]
pub type FLT3INEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT3INP` reader - Fault 3 input polarity"]
pub type FLT3INP_R = crate::BitReader;
#[doc = "Field `FLT3INP` writer - Fault 3 input polarity"]
pub type FLT3INP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT3INSRC` reader - Fault 3 input source"]
pub type FLT3INSRC_R = crate::BitReader;
#[doc = "Field `FLT3INSRC` writer - Fault 3 input source"]
pub type FLT3INSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT3INFC` reader - Fault 3 input filter control"]
pub type FLT3INFC_R = crate::FieldReader;
#[doc = "Field `FLT3INFC` writer - Fault 3 input filter control"]
pub type FLT3INFC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FLT3INPROT` reader - Protect fault 3 input configuration"]
pub type FLT3INPROT_R = crate::BitReader;
#[doc = "Field `FLT3INPROT` writer - Protect fault 3 input configuration"]
pub type FLT3INPROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Fault 0 input enable"]
    #[inline(always)]
    pub fn flt0inen(&self) -> FLT0INEN_R {
        FLT0INEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 0 input polarity"]
    #[inline(always)]
    pub fn flt0inp(&self) -> FLT0INP_R {
        FLT0INP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 0 input source"]
    #[inline(always)]
    pub fn flt0insrc(&self) -> FLT0INSRC_R {
        FLT0INSRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Fault 0 input filter control"]
    #[inline(always)]
    pub fn flt0infc(&self) -> FLT0INFC_R {
        FLT0INFC_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Protect fault 0 input configuration"]
    #[inline(always)]
    pub fn flt0inprot(&self) -> FLT0INPROT_R {
        FLT0INPROT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fault 1 input enable"]
    #[inline(always)]
    pub fn flt1inen(&self) -> FLT1INEN_R {
        FLT1INEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fault 1 input polarity"]
    #[inline(always)]
    pub fn flt1inp(&self) -> FLT1INP_R {
        FLT1INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fault 2 input source"]
    #[inline(always)]
    pub fn flt1insrc(&self) -> FLT1INSRC_R {
        FLT1INSRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Fault 1 input filter control"]
    #[inline(always)]
    pub fn flt1infc(&self) -> FLT1INFC_R {
        FLT1INFC_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Protect fault 1 input configuration"]
    #[inline(always)]
    pub fn flt1inprot(&self) -> FLT1INPROT_R {
        FLT1INPROT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault 2 input enable"]
    #[inline(always)]
    pub fn flt2inen(&self) -> FLT2INEN_R {
        FLT2INEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault 2 input polarity"]
    #[inline(always)]
    pub fn flt2inp(&self) -> FLT2INP_R {
        FLT2INP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault 2 input source"]
    #[inline(always)]
    pub fn flt2insrc(&self) -> FLT2INSRC_R {
        FLT2INSRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Fault 2 input filter control"]
    #[inline(always)]
    pub fn flt2infc(&self) -> FLT2INFC_R {
        FLT2INFC_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Protect fault 2 input configuration"]
    #[inline(always)]
    pub fn flt2inprot(&self) -> FLT2INPROT_R {
        FLT2INPROT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fault 3 input enable"]
    #[inline(always)]
    pub fn flt3inen(&self) -> FLT3INEN_R {
        FLT3INEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Fault 3 input polarity"]
    #[inline(always)]
    pub fn flt3inp(&self) -> FLT3INP_R {
        FLT3INP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Fault 3 input source"]
    #[inline(always)]
    pub fn flt3insrc(&self) -> FLT3INSRC_R {
        FLT3INSRC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - Fault 3 input filter control"]
    #[inline(always)]
    pub fn flt3infc(&self) -> FLT3INFC_R {
        FLT3INFC_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Protect fault 3 input configuration"]
    #[inline(always)]
    pub fn flt3inprot(&self) -> FLT3INPROT_R {
        FLT3INPROT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 0 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt0inen(&mut self) -> FLT0INEN_W<FLTINCFG0_SPEC, 0> {
        FLT0INEN_W::new(self)
    }
    #[doc = "Bit 1 - Fault 0 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt0inp(&mut self) -> FLT0INP_W<FLTINCFG0_SPEC, 1> {
        FLT0INP_W::new(self)
    }
    #[doc = "Bit 2 - Fault 0 input source"]
    #[inline(always)]
    #[must_use]
    pub fn flt0insrc(&mut self) -> FLT0INSRC_W<FLTINCFG0_SPEC, 2> {
        FLT0INSRC_W::new(self)
    }
    #[doc = "Bits 3:6 - Fault 0 input filter control"]
    #[inline(always)]
    #[must_use]
    pub fn flt0infc(&mut self) -> FLT0INFC_W<FLTINCFG0_SPEC, 3> {
        FLT0INFC_W::new(self)
    }
    #[doc = "Bit 7 - Protect fault 0 input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn flt0inprot(&mut self) -> FLT0INPROT_W<FLTINCFG0_SPEC, 7> {
        FLT0INPROT_W::new(self)
    }
    #[doc = "Bit 8 - Fault 1 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1inen(&mut self) -> FLT1INEN_W<FLTINCFG0_SPEC, 8> {
        FLT1INEN_W::new(self)
    }
    #[doc = "Bit 9 - Fault 1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt1inp(&mut self) -> FLT1INP_W<FLTINCFG0_SPEC, 9> {
        FLT1INP_W::new(self)
    }
    #[doc = "Bit 10 - Fault 2 input source"]
    #[inline(always)]
    #[must_use]
    pub fn flt1insrc(&mut self) -> FLT1INSRC_W<FLTINCFG0_SPEC, 10> {
        FLT1INSRC_W::new(self)
    }
    #[doc = "Bits 11:14 - Fault 1 input filter control"]
    #[inline(always)]
    #[must_use]
    pub fn flt1infc(&mut self) -> FLT1INFC_W<FLTINCFG0_SPEC, 11> {
        FLT1INFC_W::new(self)
    }
    #[doc = "Bit 15 - Protect fault 1 input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn flt1inprot(&mut self) -> FLT1INPROT_W<FLTINCFG0_SPEC, 15> {
        FLT1INPROT_W::new(self)
    }
    #[doc = "Bit 16 - Fault 2 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2inen(&mut self) -> FLT2INEN_W<FLTINCFG0_SPEC, 16> {
        FLT2INEN_W::new(self)
    }
    #[doc = "Bit 17 - Fault 2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt2inp(&mut self) -> FLT2INP_W<FLTINCFG0_SPEC, 17> {
        FLT2INP_W::new(self)
    }
    #[doc = "Bit 18 - Fault 2 input source"]
    #[inline(always)]
    #[must_use]
    pub fn flt2insrc(&mut self) -> FLT2INSRC_W<FLTINCFG0_SPEC, 18> {
        FLT2INSRC_W::new(self)
    }
    #[doc = "Bits 19:22 - Fault 2 input filter control"]
    #[inline(always)]
    #[must_use]
    pub fn flt2infc(&mut self) -> FLT2INFC_W<FLTINCFG0_SPEC, 19> {
        FLT2INFC_W::new(self)
    }
    #[doc = "Bit 23 - Protect fault 2 input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn flt2inprot(&mut self) -> FLT2INPROT_W<FLTINCFG0_SPEC, 23> {
        FLT2INPROT_W::new(self)
    }
    #[doc = "Bit 24 - Fault 3 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3inen(&mut self) -> FLT3INEN_W<FLTINCFG0_SPEC, 24> {
        FLT3INEN_W::new(self)
    }
    #[doc = "Bit 25 - Fault 3 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt3inp(&mut self) -> FLT3INP_W<FLTINCFG0_SPEC, 25> {
        FLT3INP_W::new(self)
    }
    #[doc = "Bit 26 - Fault 3 input source"]
    #[inline(always)]
    #[must_use]
    pub fn flt3insrc(&mut self) -> FLT3INSRC_W<FLTINCFG0_SPEC, 26> {
        FLT3INSRC_W::new(self)
    }
    #[doc = "Bits 27:30 - Fault 3 input filter control"]
    #[inline(always)]
    #[must_use]
    pub fn flt3infc(&mut self) -> FLT3INFC_W<FLTINCFG0_SPEC, 27> {
        FLT3INFC_W::new(self)
    }
    #[doc = "Bit 31 - Protect fault 3 input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn flt3inprot(&mut self) -> FLT3INPROT_W<FLTINCFG0_SPEC, 31> {
        FLT3INPROT_W::new(self)
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
#[doc = "SHRTIMER fault input configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltincfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltincfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTINCFG0_SPEC;
impl crate::RegisterSpec for FLTINCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltincfg0::R`](R) reader structure"]
impl crate::Readable for FLTINCFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fltincfg0::W`](W) writer structure"]
impl crate::Writable for FLTINCFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTINCFG0 to value 0"]
impl crate::Resettable for FLTINCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
