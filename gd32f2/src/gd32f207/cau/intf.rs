#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Field `IINTF` reader - In FIFO enabled interrupt flag"]
pub type IINTF_R = crate::BitReader;
#[doc = "Field `OINTF` reader - Out FIFO enabled interrupt flag"]
pub type OINTF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - In FIFO enabled interrupt flag"]
    #[inline(always)]
    pub fn iintf(&self) -> IINTF_R {
        IINTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out FIFO enabled interrupt flag"]
    #[inline(always)]
    pub fn ointf(&self) -> OINTF_R {
        OINTF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "CAU enable interrupt status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
