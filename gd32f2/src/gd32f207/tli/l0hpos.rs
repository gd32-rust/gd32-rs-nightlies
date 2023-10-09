#[doc = "Register `L0HPOS` reader"]
pub type R = crate::R<L0HPOS_SPEC>;
#[doc = "Register `L0HPOS` writer"]
pub type W = crate::W<L0HPOS_SPEC>;
#[doc = "Field `WLP` reader - Window left position"]
pub type WLP_R = crate::FieldReader<u16>;
#[doc = "Field `WLP` writer - Window left position"]
pub type WLP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `WRP` reader - Window right position"]
pub type WRP_R = crate::FieldReader<u16>;
#[doc = "Field `WRP` writer - Window right position"]
pub type WRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Window left position"]
    #[inline(always)]
    pub fn wlp(&self) -> WLP_R {
        WLP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window right position"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window left position"]
    #[inline(always)]
    #[must_use]
    pub fn wlp(&mut self) -> WLP_W<L0HPOS_SPEC, 0> {
        WLP_W::new(self)
    }
    #[doc = "Bits 16:27 - Window right position"]
    #[inline(always)]
    #[must_use]
    pub fn wrp(&mut self) -> WRP_W<L0HPOS_SPEC, 16> {
        WRP_W::new(self)
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
#[doc = "Layer 0 horizontal position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0hpos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0hpos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0HPOS_SPEC;
impl crate::RegisterSpec for L0HPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0hpos::R`](R) reader structure"]
impl crate::Readable for L0HPOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l0hpos::W`](W) writer structure"]
impl crate::Writable for L0HPOS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L0HPOS to value 0"]
impl crate::Resettable for L0HPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
