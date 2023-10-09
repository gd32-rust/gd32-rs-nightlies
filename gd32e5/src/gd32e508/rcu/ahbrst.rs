#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AHBRST_SPEC>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AHBRST_SPEC>;
#[doc = "Field `USBHSRST` reader - USBHS reset"]
pub type USBHSRST_R = crate::BitReader;
#[doc = "Field `USBHSRST` writer - USBHS reset"]
pub type USBHSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENETRST` reader - ENET reset"]
pub type ENETRST_R = crate::BitReader;
#[doc = "Field `ENETRST` writer - ENET reset"]
pub type ENETRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMURST` reader - TMU reset"]
pub type TMURST_R = crate::BitReader;
#[doc = "Field `TMURST` writer - TMU reset"]
pub type TMURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SQPIRST` reader - SQPI reset"]
pub type SQPIRST_R = crate::BitReader;
#[doc = "Field `SQPIRST` writer - SQPI reset"]
pub type SQPIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 12 - USBHS reset"]
    #[inline(always)]
    pub fn usbhsrst(&self) -> USBHSRST_R {
        USBHSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    pub fn enetrst(&self) -> ENETRST_R {
        ENETRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 30 - TMU reset"]
    #[inline(always)]
    pub fn tmurst(&self) -> TMURST_R {
        TMURST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SQPI reset"]
    #[inline(always)]
    pub fn sqpirst(&self) -> SQPIRST_R {
        SQPIRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBHS reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbhsrst(&mut self) -> USBHSRST_W<AHBRST_SPEC, 12> {
        USBHSRST_W::new(self)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    #[must_use]
    pub fn enetrst(&mut self) -> ENETRST_W<AHBRST_SPEC, 14> {
        ENETRST_W::new(self)
    }
    #[doc = "Bit 30 - TMU reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmurst(&mut self) -> TMURST_W<AHBRST_SPEC, 30> {
        TMURST_W::new(self)
    }
    #[doc = "Bit 31 - SQPI reset"]
    #[inline(always)]
    #[must_use]
    pub fn sqpirst(&mut self) -> SQPIRST_W<AHBRST_SPEC, 31> {
        SQPIRST_W::new(self)
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
#[doc = "AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST_SPEC;
impl crate::RegisterSpec for AHBRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AHBRST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AHBRST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AHBRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
