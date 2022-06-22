#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bidirectional enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDEN_A {
    #[doc = "0: 2-line unidirectional data mode selected"]
    UNIDIRECTIONAL = 0,
    #[doc = "1: 1-line bidirectional data mode selected"]
    BIDIRECTIONAL = 1,
}
impl From<BDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDEN` reader - Bidirectional enable"]
pub type BDEN_R = crate::BitReader<BDEN_A>;
impl BDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDEN_A {
        match self.bits {
            false => BDEN_A::UNIDIRECTIONAL,
            true => BDEN_A::BIDIRECTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `UNIDIRECTIONAL`"]
    #[inline(always)]
    pub fn is_unidirectional(&self) -> bool {
        *self == BDEN_A::UNIDIRECTIONAL
    }
    #[doc = "Checks if the value of the field is `BIDIRECTIONAL`"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BDEN_A::BIDIRECTIONAL
    }
}
#[doc = "Field `BDEN` writer - Bidirectional enable"]
pub type BDEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, BDEN_A, 15>;
impl<'a> BDEN_W<'a> {
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn unidirectional(self) -> &'a mut W {
        self.variant(BDEN_A::UNIDIRECTIONAL)
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BDEN_A::BIDIRECTIONAL)
    }
}
#[doc = "Bidirectional Transmit output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDOEN_A {
    #[doc = "0: Receive-only mode"]
    RECEIVEONLY = 0,
    #[doc = "1: Transmit-only mode"]
    TRANSMITONLY = 1,
}
impl From<BDOEN_A> for bool {
    #[inline(always)]
    fn from(variant: BDOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDOEN` reader - Bidirectional Transmit output enable"]
pub type BDOEN_R = crate::BitReader<BDOEN_A>;
impl BDOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDOEN_A {
        match self.bits {
            false => BDOEN_A::RECEIVEONLY,
            true => BDOEN_A::TRANSMITONLY,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVEONLY`"]
    #[inline(always)]
    pub fn is_receive_only(&self) -> bool {
        *self == BDOEN_A::RECEIVEONLY
    }
    #[doc = "Checks if the value of the field is `TRANSMITONLY`"]
    #[inline(always)]
    pub fn is_transmit_only(&self) -> bool {
        *self == BDOEN_A::TRANSMITONLY
    }
}
#[doc = "Field `BDOEN` writer - Bidirectional Transmit output enable"]
pub type BDOEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, BDOEN_A, 14>;
impl<'a> BDOEN_W<'a> {
    #[doc = "Receive-only mode"]
    #[inline(always)]
    pub fn receive_only(self) -> &'a mut W {
        self.variant(BDOEN_A::RECEIVEONLY)
    }
    #[doc = "Transmit-only mode"]
    #[inline(always)]
    pub fn transmit_only(self) -> &'a mut W {
        self.variant(BDOEN_A::TRANSMITONLY)
    }
}
#[doc = "CRC Calculation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCEN_A {
    #[doc = "0: CRC calculation disabled"]
    DISABLED = 0,
    #[doc = "1: CRC calculation enabled"]
    ENABLED = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - CRC Calculation Enable"]
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::DISABLED,
            true => CRCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::ENABLED
    }
}
#[doc = "Field `CRCEN` writer - CRC Calculation Enable"]
pub type CRCEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, CRCEN_A, 13>;
impl<'a> CRCEN_W<'a> {
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::DISABLED)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::ENABLED)
    }
}
#[doc = "CRC Next Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCNT_A {
    #[doc = "0: Next transmit value is data from Tx buffer"]
    DATA = 0,
    #[doc = "1: Next transmit value is CRC value from TCRC"]
    CRC = 1,
}
impl From<CRCNT_A> for bool {
    #[inline(always)]
    fn from(variant: CRCNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCNT` reader - CRC Next Transfer"]
pub type CRCNT_R = crate::BitReader<CRCNT_A>;
impl CRCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCNT_A {
        match self.bits {
            false => CRCNT_A::DATA,
            true => CRCNT_A::CRC,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == CRCNT_A::DATA
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == CRCNT_A::CRC
    }
}
#[doc = "Field `CRCNT` writer - CRC Next Transfer"]
pub type CRCNT_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, CRCNT_A, 12>;
impl<'a> CRCNT_W<'a> {
    #[doc = "Next transmit value is data from Tx buffer"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(CRCNT_A::DATA)
    }
    #[doc = "Next transmit value is CRC value from TCRC"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(CRCNT_A::CRC)
    }
}
#[doc = "Data frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FF16_A {
    #[doc = "0: 8-bit data frame format is selected for transmission/reception"]
    EIGHTBIT = 0,
    #[doc = "1: 16-bit data frame format is selected for transmission/reception"]
    SIXTEENBIT = 1,
}
impl From<FF16_A> for bool {
    #[inline(always)]
    fn from(variant: FF16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FF16` reader - Data frame format"]
pub type FF16_R = crate::BitReader<FF16_A>;
impl FF16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FF16_A {
        match self.bits {
            false => FF16_A::EIGHTBIT,
            true => FF16_A::SIXTEENBIT,
        }
    }
    #[doc = "Checks if the value of the field is `EIGHTBIT`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == FF16_A::EIGHTBIT
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == FF16_A::SIXTEENBIT
    }
}
#[doc = "Field `FF16` writer - Data frame format"]
pub type FF16_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, FF16_A, 11>;
impl<'a> FF16_W<'a> {
    #[doc = "8-bit data frame format is selected for transmission/reception"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(FF16_A::EIGHTBIT)
    }
    #[doc = "16-bit data frame format is selected for transmission/reception"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(FF16_A::SIXTEENBIT)
    }
}
#[doc = "Receive only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_A {
    #[doc = "0: Full duplex (Transmit and receive)"]
    FULLDUPLEX = 0,
    #[doc = "1: Output disabled (Receive-only mode)"]
    RECEIVEONLY = 1,
}
impl From<RO_A> for bool {
    #[inline(always)]
    fn from(variant: RO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RO` reader - Receive only"]
pub type RO_R = crate::BitReader<RO_A>;
impl RO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO_A {
        match self.bits {
            false => RO_A::FULLDUPLEX,
            true => RO_A::RECEIVEONLY,
        }
    }
    #[doc = "Checks if the value of the field is `FULLDUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RO_A::FULLDUPLEX
    }
    #[doc = "Checks if the value of the field is `RECEIVEONLY`"]
    #[inline(always)]
    pub fn is_receive_only(&self) -> bool {
        *self == RO_A::RECEIVEONLY
    }
}
#[doc = "Field `RO` writer - Receive only"]
pub type RO_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, RO_A, 10>;
impl<'a> RO_W<'a> {
    #[doc = "Full duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(RO_A::FULLDUPLEX)
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn receive_only(self) -> &'a mut W {
        self.variant(RO_A::RECEIVEONLY)
    }
}
#[doc = "NSS Software Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWNSSEN_A {
    #[doc = "0: Software slave management disabled"]
    HARDWARE = 0,
    #[doc = "1: Software slave management enabled"]
    SOFTWARE = 1,
}
impl From<SWNSSEN_A> for bool {
    #[inline(always)]
    fn from(variant: SWNSSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWNSSEN` reader - NSS Software Mode Selection"]
pub type SWNSSEN_R = crate::BitReader<SWNSSEN_A>;
impl SWNSSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWNSSEN_A {
        match self.bits {
            false => SWNSSEN_A::HARDWARE,
            true => SWNSSEN_A::SOFTWARE,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == SWNSSEN_A::HARDWARE
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == SWNSSEN_A::SOFTWARE
    }
}
#[doc = "Field `SWNSSEN` writer - NSS Software Mode Selection"]
pub type SWNSSEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, SWNSSEN_A, 9>;
impl<'a> SWNSSEN_W<'a> {
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(SWNSSEN_A::HARDWARE)
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(SWNSSEN_A::SOFTWARE)
    }
}
#[doc = "NSS Pin Selection In NSS Software Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWNSS_A {
    #[doc = "0: NSS is pulled low"]
    SLAVESELECTED = 0,
    #[doc = "1: NSS is pulled high"]
    SLAVENOTSELECTED = 1,
}
impl From<SWNSS_A> for bool {
    #[inline(always)]
    fn from(variant: SWNSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWNSS` reader - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_R = crate::BitReader<SWNSS_A>;
impl SWNSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWNSS_A {
        match self.bits {
            false => SWNSS_A::SLAVESELECTED,
            true => SWNSS_A::SLAVENOTSELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVESELECTED`"]
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SWNSS_A::SLAVESELECTED
    }
    #[doc = "Checks if the value of the field is `SLAVENOTSELECTED`"]
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SWNSS_A::SLAVENOTSELECTED
    }
}
#[doc = "Field `SWNSS` writer - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, SWNSS_A, 8>;
impl<'a> SWNSS_W<'a> {
    #[doc = "NSS is pulled low"]
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut W {
        self.variant(SWNSS_A::SLAVESELECTED)
    }
    #[doc = "NSS is pulled high"]
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut W {
        self.variant(SWNSS_A::SLAVENOTSELECTED)
    }
}
#[doc = "LSB First Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LF_A {
    #[doc = "0: Data is transmitted/received with the MSB first"]
    MSBFIRST = 0,
    #[doc = "1: Data is transmitted/received with the LSB first"]
    LSBFIRST = 1,
}
impl From<LF_A> for bool {
    #[inline(always)]
    fn from(variant: LF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LF` reader - LSB First Mode"]
pub type LF_R = crate::BitReader<LF_A>;
impl LF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LF_A {
        match self.bits {
            false => LF_A::MSBFIRST,
            true => LF_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LF_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LF_A::LSBFIRST
    }
}
#[doc = "Field `LF` writer - LSB First Mode"]
pub type LF_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, LF_A, 7>;
impl<'a> LF_W<'a> {
    #[doc = "Data is transmitted/received with the MSB first"]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(LF_A::MSBFIRST)
    }
    #[doc = "Data is transmitted/received with the LSB first"]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(LF_A::LSBFIRST)
    }
}
#[doc = "SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIEN_A {
    #[doc = "0: Peripheral disabled"]
    DISABLED = 0,
    #[doc = "1: Peripheral enabled"]
    ENABLED = 1,
}
impl From<SPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SPIEN_R = crate::BitReader<SPIEN_A>;
impl SPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIEN_A {
        match self.bits {
            false => SPIEN_A::DISABLED,
            true => SPIEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPIEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPIEN_A::ENABLED
    }
}
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SPIEN_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, SPIEN_A, 6>;
impl<'a> SPIEN_W<'a> {
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPIEN_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPIEN_A::ENABLED)
    }
}
#[doc = "Master Clock Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: PCLK / 2"]
    DIV2 = 0,
    #[doc = "1: PCLK / 4"]
    DIV4 = 1,
    #[doc = "2: PCLK / 8"]
    DIV8 = 2,
    #[doc = "3: PCLK / 16"]
    DIV16 = 3,
    #[doc = "4: PCLK / 32"]
    DIV32 = 4,
    #[doc = "5: PCLK / 64"]
    DIV64 = 5,
    #[doc = "6: PCLK / 128"]
    DIV128 = 6,
    #[doc = "7: PCLK / 256"]
    DIV256 = 7,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSC` reader - Master Clock Prescaler Selection"]
pub type PSC_R = crate::FieldReader<u8, PSC_A>;
impl PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSC_A {
        match self.bits {
            0 => PSC_A::DIV2,
            1 => PSC_A::DIV4,
            2 => PSC_A::DIV8,
            3 => PSC_A::DIV16,
            4 => PSC_A::DIV32,
            5 => PSC_A::DIV64,
            6 => PSC_A::DIV128,
            7 => PSC_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PSC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PSC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PSC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PSC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PSC_A::DIV256
    }
}
#[doc = "Field `PSC` writer - Master Clock Prescaler Selection"]
pub type PSC_W<'a> = crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, PSC_A, 3, 3>;
impl<'a> PSC_W<'a> {
    #[doc = "PCLK / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PSC_A::DIV2)
    }
    #[doc = "PCLK / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSC_A::DIV4)
    }
    #[doc = "PCLK / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSC_A::DIV8)
    }
    #[doc = "PCLK / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PSC_A::DIV16)
    }
    #[doc = "PCLK / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PSC_A::DIV32)
    }
    #[doc = "PCLK / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PSC_A::DIV64)
    }
    #[doc = "PCLK / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PSC_A::DIV128)
    }
    #[doc = "PCLK / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PSC_A::DIV256)
    }
}
#[doc = "Master Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTMOD_A {
    #[doc = "0: Slave configuration"]
    SLAVE = 0,
    #[doc = "1: Master configuration"]
    MASTER = 1,
}
impl From<MSTMOD_A> for bool {
    #[inline(always)]
    fn from(variant: MSTMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTMOD` reader - Master Mode Enable"]
pub type MSTMOD_R = crate::BitReader<MSTMOD_A>;
impl MSTMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTMOD_A {
        match self.bits {
            false => MSTMOD_A::SLAVE,
            true => MSTMOD_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTMOD_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTMOD_A::MASTER
    }
}
#[doc = "Field `MSTMOD` writer - Master Mode Enable"]
pub type MSTMOD_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, MSTMOD_A, 2>;
impl<'a> MSTMOD_W<'a> {
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTMOD_A::SLAVE)
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTMOD_A::MASTER)
    }
}
#[doc = "Clock polarity Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPL_A {
    #[doc = "0: CLK pulled low when idle"]
    IDLELOW = 0,
    #[doc = "1: CLK pulled high when idle"]
    IDLEHIGH = 1,
}
impl From<CKPL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPL` reader - Clock polarity Selection"]
pub type CKPL_R = crate::BitReader<CKPL_A>;
impl CKPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPL_A {
        match self.bits {
            false => CKPL_A::IDLELOW,
            true => CKPL_A::IDLEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPL_A::IDLELOW
    }
    #[doc = "Checks if the value of the field is `IDLEHIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPL_A::IDLEHIGH
    }
}
#[doc = "Field `CKPL` writer - Clock polarity Selection"]
pub type CKPL_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, CKPL_A, 1>;
impl<'a> CKPL_W<'a> {
    #[doc = "CLK pulled low when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CKPL_A::IDLELOW)
    }
    #[doc = "CLK pulled high when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CKPL_A::IDLEHIGH)
    }
}
#[doc = "Clock Phase Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPH_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FIRSTEDGE = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SECONDEDGE = 1,
}
impl From<CKPH_A> for bool {
    #[inline(always)]
    fn from(variant: CKPH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPH` reader - Clock Phase Selection"]
pub type CKPH_R = crate::BitReader<CKPH_A>;
impl CKPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPH_A {
        match self.bits {
            false => CKPH_A::FIRSTEDGE,
            true => CKPH_A::SECONDEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FIRSTEDGE`"]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CKPH_A::FIRSTEDGE
    }
    #[doc = "Checks if the value of the field is `SECONDEDGE`"]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CKPH_A::SECONDEDGE
    }
}
#[doc = "Field `CKPH` writer - Clock Phase Selection"]
pub type CKPH_W<'a> = crate::BitWriter<'a, u32, CTL0_SPEC, CKPH_A, 0>;
impl<'a> CKPH_W<'a> {
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CKPH_A::FIRSTEDGE)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CKPH_A::SECONDEDGE)
    }
}
impl R {
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    pub fn bden(&self) -> BDEN_R {
        BDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    pub fn bdoen(&self) -> BDOEN_R {
        BDOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC Calculation Enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC Next Transfer"]
    #[inline(always)]
    pub fn crcnt(&self) -> CRCNT_R {
        CRCNT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn ff16(&self) -> FF16_R {
        FF16_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    pub fn swnssen(&self) -> SWNSSEN_R {
        SWNSSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    pub fn swnss(&self) -> SWNSS_R {
        SWNSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    pub fn lf(&self) -> LF_R {
        LF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    pub fn mstmod(&self) -> MSTMOD_R {
        MSTMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity Selection"]
    #[inline(always)]
    pub fn ckpl(&self) -> CKPL_R {
        CKPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    pub fn ckph(&self) -> CKPH_R {
        CKPH_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    pub fn bden(&mut self) -> BDEN_W {
        BDEN_W::new(self)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    pub fn bdoen(&mut self) -> BDOEN_W {
        BDOEN_W::new(self)
    }
    #[doc = "Bit 13 - CRC Calculation Enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC Next Transfer"]
    #[inline(always)]
    pub fn crcnt(&mut self) -> CRCNT_W {
        CRCNT_W::new(self)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn ff16(&mut self) -> FF16_W {
        FF16_W::new(self)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn ro(&mut self) -> RO_W {
        RO_W::new(self)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    pub fn swnssen(&mut self) -> SWNSSEN_W {
        SWNSSEN_W::new(self)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    pub fn swnss(&mut self) -> SWNSS_W {
        SWNSS_W::new(self)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    pub fn lf(&mut self) -> LF_W {
        LF_W::new(self)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W::new(self)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W::new(self)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    pub fn mstmod(&mut self) -> MSTMOD_W {
        MSTMOD_W::new(self)
    }
    #[doc = "Bit 1 - Clock polarity Selection"]
    #[inline(always)]
    pub fn ckpl(&mut self) -> CKPL_W {
        CKPL_W::new(self)
    }
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    pub fn ckph(&mut self) -> CKPH_W {
        CKPH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
