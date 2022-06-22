#[doc = "Register `AHB1RST` reader"]
pub struct R(crate::R<AHB1RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1RST` writer"]
pub struct W(crate::W<AHB1RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AHB1RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub type USBFSRST_R = crate::BitReader<bool>;
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub type USBFSRST_W<'a> = crate::BitWriter<'a, u32, AHB1RST_SPEC, bool, 12>;
#[doc = "Field `ENETRST` reader - ENET reset"]
pub type ENETRST_R = crate::BitReader<bool>;
#[doc = "Field `ENETRST` writer - ENET reset"]
pub type ENETRST_W<'a> = crate::BitWriter<'a, u32, AHB1RST_SPEC, bool, 14>;
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
    pub fn usbfsrst(&mut self) -> USBFSRST_W {
        USBFSRST_W::new(self)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    pub fn enetrst(&mut self) -> ENETRST_W {
        ENETRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rst](index.html) module"]
pub struct AHB1RST_SPEC;
impl crate::RegisterSpec for AHB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1rst::R](R) reader structure"]
impl crate::Readable for AHB1RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1rst::W](W) writer structure"]
impl crate::Writable for AHB1RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB1RST to value 0"]
impl crate::Resettable for AHB1RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
