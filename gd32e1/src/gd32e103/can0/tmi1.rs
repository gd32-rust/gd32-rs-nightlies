#[doc = "Register `TMI1` reader"]
pub type R = crate::R<TMI1_SPEC>;
#[doc = "Register `TMI1` writer"]
pub type W = crate::W<TMI1_SPEC>;
#[doc = "Field `TEN` reader - Transmit enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
#[doc = "Transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    #[doc = "0: Transmit disabled"]
    DISABLED = 0,
    #[doc = "1: Transmit enabled"]
    ENABLED = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::DISABLED,
            true => TEN_A::ENABLED,
        }
    }
    #[doc = "Transmit disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN_A::DISABLED
    }
    #[doc = "Transmit enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN_A::ENABLED
    }
}
#[doc = "Field `TEN` writer - Transmit enable"]
pub type TEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TEN_A>;
impl<'a, REG, const O: u8> TEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN_A::DISABLED)
    }
    #[doc = "Transmit enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEN_A::ENABLED)
    }
}
#[doc = "Field `FT` reader - Frame type"]
pub type FT_R = crate::BitReader<FT_A>;
#[doc = "Frame type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT_A {
    #[doc = "0: Data frame"]
    DATA = 0,
    #[doc = "1: Remote frame"]
    REMOTE = 1,
}
impl From<FT_A> for bool {
    #[inline(always)]
    fn from(variant: FT_A) -> Self {
        variant as u8 != 0
    }
}
impl FT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT_A {
        match self.bits {
            false => FT_A::DATA,
            true => FT_A::REMOTE,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == FT_A::DATA
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == FT_A::REMOTE
    }
}
#[doc = "Field `FT` writer - Frame type"]
pub type FT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FT_A>;
impl<'a, REG, const O: u8> FT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(FT_A::DATA)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn remote(self) -> &'a mut crate::W<REG> {
        self.variant(FT_A::REMOTE)
    }
}
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader<FF_A>;
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FF_A {
    #[doc = "0: Standard format frame"]
    STANDARD = 0,
    #[doc = "1: Extended format frame"]
    EXTENDED = 1,
}
impl From<FF_A> for bool {
    #[inline(always)]
    fn from(variant: FF_A) -> Self {
        variant as u8 != 0
    }
}
impl FF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FF_A {
        match self.bits {
            false => FF_A::STANDARD,
            true => FF_A::EXTENDED,
        }
    }
    #[doc = "Standard format frame"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FF_A::STANDARD
    }
    #[doc = "Extended format frame"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == FF_A::EXTENDED
    }
}
#[doc = "Field `FF` writer - Frame format"]
pub type FF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FF_A>;
impl<'a, REG, const O: u8> FF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard format frame"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(FF_A::STANDARD)
    }
    #[doc = "Extended format frame"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(FF_A::EXTENDED)
    }
}
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EFID_R = crate::FieldReader<u32>;
#[doc = "Field `EFID` writer - The frame identifier"]
pub type EFID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SFID_EFID_R = crate::FieldReader<u16>;
#[doc = "Field `SFID_EFID` writer - The frame identifier"]
pub type SFID_EFID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<TMI1_SPEC, 0> {
        TEN_W::new(self)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    #[must_use]
    pub fn ft(&mut self) -> FT_W<TMI1_SPEC, 1> {
        FT_W::new(self)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FF_W<TMI1_SPEC, 2> {
        FF_W::new(self)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    #[must_use]
    pub fn efid(&mut self) -> EFID_W<TMI1_SPEC, 3> {
        EFID_W::new(self)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    #[must_use]
    pub fn sfid_efid(&mut self) -> SFID_EFID_W<TMI1_SPEC, 21> {
        SFID_EFID_W::new(self)
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
#[doc = "Transmit mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMI1_SPEC;
impl crate::RegisterSpec for TMI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi1::R`](R) reader structure"]
impl crate::Readable for TMI1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmi1::W`](W) writer structure"]
impl crate::Writable for TMI1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMI1 to value 0"]
impl crate::Resettable for TMI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
