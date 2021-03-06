#[doc = "Register `IVREF_CTL` reader"]
pub struct R(crate::R<IVREF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVREF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVREF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVREF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IVREF_CTL` writer"]
pub struct W(crate::W<IVREF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVREF_CTL_SPEC>;
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
impl From<crate::W<IVREF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVREF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSDT` reader - Current step data"]
pub type CSDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSDT` writer - Current step data"]
pub type CSDT_W<'a> = crate::FieldWriterSafe<'a, u32, IVREF_CTL_SPEC, u8, u8, 6, 0>;
#[doc = "Sink current mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCMOD_A {
    #[doc = "0: Source current"]
    SOURCE = 0,
    #[doc = "1: Sink current"]
    SINK = 1,
}
impl From<SCMOD_A> for bool {
    #[inline(always)]
    fn from(variant: SCMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMOD` reader - Sink current mode"]
pub type SCMOD_R = crate::BitReader<SCMOD_A>;
impl SCMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCMOD_A {
        match self.bits {
            false => SCMOD_A::SOURCE,
            true => SCMOD_A::SINK,
        }
    }
    #[doc = "Checks if the value of the field is `SOURCE`"]
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == SCMOD_A::SOURCE
    }
    #[doc = "Checks if the value of the field is `SINK`"]
    #[inline(always)]
    pub fn is_sink(&self) -> bool {
        *self == SCMOD_A::SINK
    }
}
#[doc = "Field `SCMOD` writer - Sink current mode"]
pub type SCMOD_W<'a> = crate::BitWriter<'a, u32, IVREF_CTL_SPEC, SCMOD_A, 7>;
impl<'a> SCMOD_W<'a> {
    #[doc = "Source current"]
    #[inline(always)]
    pub fn source(self) -> &'a mut W {
        self.variant(SCMOD_A::SOURCE)
    }
    #[doc = "Sink current"]
    #[inline(always)]
    pub fn sink(self) -> &'a mut W {
        self.variant(SCMOD_A::SINK)
    }
}
#[doc = "Current precision trim\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPT_A {
    #[doc = "0: Trim -15%"]
    MINUS15 = 0,
    #[doc = "1: Trim -14%"]
    MINUS14 = 1,
    #[doc = "2: Trim -13%"]
    MINUS13 = 2,
    #[doc = "3: Trim -12%"]
    MINUS12 = 3,
    #[doc = "4: Trim -11%"]
    MINUS11 = 4,
    #[doc = "5: Trim -10%"]
    MINUS10 = 5,
    #[doc = "6: Trim -9%"]
    MINUS9 = 6,
    #[doc = "7: Trim -8%"]
    MINUS8 = 7,
    #[doc = "8: Trim -7%"]
    MINUS7 = 8,
    #[doc = "9: Trim -6%"]
    MINUS6 = 9,
    #[doc = "10: Trim -5%"]
    MINUS5 = 10,
    #[doc = "11: Trim -4%"]
    MINUS4 = 11,
    #[doc = "12: Trim -3%"]
    MINUS3 = 12,
    #[doc = "13: Trim -2%"]
    MINUS2 = 13,
    #[doc = "14: Trim -1%"]
    MINUS1 = 14,
    #[doc = "15: Trim 0%"]
    ZERO = 15,
    #[doc = "16: Trim +1%"]
    PLUS1 = 16,
    #[doc = "17: Trim +2%"]
    PLUS2 = 17,
    #[doc = "18: Trim +3%"]
    PLUS3 = 18,
    #[doc = "19: Trim +4%"]
    PLUS4 = 19,
    #[doc = "20: Trim +5%"]
    PLUS5 = 20,
    #[doc = "21: Trim +6%"]
    PLUS6 = 21,
    #[doc = "22: Trim +7%"]
    PLUS7 = 22,
    #[doc = "23: Trim +8%"]
    PLUS8 = 23,
    #[doc = "24: Trim +9%"]
    PLUS9 = 24,
    #[doc = "25: Trim +10%"]
    PLUS10 = 25,
    #[doc = "26: Trim +11%"]
    PLUS11 = 26,
    #[doc = "27: Trim +12%"]
    PLUS12 = 27,
    #[doc = "28: Trim +13%"]
    PLUS13 = 28,
    #[doc = "29: Trim +14%"]
    PLUS14 = 29,
    #[doc = "30: Trim +15%"]
    PLUS15 = 30,
    #[doc = "31: Trim +16%"]
    PLUS16 = 31,
}
impl From<CPT_A> for u8 {
    #[inline(always)]
    fn from(variant: CPT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPT` reader - Current precision trim"]
pub type CPT_R = crate::FieldReader<u8, CPT_A>;
impl CPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPT_A {
        match self.bits {
            0 => CPT_A::MINUS15,
            1 => CPT_A::MINUS14,
            2 => CPT_A::MINUS13,
            3 => CPT_A::MINUS12,
            4 => CPT_A::MINUS11,
            5 => CPT_A::MINUS10,
            6 => CPT_A::MINUS9,
            7 => CPT_A::MINUS8,
            8 => CPT_A::MINUS7,
            9 => CPT_A::MINUS6,
            10 => CPT_A::MINUS5,
            11 => CPT_A::MINUS4,
            12 => CPT_A::MINUS3,
            13 => CPT_A::MINUS2,
            14 => CPT_A::MINUS1,
            15 => CPT_A::ZERO,
            16 => CPT_A::PLUS1,
            17 => CPT_A::PLUS2,
            18 => CPT_A::PLUS3,
            19 => CPT_A::PLUS4,
            20 => CPT_A::PLUS5,
            21 => CPT_A::PLUS6,
            22 => CPT_A::PLUS7,
            23 => CPT_A::PLUS8,
            24 => CPT_A::PLUS9,
            25 => CPT_A::PLUS10,
            26 => CPT_A::PLUS11,
            27 => CPT_A::PLUS12,
            28 => CPT_A::PLUS13,
            29 => CPT_A::PLUS14,
            30 => CPT_A::PLUS15,
            31 => CPT_A::PLUS16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MINUS15`"]
    #[inline(always)]
    pub fn is_minus15(&self) -> bool {
        *self == CPT_A::MINUS15
    }
    #[doc = "Checks if the value of the field is `MINUS14`"]
    #[inline(always)]
    pub fn is_minus14(&self) -> bool {
        *self == CPT_A::MINUS14
    }
    #[doc = "Checks if the value of the field is `MINUS13`"]
    #[inline(always)]
    pub fn is_minus13(&self) -> bool {
        *self == CPT_A::MINUS13
    }
    #[doc = "Checks if the value of the field is `MINUS12`"]
    #[inline(always)]
    pub fn is_minus12(&self) -> bool {
        *self == CPT_A::MINUS12
    }
    #[doc = "Checks if the value of the field is `MINUS11`"]
    #[inline(always)]
    pub fn is_minus11(&self) -> bool {
        *self == CPT_A::MINUS11
    }
    #[doc = "Checks if the value of the field is `MINUS10`"]
    #[inline(always)]
    pub fn is_minus10(&self) -> bool {
        *self == CPT_A::MINUS10
    }
    #[doc = "Checks if the value of the field is `MINUS9`"]
    #[inline(always)]
    pub fn is_minus9(&self) -> bool {
        *self == CPT_A::MINUS9
    }
    #[doc = "Checks if the value of the field is `MINUS8`"]
    #[inline(always)]
    pub fn is_minus8(&self) -> bool {
        *self == CPT_A::MINUS8
    }
    #[doc = "Checks if the value of the field is `MINUS7`"]
    #[inline(always)]
    pub fn is_minus7(&self) -> bool {
        *self == CPT_A::MINUS7
    }
    #[doc = "Checks if the value of the field is `MINUS6`"]
    #[inline(always)]
    pub fn is_minus6(&self) -> bool {
        *self == CPT_A::MINUS6
    }
    #[doc = "Checks if the value of the field is `MINUS5`"]
    #[inline(always)]
    pub fn is_minus5(&self) -> bool {
        *self == CPT_A::MINUS5
    }
    #[doc = "Checks if the value of the field is `MINUS4`"]
    #[inline(always)]
    pub fn is_minus4(&self) -> bool {
        *self == CPT_A::MINUS4
    }
    #[doc = "Checks if the value of the field is `MINUS3`"]
    #[inline(always)]
    pub fn is_minus3(&self) -> bool {
        *self == CPT_A::MINUS3
    }
    #[doc = "Checks if the value of the field is `MINUS2`"]
    #[inline(always)]
    pub fn is_minus2(&self) -> bool {
        *self == CPT_A::MINUS2
    }
    #[doc = "Checks if the value of the field is `MINUS1`"]
    #[inline(always)]
    pub fn is_minus1(&self) -> bool {
        *self == CPT_A::MINUS1
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CPT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `PLUS1`"]
    #[inline(always)]
    pub fn is_plus1(&self) -> bool {
        *self == CPT_A::PLUS1
    }
    #[doc = "Checks if the value of the field is `PLUS2`"]
    #[inline(always)]
    pub fn is_plus2(&self) -> bool {
        *self == CPT_A::PLUS2
    }
    #[doc = "Checks if the value of the field is `PLUS3`"]
    #[inline(always)]
    pub fn is_plus3(&self) -> bool {
        *self == CPT_A::PLUS3
    }
    #[doc = "Checks if the value of the field is `PLUS4`"]
    #[inline(always)]
    pub fn is_plus4(&self) -> bool {
        *self == CPT_A::PLUS4
    }
    #[doc = "Checks if the value of the field is `PLUS5`"]
    #[inline(always)]
    pub fn is_plus5(&self) -> bool {
        *self == CPT_A::PLUS5
    }
    #[doc = "Checks if the value of the field is `PLUS6`"]
    #[inline(always)]
    pub fn is_plus6(&self) -> bool {
        *self == CPT_A::PLUS6
    }
    #[doc = "Checks if the value of the field is `PLUS7`"]
    #[inline(always)]
    pub fn is_plus7(&self) -> bool {
        *self == CPT_A::PLUS7
    }
    #[doc = "Checks if the value of the field is `PLUS8`"]
    #[inline(always)]
    pub fn is_plus8(&self) -> bool {
        *self == CPT_A::PLUS8
    }
    #[doc = "Checks if the value of the field is `PLUS9`"]
    #[inline(always)]
    pub fn is_plus9(&self) -> bool {
        *self == CPT_A::PLUS9
    }
    #[doc = "Checks if the value of the field is `PLUS10`"]
    #[inline(always)]
    pub fn is_plus10(&self) -> bool {
        *self == CPT_A::PLUS10
    }
    #[doc = "Checks if the value of the field is `PLUS11`"]
    #[inline(always)]
    pub fn is_plus11(&self) -> bool {
        *self == CPT_A::PLUS11
    }
    #[doc = "Checks if the value of the field is `PLUS12`"]
    #[inline(always)]
    pub fn is_plus12(&self) -> bool {
        *self == CPT_A::PLUS12
    }
    #[doc = "Checks if the value of the field is `PLUS13`"]
    #[inline(always)]
    pub fn is_plus13(&self) -> bool {
        *self == CPT_A::PLUS13
    }
    #[doc = "Checks if the value of the field is `PLUS14`"]
    #[inline(always)]
    pub fn is_plus14(&self) -> bool {
        *self == CPT_A::PLUS14
    }
    #[doc = "Checks if the value of the field is `PLUS15`"]
    #[inline(always)]
    pub fn is_plus15(&self) -> bool {
        *self == CPT_A::PLUS15
    }
    #[doc = "Checks if the value of the field is `PLUS16`"]
    #[inline(always)]
    pub fn is_plus16(&self) -> bool {
        *self == CPT_A::PLUS16
    }
}
#[doc = "Field `CPT` writer - Current precision trim"]
pub type CPT_W<'a> = crate::FieldWriterSafe<'a, u32, IVREF_CTL_SPEC, u8, CPT_A, 5, 8>;
impl<'a> CPT_W<'a> {
    #[doc = "Trim -15%"]
    #[inline(always)]
    pub fn minus15(self) -> &'a mut W {
        self.variant(CPT_A::MINUS15)
    }
    #[doc = "Trim -14%"]
    #[inline(always)]
    pub fn minus14(self) -> &'a mut W {
        self.variant(CPT_A::MINUS14)
    }
    #[doc = "Trim -13%"]
    #[inline(always)]
    pub fn minus13(self) -> &'a mut W {
        self.variant(CPT_A::MINUS13)
    }
    #[doc = "Trim -12%"]
    #[inline(always)]
    pub fn minus12(self) -> &'a mut W {
        self.variant(CPT_A::MINUS12)
    }
    #[doc = "Trim -11%"]
    #[inline(always)]
    pub fn minus11(self) -> &'a mut W {
        self.variant(CPT_A::MINUS11)
    }
    #[doc = "Trim -10%"]
    #[inline(always)]
    pub fn minus10(self) -> &'a mut W {
        self.variant(CPT_A::MINUS10)
    }
    #[doc = "Trim -9%"]
    #[inline(always)]
    pub fn minus9(self) -> &'a mut W {
        self.variant(CPT_A::MINUS9)
    }
    #[doc = "Trim -8%"]
    #[inline(always)]
    pub fn minus8(self) -> &'a mut W {
        self.variant(CPT_A::MINUS8)
    }
    #[doc = "Trim -7%"]
    #[inline(always)]
    pub fn minus7(self) -> &'a mut W {
        self.variant(CPT_A::MINUS7)
    }
    #[doc = "Trim -6%"]
    #[inline(always)]
    pub fn minus6(self) -> &'a mut W {
        self.variant(CPT_A::MINUS6)
    }
    #[doc = "Trim -5%"]
    #[inline(always)]
    pub fn minus5(self) -> &'a mut W {
        self.variant(CPT_A::MINUS5)
    }
    #[doc = "Trim -4%"]
    #[inline(always)]
    pub fn minus4(self) -> &'a mut W {
        self.variant(CPT_A::MINUS4)
    }
    #[doc = "Trim -3%"]
    #[inline(always)]
    pub fn minus3(self) -> &'a mut W {
        self.variant(CPT_A::MINUS3)
    }
    #[doc = "Trim -2%"]
    #[inline(always)]
    pub fn minus2(self) -> &'a mut W {
        self.variant(CPT_A::MINUS2)
    }
    #[doc = "Trim -1%"]
    #[inline(always)]
    pub fn minus1(self) -> &'a mut W {
        self.variant(CPT_A::MINUS1)
    }
    #[doc = "Trim 0%"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CPT_A::ZERO)
    }
    #[doc = "Trim +1%"]
    #[inline(always)]
    pub fn plus1(self) -> &'a mut W {
        self.variant(CPT_A::PLUS1)
    }
    #[doc = "Trim +2%"]
    #[inline(always)]
    pub fn plus2(self) -> &'a mut W {
        self.variant(CPT_A::PLUS2)
    }
    #[doc = "Trim +3%"]
    #[inline(always)]
    pub fn plus3(self) -> &'a mut W {
        self.variant(CPT_A::PLUS3)
    }
    #[doc = "Trim +4%"]
    #[inline(always)]
    pub fn plus4(self) -> &'a mut W {
        self.variant(CPT_A::PLUS4)
    }
    #[doc = "Trim +5%"]
    #[inline(always)]
    pub fn plus5(self) -> &'a mut W {
        self.variant(CPT_A::PLUS5)
    }
    #[doc = "Trim +6%"]
    #[inline(always)]
    pub fn plus6(self) -> &'a mut W {
        self.variant(CPT_A::PLUS6)
    }
    #[doc = "Trim +7%"]
    #[inline(always)]
    pub fn plus7(self) -> &'a mut W {
        self.variant(CPT_A::PLUS7)
    }
    #[doc = "Trim +8%"]
    #[inline(always)]
    pub fn plus8(self) -> &'a mut W {
        self.variant(CPT_A::PLUS8)
    }
    #[doc = "Trim +9%"]
    #[inline(always)]
    pub fn plus9(self) -> &'a mut W {
        self.variant(CPT_A::PLUS9)
    }
    #[doc = "Trim +10%"]
    #[inline(always)]
    pub fn plus10(self) -> &'a mut W {
        self.variant(CPT_A::PLUS10)
    }
    #[doc = "Trim +11%"]
    #[inline(always)]
    pub fn plus11(self) -> &'a mut W {
        self.variant(CPT_A::PLUS11)
    }
    #[doc = "Trim +12%"]
    #[inline(always)]
    pub fn plus12(self) -> &'a mut W {
        self.variant(CPT_A::PLUS12)
    }
    #[doc = "Trim +13%"]
    #[inline(always)]
    pub fn plus13(self) -> &'a mut W {
        self.variant(CPT_A::PLUS13)
    }
    #[doc = "Trim +14%"]
    #[inline(always)]
    pub fn plus14(self) -> &'a mut W {
        self.variant(CPT_A::PLUS14)
    }
    #[doc = "Trim +15%"]
    #[inline(always)]
    pub fn plus15(self) -> &'a mut W {
        self.variant(CPT_A::PLUS15)
    }
    #[doc = "Trim +16%"]
    #[inline(always)]
    pub fn plus16(self) -> &'a mut W {
        self.variant(CPT_A::PLUS16)
    }
}
#[doc = "Step selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEL_A {
    #[doc = "0: Low power, 1uA step"]
    LOWPOWER = 0,
    #[doc = "1: Low power, 8uA step"]
    HIGHPOWER = 1,
}
impl From<SSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEL` reader - Step selection"]
pub type SSEL_R = crate::BitReader<SSEL_A>;
impl SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSEL_A {
        match self.bits {
            false => SSEL_A::LOWPOWER,
            true => SSEL_A::HIGHPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == SSEL_A::LOWPOWER
    }
    #[doc = "Checks if the value of the field is `HIGHPOWER`"]
    #[inline(always)]
    pub fn is_high_power(&self) -> bool {
        *self == SSEL_A::HIGHPOWER
    }
}
#[doc = "Field `SSEL` writer - Step selection"]
pub type SSEL_W<'a> = crate::BitWriter<'a, u32, IVREF_CTL_SPEC, SSEL_A, 14>;
impl<'a> SSEL_W<'a> {
    #[doc = "Low power, 1uA step"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(SSEL_A::LOWPOWER)
    }
    #[doc = "Low power, 8uA step"]
    #[inline(always)]
    pub fn high_power(self) -> &'a mut W {
        self.variant(SSEL_A::HIGHPOWER)
    }
}
#[doc = "Current reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREN_A {
    #[doc = "0: Disable current reference"]
    DISABLED = 0,
    #[doc = "1: Enable current reference"]
    ENABLED = 1,
}
impl From<CREN_A> for bool {
    #[inline(always)]
    fn from(variant: CREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREN` reader - Current reference enable"]
pub type CREN_R = crate::BitReader<CREN_A>;
impl CREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CREN_A {
        match self.bits {
            false => CREN_A::DISABLED,
            true => CREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CREN_A::ENABLED
    }
}
#[doc = "Field `CREN` writer - Current reference enable"]
pub type CREN_W<'a> = crate::BitWriter<'a, u32, IVREF_CTL_SPEC, CREN_A, 15>;
impl<'a> CREN_W<'a> {
    #[doc = "Disable current reference"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CREN_A::DISABLED)
    }
    #[doc = "Enable current reference"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CREN_A::ENABLED)
    }
}
#[doc = "Voltage precision tirm\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VPT_A {
    #[doc = "0: Trim -6.4%"]
    MINUS6_4 = 0,
    #[doc = "1: Trim -6.0%"]
    MINUS6_0 = 1,
    #[doc = "2: Trim -5.6%"]
    MINUS5_6 = 2,
    #[doc = "3: Trim -5.2%"]
    MINUS5_2 = 3,
    #[doc = "4: Trim -4.8%"]
    MINUS4_8 = 4,
    #[doc = "5: Trim -4.4%"]
    MINUS4_4 = 5,
    #[doc = "6: Trim -4.0%"]
    MINUS4_0 = 6,
    #[doc = "7: Trim -3.6%"]
    MINUS3_6 = 7,
    #[doc = "8: Trim -3.2%"]
    MINUS3_2 = 8,
    #[doc = "9: Trim -2.8%"]
    MINUS2_8 = 9,
    #[doc = "10: Trim -2.4%"]
    MINUS2_4 = 10,
    #[doc = "11: Trim -2.0%"]
    MINUS2_0 = 11,
    #[doc = "12: Trim -1.6%"]
    MINUS1_6 = 12,
    #[doc = "13: Trim -1.2%"]
    MINUS1_2 = 13,
    #[doc = "14: Trim -0.8%"]
    MINUS0_8 = 14,
    #[doc = "15: Trim -0.4%"]
    MINUS0_4 = 15,
    #[doc = "16: Trim 0%"]
    ZERO = 16,
    #[doc = "17: Trim +0.4%"]
    PLUS0_4 = 17,
    #[doc = "18: Trim +0.8%"]
    PLUS0_8 = 18,
    #[doc = "19: Trim +1.2%"]
    PLUS1_2 = 19,
    #[doc = "20: Trim +1.6%"]
    PLUS1_6 = 20,
    #[doc = "22: Trim +2.4%"]
    PLUS2_4 = 22,
    #[doc = "21: Trim +2.0%"]
    PLUS2_0 = 21,
    #[doc = "23: Trim +2.8%"]
    PLUS2_8 = 23,
    #[doc = "24: Trim +3.2%"]
    PLUS3_2 = 24,
    #[doc = "25: Trim +3.6%"]
    PLUS3_6 = 25,
    #[doc = "26: Trim +4.0%"]
    PLUS4_0 = 26,
    #[doc = "27: Trim +4.4%"]
    PLUS4_4 = 27,
    #[doc = "28: Trim +4.8%"]
    PLUS4_8 = 28,
    #[doc = "29: Trim +5.2%"]
    PLUS5_2 = 29,
    #[doc = "30: Trim +5.6%"]
    PLUS5_6 = 30,
    #[doc = "31: Trim +6.0%"]
    PLUS6_0 = 31,
}
impl From<VPT_A> for u8 {
    #[inline(always)]
    fn from(variant: VPT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VPT` reader - Voltage precision tirm"]
pub type VPT_R = crate::FieldReader<u8, VPT_A>;
impl VPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPT_A {
        match self.bits {
            0 => VPT_A::MINUS6_4,
            1 => VPT_A::MINUS6_0,
            2 => VPT_A::MINUS5_6,
            3 => VPT_A::MINUS5_2,
            4 => VPT_A::MINUS4_8,
            5 => VPT_A::MINUS4_4,
            6 => VPT_A::MINUS4_0,
            7 => VPT_A::MINUS3_6,
            8 => VPT_A::MINUS3_2,
            9 => VPT_A::MINUS2_8,
            10 => VPT_A::MINUS2_4,
            11 => VPT_A::MINUS2_0,
            12 => VPT_A::MINUS1_6,
            13 => VPT_A::MINUS1_2,
            14 => VPT_A::MINUS0_8,
            15 => VPT_A::MINUS0_4,
            16 => VPT_A::ZERO,
            17 => VPT_A::PLUS0_4,
            18 => VPT_A::PLUS0_8,
            19 => VPT_A::PLUS1_2,
            20 => VPT_A::PLUS1_6,
            22 => VPT_A::PLUS2_4,
            21 => VPT_A::PLUS2_0,
            23 => VPT_A::PLUS2_8,
            24 => VPT_A::PLUS3_2,
            25 => VPT_A::PLUS3_6,
            26 => VPT_A::PLUS4_0,
            27 => VPT_A::PLUS4_4,
            28 => VPT_A::PLUS4_8,
            29 => VPT_A::PLUS5_2,
            30 => VPT_A::PLUS5_6,
            31 => VPT_A::PLUS6_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MINUS6_4`"]
    #[inline(always)]
    pub fn is_minus6_4(&self) -> bool {
        *self == VPT_A::MINUS6_4
    }
    #[doc = "Checks if the value of the field is `MINUS6_0`"]
    #[inline(always)]
    pub fn is_minus6_0(&self) -> bool {
        *self == VPT_A::MINUS6_0
    }
    #[doc = "Checks if the value of the field is `MINUS5_6`"]
    #[inline(always)]
    pub fn is_minus5_6(&self) -> bool {
        *self == VPT_A::MINUS5_6
    }
    #[doc = "Checks if the value of the field is `MINUS5_2`"]
    #[inline(always)]
    pub fn is_minus5_2(&self) -> bool {
        *self == VPT_A::MINUS5_2
    }
    #[doc = "Checks if the value of the field is `MINUS4_8`"]
    #[inline(always)]
    pub fn is_minus4_8(&self) -> bool {
        *self == VPT_A::MINUS4_8
    }
    #[doc = "Checks if the value of the field is `MINUS4_4`"]
    #[inline(always)]
    pub fn is_minus4_4(&self) -> bool {
        *self == VPT_A::MINUS4_4
    }
    #[doc = "Checks if the value of the field is `MINUS4_0`"]
    #[inline(always)]
    pub fn is_minus4_0(&self) -> bool {
        *self == VPT_A::MINUS4_0
    }
    #[doc = "Checks if the value of the field is `MINUS3_6`"]
    #[inline(always)]
    pub fn is_minus3_6(&self) -> bool {
        *self == VPT_A::MINUS3_6
    }
    #[doc = "Checks if the value of the field is `MINUS3_2`"]
    #[inline(always)]
    pub fn is_minus3_2(&self) -> bool {
        *self == VPT_A::MINUS3_2
    }
    #[doc = "Checks if the value of the field is `MINUS2_8`"]
    #[inline(always)]
    pub fn is_minus2_8(&self) -> bool {
        *self == VPT_A::MINUS2_8
    }
    #[doc = "Checks if the value of the field is `MINUS2_4`"]
    #[inline(always)]
    pub fn is_minus2_4(&self) -> bool {
        *self == VPT_A::MINUS2_4
    }
    #[doc = "Checks if the value of the field is `MINUS2_0`"]
    #[inline(always)]
    pub fn is_minus2_0(&self) -> bool {
        *self == VPT_A::MINUS2_0
    }
    #[doc = "Checks if the value of the field is `MINUS1_6`"]
    #[inline(always)]
    pub fn is_minus1_6(&self) -> bool {
        *self == VPT_A::MINUS1_6
    }
    #[doc = "Checks if the value of the field is `MINUS1_2`"]
    #[inline(always)]
    pub fn is_minus1_2(&self) -> bool {
        *self == VPT_A::MINUS1_2
    }
    #[doc = "Checks if the value of the field is `MINUS0_8`"]
    #[inline(always)]
    pub fn is_minus0_8(&self) -> bool {
        *self == VPT_A::MINUS0_8
    }
    #[doc = "Checks if the value of the field is `MINUS0_4`"]
    #[inline(always)]
    pub fn is_minus0_4(&self) -> bool {
        *self == VPT_A::MINUS0_4
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == VPT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `PLUS0_4`"]
    #[inline(always)]
    pub fn is_plus0_4(&self) -> bool {
        *self == VPT_A::PLUS0_4
    }
    #[doc = "Checks if the value of the field is `PLUS0_8`"]
    #[inline(always)]
    pub fn is_plus0_8(&self) -> bool {
        *self == VPT_A::PLUS0_8
    }
    #[doc = "Checks if the value of the field is `PLUS1_2`"]
    #[inline(always)]
    pub fn is_plus1_2(&self) -> bool {
        *self == VPT_A::PLUS1_2
    }
    #[doc = "Checks if the value of the field is `PLUS1_6`"]
    #[inline(always)]
    pub fn is_plus1_6(&self) -> bool {
        *self == VPT_A::PLUS1_6
    }
    #[doc = "Checks if the value of the field is `PLUS2_4`"]
    #[inline(always)]
    pub fn is_plus2_4(&self) -> bool {
        *self == VPT_A::PLUS2_4
    }
    #[doc = "Checks if the value of the field is `PLUS2_0`"]
    #[inline(always)]
    pub fn is_plus2_0(&self) -> bool {
        *self == VPT_A::PLUS2_0
    }
    #[doc = "Checks if the value of the field is `PLUS2_8`"]
    #[inline(always)]
    pub fn is_plus2_8(&self) -> bool {
        *self == VPT_A::PLUS2_8
    }
    #[doc = "Checks if the value of the field is `PLUS3_2`"]
    #[inline(always)]
    pub fn is_plus3_2(&self) -> bool {
        *self == VPT_A::PLUS3_2
    }
    #[doc = "Checks if the value of the field is `PLUS3_6`"]
    #[inline(always)]
    pub fn is_plus3_6(&self) -> bool {
        *self == VPT_A::PLUS3_6
    }
    #[doc = "Checks if the value of the field is `PLUS4_0`"]
    #[inline(always)]
    pub fn is_plus4_0(&self) -> bool {
        *self == VPT_A::PLUS4_0
    }
    #[doc = "Checks if the value of the field is `PLUS4_4`"]
    #[inline(always)]
    pub fn is_plus4_4(&self) -> bool {
        *self == VPT_A::PLUS4_4
    }
    #[doc = "Checks if the value of the field is `PLUS4_8`"]
    #[inline(always)]
    pub fn is_plus4_8(&self) -> bool {
        *self == VPT_A::PLUS4_8
    }
    #[doc = "Checks if the value of the field is `PLUS5_2`"]
    #[inline(always)]
    pub fn is_plus5_2(&self) -> bool {
        *self == VPT_A::PLUS5_2
    }
    #[doc = "Checks if the value of the field is `PLUS5_6`"]
    #[inline(always)]
    pub fn is_plus5_6(&self) -> bool {
        *self == VPT_A::PLUS5_6
    }
    #[doc = "Checks if the value of the field is `PLUS6_0`"]
    #[inline(always)]
    pub fn is_plus6_0(&self) -> bool {
        *self == VPT_A::PLUS6_0
    }
}
#[doc = "Field `VPT` writer - Voltage precision tirm"]
pub type VPT_W<'a> = crate::FieldWriterSafe<'a, u32, IVREF_CTL_SPEC, u8, VPT_A, 5, 24>;
impl<'a> VPT_W<'a> {
    #[doc = "Trim -6.4%"]
    #[inline(always)]
    pub fn minus6_4(self) -> &'a mut W {
        self.variant(VPT_A::MINUS6_4)
    }
    #[doc = "Trim -6.0%"]
    #[inline(always)]
    pub fn minus6_0(self) -> &'a mut W {
        self.variant(VPT_A::MINUS6_0)
    }
    #[doc = "Trim -5.6%"]
    #[inline(always)]
    pub fn minus5_6(self) -> &'a mut W {
        self.variant(VPT_A::MINUS5_6)
    }
    #[doc = "Trim -5.2%"]
    #[inline(always)]
    pub fn minus5_2(self) -> &'a mut W {
        self.variant(VPT_A::MINUS5_2)
    }
    #[doc = "Trim -4.8%"]
    #[inline(always)]
    pub fn minus4_8(self) -> &'a mut W {
        self.variant(VPT_A::MINUS4_8)
    }
    #[doc = "Trim -4.4%"]
    #[inline(always)]
    pub fn minus4_4(self) -> &'a mut W {
        self.variant(VPT_A::MINUS4_4)
    }
    #[doc = "Trim -4.0%"]
    #[inline(always)]
    pub fn minus4_0(self) -> &'a mut W {
        self.variant(VPT_A::MINUS4_0)
    }
    #[doc = "Trim -3.6%"]
    #[inline(always)]
    pub fn minus3_6(self) -> &'a mut W {
        self.variant(VPT_A::MINUS3_6)
    }
    #[doc = "Trim -3.2%"]
    #[inline(always)]
    pub fn minus3_2(self) -> &'a mut W {
        self.variant(VPT_A::MINUS3_2)
    }
    #[doc = "Trim -2.8%"]
    #[inline(always)]
    pub fn minus2_8(self) -> &'a mut W {
        self.variant(VPT_A::MINUS2_8)
    }
    #[doc = "Trim -2.4%"]
    #[inline(always)]
    pub fn minus2_4(self) -> &'a mut W {
        self.variant(VPT_A::MINUS2_4)
    }
    #[doc = "Trim -2.0%"]
    #[inline(always)]
    pub fn minus2_0(self) -> &'a mut W {
        self.variant(VPT_A::MINUS2_0)
    }
    #[doc = "Trim -1.6%"]
    #[inline(always)]
    pub fn minus1_6(self) -> &'a mut W {
        self.variant(VPT_A::MINUS1_6)
    }
    #[doc = "Trim -1.2%"]
    #[inline(always)]
    pub fn minus1_2(self) -> &'a mut W {
        self.variant(VPT_A::MINUS1_2)
    }
    #[doc = "Trim -0.8%"]
    #[inline(always)]
    pub fn minus0_8(self) -> &'a mut W {
        self.variant(VPT_A::MINUS0_8)
    }
    #[doc = "Trim -0.4%"]
    #[inline(always)]
    pub fn minus0_4(self) -> &'a mut W {
        self.variant(VPT_A::MINUS0_4)
    }
    #[doc = "Trim 0%"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(VPT_A::ZERO)
    }
    #[doc = "Trim +0.4%"]
    #[inline(always)]
    pub fn plus0_4(self) -> &'a mut W {
        self.variant(VPT_A::PLUS0_4)
    }
    #[doc = "Trim +0.8%"]
    #[inline(always)]
    pub fn plus0_8(self) -> &'a mut W {
        self.variant(VPT_A::PLUS0_8)
    }
    #[doc = "Trim +1.2%"]
    #[inline(always)]
    pub fn plus1_2(self) -> &'a mut W {
        self.variant(VPT_A::PLUS1_2)
    }
    #[doc = "Trim +1.6%"]
    #[inline(always)]
    pub fn plus1_6(self) -> &'a mut W {
        self.variant(VPT_A::PLUS1_6)
    }
    #[doc = "Trim +2.4%"]
    #[inline(always)]
    pub fn plus2_4(self) -> &'a mut W {
        self.variant(VPT_A::PLUS2_4)
    }
    #[doc = "Trim +2.0%"]
    #[inline(always)]
    pub fn plus2_0(self) -> &'a mut W {
        self.variant(VPT_A::PLUS2_0)
    }
    #[doc = "Trim +2.8%"]
    #[inline(always)]
    pub fn plus2_8(self) -> &'a mut W {
        self.variant(VPT_A::PLUS2_8)
    }
    #[doc = "Trim +3.2%"]
    #[inline(always)]
    pub fn plus3_2(self) -> &'a mut W {
        self.variant(VPT_A::PLUS3_2)
    }
    #[doc = "Trim +3.6%"]
    #[inline(always)]
    pub fn plus3_6(self) -> &'a mut W {
        self.variant(VPT_A::PLUS3_6)
    }
    #[doc = "Trim +4.0%"]
    #[inline(always)]
    pub fn plus4_0(self) -> &'a mut W {
        self.variant(VPT_A::PLUS4_0)
    }
    #[doc = "Trim +4.4%"]
    #[inline(always)]
    pub fn plus4_4(self) -> &'a mut W {
        self.variant(VPT_A::PLUS4_4)
    }
    #[doc = "Trim +4.8%"]
    #[inline(always)]
    pub fn plus4_8(self) -> &'a mut W {
        self.variant(VPT_A::PLUS4_8)
    }
    #[doc = "Trim +5.2%"]
    #[inline(always)]
    pub fn plus5_2(self) -> &'a mut W {
        self.variant(VPT_A::PLUS5_2)
    }
    #[doc = "Trim +5.6%"]
    #[inline(always)]
    pub fn plus5_6(self) -> &'a mut W {
        self.variant(VPT_A::PLUS5_6)
    }
    #[doc = "Trim +6.0%"]
    #[inline(always)]
    pub fn plus6_0(self) -> &'a mut W {
        self.variant(VPT_A::PLUS6_0)
    }
}
#[doc = "Disconnect external capacitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP_A {
    #[doc = "0: External capacitor connected"]
    CONNECTED = 0,
    #[doc = "1: External capacitor disonnected"]
    DISCONNECTED = 1,
}
impl From<DECAP_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECAP` reader - Disconnect external capacitor"]
pub type DECAP_R = crate::BitReader<DECAP_A>;
impl DECAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP_A {
        match self.bits {
            false => DECAP_A::CONNECTED,
            true => DECAP_A::DISCONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == DECAP_A::CONNECTED
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == DECAP_A::DISCONNECTED
    }
}
#[doc = "Field `DECAP` writer - Disconnect external capacitor"]
pub type DECAP_W<'a> = crate::BitWriter<'a, u32, IVREF_CTL_SPEC, DECAP_A, 30>;
impl<'a> DECAP_W<'a> {
    #[doc = "External capacitor connected"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(DECAP_A::CONNECTED)
    }
    #[doc = "External capacitor disonnected"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(DECAP_A::DISCONNECTED)
    }
}
#[doc = "Voltage reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREN_A {
    #[doc = "0: Disable voltage reference"]
    DISABLED = 0,
    #[doc = "1: Enable voltage reference"]
    ENABLED = 1,
}
impl From<VREN_A> for bool {
    #[inline(always)]
    fn from(variant: VREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREN` reader - Voltage reference enable"]
pub type VREN_R = crate::BitReader<VREN_A>;
impl VREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREN_A {
        match self.bits {
            false => VREN_A::DISABLED,
            true => VREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREN_A::ENABLED
    }
}
#[doc = "Field `VREN` writer - Voltage reference enable"]
pub type VREN_W<'a> = crate::BitWriter<'a, u32, IVREF_CTL_SPEC, VREN_A, 31>;
impl<'a> VREN_W<'a> {
    #[doc = "Disable voltage reference"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREN_A::DISABLED)
    }
    #[doc = "Enable voltage reference"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:5 - Current step data"]
    #[inline(always)]
    pub fn csdt(&self) -> CSDT_R {
        CSDT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Sink current mode"]
    #[inline(always)]
    pub fn scmod(&self) -> SCMOD_R {
        SCMOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Current precision trim"]
    #[inline(always)]
    pub fn cpt(&self) -> CPT_R {
        CPT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Step selection"]
    #[inline(always)]
    pub fn ssel(&self) -> SSEL_R {
        SSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Current reference enable"]
    #[inline(always)]
    pub fn cren(&self) -> CREN_R {
        CREN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Voltage precision tirm"]
    #[inline(always)]
    pub fn vpt(&self) -> VPT_R {
        VPT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Disconnect external capacitor"]
    #[inline(always)]
    pub fn decap(&self) -> DECAP_R {
        DECAP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Voltage reference enable"]
    #[inline(always)]
    pub fn vren(&self) -> VREN_R {
        VREN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Current step data"]
    #[inline(always)]
    pub fn csdt(&mut self) -> CSDT_W {
        CSDT_W::new(self)
    }
    #[doc = "Bit 7 - Sink current mode"]
    #[inline(always)]
    pub fn scmod(&mut self) -> SCMOD_W {
        SCMOD_W::new(self)
    }
    #[doc = "Bits 8:12 - Current precision trim"]
    #[inline(always)]
    pub fn cpt(&mut self) -> CPT_W {
        CPT_W::new(self)
    }
    #[doc = "Bit 14 - Step selection"]
    #[inline(always)]
    pub fn ssel(&mut self) -> SSEL_W {
        SSEL_W::new(self)
    }
    #[doc = "Bit 15 - Current reference enable"]
    #[inline(always)]
    pub fn cren(&mut self) -> CREN_W {
        CREN_W::new(self)
    }
    #[doc = "Bits 24:28 - Voltage precision tirm"]
    #[inline(always)]
    pub fn vpt(&mut self) -> VPT_W {
        VPT_W::new(self)
    }
    #[doc = "Bit 30 - Disconnect external capacitor"]
    #[inline(always)]
    pub fn decap(&mut self) -> DECAP_W {
        DECAP_W::new(self)
    }
    #[doc = "Bit 31 - Voltage reference enable"]
    #[inline(always)]
    pub fn vren(&mut self) -> VREN_W {
        VREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IVREF control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivref_ctl](index.html) module"]
pub struct IVREF_CTL_SPEC;
impl crate::RegisterSpec for IVREF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivref_ctl::R](R) reader structure"]
impl crate::Readable for IVREF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ivref_ctl::W](W) writer structure"]
impl crate::Writable for IVREF_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVREF_CTL to value 0x1000_0f00"]
impl crate::Resettable for IVREF_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0f00
    }
}
