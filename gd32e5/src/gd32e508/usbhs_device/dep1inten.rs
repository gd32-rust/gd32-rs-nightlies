#[doc = "Register `DEP1INTEN` reader"]
pub type R = crate::R<DEP1INTEN_SPEC>;
#[doc = "Field `IEP1INTEN` reader - IN Endpoint 1 interrupt enable"]
pub type IEP1INTEN_R = crate::BitReader;
#[doc = "Field `OEP1INTEN` reader - OUT Endpoint 1 interrupt enable"]
pub type OEP1INTEN_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt enable"]
    #[inline(always)]
    pub fn iep1inten(&self) -> IEP1INTEN_R {
        IEP1INTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt enable"]
    #[inline(always)]
    pub fn oep1inten(&self) -> OEP1INTEN_R {
        OEP1INTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Device endpoint 1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dep1inten::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEP1INTEN_SPEC;
impl crate::RegisterSpec for DEP1INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dep1inten::R`](R) reader structure"]
impl crate::Readable for DEP1INTEN_SPEC {}
#[doc = "`reset()` method sets DEP1INTEN to value 0"]
impl crate::Resettable for DEP1INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
