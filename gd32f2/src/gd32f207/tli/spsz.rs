#[doc = "Register `SPSZ` reader"]
pub type R = crate::R<SPSZ_SPEC>;
#[doc = "Register `SPSZ` writer"]
pub type W = crate::W<SPSZ_SPEC>;
#[doc = "Field `VPSZ` reader - size of vertical synchronous pluse"]
pub type VPSZ_R = crate::FieldReader<u16>;
#[doc = "Field `VPSZ` writer - size of vertical synchronous pluse"]
pub type VPSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `HPSZ` reader - size of horizontal synchronous pluse"]
pub type HPSZ_R = crate::FieldReader<u16>;
#[doc = "Field `HPSZ` writer - size of horizontal synchronous pluse"]
pub type HPSZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - size of vertical synchronous pluse"]
    #[inline(always)]
    pub fn vpsz(&self) -> VPSZ_R {
        VPSZ_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - size of horizontal synchronous pluse"]
    #[inline(always)]
    pub fn hpsz(&self) -> HPSZ_R {
        HPSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - size of vertical synchronous pluse"]
    #[inline(always)]
    #[must_use]
    pub fn vpsz(&mut self) -> VPSZ_W<SPSZ_SPEC, 0> {
        VPSZ_W::new(self)
    }
    #[doc = "Bits 16:27 - size of horizontal synchronous pluse"]
    #[inline(always)]
    #[must_use]
    pub fn hpsz(&mut self) -> HPSZ_W<SPSZ_SPEC, 16> {
        HPSZ_W::new(self)
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
#[doc = "Synchronous pulse size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPSZ_SPEC;
impl crate::RegisterSpec for SPSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spsz::R`](R) reader structure"]
impl crate::Readable for SPSZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spsz::W`](W) writer structure"]
impl crate::Writable for SPSZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPSZ to value 0"]
impl crate::Resettable for SPSZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
