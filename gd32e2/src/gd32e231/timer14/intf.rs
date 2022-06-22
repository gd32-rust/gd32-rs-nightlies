#[doc = "Register `INTF` reader"]
pub struct R(crate::R<INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTF` writer"]
pub struct W(crate::W<INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTF_SPEC>;
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
impl From<crate::W<INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1OF` reader - Capture/compare 1 overcapture flag"]
pub type CH1OF_R = crate::BitReader<bool>;
#[doc = "Field `CH1OF` writer - Capture/compare 1 overcapture flag"]
pub type CH1OF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 10>;
#[doc = "Field `CH0OF` reader - Capture/Compare 0 overcapture flag"]
pub type CH0OF_R = crate::BitReader<bool>;
#[doc = "Field `CH0OF` writer - Capture/Compare 0 overcapture flag"]
pub type CH0OF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 9>;
#[doc = "Field `BRKIF` reader - Break interrupt flag"]
pub type BRKIF_R = crate::BitReader<bool>;
#[doc = "Field `BRKIF` writer - Break interrupt flag"]
pub type BRKIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 7>;
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TRGIF_R = crate::BitReader<bool>;
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TRGIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 6>;
#[doc = "Field `CMTIF` reader - COM interrupt flag"]
pub type CMTIF_R = crate::BitReader<bool>;
#[doc = "Field `CMTIF` writer - COM interrupt flag"]
pub type CMTIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 5>;
#[doc = "Field `CH1IF` reader - Capture/Compare 1 interrupt flag"]
pub type CH1IF_R = crate::BitReader<bool>;
#[doc = "Field `CH1IF` writer - Capture/Compare 1 interrupt flag"]
pub type CH1IF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 2>;
#[doc = "Field `CH0IF` reader - Capture/compare 0 interrupt flag"]
pub type CH0IF_R = crate::BitReader<bool>;
#[doc = "Field `CH0IF` writer - Capture/compare 0 interrupt flag"]
pub type CH0IF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 1>;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UPIF_R = crate::BitReader<bool>;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub type UPIF_W<'a> = crate::BitWriter<'a, u32, INTF_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 10 - Capture/compare 1 overcapture flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 0 overcapture flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&self) -> BRKIF_R {
        BRKIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TRGIF_R {
        TRGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&self) -> CMTIF_R {
        CMTIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn ch1if(&self) -> CH1IF_R {
        CH1IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 0 interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> CH0IF_R {
        CH0IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Capture/compare 1 overcapture flag"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> CH1OF_W {
        CH1OF_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 0 overcapture flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&mut self) -> BRKIF_W {
        BRKIF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&mut self) -> TRGIF_W {
        TRGIF_W::new(self)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&mut self) -> CMTIF_W {
        CMTIF_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn ch1if(&mut self) -> CH1IF_W {
        CH1IF_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 0 interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&mut self) -> CH0IF_W {
        CH0IF_W::new(self)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&mut self) -> UPIF_W {
        UPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intf::W](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
