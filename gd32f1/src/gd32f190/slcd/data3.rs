#[doc = "Register `DATA3` reader"]
pub type R = crate::R<DATA3_SPEC>;
#[doc = "Register `DATA3` writer"]
pub type W = crate::W<DATA3_SPEC>;
#[doc = "Field `SEG_DATA3` reader - Each bit corresponds to one segment to display"]
pub type SEG_DATA3_R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA3` writer - Each bit corresponds to one segment to display"]
pub type SEG_DATA3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data3(&self) -> SEG_DATA3_R {
        SEG_DATA3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data3(&mut self) -> SEG_DATA3_W<DATA3_SPEC, 0> {
        SEG_DATA3_W::new(self)
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
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA3_SPEC;
impl crate::RegisterSpec for DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data3::R`](R) reader structure"]
impl crate::Readable for DATA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data3::W`](W) writer structure"]
impl crate::Writable for DATA3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA3 to value 0"]
impl crate::Resettable for DATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
