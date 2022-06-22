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
#[doc = "Field `ELIF` reader - End of Line Interrupt Flag"]
pub type ELIF_R = crate::BitReader<bool>;
#[doc = "Field `VSIF` reader - Vsync Interrupt Flag"]
pub type VSIF_R = crate::BitReader<bool>;
#[doc = "Field `ESEIF` reader - Embedded Synchronous Error Interrupt Flag"]
pub type ESEIF_R = crate::BitReader<bool>;
#[doc = "Field `OVRIF` reader - FIFO Overrun Interrupt Flag"]
pub type OVRIF_R = crate::BitReader<bool>;
#[doc = "Field `EFIF` reader - End of Frame Interrupt Flag"]
pub type EFIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - End of Line Interrupt Flag"]
    #[inline(always)]
    pub fn elif(&self) -> ELIF_R {
        ELIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Vsync Interrupt Flag"]
    #[inline(always)]
    pub fn vsif(&self) -> VSIF_R {
        VSIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded Synchronous Error Interrupt Flag"]
    #[inline(always)]
    pub fn eseif(&self) -> ESEIF_R {
        ESEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overrun Interrupt Flag"]
    #[inline(always)]
    pub fn ovrif(&self) -> OVRIF_R {
        OVRIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - End of Frame Interrupt Flag"]
    #[inline(always)]
    pub fn efif(&self) -> EFIF_R {
        EFIF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DCI Interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
