#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `ADDM` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM_R = crate::BitReader<ADDM_A>;
#[doc = "7-bit Address Detection/4-bit Address Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ADDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDM_A {
        match self.bits {
            false => ADDM_A::BIT4,
            true => ADDM_A::FULL,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM_A::BIT4
    }
    #[doc = "Full-bit address detection"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == ADDM_A::FULL
    }
}
#[doc = "Field `ADDM` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADDM_A>;
impl<'a, REG, const O: u8> ADDM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn bit4(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM_A::BIT4)
    }
    #[doc = "Full-bit address detection"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM_A::FULL)
    }
}
#[doc = "Field `LBLEN` reader - LIN break detection length"]
pub type LBLEN_R = crate::BitReader<LBLEN_A>;
#[doc = "LIN break detection length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LBLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBLEN_A {
        match self.bits {
            false => LBLEN_A::BIT10,
            true => LBLEN_A::BIT11,
        }
    }
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == LBLEN_A::BIT10
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == LBLEN_A::BIT11
    }
}
#[doc = "Field `LBLEN` writer - LIN break detection length"]
pub type LBLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LBLEN_A>;
impl<'a, REG, const O: u8> LBLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(LBLEN_A::BIT10)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(LBLEN_A::BIT11)
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader<LBDIE_A>;
#[doc = "LIN break detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBDIE_A {
        match self.bits {
            false => LBDIE_A::DISABLED,
            true => LBDIE_A::ENABLED,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE_A::DISABLED
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE_A::ENABLED
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LBDIE_A>;
impl<'a, REG, const O: u8> LBDIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE_A::ENABLED)
    }
}
#[doc = "Field `CLEN` reader - Last bit clock pulse"]
pub type CLEN_R = crate::BitReader<CLEN_A>;
#[doc = "Last bit clock pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLEN_A {
    #[doc = "0: The clock pulse of the last data bit is not output to the CK pin"]
    NOT_OUTPUT = 0,
    #[doc = "1: The clock pulse of the last data bit is output to the CK pin"]
    OUTPUT = 1,
}
impl From<CLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLEN_A {
        match self.bits {
            false => CLEN_A::NOT_OUTPUT,
            true => CLEN_A::OUTPUT,
        }
    }
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == CLEN_A::NOT_OUTPUT
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CLEN_A::OUTPUT
    }
}
#[doc = "Field `CLEN` writer - Last bit clock pulse"]
pub type CLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLEN_A>;
impl<'a, REG, const O: u8> CLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn not_output(self) -> &'a mut crate::W<REG> {
        self.variant(CLEN_A::NOT_OUTPUT)
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CLEN_A::OUTPUT)
    }
}
#[doc = "Field `CPH` reader - Clock phase"]
pub type CPH_R = crate::BitReader<CPH_A>;
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPH_A {
        match self.bits {
            false => CPH_A::FIRST,
            true => CPH_A::SECOND,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPH_A::FIRST
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPH_A::SECOND
    }
}
#[doc = "Field `CPH` writer - Clock phase"]
pub type CPH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPH_A>;
impl<'a, REG, const O: u8> CPH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(CPH_A::FIRST)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(CPH_A::SECOND)
    }
}
#[doc = "Field `CPL` reader - Clock polarity"]
pub type CPL_R = crate::BitReader<CPL_A>;
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPL_A {
    #[doc = "0: Steady low value on CK pin outside tranmission window"]
    NOT_INVERTED = 0,
    #[doc = "1: Steady high value on CK pin outside tranmission window"]
    INVERTED = 1,
}
impl From<CPL_A> for bool {
    #[inline(always)]
    fn from(variant: CPL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPL_A {
        match self.bits {
            false => CPL_A::NOT_INVERTED,
            true => CPL_A::INVERTED,
        }
    }
    #[doc = "Steady low value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == CPL_A::NOT_INVERTED
    }
    #[doc = "Steady high value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CPL_A::INVERTED
    }
}
#[doc = "Field `CPL` writer - Clock polarity"]
pub type CPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPL_A>;
impl<'a, REG, const O: u8> CPL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Steady low value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CPL_A::NOT_INVERTED)
    }
    #[doc = "Steady high value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CPL_A::INVERTED)
    }
}
#[doc = "Field `CKEN` reader - Clock enable"]
pub type CKEN_R = crate::BitReader<CKEN_A>;
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKEN_A {
        match self.bits {
            false => CKEN_A::DISABLED,
            true => CKEN_A::ENABLED,
        }
    }
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKEN_A::DISABLED
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKEN_A::ENABLED
    }
}
#[doc = "Field `CKEN` writer - Clock enable"]
pub type CKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CKEN_A>;
impl<'a, REG, const O: u8> CKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKEN_A::DISABLED)
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKEN_A::ENABLED)
    }
}
#[doc = "Field `STB` reader - STOP bits"]
pub type STB_R = crate::FieldReader<STB_A>;
#[doc = "STOP bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for STB_A {
    type Ux = u8;
}
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
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STB_A::STOP1
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STB_A::STOP0P5
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STB_A::STOP2
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STB_A::STOP1P5
    }
}
#[doc = "Field `STB` writer - STOP bits"]
pub type STB_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, STB_A>;
impl<'a, REG, const O: u8> STB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STB_A::STOP1)
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut crate::W<REG> {
        self.variant(STB_A::STOP0P5)
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STB_A::STOP2)
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut crate::W<REG> {
        self.variant(STB_A::STOP1P5)
    }
}
#[doc = "Field `LMEN` reader - LIN mode enable"]
pub type LMEN_R = crate::BitReader<LMEN_A>;
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMEN_A {
        match self.bits {
            false => LMEN_A::DISABLED,
            true => LMEN_A::ENABLED,
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LMEN_A::DISABLED
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LMEN_A::ENABLED
    }
}
#[doc = "Field `LMEN` writer - LIN mode enable"]
pub type LMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LMEN_A>;
impl<'a, REG, const O: u8> LMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LMEN_A::DISABLED)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LMEN_A::ENABLED)
    }
}
#[doc = "Field `STRP` reader - Swap TX/RX pins"]
pub type STRP_R = crate::BitReader<STRP_A>;
#[doc = "Swap TX/RX pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl STRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRP_A {
        match self.bits {
            false => STRP_A::STANDARD,
            true => STRP_A::SWAPPED,
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == STRP_A::STANDARD
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == STRP_A::SWAPPED
    }
}
#[doc = "Field `STRP` writer - Swap TX/RX pins"]
pub type STRP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STRP_A>;
impl<'a, REG, const O: u8> STRP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(STRP_A::STANDARD)
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(STRP_A::SWAPPED)
    }
}
#[doc = "Field `RINV` reader - RX pin active level inversion"]
pub type RINV_R = crate::BitReader<RINV_A>;
#[doc = "RX pin active level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RINV_A {
        match self.bits {
            false => RINV_A::STANDARD,
            true => RINV_A::INVERTED,
        }
    }
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RINV_A::STANDARD
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RINV_A::INVERTED
    }
}
#[doc = "Field `RINV` writer - RX pin active level inversion"]
pub type RINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RINV_A>;
impl<'a, REG, const O: u8> RINV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(RINV_A::STANDARD)
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(RINV_A::INVERTED)
    }
}
#[doc = "Field `TINV` reader - TX pin active level inversion"]
pub type TINV_R = crate::BitReader<TINV_A>;
#[doc = "TX pin active level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TINV_A {
        match self.bits {
            false => TINV_A::STANDARD,
            true => TINV_A::INVERTED,
        }
    }
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TINV_A::STANDARD
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TINV_A::INVERTED
    }
}
#[doc = "Field `TINV` writer - TX pin active level inversion"]
pub type TINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TINV_A>;
impl<'a, REG, const O: u8> TINV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(TINV_A::STANDARD)
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(TINV_A::INVERTED)
    }
}
#[doc = "Field `DINV` reader - Binary data inversion"]
pub type DINV_R = crate::BitReader<DINV_A>;
#[doc = "Binary data inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINV_A {
        match self.bits {
            false => DINV_A::POSITIVE,
            true => DINV_A::NEGATIVE,
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DINV_A::POSITIVE
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DINV_A::NEGATIVE
    }
}
#[doc = "Field `DINV` writer - Binary data inversion"]
pub type DINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DINV_A>;
impl<'a, REG, const O: u8> DINV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(DINV_A::POSITIVE)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(DINV_A::NEGATIVE)
    }
}
#[doc = "Field `MSBF` reader - Most significant bit first"]
pub type MSBF_R = crate::BitReader<MSBF_A>;
#[doc = "Most significant bit first\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBF_A {
        match self.bits {
            false => MSBF_A::LSB,
            true => MSBF_A::MSB,
        }
    }
    #[doc = "Data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBF_A::LSB
    }
    #[doc = "Data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBF_A::MSB
    }
}
#[doc = "Field `MSBF` writer - Most significant bit first"]
pub type MSBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSBF_A>;
impl<'a, REG, const O: u8> MSBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBF_A::LSB)
    }
    #[doc = "Data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBF_A::MSB)
    }
}
#[doc = "Field `ABDEN` reader - Auto baud rate enable"]
pub type ABDEN_R = crate::BitReader<ABDEN_A>;
#[doc = "Auto baud rate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ABDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABDEN_A {
        match self.bits {
            false => ABDEN_A::DISABLED,
            true => ABDEN_A::ENABLED,
        }
    }
    #[doc = "Auto baud rate detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABDEN_A::DISABLED
    }
    #[doc = "Auto baud rate detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABDEN_A::ENABLED
    }
}
#[doc = "Field `ABDEN` writer - Auto baud rate enable"]
pub type ABDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABDEN_A>;
impl<'a, REG, const O: u8> ABDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto baud rate detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABDEN_A::DISABLED)
    }
    #[doc = "Auto baud rate detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABDEN_A::ENABLED)
    }
}
#[doc = "Field `ABDM` reader - Auto baud rate mode"]
pub type ABDM_R = crate::FieldReader<ABDM_A>;
#[doc = "Auto baud rate mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for ABDM_A {
    type Ux = u8;
}
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
    #[doc = "Measurement of the start bit is used to detect the baud rate"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ABDM_A::START
    }
    #[doc = "Falling edge to falling edge measurement"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ABDM_A::EDGE
    }
}
#[doc = "Field `ABDM` writer - Auto baud rate mode"]
pub type ABDM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ABDM_A>;
impl<'a, REG, const O: u8> ABDM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Measurement of the start bit is used to detect the baud rate"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ABDM_A::START)
    }
    #[doc = "Falling edge to falling edge measurement"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(ABDM_A::EDGE)
    }
}
#[doc = "Field `RTEN` reader - Receiver timeout enable"]
pub type RTEN_R = crate::BitReader<RTEN_A>;
#[doc = "Receiver timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTEN_A {
        match self.bits {
            false => RTEN_A::DISABLED,
            true => RTEN_A::ENABLED,
        }
    }
    #[doc = "Receiver timeout feature disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTEN_A::DISABLED
    }
    #[doc = "Receiver timeout feature enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTEN_A::ENABLED
    }
}
#[doc = "Field `RTEN` writer - Receiver timeout enable"]
pub type RTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTEN_A>;
impl<'a, REG, const O: u8> RTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver timeout feature disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTEN_A::DISABLED)
    }
    #[doc = "Receiver timeout feature enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTEN_A::ENABLED)
    }
}
#[doc = "Field `ADDR` reader - Address of the USART node"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address of the USART node"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm(&self) -> ADDM_R {
        ADDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lblen(&self) -> LBLEN_R {
        LBLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&self) -> CPH_R {
        CPH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stb(&self) -> STB_R {
        STB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LMEN_R {
        LMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn strp(&self) -> STRP_R {
        STRP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rinv(&self) -> RINV_R {
        RINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn tinv(&self) -> TINV_R {
        TINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn dinv(&self) -> DINV_R {
        DINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abden(&self) -> ABDEN_R {
        ABDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abdm(&self) -> ABDM_R {
        ABDM_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rten(&self) -> RTEN_R {
        RTEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    #[must_use]
    pub fn addm(&mut self) -> ADDM_W<CTL1_SPEC, 4> {
        ADDM_W::new(self)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    #[must_use]
    pub fn lblen(&mut self) -> LBLEN_W<CTL1_SPEC, 5> {
        LBLEN_W::new(self)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<CTL1_SPEC, 6> {
        LBDIE_W::new(self)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn clen(&mut self) -> CLEN_W<CTL1_SPEC, 8> {
        CLEN_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cph(&mut self) -> CPH_W<CTL1_SPEC, 9> {
        CPH_W::new(self)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<CTL1_SPEC, 10> {
        CPL_W::new(self)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CKEN_W<CTL1_SPEC, 11> {
        CKEN_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> STB_W<CTL1_SPEC, 12> {
        STB_W::new(self)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmen(&mut self) -> LMEN_W<CTL1_SPEC, 14> {
        LMEN_W::new(self)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    #[must_use]
    pub fn strp(&mut self) -> STRP_W<CTL1_SPEC, 15> {
        STRP_W::new(self)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rinv(&mut self) -> RINV_W<CTL1_SPEC, 16> {
        RINV_W::new(self)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn tinv(&mut self) -> TINV_W<CTL1_SPEC, 17> {
        TINV_W::new(self)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    #[must_use]
    pub fn dinv(&mut self) -> DINV_W<CTL1_SPEC, 18> {
        DINV_W::new(self)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<CTL1_SPEC, 19> {
        MSBF_W::new(self)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    #[must_use]
    pub fn abden(&mut self) -> ABDEN_W<CTL1_SPEC, 20> {
        ABDEN_W::new(self)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    #[must_use]
    pub fn abdm(&mut self) -> ABDM_W<CTL1_SPEC, 21> {
        ABDM_W::new(self)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rten(&mut self) -> RTEN_W<CTL1_SPEC, 23> {
        RTEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<CTL1_SPEC, 24> {
        ADDR_W::new(self)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
