#[doc = "Register `DATA2` reader"]
pub type R = crate::R<DATA2_SPEC>;
#[doc = "Register `DATA2` writer"]
pub type W = crate::W<DATA2_SPEC>;
#[doc = "Field `SEG_DATA2` reader - Each bit corresponds to one segment to display"]
pub type SEG_DATA2_R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA2` writer - Each bit corresponds to one segment to display"]
pub type SEG_DATA2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    pub fn seg_data2(&self) -> SEG_DATA2_R {
        SEG_DATA2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one segment to display"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data2(&mut self) -> SEG_DATA2_W<DATA2_SPEC, 0> {
        SEG_DATA2_W::new(self)
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
#[doc = "SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA2_SPEC;
impl crate::RegisterSpec for DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data2::R`](R) reader structure"]
impl crate::Readable for DATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data2::W`](W) writer structure"]
impl crate::Writable for DATA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA2 to value 0"]
impl crate::Resettable for DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
