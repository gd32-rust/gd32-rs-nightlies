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
#[doc = "Channel 0 Capture overflow flag"]
pub use crate::gd32f150::timer0::intf::CH0OF_A;
#[doc = "Field `CH0OF` reader - Channel 0 Capture overflow flag"]
pub use crate::gd32f150::timer0::intf::CH0OF_R;
#[doc = "Field `CH0OF` writer - Channel 0 Capture overflow flag"]
pub type CH0OF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, CH0OF_A, 9>;
impl<'a> CH0OF_W<'a> {
    #[doc = "No over capture occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH0OF_A::CLEAR)
    }
    #[doc = "A capture event occured while CHnIF was already set"]
    #[inline(always)]
    pub fn over_capture(self) -> &'a mut W {
        self.variant(CH0OF_A::OVERCAPTURE)
    }
}
#[doc = "Break interrupt flag"]
pub use crate::gd32f150::timer0::intf::BRKIF_A;
#[doc = "Field `BRKIF` reader - Break interrupt flag"]
pub use crate::gd32f150::timer0::intf::BRKIF_R;
#[doc = "Field `BRKIF` writer - Break interrupt flag"]
pub type BRKIF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, BRKIF_A, 7>;
impl<'a> BRKIF_W<'a> {
    #[doc = "No active level break detected"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BRKIF_A::CLEAR)
    }
    #[doc = "Active level detected"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(BRKIF_A::BREAK)
    }
}
#[doc = "Channel commutation interrupt flag"]
pub use crate::gd32f150::timer0::intf::CMTIF_A;
#[doc = "Field `CMTIF` reader - Channel commutation interrupt flag"]
pub use crate::gd32f150::timer0::intf::CMTIF_R;
#[doc = "Field `CMTIF` writer - Channel commutation interrupt flag"]
pub type CMTIF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, CMTIF_A, 5>;
impl<'a> CMTIF_W<'a> {
    #[doc = "No channel commutation event occured"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMTIF_A::CLEAR)
    }
    #[doc = "Channel commutation event occurred"]
    #[inline(always)]
    pub fn commutation(self) -> &'a mut W {
        self.variant(CMTIF_A::COMMUTATION)
    }
}
#[doc = "Channel 0 interrupt flag"]
pub use crate::gd32f150::timer0::intf::CH0IF_A;
#[doc = "Field `CH0IF` reader - Channel 0 interrupt flag"]
pub use crate::gd32f150::timer0::intf::CH0IF_R;
#[doc = "Field `CH0IF` writer - Channel 0 interrupt flag"]
pub type CH0IF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, CH0IF_A, 1>;
impl<'a> CH0IF_W<'a> {
    #[doc = "No capture or compare interrupt occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH0IF_A::CLEAR)
    }
    #[doc = "A capture or compare event occurred"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut W {
        self.variant(CH0IF_A::CAPTURECOMPARE)
    }
}
#[doc = "Update interrupt flag"]
pub use crate::gd32f150::timer0::intf::UPIF_A;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub use crate::gd32f150::timer0::intf::UPIF_R;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub type UPIF_W<'a> = crate::BitWriter<'a, u16, INTF_SPEC, UPIF_A, 0>;
impl<'a> UPIF_W<'a> {
    #[doc = "No update interrupt occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UPIF_A::CLEAR)
    }
    #[doc = "Update interrupt pending."]
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut W {
        self.variant(UPIF_A::UPDATEPENDING)
    }
}
impl R {
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&self) -> BRKIF_R {
        BRKIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&self) -> CMTIF_R {
        CMTIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 interrupt flag"]
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
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&mut self) -> BRKIF_W {
        BRKIF_W::new(self)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&mut self) -> CMTIF_W {
        CMTIF_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 interrupt flag"]
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u16;
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
