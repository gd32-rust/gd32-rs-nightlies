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
#[doc = "Drive IO2 and IO3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO23_DRV_A {
    #[doc = "0: IO2 and IO3 are not driven in single wire mode"]
    DISABLED = 0,
    #[doc = "1: IO2 and IO3 are driven high in single wire mode"]
    HIGH = 1,
}
impl From<IO23_DRV_A> for bool {
    #[inline(always)]
    fn from(variant: IO23_DRV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO23_DRV` reader - Drive IO2 and IO3 enable"]
pub type IO23_DRV_R = crate::BitReader<IO23_DRV_A>;
impl IO23_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO23_DRV_A {
        match self.bits {
            false => IO23_DRV_A::DISABLED,
            true => IO23_DRV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IO23_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IO23_DRV_A::HIGH
    }
}
#[doc = "Field `IO23_DRV` writer - Drive IO2 and IO3 enable"]
pub type IO23_DRV_W<'a> = crate::BitWriter<'a, u16, QCTL_SPEC, IO23_DRV_A, 2>;
impl<'a> IO23_DRV_W<'a> {
    #[doc = "IO2 and IO3 are not driven in single wire mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IO23_DRV_A::DISABLED)
    }
    #[doc = "IO2 and IO3 are driven high in single wire mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IO23_DRV_A::HIGH)
    }
}
#[doc = "Quad wire read select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QRD_A {
    #[doc = "0: Quad wire write mode"]
    WRITE = 0,
    #[doc = "1: Quad wire read mode"]
    READ = 1,
}
impl From<QRD_A> for bool {
    #[inline(always)]
    fn from(variant: QRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QRD` reader - Quad wire read select"]
pub type QRD_R = crate::BitReader<QRD_A>;
impl QRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QRD_A {
        match self.bits {
            false => QRD_A::WRITE,
            true => QRD_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == QRD_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == QRD_A::READ
    }
}
#[doc = "Field `QRD` writer - Quad wire read select"]
pub type QRD_W<'a> = crate::BitWriter<'a, u16, QCTL_SPEC, QRD_A, 1>;
impl<'a> QRD_W<'a> {
    #[doc = "Quad wire write mode"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(QRD_A::WRITE)
    }
    #[doc = "Quad wire read mode"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(QRD_A::READ)
    }
}
#[doc = "Quad wire mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QMOD_A {
    #[doc = "0: Single wire mode"]
    SINGLE = 0,
    #[doc = "1: Quad-SPI mode"]
    QUAD = 1,
}
impl From<QMOD_A> for bool {
    #[inline(always)]
    fn from(variant: QMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QMOD` reader - Quad wire mode enable"]
pub type QMOD_R = crate::BitReader<QMOD_A>;
impl QMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QMOD_A {
        match self.bits {
            false => QMOD_A::SINGLE,
            true => QMOD_A::QUAD,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == QMOD_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == QMOD_A::QUAD
    }
}
#[doc = "Field `QMOD` writer - Quad wire mode enable"]
pub type QMOD_W<'a> = crate::BitWriter<'a, u16, QCTL_SPEC, QMOD_A, 0>;
impl<'a> QMOD_W<'a> {
    #[doc = "Single wire mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(QMOD_A::SINGLE)
    }
    #[doc = "Quad-SPI mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(QMOD_A::QUAD)
    }
}
impl R {
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    pub fn io23_drv(&self) -> IO23_DRV_R {
        IO23_DRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Quad wire read select"]
    #[inline(always)]
    pub fn qrd(&self) -> QRD_R {
        QRD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Quad wire mode enable"]
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
    #[doc = "Bit 1 - Quad wire read select"]
    #[inline(always)]
    pub fn qrd(&mut self) -> QRD_W {
        QRD_W::new(self)
    }
    #[doc = "Bit 0 - Quad wire mode enable"]
    #[inline(always)]
    pub fn qmod(&mut self) -> QMOD_W {
        QMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quad wire control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qctl](index.html) module"]
pub struct QCTL_SPEC;
impl crate::RegisterSpec for QCTL_SPEC {
    type Ux = u16;
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
