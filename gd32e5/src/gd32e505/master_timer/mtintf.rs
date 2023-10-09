#[doc = "Register `MTINTF` reader"]
pub type R = crate::R<MTINTF_SPEC>;
#[doc = "Field `CMP0IF` reader - Compare 0 interrupt flag"]
pub type CMP0IF_R = crate::BitReader;
#[doc = "Field `CMP1IF` reader - Compare 1 interrupt flag"]
pub type CMP1IF_R = crate::BitReader;
#[doc = "Field `CMP2IF` reader - Compare 2 interrupt flag"]
pub type CMP2IF_R = crate::BitReader;
#[doc = "Field `CMP3IF` reader - Compare 3 interrupt flag"]
pub type CMP3IF_R = crate::BitReader;
#[doc = "Field `REPIF` reader - Repetition interrupt flag"]
pub type REPIF_R = crate::BitReader;
#[doc = "Field `SYNIIF` reader - Synchronization input interrupt flag"]
pub type SYNIIF_R = crate::BitReader;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UPIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compare 0 interrupt flag"]
    #[inline(always)]
    pub fn cmp0if(&self) -> CMP0IF_R {
        CMP0IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cmp1if(&self) -> CMP1IF_R {
        CMP1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cmp2if(&self) -> CMP2IF_R {
        CMP2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cmp3if(&self) -> CMP3IF_R {
        CMP3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repetition interrupt flag"]
    #[inline(always)]
    pub fn repif(&self) -> REPIF_R {
        REPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization input interrupt flag"]
    #[inline(always)]
    pub fn syniif(&self) -> SYNIIF_R {
        SYNIIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UPIF_R {
        UPIF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "SHRTIMER Master_TIMER interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtintf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTINTF_SPEC;
impl crate::RegisterSpec for MTINTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtintf::R`](R) reader structure"]
impl crate::Readable for MTINTF_SPEC {}
#[doc = "`reset()` method sets MTINTF to value 0"]
impl crate::Resettable for MTINTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
