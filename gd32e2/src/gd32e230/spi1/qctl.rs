#[doc = "Register `QCTL` reader"]
pub type R = crate::R<QCTL_SPEC>;
#[doc = "Register `QCTL` writer"]
pub type W = crate::W<QCTL_SPEC>;
#[doc = "Field `QMOD` reader - Quad wire mode enable"]
pub type QMOD_R = crate::BitReader;
#[doc = "Field `QMOD` writer - Quad wire mode enable"]
pub type QMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QRD` reader - Quad wire read select"]
pub type QRD_R = crate::BitReader;
#[doc = "Field `QRD` writer - Quad wire read select"]
pub type QRD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IO23_DRV` reader - Drive IO2 and IO3 enable"]
pub type IO23_DRV_R = crate::BitReader;
#[doc = "Field `IO23_DRV` writer - Drive IO2 and IO3 enable"]
pub type IO23_DRV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Quad wire mode enable"]
    #[inline(always)]
    pub fn qmod(&self) -> QMOD_R {
        QMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Quad wire read select"]
    #[inline(always)]
    pub fn qrd(&self) -> QRD_R {
        QRD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    pub fn io23_drv(&self) -> IO23_DRV_R {
        IO23_DRV_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quad wire mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn qmod(&mut self) -> QMOD_W<QCTL_SPEC, 0> {
        QMOD_W::new(self)
    }
    #[doc = "Bit 1 - Quad wire read select"]
    #[inline(always)]
    #[must_use]
    pub fn qrd(&mut self) -> QRD_W<QCTL_SPEC, 1> {
        QRD_W::new(self)
    }
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn io23_drv(&mut self) -> IO23_DRV_W<QCTL_SPEC, 2> {
        IO23_DRV_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI quad wird control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QCTL_SPEC;
impl crate::RegisterSpec for QCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qctl::R`](R) reader structure"]
impl crate::Readable for QCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qctl::W`](W) writer structure"]
impl crate::Writable for QCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QCTL to value 0"]
impl crate::Resettable for QCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
