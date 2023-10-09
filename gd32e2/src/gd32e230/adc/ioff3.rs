#[doc = "Register `IOFF3` reader"]
pub type R = crate::R<IOFF3_SPEC>;
#[doc = "Register `IOFF3` writer"]
pub type W = crate::W<IOFF3_SPEC>;
#[doc = "Field `IOFF` reader - Data offset for injected channel x"]
pub type IOFF_R = crate::FieldReader<u16>;
#[doc = "Field `IOFF` writer - Data offset for injected channel x"]
pub type IOFF_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn ioff(&self) -> IOFF_R {
        IOFF_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn ioff(&mut self) -> IOFF_W<IOFF3_SPEC, 0> {
        IOFF_W::new(self)
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
#[doc = "Inserted channel data offset register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOFF3_SPEC;
impl crate::RegisterSpec for IOFF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioff3::R`](R) reader structure"]
impl crate::Readable for IOFF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioff3::W`](W) writer structure"]
impl crate::Writable for IOFF3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOFF3 to value 0"]
impl crate::Resettable for IOFF3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
