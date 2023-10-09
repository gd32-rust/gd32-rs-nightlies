#[doc = "Register `DEP1INT` reader"]
pub type R = crate::R<DEP1INT_SPEC>;
#[doc = "Field `IEP1INT` reader - IN Endpoint 1 interrupt"]
pub type IEP1INT_R = crate::BitReader;
#[doc = "Field `OEP1INT` reader - OUT Endpoint 1 interrupt"]
pub type OEP1INT_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Device endpoint 1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dep1int::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEP1INT_SPEC;
impl crate::RegisterSpec for DEP1INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dep1int::R`](R) reader structure"]
impl crate::Readable for DEP1INT_SPEC {}
#[doc = "`reset()` method sets DEP1INT to value 0"]
impl crate::Resettable for DEP1INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
