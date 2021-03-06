#[doc = "Register `CH4CTL` reader"]
pub struct R(crate::R<CH4CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4CTL` writer"]
pub struct W(crate::W<CH4CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4CTL_SPEC>;
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
impl From<crate::W<CH4CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN_A {
    #[doc = "0: Channel disabled"]
    DISABLED = 0,
    #[doc = "1: Channel enabled"]
    ENABLED = 1,
}
impl From<CHEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN` reader - Channel enable"]
pub type CHEN_R = crate::BitReader<CHEN_A>;
impl CHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN_A {
        match self.bits {
            false => CHEN_A::DISABLED,
            true => CHEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHEN_A::ENABLED
    }
}
#[doc = "Field `CHEN` writer - Channel enable"]
pub type CHEN_W<'a> = crate::BitWriter<'a, u32, CH4CTL_SPEC, CHEN_A, 0>;
impl<'a> CHEN_W<'a> {
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CHEN_A::DISABLED)
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CHEN_A::ENABLED)
    }
}
#[doc = "Enable bit for full transfer finish interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTFIE_A {
    #[doc = "0: Full transfer interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Full transfer interrupt enabled"]
    ENABLED = 1,
}
impl From<FTFIE_A> for bool {
    #[inline(always)]
    fn from(variant: FTFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTFIE` reader - Enable bit for full transfer finish interrupt"]
pub type FTFIE_R = crate::BitReader<FTFIE_A>;
impl FTFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTFIE_A {
        match self.bits {
            false => FTFIE_A::DISABLED,
            true => FTFIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FTFIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FTFIE_A::ENABLED
    }
}
#[doc = "Field `FTFIE` writer - Enable bit for full transfer finish interrupt"]
pub type FTFIE_W<'a> = crate::BitWriter<'a, u32, CH4CTL_SPEC, FTFIE_A, 1>;
impl<'a> FTFIE_W<'a> {
    #[doc = "Full transfer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FTFIE_A::DISABLED)
    }
    #[doc = "Full transfer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FTFIE_A::ENABLED)
    }
}
#[doc = "Enable bit for half transfer finish interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTFIE_A {
    #[doc = "0: Half transfer interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Half transfer interrupt enabled"]
    ENABLED = 1,
}
impl From<HTFIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTFIE` reader - Enable bit for half transfer finish interrupt"]
pub type HTFIE_R = crate::BitReader<HTFIE_A>;
impl HTFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTFIE_A {
        match self.bits {
            false => HTFIE_A::DISABLED,
            true => HTFIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTFIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTFIE_A::ENABLED
    }
}
#[doc = "Field `HTFIE` writer - Enable bit for half transfer finish interrupt"]
pub type HTFIE_W<'a> = crate::BitWriter<'a, u32, CH4CTL_SPEC, HTFIE_A, 2>;
impl<'a> HTFIE_W<'a> {
    #[doc = "Half transfer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTFIE_A::DISABLED)
    }
    #[doc = "Half transfer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTFIE_A::ENABLED)
    }
}
#[doc = "Enable bit for tranfer access error interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Transfer error interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Transfer error interrupt enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Enable bit for tranfer access error interrupt"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Enable bit for tranfer access error interrupt"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, CH4CTL_SPEC, ERRIE_A, 3>;
impl<'a> ERRIE_W<'a> {
    #[doc = "Transfer error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Transfer error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Read from peripheral"]
    FROMPERIPHERAL = 0,
    #[doc = "1: Read from memory"]
    FROMMEMORY = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Transfer mode"]
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::FROMPERIPHERAL,
            true => DIR_A::FROMMEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `FROMPERIPHERAL`"]
    #[inline(always)]
    pub fn is_from_peripheral(&self) -> bool {
        *self == DIR_A::FROMPERIPHERAL
    }
    #[doc = "Checks if the value of the field is `FROMMEMORY`"]
    #[inline(always)]
    pub fn is_from_memory(&self) -> bool {
        *self == DIR_A::FROMMEMORY
    }
}
#[doc = "Field `DIR` writer - Transfer mode"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, CH4CTL_SPEC, DIR_A, 4>;
impl<'a> DIR_W<'a> {
    #[doc = "Read from peripheral"]
    #[inline(always)]
    pub fn from_peripheral(self) -> &'a mut W {
        self.variant(DIR_A::FROMPERIPHERAL)
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn from_memory(self) -> &'a mut W {
        self.variant(DIR_A::FROMMEMORY)
    }
}
#[doc = "Circular mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMEN_A {
    #[doc = "0: Circular buffer disabled"]
    DISABLED = 0,
    #[doc = "1: Circular buffer enabled"]
    ENABLED = 1,
}
impl From<CMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMEN` reader - Circular mode enable"]
pub type CMEN_R = crate::BitReader<CMEN_A>;
impl CMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMEN_A {
        match self.bits {
            false => CMEN_A::DISABLED,
            true => CMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMEN_A::ENABLED
    }
}
#[doc = "Field `CMEN` writer - Circular mode enable"]
pub type CMEN_W<'a> = crate::BitWriter<'a, u32, CH4CTL_SPEC, CMEN_A, 5>;
impl<'a> CMEN_W<'a> {
    #[doc = "Circular buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMEN_A::DISABLED)
    }
    #[doc = "Circular buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMEN_A::ENABLED)
    }
}
#[doc = "Next address generation algorithm of peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PNAGA_A {
    #[doc = "0: Fixed address mode"]
    FIXED = 0,
    #[doc = "1: Increment address mode"]
    INCREMENT = 1,
}
impl From<PNAGA_A> for bool {
    #[inline(always)]
    fn from(variant: PNAGA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PNAGA` reader - Next address generation algorithm of peripheral"]
pub type PNAGA_R = crate::BitReader<PNAGA_A>;
impl PNAGA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PNAGA_A {
        match self.bits {
            false => PNAGA_A::FIXED,
            true => PNAGA_A::INCREMENT,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == PNAGA_A::FIXED
    }
    #[doc = "Checks if the value of the field is `INCREMENT`"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == PNAGA_A::INCREMENT
    }
}
#[doc = "Field `PNAGA` writer - Next address generation algorithm of peripheral"]
pub type PNAGA_W<'a> = crate::BitWriter<'a, u32, CH4CTL_SPEC, PNAGA_A, 6>;
impl<'a> PNAGA_W<'a> {
    #[doc = "Fixed address mode"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(PNAGA_A::FIXED)
    }
    #[doc = "Increment address mode"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut W {
        self.variant(PNAGA_A::INCREMENT)
    }
}
#[doc = "Next address generation algorithm of memory"]
pub use PNAGA_A as MNAGA_A;
#[doc = "Field `MNAGA` reader - Next address generation algorithm of memory"]
pub use PNAGA_R as MNAGA_R;
#[doc = "Field `MNAGA` writer - Next address generation algorithm of memory"]
pub use PNAGA_W as MNAGA_W;
#[doc = "Transfer data size of peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWIDTH_A {
    #[doc = "0: 8-bit size"]
    BITS8 = 0,
    #[doc = "1: 16-bit size"]
    BITS16 = 1,
    #[doc = "2: 32-bit size"]
    BITS32 = 2,
}
impl From<PWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: PWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWIDTH` reader - Transfer data size of peripheral"]
pub type PWIDTH_R = crate::FieldReader<u8, PWIDTH_A>;
impl PWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWIDTH_A> {
        match self.bits {
            0 => Some(PWIDTH_A::BITS8),
            1 => Some(PWIDTH_A::BITS16),
            2 => Some(PWIDTH_A::BITS32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PWIDTH_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PWIDTH_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PWIDTH_A::BITS32
    }
}
#[doc = "Field `PWIDTH` writer - Transfer data size of peripheral"]
pub type PWIDTH_W<'a> = crate::FieldWriter<'a, u32, CH4CTL_SPEC, u8, PWIDTH_A, 2, 8>;
impl<'a> PWIDTH_W<'a> {
    #[doc = "8-bit size"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PWIDTH_A::BITS8)
    }
    #[doc = "16-bit size"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PWIDTH_A::BITS16)
    }
    #[doc = "32-bit size"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(PWIDTH_A::BITS32)
    }
}
#[doc = "Transfer data size of memory"]
pub use PWIDTH_A as MWIDTH_A;
#[doc = "Field `MWIDTH` reader - Transfer data size of memory"]
pub use PWIDTH_R as MWIDTH_R;
#[doc = "Field `MWIDTH` writer - Transfer data size of memory"]
pub use PWIDTH_W as MWIDTH_W;
#[doc = "Priority Level of this channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRIO_A {
    #[doc = "0: Low priority"]
    LOW = 0,
    #[doc = "1: Medium priority"]
    MEDIUM = 1,
    #[doc = "2: High priority"]
    HIGH = 2,
    #[doc = "3: Very high priority"]
    VERYHIGH = 3,
}
impl From<PRIO_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRIO` reader - Priority Level of this channel"]
pub type PRIO_R = crate::FieldReader<u8, PRIO_A>;
impl PRIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIO_A {
        match self.bits {
            0 => PRIO_A::LOW,
            1 => PRIO_A::MEDIUM,
            2 => PRIO_A::HIGH,
            3 => PRIO_A::VERYHIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PRIO_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PRIO_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PRIO_A::HIGH
    }
    #[doc = "Checks if the value of the field is `VERYHIGH`"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PRIO_A::VERYHIGH
    }
}
#[doc = "Field `PRIO` writer - Priority Level of this channel"]
pub type PRIO_W<'a> = crate::FieldWriterSafe<'a, u32, CH4CTL_SPEC, u8, PRIO_A, 2, 12>;
impl<'a> PRIO_W<'a> {
    #[doc = "Low priority"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PRIO_A::LOW)
    }
    #[doc = "Medium priority"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(PRIO_A::MEDIUM)
    }
    #[doc = "High priority"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PRIO_A::HIGH)
    }
    #[doc = "Very high priority"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PRIO_A::VERYHIGH)
    }
}
#[doc = "Memory to memory mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M2M_A {
    #[doc = "0: Memory to memory mode disabled"]
    DISABLED = 0,
    #[doc = "1: Memory to memory mode enabled"]
    ENABLED = 1,
}
impl From<M2M_A> for bool {
    #[inline(always)]
    fn from(variant: M2M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M2M` reader - Memory to memory mode"]
pub type M2M_R = crate::BitReader<M2M_A>;
impl M2M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2M_A {
        match self.bits {
            false => M2M_A::DISABLED,
            true => M2M_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M2M_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M2M_A::ENABLED
    }
}
#[doc = "Field `M2M` writer - Memory to memory mode"]
pub type M2M_W<'a> = crate::BitWriter<'a, u32, CH4CTL_SPEC, M2M_A, 14>;
impl<'a> M2M_W<'a> {
    #[doc = "Memory to memory mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M2M_A::DISABLED)
    }
    #[doc = "Memory to memory mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M2M_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bit for full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&self) -> FTFIE_R {
        FTFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&self) -> HTFIE_R {
        HTFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable bit for tranfer access error interrupt"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer mode"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&self) -> CMEN_R {
        CMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&self) -> PNAGA_R {
        PNAGA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&self) -> MNAGA_R {
        MNAGA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Priority Level of this channel"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2M_R {
        M2M_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable bit for full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&mut self) -> FTFIE_W {
        FTFIE_W::new(self)
    }
    #[doc = "Bit 2 - Enable bit for half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&mut self) -> HTFIE_W {
        HTFIE_W::new(self)
    }
    #[doc = "Bit 3 - Enable bit for tranfer access error interrupt"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 4 - Transfer mode"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&mut self) -> CMEN_W {
        CMEN_W::new(self)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&mut self) -> PNAGA_W {
        PNAGA_W::new(self)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&mut self) -> MNAGA_W {
        MNAGA_W::new(self)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&mut self) -> PWIDTH_W {
        PWIDTH_W::new(self)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&mut self) -> MWIDTH_W {
        MWIDTH_W::new(self)
    }
    #[doc = "Bits 12:13 - Priority Level of this channel"]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W {
        PRIO_W::new(self)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn m2m(&mut self) -> M2M_W {
        M2M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel configuration register (DMA_CH4CTL0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4ctl](index.html) module"]
pub struct CH4CTL_SPEC;
impl crate::RegisterSpec for CH4CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4ctl::R](R) reader structure"]
impl crate::Readable for CH4CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4ctl::W](W) writer structure"]
impl crate::Writable for CH4CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH4CTL to value 0"]
impl crate::Resettable for CH4CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
