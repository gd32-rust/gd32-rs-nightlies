#[doc = "Register `BT` reader"]
pub type R = crate::R<BtSpec>;
#[doc = "Register `BT` writer"]
pub type W = crate::W<BtSpec>;
#[doc = "Field `BAUDPSC` reader - Baud rate prescaler"]
pub type BaudpscR = crate::FieldReader<u16>;
#[doc = "Field `BAUDPSC` writer - Baud rate prescaler"]
pub type BaudpscW<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
#[doc = "Field `BS1` reader - Bit segment 1"]
pub type Bs1R = crate::FieldReader;
#[doc = "Field `BS1` writer - Bit segment 1"]
pub type Bs1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BS2` reader - Bit segment 2"]
pub type Bs2R = crate::FieldReader;
#[doc = "Field `BS2` writer - Bit segment 2"]
pub type Bs2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SJW` reader - Resynchronization jump width"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - Resynchronization jump width"]
pub type SjwW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Loopback communication mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcmod {
    #[doc = "0: Loop Back Mode disabled"]
    Disabled = 0,
    #[doc = "1: Loop Back Mode enabled"]
    Enabled = 1,
}
impl From<Lcmod> for bool {
    #[inline(always)]
    fn from(variant: Lcmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCMOD` reader - Loopback communication mode"]
pub type LcmodR = crate::BitReader<Lcmod>;
impl LcmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcmod {
        match self.bits {
            false => Lcmod::Disabled,
            true => Lcmod::Enabled,
        }
    }
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lcmod::Disabled
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lcmod::Enabled
    }
}
#[doc = "Field `LCMOD` writer - Loopback communication mode"]
pub type LcmodW<'a, REG> = crate::BitWriter<'a, REG, Lcmod>;
impl<'a, REG> LcmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lcmod::Disabled)
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lcmod::Enabled)
    }
}
#[doc = "Silent communication mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scmod {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Silent Mode"]
    Silent = 1,
}
impl From<Scmod> for bool {
    #[inline(always)]
    fn from(variant: Scmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMOD` reader - Silent communication mode"]
pub type ScmodR = crate::BitReader<Scmod>;
impl ScmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scmod {
        match self.bits {
            false => Scmod::Normal,
            true => Scmod::Silent,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Scmod::Normal
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        *self == Scmod::Silent
    }
}
#[doc = "Field `SCMOD` writer - Silent communication mode"]
pub type ScmodW<'a, REG> = crate::BitWriter<'a, REG, Scmod>;
impl<'a, REG> ScmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Scmod::Normal)
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn silent(self) -> &'a mut crate::W<REG> {
        self.variant(Scmod::Silent)
    }
}
impl R {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn baudpsc(&self) -> BaudpscR {
        BaudpscR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    pub fn bs1(&self) -> Bs1R {
        Bs1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    pub fn bs2(&self) -> Bs2R {
        Bs2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    pub fn lcmod(&self) -> LcmodR {
        LcmodR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    pub fn scmod(&self) -> ScmodR {
        ScmodR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn baudpsc(&mut self) -> BaudpscW<BtSpec> {
        BaudpscW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> Bs1W<BtSpec> {
        Bs1W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> Bs2W<BtSpec> {
        Bs2W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SjwW<BtSpec> {
        SjwW::new(self, 24)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcmod(&mut self) -> LcmodW<BtSpec> {
        LcmodW::new(self, 30)
    }
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    #[must_use]
    pub fn scmod(&mut self) -> ScmodW<BtSpec> {
        ScmodW::new(self, 31)
    }
}
#[doc = "Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtSpec;
impl crate::RegisterSpec for BtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt::R`](R) reader structure"]
impl crate::Readable for BtSpec {}
#[doc = "`write(|w| ..)` method takes [`bt::W`](W) writer structure"]
impl crate::Writable for BtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BT to value 0x0123_0000"]
impl crate::Resettable for BtSpec {
    const RESET_VALUE: u32 = 0x0123_0000;
}
