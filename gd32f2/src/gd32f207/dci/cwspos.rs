#[doc = "Register `CWSPOS` reader"]
pub type R = crate::R<CWSPOS_SPEC>;
#[doc = "Register `CWSPOS` writer"]
pub type W = crate::W<CWSPOS_SPEC>;
#[doc = "Field `WHSP` reader - Window horizontal start position"]
pub type WHSP_R = crate::FieldReader<u16>;
#[doc = "Field `WHSP` writer - Window horizontal start position"]
pub type WHSP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `WVSP` reader - Window Vertical start position"]
pub type WVSP_R = crate::FieldReader<u16>;
#[doc = "Field `WVSP` writer - Window Vertical start position"]
pub type WVSP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Window horizontal start position"]
    #[inline(always)]
    pub fn whsp(&self) -> WHSP_R {
        WHSP_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Window Vertical start position"]
    #[inline(always)]
    pub fn wvsp(&self) -> WVSP_R {
        WVSP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Window horizontal start position"]
    #[inline(always)]
    #[must_use]
    pub fn whsp(&mut self) -> WHSP_W<CWSPOS_SPEC, 0> {
        WHSP_W::new(self)
    }
    #[doc = "Bits 16:28 - Window Vertical start position"]
    #[inline(always)]
    #[must_use]
    pub fn wvsp(&mut self) -> WVSP_W<CWSPOS_SPEC, 16> {
        WVSP_W::new(self)
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
#[doc = "DCI Cropping window start position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwspos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwspos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWSPOS_SPEC;
impl crate::RegisterSpec for CWSPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwspos::R`](R) reader structure"]
impl crate::Readable for CWSPOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwspos::W`](W) writer structure"]
impl crate::Writable for CWSPOS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWSPOS to value 0"]
impl crate::Resettable for CWSPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
