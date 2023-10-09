#[doc = "Register `BT` reader"]
pub type R = crate::R<BT_SPEC>;
#[doc = "Register `BT` writer"]
pub type W = crate::W<BT_SPEC>;
#[doc = "Field `BAUDPSC` reader - Baud rate prescaler"]
pub type BAUDPSC_R = crate::FieldReader<u16>;
#[doc = "Field `BAUDPSC` writer - Baud rate prescaler"]
pub type BAUDPSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `BS1` reader - Bit segment 1"]
pub type BS1_R = crate::FieldReader;
#[doc = "Field `BS1` writer - Bit segment 1"]
pub type BS1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BS2` reader - Bit segment 2"]
pub type BS2_R = crate::FieldReader;
#[doc = "Field `BS2` writer - Bit segment 2"]
pub type BS2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SJW` reader - Resynchronization jump width"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - Resynchronization jump width"]
pub type SJW_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O>;
#[doc = "Field `LCMOD` reader - Loopback communication mode"]
pub type LCMOD_R = crate::BitReader<LCMOD_A>;
#[doc = "Loopback communication mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCMOD_A {
    #[doc = "0: Loop Back Mode disabled"]
    DISABLED = 0,
    #[doc = "1: Loop Back Mode enabled"]
    ENABLED = 1,
}
impl From<LCMOD_A> for bool {
    #[inline(always)]
    fn from(variant: LCMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl LCMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCMOD_A {
        match self.bits {
            false => LCMOD_A::DISABLED,
            true => LCMOD_A::ENABLED,
        }
    }
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LCMOD_A::DISABLED
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LCMOD_A::ENABLED
    }
}
#[doc = "Field `LCMOD` writer - Loopback communication mode"]
pub type LCMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LCMOD_A>;
impl<'a, REG, const O: u8> LCMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LCMOD_A::DISABLED)
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LCMOD_A::ENABLED)
    }
}
#[doc = "Field `SCMOD` reader - Silent communication mode"]
pub type SCMOD_R = crate::BitReader<SCMOD_A>;
#[doc = "Silent communication mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCMOD_A {
    #[doc = "0: Normal operation"]
    NORMAL = 0,
    #[doc = "1: Silent Mode"]
    SILENT = 1,
}
impl From<SCMOD_A> for bool {
    #[inline(always)]
    fn from(variant: SCMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl SCMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCMOD_A {
        match self.bits {
            false => SCMOD_A::NORMAL,
            true => SCMOD_A::SILENT,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SCMOD_A::NORMAL
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        *self == SCMOD_A::SILENT
    }
}
#[doc = "Field `SCMOD` writer - Silent communication mode"]
pub type SCMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCMOD_A>;
impl<'a, REG, const O: u8> SCMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SCMOD_A::NORMAL)
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn silent(self) -> &'a mut crate::W<REG> {
        self.variant(SCMOD_A::SILENT)
    }
}
impl R {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn baudpsc(&self) -> BAUDPSC_R {
        BAUDPSC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    pub fn bs1(&self) -> BS1_R {
        BS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    pub fn bs2(&self) -> BS2_R {
        BS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    pub fn lcmod(&self) -> LCMOD_R {
        LCMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    pub fn scmod(&self) -> SCMOD_R {
        SCMOD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn baudpsc(&mut self) -> BAUDPSC_W<BT_SPEC, 0> {
        BAUDPSC_W::new(self)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<BT_SPEC, 16> {
        BS1_W::new(self)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<BT_SPEC, 20> {
        BS2_W::new(self)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<BT_SPEC, 24> {
        SJW_W::new(self)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcmod(&mut self) -> LCMOD_W<BT_SPEC, 30> {
        LCMOD_W::new(self)
    }
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmod(&mut self) -> SCMOD_W<BT_SPEC, 31> {
        SCMOD_W::new(self)
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
#[doc = "Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_SPEC;
impl crate::RegisterSpec for BT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt::R`](R) reader structure"]
impl crate::Readable for BT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt::W`](W) writer structure"]
impl crate::Writable for BT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BT to value 0x1230_0000"]
impl crate::Resettable for BT_SPEC {
    const RESET_VALUE: Self::Ux = 0x1230_0000;
}
