#[doc = "Register `AHB1RST` reader"]
pub type R = crate::R<AHB1RST_SPEC>;
#[doc = "Register `AHB1RST` writer"]
pub type W = crate::W<AHB1RST_SPEC>;
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub type USBFSRST_R = crate::BitReader;
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub type USBFSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENETRST` reader - ENET reset"]
pub type ENETRST_R = crate::BitReader;
#[doc = "Field `ENETRST` writer - ENET reset"]
pub type ENETRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> USBFSRST_R {
        USBFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    pub fn enetrst(&self) -> ENETRST_R {
        ENETRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsrst(&mut self) -> USBFSRST_W<AHB1RST_SPEC, 12> {
        USBFSRST_W::new(self)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    #[must_use]
    pub fn enetrst(&mut self) -> ENETRST_W<AHB1RST_SPEC, 14> {
        ENETRST_W::new(self)
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
#[doc = "AHB1 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1RST_SPEC;
impl crate::RegisterSpec for AHB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rst::R`](R) reader structure"]
impl crate::Readable for AHB1RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb1rst::W`](W) writer structure"]
impl crate::Writable for AHB1RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1RST to value 0"]
impl crate::Resettable for AHB1RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
