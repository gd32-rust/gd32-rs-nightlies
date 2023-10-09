#[doc = "Register `CWSZ` reader"]
pub type R = crate::R<CWSZ_SPEC>;
#[doc = "Register `CWSZ` writer"]
pub type W = crate::W<CWSZ_SPEC>;
#[doc = "Field `WHSZ` reader - Window horizontal Size"]
pub type WHSZ_R = crate::FieldReader<u16>;
#[doc = "Field `WHSZ` writer - Window horizontal Size"]
pub type WHSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `WVSZ` reader - Window Vertical Size"]
pub type WVSZ_R = crate::FieldReader<u16>;
#[doc = "Field `WVSZ` writer - Window Vertical Size"]
pub type WVSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Window horizontal Size"]
    #[inline(always)]
    pub fn whsz(&self) -> WHSZ_R {
        WHSZ_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Window Vertical Size"]
    #[inline(always)]
    pub fn wvsz(&self) -> WVSZ_R {
        WVSZ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Window horizontal Size"]
    #[inline(always)]
    #[must_use]
    pub fn whsz(&mut self) -> WHSZ_W<CWSZ_SPEC, 0> {
        WHSZ_W::new(self)
    }
    #[doc = "Bits 16:29 - Window Vertical Size"]
    #[inline(always)]
    #[must_use]
    pub fn wvsz(&mut self) -> WVSZ_W<CWSZ_SPEC, 16> {
        WVSZ_W::new(self)
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
#[doc = "DCI Cropping window size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWSZ_SPEC;
impl crate::RegisterSpec for CWSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwsz::R`](R) reader structure"]
impl crate::Readable for CWSZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwsz::W`](W) writer structure"]
impl crate::Writable for CWSZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWSZ to value 0"]
impl crate::Resettable for CWSZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
