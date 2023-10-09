#[doc = "Register `ALRM0SS` reader"]
pub type R = crate::R<ALRM0SS_SPEC>;
#[doc = "Register `ALRM0SS` writer"]
pub type W = crate::W<ALRM0SS_SPEC>;
#[doc = "Field `SSC` reader - Alarm sub second value"]
pub type SSC_R = crate::FieldReader<u16>;
#[doc = "Field `SSC` writer - Alarm sub second value"]
pub type SSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `MSKSSC` reader - Mask control bit of SSC"]
pub type MSKSSC_R = crate::FieldReader;
#[doc = "Field `MSKSSC` writer - Mask control bit of SSC"]
pub type MSKSSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:14 - Alarm sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask control bit of SSC"]
    #[inline(always)]
    pub fn mskssc(&self) -> MSKSSC_R {
        MSKSSC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Alarm sub second value"]
    #[inline(always)]
    #[must_use]
    pub fn ssc(&mut self) -> SSC_W<ALRM0SS_SPEC, 0> {
        SSC_W::new(self)
    }
    #[doc = "Bits 24:27 - Mask control bit of SSC"]
    #[inline(always)]
    #[must_use]
    pub fn mskssc(&mut self) -> MSKSSC_W<ALRM0SS_SPEC, 24> {
        MSKSSC_W::new(self)
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
#[doc = "Alarm 0 sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm0ss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm0ss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRM0SS_SPEC;
impl crate::RegisterSpec for ALRM0SS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrm0ss::R`](R) reader structure"]
impl crate::Readable for ALRM0SS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrm0ss::W`](W) writer structure"]
impl crate::Writable for ALRM0SS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRM0SS to value 0"]
impl crate::Resettable for ALRM0SS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
