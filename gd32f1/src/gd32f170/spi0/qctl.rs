#[doc = "Register `QCTL` reader"]
pub type R = crate::R<QCTL_SPEC>;
#[doc = "Register `QCTL` writer"]
pub type W = crate::W<QCTL_SPEC>;
#[doc = "Field `QMOD` reader - Quad wire mode enable"]
pub type QMOD_R = crate::BitReader<QMOD_A>;
#[doc = "Quad wire mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl QMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QMOD_A {
        match self.bits {
            false => QMOD_A::SINGLE,
            true => QMOD_A::QUAD,
        }
    }
    #[doc = "Single wire mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == QMOD_A::SINGLE
    }
    #[doc = "Quad-SPI mode"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == QMOD_A::QUAD
    }
}
#[doc = "Field `QMOD` writer - Quad wire mode enable"]
pub type QMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, QMOD_A>;
impl<'a, REG, const O: u8> QMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single wire mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(QMOD_A::SINGLE)
    }
    #[doc = "Quad-SPI mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(QMOD_A::QUAD)
    }
}
#[doc = "Field `QRD` reader - Quad wire read select"]
pub type QRD_R = crate::BitReader<QRD_A>;
#[doc = "Quad wire read select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl QRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QRD_A {
        match self.bits {
            false => QRD_A::WRITE,
            true => QRD_A::READ,
        }
    }
    #[doc = "Quad wire write mode"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == QRD_A::WRITE
    }
    #[doc = "Quad wire read mode"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == QRD_A::READ
    }
}
#[doc = "Field `QRD` writer - Quad wire read select"]
pub type QRD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, QRD_A>;
impl<'a, REG, const O: u8> QRD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quad wire write mode"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(QRD_A::WRITE)
    }
    #[doc = "Quad wire read mode"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(QRD_A::READ)
    }
}
#[doc = "Field `IO23_DRV` reader - Drive IO2 and IO3 enable"]
pub type IO23_DRV_R = crate::BitReader<IO23_DRV_A>;
#[doc = "Drive IO2 and IO3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl IO23_DRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO23_DRV_A {
        match self.bits {
            false => IO23_DRV_A::DISABLED,
            true => IO23_DRV_A::HIGH,
        }
    }
    #[doc = "IO2 and IO3 are not driven in single wire mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IO23_DRV_A::DISABLED
    }
    #[doc = "IO2 and IO3 are driven high in single wire mode"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IO23_DRV_A::HIGH
    }
}
#[doc = "Field `IO23_DRV` writer - Drive IO2 and IO3 enable"]
pub type IO23_DRV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IO23_DRV_A>;
impl<'a, REG, const O: u8> IO23_DRV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO2 and IO3 are not driven in single wire mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IO23_DRV_A::DISABLED)
    }
    #[doc = "IO2 and IO3 are driven high in single wire mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IO23_DRV_A::HIGH)
    }
}
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Quad wire control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QCTL_SPEC;
impl crate::RegisterSpec for QCTL_SPEC {
    type Ux = u16;
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
