#[doc = "Register `DATA6` reader"]
pub type R = crate::R<DATA6_SPEC>;
#[doc = "Register `DATA6` writer"]
pub type W = crate::W<DATA6_SPEC>;
#[doc = "Field `SEG_DATA6` reader - Each bit corresponds to one segment to display"]
pub type SEG_DATA6_R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA6` writer - Each bit corresponds to one segment to display"]
pub type SEG_DATA6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data6(&self) -> SEG_DATA6_R {
        SEG_DATA6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data6(&mut self) -> SEG_DATA6_W<DATA6_SPEC, 0> {
        SEG_DATA6_W::new(self)
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
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA6_SPEC;
impl crate::RegisterSpec for DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data6::R`](R) reader structure"]
impl crate::Readable for DATA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data6::W`](W) writer structure"]
impl crate::Writable for DATA6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA6 to value 0"]
impl crate::Resettable for DATA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
