#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Address of the USART terminal"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - Address of the USART terminal"]
pub type ADDR_W<'a> = crate::FieldWriterSafe<'a, u32, CTL1_SPEC, u8, u8, 8, 24>;
#[doc = "Receiver timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTEN_A {
    #[doc = "0: Receiver timeout feature disabled"]
    DISABLED = 0,
    #[doc = "1: Receiver timeout feature enabled"]
    ENABLED = 1,
}
impl From<RTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTEN` reader - Receiver timeout enable"]
pub type RTEN_R = crate::BitReader<RTEN_A>;
impl RTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTEN_A {
        match self.bits {
            false => RTEN_A::DISABLED,
            true => RTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTEN_A::ENABLED
    }
}
#[doc = "Field `RTEN` writer - Receiver timeout enable"]
pub type RTEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, RTEN_A, 23>;
impl<'a> RTEN_W<'a> {
    #[doc = "Receiver timeout feature disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTEN_A::DISABLED)
    }
    #[doc = "Receiver timeout feature enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTEN_A::ENABLED)
    }
}
#[doc = "Auto baud rate mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ABDM_A {
    #[doc = "0: Measurement of the start bit is used to detect the baud rate"]
    START = 0,
    #[doc = "1: Falling edge to falling edge measurement"]
    EDGE = 1,
}
impl From<ABDM_A> for u8 {
    #[inline(always)]
    fn from(variant: ABDM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ABDM` reader - Auto baud rate mode"]
pub type ABDM_R = crate::FieldReader<u8, ABDM_A>;
impl ABDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ABDM_A> {
        match self.bits {
            0 => Some(ABDM_A::START),
            1 => Some(ABDM_A::EDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ABDM_A::START
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ABDM_A::EDGE
    }
}
#[doc = "Field `ABDM` writer - Auto baud rate mode"]
pub type ABDM_W<'a> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, ABDM_A, 2, 21>;
impl<'a> ABDM_W<'a> {
    #[doc = "Measurement of the start bit is used to detect the baud rate"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ABDM_A::START)
    }
    #[doc = "Falling edge to falling edge measurement"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ABDM_A::EDGE)
    }
}
#[doc = "Auto baud rate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABDEN_A {
    #[doc = "0: Auto baud rate detection is disabled"]
    DISABLED = 0,
    #[doc = "1: Auto baud rate detection is enabled"]
    ENABLED = 1,
}
impl From<ABDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABDEN` reader - Auto baud rate enable"]
pub type ABDEN_R = crate::BitReader<ABDEN_A>;
impl ABDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABDEN_A {
        match self.bits {
            false => ABDEN_A::DISABLED,
            true => ABDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABDEN_A::ENABLED
    }
}
#[doc = "Field `ABDEN` writer - Auto baud rate enable"]
pub type ABDEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ABDEN_A, 20>;
impl<'a> ABDEN_W<'a> {
    #[doc = "Auto baud rate detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABDEN_A::DISABLED)
    }
    #[doc = "Auto baud rate detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABDEN_A::ENABLED)
    }
}
#[doc = "Most significant bit first\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBF_A {
    #[doc = "0: Data is transmitted/received with data bit 0 first, following the start bit"]
    LSB = 0,
    #[doc = "1: Data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    MSB = 1,
}
impl From<MSBF_A> for bool {
    #[inline(always)]
    fn from(variant: MSBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBF` reader - Most significant bit first"]
pub type MSBF_R = crate::BitReader<MSBF_A>;
impl MSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBF_A {
        match self.bits {
            false => MSBF_A::LSB,
            true => MSBF_A::MSB,
        }
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBF_A::LSB
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBF_A::MSB
    }
}
#[doc = "Field `MSBF` writer - Most significant bit first"]
pub type MSBF_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, MSBF_A, 19>;
impl<'a> MSBF_W<'a> {
    #[doc = "Data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MSBF_A::LSB)
    }
    #[doc = "Data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(MSBF_A::MSB)
    }
}
#[doc = "Data bit level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINV_A {
    #[doc = "0: Logical data from the data register are send/received in positive/direct logic"]
    POSITIVE = 0,
    #[doc = "1: Logical data from the data register are send/received in negative/inverse logic"]
    NEGATIVE = 1,
}
impl From<DINV_A> for bool {
    #[inline(always)]
    fn from(variant: DINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINV` reader - Data bit level inversion"]
pub type DINV_R = crate::BitReader<DINV_A>;
impl DINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINV_A {
        match self.bits {
            false => DINV_A::POSITIVE,
            true => DINV_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DINV_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DINV_A::NEGATIVE
    }
}
#[doc = "Field `DINV` writer - Data bit level inversion"]
pub type DINV_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, DINV_A, 18>;
impl<'a> DINV_W<'a> {
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(DINV_A::POSITIVE)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(DINV_A::NEGATIVE)
    }
}
#[doc = "TX pin level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TINV_A {
    #[doc = "0: TX pin signal works using the standard logic levels"]
    STANDARD = 0,
    #[doc = "1: TX pin signal values are inverted"]
    INVERTED = 1,
}
impl From<TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TINV` reader - TX pin level inversion"]
pub type TINV_R = crate::BitReader<TINV_A>;
impl TINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TINV_A {
        match self.bits {
            false => TINV_A::STANDARD,
            true => TINV_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TINV_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TINV_A::INVERTED
    }
}
#[doc = "Field `TINV` writer - TX pin level inversion"]
pub type TINV_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, TINV_A, 17>;
impl<'a> TINV_W<'a> {
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(TINV_A::STANDARD)
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TINV_A::INVERTED)
    }
}
#[doc = "RX pin level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RINV_A {
    #[doc = "0: RX pin signal works using the standard logic levels"]
    STANDARD = 0,
    #[doc = "1: RX pin signal values are inverted"]
    INVERTED = 1,
}
impl From<RINV_A> for bool {
    #[inline(always)]
    fn from(variant: RINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RINV` reader - RX pin level inversion"]
pub type RINV_R = crate::BitReader<RINV_A>;
impl RINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RINV_A {
        match self.bits {
            false => RINV_A::STANDARD,
            true => RINV_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RINV_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RINV_A::INVERTED
    }
}
#[doc = "Field `RINV` writer - RX pin level inversion"]
pub type RINV_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, RINV_A, 16>;
impl<'a> RINV_W<'a> {
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(RINV_A::STANDARD)
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RINV_A::INVERTED)
    }
}
#[doc = "Swap TX/RX pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRP_A {
    #[doc = "0: TX/RX pins are used as defined in standard pinout"]
    STANDARD = 0,
    #[doc = "1: The TX and RX pins functions are swapped"]
    SWAPPED = 1,
}
impl From<STRP_A> for bool {
    #[inline(always)]
    fn from(variant: STRP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRP` reader - Swap TX/RX pins"]
pub type STRP_R = crate::BitReader<STRP_A>;
impl STRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRP_A {
        match self.bits {
            false => STRP_A::STANDARD,
            true => STRP_A::SWAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == STRP_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `SWAPPED`"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == STRP_A::SWAPPED
    }
}
#[doc = "Field `STRP` writer - Swap TX/RX pins"]
pub type STRP_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, STRP_A, 15>;
impl<'a> STRP_W<'a> {
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(STRP_A::STANDARD)
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut W {
        self.variant(STRP_A::SWAPPED)
    }
}
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMEN_A {
    #[doc = "0: LIN mode disabled"]
    DISABLED = 0,
    #[doc = "1: LIN mode enabled"]
    ENABLED = 1,
}
impl From<LMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMEN` reader - LIN mode enable"]
pub type LMEN_R = crate::BitReader<LMEN_A>;
impl LMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMEN_A {
        match self.bits {
            false => LMEN_A::DISABLED,
            true => LMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LMEN_A::ENABLED
    }
}
#[doc = "Field `LMEN` writer - LIN mode enable"]
pub type LMEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, LMEN_A, 14>;
impl<'a> LMEN_W<'a> {
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LMEN_A::DISABLED)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LMEN_A::ENABLED)
    }
}
#[doc = "STOP bits length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STB_A {
    #[doc = "0: 1 stop bit"]
    STOP1 = 0,
    #[doc = "1: 0.5 stop bit"]
    STOP0P5 = 1,
    #[doc = "2: 2 stop bit"]
    STOP2 = 2,
    #[doc = "3: 1.5 stop bit"]
    STOP1P5 = 3,
}
impl From<STB_A> for u8 {
    #[inline(always)]
    fn from(variant: STB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STB` reader - STOP bits length"]
pub type STB_R = crate::FieldReader<u8, STB_A>;
impl STB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STB_A {
        match self.bits {
            0 => STB_A::STOP1,
            1 => STB_A::STOP0P5,
            2 => STB_A::STOP2,
            3 => STB_A::STOP1P5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STB_A::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP0P5`"]
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STB_A::STOP0P5
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STB_A::STOP2
    }
    #[doc = "Checks if the value of the field is `STOP1P5`"]
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STB_A::STOP1P5
    }
}
#[doc = "Field `STB` writer - STOP bits length"]
pub type STB_W<'a> = crate::FieldWriterSafe<'a, u32, CTL1_SPEC, u8, STB_A, 2, 12>;
impl<'a> STB_W<'a> {
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STB_A::STOP1)
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STB_A::STOP0P5)
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STB_A::STOP2)
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STB_A::STOP1P5)
    }
}
#[doc = "CK pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKEN_A {
    #[doc = "0: CK pin disabled"]
    DISABLED = 0,
    #[doc = "1: CK pin enabled"]
    ENABLED = 1,
}
impl From<CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKEN` reader - CK pin enable"]
pub type CKEN_R = crate::BitReader<CKEN_A>;
impl CKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKEN_A {
        match self.bits {
            false => CKEN_A::DISABLED,
            true => CKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKEN_A::ENABLED
    }
}
#[doc = "Field `CKEN` writer - CK pin enable"]
pub type CKEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CKEN_A, 11>;
impl<'a> CKEN_W<'a> {
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CKEN_A::DISABLED)
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CKEN_A::ENABLED)
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPL_A {
    #[doc = "0: Steady low value on CK pin outside tranmission window"]
    NOTINVERTED = 0,
    #[doc = "1: Steady high value on CK pin outside tranmission window"]
    INVERTED = 1,
}
impl From<CPL_A> for bool {
    #[inline(always)]
    fn from(variant: CPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPL` reader - Clock polarity"]
pub type CPL_R = crate::BitReader<CPL_A>;
impl CPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPL_A {
        match self.bits {
            false => CPL_A::NOTINVERTED,
            true => CPL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == CPL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CPL_A::INVERTED
    }
}
#[doc = "Field `CPL` writer - Clock polarity"]
pub type CPL_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CPL_A, 10>;
impl<'a> CPL_W<'a> {
    #[doc = "Steady low value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(CPL_A::NOTINVERTED)
    }
    #[doc = "Steady high value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CPL_A::INVERTED)
    }
}
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPH_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FIRST = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SECOND = 1,
}
impl From<CPH_A> for bool {
    #[inline(always)]
    fn from(variant: CPH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPH` reader - Clock phase"]
pub type CPH_R = crate::BitReader<CPH_A>;
impl CPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPH_A {
        match self.bits {
            false => CPH_A::FIRST,
            true => CPH_A::SECOND,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST`"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPH_A::FIRST
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPH_A::SECOND
    }
}
#[doc = "Field `CPH` writer - Clock phase"]
pub type CPH_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CPH_A, 9>;
impl<'a> CPH_W<'a> {
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut W {
        self.variant(CPH_A::FIRST)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(CPH_A::SECOND)
    }
}
#[doc = "CK length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEN_A {
    #[doc = "0: The clock pulse of the last data bit is not output to the CK pin"]
    NOTOUTPUT = 0,
    #[doc = "1: The clock pulse of the last data bit is output to the CK pin"]
    OUTPUT = 1,
}
impl From<CLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEN` reader - CK length"]
pub type CLEN_R = crate::BitReader<CLEN_A>;
impl CLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLEN_A {
        match self.bits {
            false => CLEN_A::NOTOUTPUT,
            true => CLEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTOUTPUT`"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == CLEN_A::NOTOUTPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CLEN_A::OUTPUT
    }
}
#[doc = "Field `CLEN` writer - CK length"]
pub type CLEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, CLEN_A, 8>;
impl<'a> CLEN_W<'a> {
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn not_output(self) -> &'a mut W {
        self.variant(CLEN_A::NOTOUTPUT)
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CLEN_A::OUTPUT)
    }
}
#[doc = "LIN break detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated whenever LBDF=1 in the STAT register"]
    ENABLED = 1,
}
impl From<LBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader<LBDIE_A>;
impl LBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBDIE_A {
        match self.bits {
            false => LBDIE_A::DISABLED,
            true => LBDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE_A::ENABLED
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, LBDIE_A, 6>;
impl<'a> LBDIE_W<'a> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBDIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBDIE_A::ENABLED)
    }
}
#[doc = "LIN break frame length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBLEN_A {
    #[doc = "0: 10-bit break detection"]
    BIT10 = 0,
    #[doc = "1: 11-bit break detection"]
    BIT11 = 1,
}
impl From<LBLEN_A> for bool {
    #[inline(always)]
    fn from(variant: LBLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBLEN` reader - LIN break frame length"]
pub type LBLEN_R = crate::BitReader<LBLEN_A>;
impl LBLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBLEN_A {
        match self.bits {
            false => LBLEN_A::BIT10,
            true => LBLEN_A::BIT11,
        }
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == LBLEN_A::BIT10
    }
    #[doc = "Checks if the value of the field is `BIT11`"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == LBLEN_A::BIT11
    }
}
#[doc = "Field `LBLEN` writer - LIN break frame length"]
pub type LBLEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, LBLEN_A, 5>;
impl<'a> LBLEN_W<'a> {
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(LBLEN_A::BIT10)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut W {
        self.variant(LBLEN_A::BIT11)
    }
}
#[doc = "Address detection mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDM_A {
    #[doc = "0: 4-bit address detection"]
    BIT4 = 0,
    #[doc = "1: Full-bit address detection"]
    FULL = 1,
}
impl From<ADDM_A> for bool {
    #[inline(always)]
    fn from(variant: ADDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDM` reader - Address detection mode"]
pub type ADDM_R = crate::BitReader<ADDM_A>;
impl ADDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDM_A {
        match self.bits {
            false => ADDM_A::BIT4,
            true => ADDM_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `BIT4`"]
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM_A::BIT4
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == ADDM_A::FULL
    }
}
#[doc = "Field `ADDM` writer - Address detection mode"]
pub type ADDM_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ADDM_A, 4>;
impl<'a> ADDM_W<'a> {
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn bit4(self) -> &'a mut W {
        self.variant(ADDM_A::BIT4)
    }
    #[doc = "Full-bit address detection"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(ADDM_A::FULL)
    }
}
impl R {
    #[doc = "Bits 24:31 - Address of the USART terminal"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&self) -> RTEN_R {
        RTEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abdm(&self) -> ABDM_R {
        ABDM_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abden(&self) -> ABDEN_R {
        ABDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Data bit level inversion"]
    #[inline(always)]
    pub fn dinv(&self) -> DINV_R {
        DINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin level inversion"]
    #[inline(always)]
    pub fn tinv(&self) -> TINV_R {
        TINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin level inversion"]
    #[inline(always)]
    pub fn rinv(&self) -> RINV_R {
        RINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn strp(&self) -> STRP_R {
        STRP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LMEN_R {
        LMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&self) -> STB_R {
        STB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&self) -> CPH_R {
        CPH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - CK length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&self) -> LBLEN_R {
        LBLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Address detection mode"]
    #[inline(always)]
    pub fn addm(&self) -> ADDM_R {
        ADDM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - Address of the USART terminal"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&mut self) -> RTEN_W {
        RTEN_W::new(self)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abdm(&mut self) -> ABDM_W {
        ABDM_W::new(self)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abden(&mut self) -> ABDEN_W {
        ABDEN_W::new(self)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W::new(self)
    }
    #[doc = "Bit 18 - Data bit level inversion"]
    #[inline(always)]
    pub fn dinv(&mut self) -> DINV_W {
        DINV_W::new(self)
    }
    #[doc = "Bit 17 - TX pin level inversion"]
    #[inline(always)]
    pub fn tinv(&mut self) -> TINV_W {
        TINV_W::new(self)
    }
    #[doc = "Bit 16 - RX pin level inversion"]
    #[inline(always)]
    pub fn rinv(&mut self) -> RINV_W {
        RINV_W::new(self)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn strp(&mut self) -> STRP_W {
        STRP_W::new(self)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&mut self) -> LMEN_W {
        LMEN_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&mut self) -> STB_W {
        STB_W::new(self)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W {
        CKEN_W::new(self)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&mut self) -> CPH_W {
        CPH_W::new(self)
    }
    #[doc = "Bit 8 - CK length"]
    #[inline(always)]
    pub fn clen(&mut self) -> CLEN_W {
        CLEN_W::new(self)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W {
        LBDIE_W::new(self)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&mut self) -> LBLEN_W {
        LBLEN_W::new(self)
    }
    #[doc = "Bit 4 - Address detection mode"]
    #[inline(always)]
    pub fn addm(&mut self) -> ADDM_W {
        ADDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
