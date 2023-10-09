#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Field `FLT0IF` reader - Fault 0 interrupt flag"]
pub type FLT0IF_R = crate::BitReader;
#[doc = "Field `FLT1IF` reader - Fault 1 interrupt flag"]
pub type FLT1IF_R = crate::BitReader;
#[doc = "Field `FLT2IF` reader - Fault 2 interrupt flag"]
pub type FLT2IF_R = crate::BitReader;
#[doc = "Field `FLT3IF` reader - Fault 3 interrupt flag"]
pub type FLT3IF_R = crate::BitReader;
#[doc = "Field `FLT4IF` reader - Fault 4 interrupt flag"]
pub type FLT4IF_R = crate::BitReader;
#[doc = "Field `SYSFLTIF` reader - System fault interrupt flag"]
pub type SYSFLTIF_R = crate::BitReader;
#[doc = "Field `DLLCALIF` reader - DLL calibration completed interrupt flag"]
pub type DLLCALIF_R = crate::BitReader;
#[doc = "Field `BMPERIF` reader - Bunch mode period interrupt flag"]
pub type BMPERIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fault 0 interrupt flag"]
    #[inline(always)]
    pub fn flt0if(&self) -> FLT0IF_R {
        FLT0IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 1 interrupt flag"]
    #[inline(always)]
    pub fn flt1if(&self) -> FLT1IF_R {
        FLT1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 2 interrupt flag"]
    #[inline(always)]
    pub fn flt2if(&self) -> FLT2IF_R {
        FLT2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 3 interrupt flag"]
    #[inline(always)]
    pub fn flt3if(&self) -> FLT3IF_R {
        FLT3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 4 interrupt flag"]
    #[inline(always)]
    pub fn flt4if(&self) -> FLT4IF_R {
        FLT4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System fault interrupt flag"]
    #[inline(always)]
    pub fn sysfltif(&self) -> SYSFLTIF_R {
        SYSFLTIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL calibration completed interrupt flag"]
    #[inline(always)]
    pub fn dllcalif(&self) -> DLLCALIF_R {
        DLLCALIF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bunch mode period interrupt flag"]
    #[inline(always)]
    pub fn bmperif(&self) -> BMPERIF_R {
        BMPERIF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "SHRTIMER interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
