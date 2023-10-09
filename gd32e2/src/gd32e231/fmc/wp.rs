#[doc = "Register `WP` reader"]
pub type R = crate::R<WP_SPEC>;
#[doc = "Field `OB_WP` reader - Store WP\\[15:0\\]
of option bytes block after system reset"]
pub type OB_WP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Store WP\\[15:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn ob_wp(&self) -> OB_WP_R {
        OB_WP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Erase/Program Protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WP_SPEC;
impl crate::RegisterSpec for WP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wp::R`](R) reader structure"]
impl crate::Readable for WP_SPEC {}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
