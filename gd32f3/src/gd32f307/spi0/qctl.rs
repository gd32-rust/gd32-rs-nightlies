#[doc = "Register `QCTL` reader"]
pub struct R(crate::R<QCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QCTL` writer"]
pub struct W(crate::W<QCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QCTL_SPEC>;
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
impl From<crate::W<QCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO23_DRV` reader - Drive IO2 and IO3 enable"]
pub type IO23_DRV_R = crate::BitReader<bool>;
#[doc = "Field `IO23_DRV` writer - Drive IO2 and IO3 enable"]
pub type IO23_DRV_W<'a> = crate::BitWriter<'a, u32, QCTL_SPEC, bool, 2>;
#[doc = "Field `QRD` reader - Quad-SPI mode read select"]
pub type QRD_R = crate::BitReader<bool>;
#[doc = "Field `QRD` writer - Quad-SPI mode read select"]
pub type QRD_W<'a> = crate::BitWriter<'a, u32, QCTL_SPEC, bool, 1>;
#[doc = "Field `QMOD` reader - Quad-SPI mode enable"]
pub type QMOD_R = crate::BitReader<bool>;
#[doc = "Field `QMOD` writer - Quad-SPI mode enable"]
pub type QMOD_W<'a> = crate::BitWriter<'a, u32, QCTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    pub fn io23_drv(&self) -> IO23_DRV_R {
        IO23_DRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Quad-SPI mode read select"]
    #[inline(always)]
    pub fn qrd(&self) -> QRD_R {
        QRD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Quad-SPI mode enable"]
    #[inline(always)]
    pub fn qmod(&self) -> QMOD_R {
        QMOD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    pub fn io23_drv(&mut self) -> IO23_DRV_W {
        IO23_DRV_W::new(self)
    }
    #[doc = "Bit 1 - Quad-SPI mode read select"]
    #[inline(always)]
    pub fn qrd(&mut self) -> QRD_W {
        QRD_W::new(self)
    }
    #[doc = "Bit 0 - Quad-SPI mode enable"]
    #[inline(always)]
    pub fn qmod(&mut self) -> QMOD_W {
        QMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quad-SPI mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qctl](index.html) module"]
pub struct QCTL_SPEC;
impl crate::RegisterSpec for QCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qctl::R](R) reader structure"]
impl crate::Readable for QCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qctl::W](W) writer structure"]
impl crate::Writable for QCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QCTL to value 0"]
impl crate::Resettable for QCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
