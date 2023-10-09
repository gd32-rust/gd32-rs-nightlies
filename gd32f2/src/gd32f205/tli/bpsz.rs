#[doc = "Register `BPSZ` reader"]
pub type R = crate::R<BPSZ_SPEC>;
#[doc = "Register `BPSZ` writer"]
pub type W = crate::W<BPSZ_SPEC>;
#[doc = "Field `VBPSZ` reader - Size of the vertical back porch plus synchronous pulse"]
pub type VBPSZ_R = crate::FieldReader<u16>;
#[doc = "Field `VBPSZ` writer - Size of the vertical back porch plus synchronous pulse"]
pub type VBPSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `HBPSZ` reader - Size of the horizontal back porch plus synchronous pulse"]
pub type HBPSZ_R = crate::FieldReader<u16>;
#[doc = "Field `HBPSZ` writer - Size of the horizontal back porch plus synchronous pulse"]
pub type HBPSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Size of the vertical back porch plus synchronous pulse"]
    #[inline(always)]
    pub fn vbpsz(&self) -> VBPSZ_R {
        VBPSZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Size of the horizontal back porch plus synchronous pulse"]
    #[inline(always)]
    pub fn hbpsz(&self) -> HBPSZ_R {
        HBPSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Size of the vertical back porch plus synchronous pulse"]
    #[inline(always)]
    #[must_use]
    pub fn vbpsz(&mut self) -> VBPSZ_W<BPSZ_SPEC, 0> {
        VBPSZ_W::new(self)
    }
    #[doc = "Bits 16:27 - Size of the horizontal back porch plus synchronous pulse"]
    #[inline(always)]
    #[must_use]
    pub fn hbpsz(&mut self) -> HBPSZ_W<BPSZ_SPEC, 16> {
        HBPSZ_W::new(self)
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
#[doc = "Back-porch size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpsz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpsz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BPSZ_SPEC;
impl crate::RegisterSpec for BPSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpsz::R`](R) reader structure"]
impl crate::Readable for BPSZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bpsz::W`](W) writer structure"]
impl crate::Writable for BPSZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BPSZ to value 0"]
impl crate::Resettable for BPSZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
