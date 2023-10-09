#[doc = "Register `L1VPOS` reader"]
pub type R = crate::R<L1VPOS_SPEC>;
#[doc = "Register `L1VPOS` writer"]
pub type W = crate::W<L1VPOS_SPEC>;
#[doc = "Field `WTP` reader - Window top position"]
pub type WTP_R = crate::FieldReader<u16>;
#[doc = "Field `WTP` writer - Window top position"]
pub type WTP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `WBP` reader - Window bottom position"]
pub type WBP_R = crate::FieldReader<u16>;
#[doc = "Field `WBP` writer - Window bottom position"]
pub type WBP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Window top position"]
    #[inline(always)]
    pub fn wtp(&self) -> WTP_R {
        WTP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window bottom position"]
    #[inline(always)]
    pub fn wbp(&self) -> WBP_R {
        WBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window top position"]
    #[inline(always)]
    #[must_use]
    pub fn wtp(&mut self) -> WTP_W<L1VPOS_SPEC, 0> {
        WTP_W::new(self)
    }
    #[doc = "Bits 16:27 - Window bottom position"]
    #[inline(always)]
    #[must_use]
    pub fn wbp(&mut self) -> WBP_W<L1VPOS_SPEC, 16> {
        WBP_W::new(self)
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
#[doc = "Layer 1 vertical position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1vpos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1vpos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1VPOS_SPEC;
impl crate::RegisterSpec for L1VPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1vpos::R`](R) reader structure"]
impl crate::Readable for L1VPOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1vpos::W`](W) writer structure"]
impl crate::Writable for L1VPOS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1VPOS to value 0"]
impl crate::Resettable for L1VPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
