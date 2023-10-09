#[doc = "Register `MSC_RINTF` reader"]
pub type R = crate::R<MSC_RINTF_SPEC>;
#[doc = "Field `RFCE` reader - Received frames CRC error"]
pub type RFCE_R = crate::BitReader;
#[doc = "Field `RFAE` reader - Received frames alignment error"]
pub type RFAE_R = crate::BitReader;
#[doc = "Field `RGUF` reader - Received Good Unicast Frames"]
pub type RGUF_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Received frames CRC error"]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error"]
    #[inline(always)]
    pub fn rfae(&self) -> RFAE_R {
        RFAE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received Good Unicast Frames"]
    #[inline(always)]
    pub fn rguf(&self) -> RGUF_R {
        RGUF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Ethernet MSC receive interrupt flag register (MSC_RINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msc_rintf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSC_RINTF_SPEC;
impl crate::RegisterSpec for MSC_RINTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_rintf::R`](R) reader structure"]
impl crate::Readable for MSC_RINTF_SPEC {}
#[doc = "`reset()` method sets MSC_RINTF to value 0"]
impl crate::Resettable for MSC_RINTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
