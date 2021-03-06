#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
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
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCS_A {
    #[doc = "0: IRC8M used as system clock"]
    IRC8M = 0,
    #[doc = "1: HXTAL used as system clock"]
    HXTAL = 1,
    #[doc = "2: PLL used as system clock"]
    PLL = 2,
}
impl From<SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCS` reader - System clock switch"]
pub type SCS_R = crate::FieldReader<u8, SCS_A>;
impl SCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCS_A> {
        match self.bits {
            0 => Some(SCS_A::IRC8M),
            1 => Some(SCS_A::HXTAL),
            2 => Some(SCS_A::PLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IRC8M`"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == SCS_A::IRC8M
    }
    #[doc = "Checks if the value of the field is `HXTAL`"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == SCS_A::HXTAL
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SCS_A::PLL
    }
}
#[doc = "Field `SCS` writer - System clock switch"]
pub type SCS_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, SCS_A, 2, 0>;
impl<'a> SCS_W<'a> {
    #[doc = "IRC8M used as system clock"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut W {
        self.variant(SCS_A::IRC8M)
    }
    #[doc = "HXTAL used as system clock"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut W {
        self.variant(SCS_A::HXTAL)
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SCS_A::PLL)
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCSS_A {
    #[doc = "0: IRC8M used as system clock"]
    IRC8M = 0,
    #[doc = "1: HXTAL used as system clock"]
    HXTAL = 1,
    #[doc = "2: PLL used as system clock"]
    PLL = 2,
}
impl From<SCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCSS` reader - System clock switch status"]
pub type SCSS_R = crate::FieldReader<u8, SCSS_A>;
impl SCSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCSS_A> {
        match self.bits {
            0 => Some(SCSS_A::IRC8M),
            1 => Some(SCSS_A::HXTAL),
            2 => Some(SCSS_A::PLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IRC8M`"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == SCSS_A::IRC8M
    }
    #[doc = "Checks if the value of the field is `HXTAL`"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == SCSS_A::HXTAL
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SCSS_A::PLL
    }
}
#[doc = "AHB prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHBPSC_A {
    #[doc = "0: CK_SYS"]
    DIV1 = 0,
    #[doc = "8: CK_SYS divided by 2"]
    DIV2 = 8,
    #[doc = "9: CK_SYS divided by 4"]
    DIV4 = 9,
    #[doc = "10: CK_SYS divided by 8"]
    DIV8 = 10,
    #[doc = "11: CK_SYS divided by 16"]
    DIV16 = 11,
    #[doc = "12: CK_SYS divided by 64"]
    DIV64 = 12,
    #[doc = "13: CK_SYS divided by 128"]
    DIV128 = 13,
    #[doc = "14: CK_SYS divided by 256"]
    DIV256 = 14,
    #[doc = "15: CK_SYS divided by 512"]
    DIV512 = 15,
}
impl From<AHBPSC_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBPSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AHBPSC` reader - AHB prescaler selection"]
pub type AHBPSC_R = crate::FieldReader<u8, AHBPSC_A>;
impl AHBPSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AHBPSC_A> {
        match self.bits {
            0 => Some(AHBPSC_A::DIV1),
            8 => Some(AHBPSC_A::DIV2),
            9 => Some(AHBPSC_A::DIV4),
            10 => Some(AHBPSC_A::DIV8),
            11 => Some(AHBPSC_A::DIV16),
            12 => Some(AHBPSC_A::DIV64),
            13 => Some(AHBPSC_A::DIV128),
            14 => Some(AHBPSC_A::DIV256),
            15 => Some(AHBPSC_A::DIV512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == AHBPSC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AHBPSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AHBPSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == AHBPSC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == AHBPSC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == AHBPSC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == AHBPSC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == AHBPSC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == AHBPSC_A::DIV512
    }
}
#[doc = "Field `AHBPSC` writer - AHB prescaler selection"]
pub type AHBPSC_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, AHBPSC_A, 4, 4>;
impl<'a> AHBPSC_W<'a> {
    #[doc = "CK_SYS"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV1)
    }
    #[doc = "CK_SYS divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV2)
    }
    #[doc = "CK_SYS divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV4)
    }
    #[doc = "CK_SYS divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV8)
    }
    #[doc = "CK_SYS divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV16)
    }
    #[doc = "CK_SYS divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV64)
    }
    #[doc = "CK_SYS divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV128)
    }
    #[doc = "CK_SYS divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV256)
    }
    #[doc = "CK_SYS divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(AHBPSC_A::DIV512)
    }
}
#[doc = "APB1 prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APB1PSC_A {
    #[doc = "0: CK_AHB"]
    DIV1 = 0,
    #[doc = "4: CK_AHB divided by 2"]
    DIV2 = 4,
    #[doc = "5: CK_AHB divided by 4"]
    DIV4 = 5,
    #[doc = "6: CK_AHB divided by 8"]
    DIV8 = 6,
    #[doc = "7: CK_AHB divided by 16"]
    DIV16 = 7,
}
impl From<APB1PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: APB1PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APB1PSC` reader - APB1 prescaler selection"]
pub type APB1PSC_R = crate::FieldReader<u8, APB1PSC_A>;
impl APB1PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<APB1PSC_A> {
        match self.bits {
            0 => Some(APB1PSC_A::DIV1),
            4 => Some(APB1PSC_A::DIV2),
            5 => Some(APB1PSC_A::DIV4),
            6 => Some(APB1PSC_A::DIV8),
            7 => Some(APB1PSC_A::DIV16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == APB1PSC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == APB1PSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == APB1PSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == APB1PSC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == APB1PSC_A::DIV16
    }
}
#[doc = "Field `APB1PSC` writer - APB1 prescaler selection"]
pub type APB1PSC_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, APB1PSC_A, 3, 8>;
impl<'a> APB1PSC_W<'a> {
    #[doc = "CK_AHB"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(APB1PSC_A::DIV1)
    }
    #[doc = "CK_AHB divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(APB1PSC_A::DIV2)
    }
    #[doc = "CK_AHB divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(APB1PSC_A::DIV4)
    }
    #[doc = "CK_AHB divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(APB1PSC_A::DIV8)
    }
    #[doc = "CK_AHB divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(APB1PSC_A::DIV16)
    }
}
#[doc = "APB2 prescaler selection"]
pub use APB1PSC_A as APB2PSC_A;
#[doc = "Field `APB2PSC` reader - APB2 prescaler selection"]
pub use APB1PSC_R as APB2PSC_R;
#[doc = "Field `APB2PSC` writer - APB2 prescaler selection"]
pub use APB1PSC_W as APB2PSC_W;
#[doc = "ADC clock prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCPSC_A {
    #[doc = "0: CK_APB2 divided by 2"]
    DIV2 = 0,
    #[doc = "1: CK_APB2 divided by 4"]
    DIV4 = 1,
    #[doc = "2: CK_APB2 divided by 6"]
    DIV6 = 2,
    #[doc = "3: CK_APB2 divided by 8"]
    DIV8 = 3,
}
impl From<ADCPSC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCPSC` reader - ADC clock prescaler selection"]
pub type ADCPSC_R = crate::FieldReader<u8, ADCPSC_A>;
impl ADCPSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPSC_A {
        match self.bits {
            0 => ADCPSC_A::DIV2,
            1 => ADCPSC_A::DIV4,
            2 => ADCPSC_A::DIV6,
            3 => ADCPSC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPSC_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPSC_A::DIV8
    }
}
#[doc = "Field `ADCPSC` writer - ADC clock prescaler selection"]
pub type ADCPSC_W<'a> = crate::FieldWriterSafe<'a, u32, CFG0_SPEC, u8, ADCPSC_A, 2, 14>;
impl<'a> ADCPSC_W<'a> {
    #[doc = "CK_APB2 divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADCPSC_A::DIV2)
    }
    #[doc = "CK_APB2 divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADCPSC_A::DIV4)
    }
    #[doc = "CK_APB2 divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADCPSC_A::DIV6)
    }
    #[doc = "CK_APB2 divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADCPSC_A::DIV8)
    }
}
#[doc = "PLL Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSEL_A {
    #[doc = "0: IRC8M / 2 selected as PLL source clock"]
    IRC8M_2 = 0,
    #[doc = "1: HXTAL selected as PLL source clock"]
    HXTAL = 1,
}
impl From<PLLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSEL` reader - PLL Clock Source Selection"]
pub type PLLSEL_R = crate::BitReader<PLLSEL_A>;
impl PLLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSEL_A {
        match self.bits {
            false => PLLSEL_A::IRC8M_2,
            true => PLLSEL_A::HXTAL,
        }
    }
    #[doc = "Checks if the value of the field is `IRC8M_2`"]
    #[inline(always)]
    pub fn is_irc8m_2(&self) -> bool {
        *self == PLLSEL_A::IRC8M_2
    }
    #[doc = "Checks if the value of the field is `HXTAL`"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == PLLSEL_A::HXTAL
    }
}
#[doc = "Field `PLLSEL` writer - PLL Clock Source Selection"]
pub type PLLSEL_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, PLLSEL_A, 16>;
impl<'a> PLLSEL_W<'a> {
    #[doc = "IRC8M / 2 selected as PLL source clock"]
    #[inline(always)]
    pub fn irc8m_2(self) -> &'a mut W {
        self.variant(PLLSEL_A::IRC8M_2)
    }
    #[doc = "HXTAL selected as PLL source clock"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut W {
        self.variant(PLLSEL_A::HXTAL)
    }
}
#[doc = "HXTAL divider for PLL source clock selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPREDV_A {
    #[doc = "0: HXTAL clock not divided"]
    DIV1 = 0,
    #[doc = "1: HXTAL clock divided by 2"]
    DIV2 = 1,
}
impl From<PLLPREDV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPREDV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPREDV` reader - HXTAL divider for PLL source clock selection."]
pub type PLLPREDV_R = crate::BitReader<PLLPREDV_A>;
impl PLLPREDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPREDV_A {
        match self.bits {
            false => PLLPREDV_A::DIV1,
            true => PLLPREDV_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLPREDV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLPREDV_A::DIV2
    }
}
#[doc = "Field `PLLPREDV` writer - HXTAL divider for PLL source clock selection."]
pub type PLLPREDV_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, PLLPREDV_A, 17>;
impl<'a> PLLPREDV_W<'a> {
    #[doc = "HXTAL clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLPREDV_A::DIV1)
    }
    #[doc = "HXTAL clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLPREDV_A::DIV2)
    }
}
#[doc = "PLL multiply factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLMF_A {
    #[doc = "0: PLL input clock x2"]
    MUL2 = 0,
    #[doc = "1: PLL input clock x3"]
    MUL3 = 1,
    #[doc = "2: PLL input clock x4"]
    MUL4 = 2,
    #[doc = "3: PLL input clock x5"]
    MUL5 = 3,
    #[doc = "4: PLL input clock x6"]
    MUL6 = 4,
    #[doc = "5: PLL input clock x7"]
    MUL7 = 5,
    #[doc = "6: PLL input clock x8"]
    MUL8 = 6,
    #[doc = "7: PLL input clock x9"]
    MUL9 = 7,
    #[doc = "8: PLL input clock x10"]
    MUL10 = 8,
    #[doc = "9: PLL input clock x11"]
    MUL11 = 9,
    #[doc = "10: PLL input clock x12"]
    MUL12 = 10,
    #[doc = "11: PLL input clock x13"]
    MUL13 = 11,
    #[doc = "12: PLL input clock x14"]
    MUL14 = 12,
    #[doc = "13: PLL input clock x15"]
    MUL15 = 13,
    #[doc = "14: PLL input clock x16"]
    MUL16 = 14,
    #[doc = "15: PLL input clock x16"]
    MUL16X = 15,
}
impl From<PLLMF_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLMF` reader - PLL multiply factor"]
pub type PLLMF_R = crate::FieldReader<u8, PLLMF_A>;
impl PLLMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLMF_A {
        match self.bits {
            0 => PLLMF_A::MUL2,
            1 => PLLMF_A::MUL3,
            2 => PLLMF_A::MUL4,
            3 => PLLMF_A::MUL5,
            4 => PLLMF_A::MUL6,
            5 => PLLMF_A::MUL7,
            6 => PLLMF_A::MUL8,
            7 => PLLMF_A::MUL9,
            8 => PLLMF_A::MUL10,
            9 => PLLMF_A::MUL11,
            10 => PLLMF_A::MUL12,
            11 => PLLMF_A::MUL13,
            12 => PLLMF_A::MUL14,
            13 => PLLMF_A::MUL15,
            14 => PLLMF_A::MUL16,
            15 => PLLMF_A::MUL16X,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == PLLMF_A::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL3`"]
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMF_A::MUL3
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMF_A::MUL4
    }
    #[doc = "Checks if the value of the field is `MUL5`"]
    #[inline(always)]
    pub fn is_mul5(&self) -> bool {
        *self == PLLMF_A::MUL5
    }
    #[doc = "Checks if the value of the field is `MUL6`"]
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMF_A::MUL6
    }
    #[doc = "Checks if the value of the field is `MUL7`"]
    #[inline(always)]
    pub fn is_mul7(&self) -> bool {
        *self == PLLMF_A::MUL7
    }
    #[doc = "Checks if the value of the field is `MUL8`"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMF_A::MUL8
    }
    #[doc = "Checks if the value of the field is `MUL9`"]
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == PLLMF_A::MUL9
    }
    #[doc = "Checks if the value of the field is `MUL10`"]
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        *self == PLLMF_A::MUL10
    }
    #[doc = "Checks if the value of the field is `MUL11`"]
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        *self == PLLMF_A::MUL11
    }
    #[doc = "Checks if the value of the field is `MUL12`"]
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMF_A::MUL12
    }
    #[doc = "Checks if the value of the field is `MUL13`"]
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        *self == PLLMF_A::MUL13
    }
    #[doc = "Checks if the value of the field is `MUL14`"]
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        *self == PLLMF_A::MUL14
    }
    #[doc = "Checks if the value of the field is `MUL15`"]
    #[inline(always)]
    pub fn is_mul15(&self) -> bool {
        *self == PLLMF_A::MUL15
    }
    #[doc = "Checks if the value of the field is `MUL16`"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMF_A::MUL16
    }
    #[doc = "Checks if the value of the field is `MUL16X`"]
    #[inline(always)]
    pub fn is_mul16x(&self) -> bool {
        *self == PLLMF_A::MUL16X
    }
}
#[doc = "Field `PLLMF` writer - PLL multiply factor"]
pub type PLLMF_W<'a> = crate::FieldWriterSafe<'a, u32, CFG0_SPEC, u8, PLLMF_A, 4, 18>;
impl<'a> PLLMF_W<'a> {
    #[doc = "PLL input clock x2"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL2)
    }
    #[doc = "PLL input clock x3"]
    #[inline(always)]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL3)
    }
    #[doc = "PLL input clock x4"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL4)
    }
    #[doc = "PLL input clock x5"]
    #[inline(always)]
    pub fn mul5(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL5)
    }
    #[doc = "PLL input clock x6"]
    #[inline(always)]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL6)
    }
    #[doc = "PLL input clock x7"]
    #[inline(always)]
    pub fn mul7(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL7)
    }
    #[doc = "PLL input clock x8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL8)
    }
    #[doc = "PLL input clock x9"]
    #[inline(always)]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL9)
    }
    #[doc = "PLL input clock x10"]
    #[inline(always)]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL10)
    }
    #[doc = "PLL input clock x11"]
    #[inline(always)]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL11)
    }
    #[doc = "PLL input clock x12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL12)
    }
    #[doc = "PLL input clock x13"]
    #[inline(always)]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL13)
    }
    #[doc = "PLL input clock x14"]
    #[inline(always)]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL14)
    }
    #[doc = "PLL input clock x15"]
    #[inline(always)]
    pub fn mul15(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL15)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL16)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16x(self) -> &'a mut W {
        self.variant(PLLMF_A::MUL16X)
    }
}
#[doc = "CK_OUT Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKOUTSEL_A {
    #[doc = "0: No clock selected"]
    NONE = 0,
    #[doc = "1: Internal 14 MHz RC oscillator clock selected"]
    IRC14M = 1,
    #[doc = "2: Internal 40 kHz RC oscillator clock selected"]
    LSI40K = 2,
    #[doc = "3: External low speed oscillator clock selected"]
    LXTAL = 3,
    #[doc = "4: System clock selected"]
    SYSCLK = 4,
    #[doc = "5: Internal RC 8 MHz (HSI) oscillator clock selected"]
    IRC8M = 5,
    #[doc = "6: External 4-32 MHz (HSE) oscillator clock selected"]
    HXTAL = 6,
    #[doc = "7: PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    PLL = 7,
}
impl From<CKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKOUTSEL` reader - CK_OUT Clock Source Selection"]
pub type CKOUTSEL_R = crate::FieldReader<u8, CKOUTSEL_A>;
impl CKOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKOUTSEL_A {
        match self.bits {
            0 => CKOUTSEL_A::NONE,
            1 => CKOUTSEL_A::IRC14M,
            2 => CKOUTSEL_A::LSI40K,
            3 => CKOUTSEL_A::LXTAL,
            4 => CKOUTSEL_A::SYSCLK,
            5 => CKOUTSEL_A::IRC8M,
            6 => CKOUTSEL_A::HXTAL,
            7 => CKOUTSEL_A::PLL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKOUTSEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `IRC14M`"]
    #[inline(always)]
    pub fn is_irc14m(&self) -> bool {
        *self == CKOUTSEL_A::IRC14M
    }
    #[doc = "Checks if the value of the field is `LSI40K`"]
    #[inline(always)]
    pub fn is_lsi40k(&self) -> bool {
        *self == CKOUTSEL_A::LSI40K
    }
    #[doc = "Checks if the value of the field is `LXTAL`"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == CKOUTSEL_A::LXTAL
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKOUTSEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `IRC8M`"]
    #[inline(always)]
    pub fn is_irc8m(&self) -> bool {
        *self == CKOUTSEL_A::IRC8M
    }
    #[doc = "Checks if the value of the field is `HXTAL`"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == CKOUTSEL_A::HXTAL
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CKOUTSEL_A::PLL
    }
}
#[doc = "Field `CKOUTSEL` writer - CK_OUT Clock Source Selection"]
pub type CKOUTSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CFG0_SPEC, u8, CKOUTSEL_A, 3, 24>;
impl<'a> CKOUTSEL_W<'a> {
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CKOUTSEL_A::NONE)
    }
    #[doc = "Internal 14 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn irc14m(self) -> &'a mut W {
        self.variant(CKOUTSEL_A::IRC14M)
    }
    #[doc = "Internal 40 kHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn lsi40k(self) -> &'a mut W {
        self.variant(CKOUTSEL_A::LSI40K)
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut W {
        self.variant(CKOUTSEL_A::LXTAL)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CKOUTSEL_A::SYSCLK)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn irc8m(self) -> &'a mut W {
        self.variant(CKOUTSEL_A::IRC8M)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut W {
        self.variant(CKOUTSEL_A::HXTAL)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending on PLLDV)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CKOUTSEL_A::PLL)
    }
}
#[doc = "Bit 4 of PLLMF register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLMF_MSB_A {
    #[doc = "0: Value of PLLMF is as set"]
    NONE = 0,
    #[doc = "1: Add 15 to the value of PLLMF"]
    PLUS15 = 1,
}
impl From<PLLMF_MSB_A> for bool {
    #[inline(always)]
    fn from(variant: PLLMF_MSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLMF_MSB` reader - Bit 4 of PLLMF register"]
pub type PLLMF_MSB_R = crate::BitReader<PLLMF_MSB_A>;
impl PLLMF_MSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLMF_MSB_A {
        match self.bits {
            false => PLLMF_MSB_A::NONE,
            true => PLLMF_MSB_A::PLUS15,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLLMF_MSB_A::NONE
    }
    #[doc = "Checks if the value of the field is `PLUS15`"]
    #[inline(always)]
    pub fn is_plus15(&self) -> bool {
        *self == PLLMF_MSB_A::PLUS15
    }
}
#[doc = "Field `PLLMF_MSB` writer - Bit 4 of PLLMF register"]
pub type PLLMF_MSB_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, PLLMF_MSB_A, 27>;
impl<'a> PLLMF_MSB_W<'a> {
    #[doc = "Value of PLLMF is as set"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PLLMF_MSB_A::NONE)
    }
    #[doc = "Add 15 to the value of PLLMF"]
    #[inline(always)]
    pub fn plus15(self) -> &'a mut W {
        self.variant(PLLMF_MSB_A::PLUS15)
    }
}
#[doc = "The CK_OUT divider which the CK_OUT frequency can be reduced\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKOUTDIV_A {
    #[doc = "0: CK_OUT is divided by 1"]
    DIV1 = 0,
    #[doc = "1: CK_OUT is divided by 2"]
    DIV2 = 1,
    #[doc = "2: CK_OUT is divided by 4"]
    DIV4 = 2,
    #[doc = "3: CK_OUT is divided by 8"]
    DIV8 = 3,
    #[doc = "4: CK_OUT is divided by 16"]
    DIV16 = 4,
    #[doc = "5: CK_OUT is divided by 32"]
    DIV32 = 5,
    #[doc = "6: CK_OUT is divided by 64"]
    DIV64 = 6,
    #[doc = "7: CK_OUT is divided by 128"]
    DIV128 = 7,
}
impl From<CKOUTDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOUTDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKOUTDIV` reader - The CK_OUT divider which the CK_OUT frequency can be reduced"]
pub type CKOUTDIV_R = crate::FieldReader<u8, CKOUTDIV_A>;
impl CKOUTDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKOUTDIV_A {
        match self.bits {
            0 => CKOUTDIV_A::DIV1,
            1 => CKOUTDIV_A::DIV2,
            2 => CKOUTDIV_A::DIV4,
            3 => CKOUTDIV_A::DIV8,
            4 => CKOUTDIV_A::DIV16,
            5 => CKOUTDIV_A::DIV32,
            6 => CKOUTDIV_A::DIV64,
            7 => CKOUTDIV_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CKOUTDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CKOUTDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CKOUTDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CKOUTDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CKOUTDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CKOUTDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CKOUTDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CKOUTDIV_A::DIV128
    }
}
#[doc = "Field `CKOUTDIV` writer - The CK_OUT divider which the CK_OUT frequency can be reduced"]
pub type CKOUTDIV_W<'a> = crate::FieldWriterSafe<'a, u32, CFG0_SPEC, u8, CKOUTDIV_A, 3, 28>;
impl<'a> CKOUTDIV_W<'a> {
    #[doc = "CK_OUT is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::DIV1)
    }
    #[doc = "CK_OUT is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::DIV2)
    }
    #[doc = "CK_OUT is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::DIV4)
    }
    #[doc = "CK_OUT is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::DIV8)
    }
    #[doc = "CK_OUT is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::DIV16)
    }
    #[doc = "CK_OUT is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::DIV32)
    }
    #[doc = "CK_OUT is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::DIV64)
    }
    #[doc = "CK_OUT is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::DIV128)
    }
}
#[doc = "The CK_PLL divide by 1 or 2 for CK_OUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLDV_A {
    #[doc = "0: PLL is divided by 2 for CK_OUT"]
    DIV2 = 0,
    #[doc = "1: PLL is not divided for CK_OUT"]
    DIV1 = 1,
}
impl From<PLLDV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLDV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLDV` reader - The CK_PLL divide by 1 or 2 for CK_OUT"]
pub type PLLDV_R = crate::BitReader<PLLDV_A>;
impl PLLDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLDV_A {
        match self.bits {
            false => PLLDV_A::DIV2,
            true => PLLDV_A::DIV1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLDV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLDV_A::DIV1
    }
}
#[doc = "Field `PLLDV` writer - The CK_PLL divide by 1 or 2 for CK_OUT"]
pub type PLLDV_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, PLLDV_A, 31>;
impl<'a> PLLDV_W<'a> {
    #[doc = "PLL is divided by 2 for CK_OUT"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLDV_A::DIV2)
    }
    #[doc = "PLL is not divided for CK_OUT"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLDV_A::DIV1)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> SCSS_R {
        SCSS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AHBPSC_R {
        AHBPSC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&self) -> APB1PSC_R {
        APB1PSC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&self) -> APB2PSC_R {
        APB2PSC_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&self) -> ADCPSC_R {
        ADCPSC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&self) -> PLLSEL_R {
        PLLSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HXTAL divider for PLL source clock selection."]
    #[inline(always)]
    pub fn pllpredv(&self) -> PLLPREDV_R {
        PLLPREDV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL multiply factor"]
    #[inline(always)]
    pub fn pllmf(&self) -> PLLMF_R {
        PLLMF_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - CK_OUT Clock Source Selection"]
    #[inline(always)]
    pub fn ckoutsel(&self) -> CKOUTSEL_R {
        CKOUTSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Bit 4 of PLLMF register"]
    #[inline(always)]
    pub fn pllmf_msb(&self) -> PLLMF_MSB_R {
        PLLMF_MSB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - The CK_OUT divider which the CK_OUT frequency can be reduced"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - The CK_PLL divide by 1 or 2 for CK_OUT"]
    #[inline(always)]
    pub fn plldv(&self) -> PLLDV_R {
        PLLDV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&mut self) -> AHBPSC_W {
        AHBPSC_W::new(self)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&mut self) -> APB1PSC_W {
        APB1PSC_W::new(self)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&mut self) -> APB2PSC_W {
        APB2PSC_W::new(self)
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc(&mut self) -> ADCPSC_W {
        ADCPSC_W::new(self)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&mut self) -> PLLSEL_W {
        PLLSEL_W::new(self)
    }
    #[doc = "Bit 17 - HXTAL divider for PLL source clock selection."]
    #[inline(always)]
    pub fn pllpredv(&mut self) -> PLLPREDV_W {
        PLLPREDV_W::new(self)
    }
    #[doc = "Bits 18:21 - PLL multiply factor"]
    #[inline(always)]
    pub fn pllmf(&mut self) -> PLLMF_W {
        PLLMF_W::new(self)
    }
    #[doc = "Bits 24:26 - CK_OUT Clock Source Selection"]
    #[inline(always)]
    pub fn ckoutsel(&mut self) -> CKOUTSEL_W {
        CKOUTSEL_W::new(self)
    }
    #[doc = "Bit 27 - Bit 4 of PLLMF register"]
    #[inline(always)]
    pub fn pllmf_msb(&mut self) -> PLLMF_MSB_W {
        PLLMF_MSB_W::new(self)
    }
    #[doc = "Bits 28:30 - The CK_OUT divider which the CK_OUT frequency can be reduced"]
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W {
        CKOUTDIV_W::new(self)
    }
    #[doc = "Bit 31 - The CK_PLL divide by 1 or 2 for CK_OUT"]
    #[inline(always)]
    pub fn plldv(&mut self) -> PLLDV_W {
        PLLDV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 0 (RCU_CFG0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
