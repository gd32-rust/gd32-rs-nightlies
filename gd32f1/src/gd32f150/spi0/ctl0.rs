#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `CKPH` reader - Clock Phase Selection"]
pub type CKPH_R = crate::BitReader<CKPH_A>;
#[doc = "Clock Phase Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPH_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FIRST_EDGE = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SECOND_EDGE = 1,
}
impl From<CKPH_A> for bool {
    #[inline(always)]
    fn from(variant: CKPH_A) -> Self {
        variant as u8 != 0
    }
}
impl CKPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPH_A {
        match self.bits {
            false => CKPH_A::FIRST_EDGE,
            true => CKPH_A::SECOND_EDGE,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CKPH_A::FIRST_EDGE
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CKPH_A::SECOND_EDGE
    }
}
#[doc = "Field `CKPH` writer - Clock Phase Selection"]
pub type CKPH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CKPH_A>;
impl<'a, REG, const O: u8> CKPH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPH_A::FIRST_EDGE)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPH_A::SECOND_EDGE)
    }
}
#[doc = "Field `CKPL` reader - Clock Polarity Selection"]
pub type CKPL_R = crate::BitReader<CKPL_A>;
#[doc = "Clock Polarity Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPL_A {
    #[doc = "0: CLK pulled low when idle"]
    IDLE_LOW = 0,
    #[doc = "1: CLK pulled high when idle"]
    IDLE_HIGH = 1,
}
impl From<CKPL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPL_A) -> Self {
        variant as u8 != 0
    }
}
impl CKPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPL_A {
        match self.bits {
            false => CKPL_A::IDLE_LOW,
            true => CKPL_A::IDLE_HIGH,
        }
    }
    #[doc = "CLK pulled low when idle"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPL_A::IDLE_LOW
    }
    #[doc = "CLK pulled high when idle"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPL_A::IDLE_HIGH
    }
}
#[doc = "Field `CKPL` writer - Clock Polarity Selection"]
pub type CKPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CKPL_A>;
impl<'a, REG, const O: u8> CKPL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLK pulled low when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CKPL_A::IDLE_LOW)
    }
    #[doc = "CLK pulled high when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(CKPL_A::IDLE_HIGH)
    }
}
#[doc = "Field `MSTMOD` reader - Master selection"]
pub type MSTMOD_R = crate::BitReader<MSTMOD_A>;
#[doc = "Master selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSTMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTMOD_A {
        match self.bits {
            false => MSTMOD_A::SLAVE,
            true => MSTMOD_A::MASTER,
        }
    }
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTMOD_A::SLAVE
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTMOD_A::MASTER
    }
}
#[doc = "Field `MSTMOD` writer - Master selection"]
pub type MSTMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTMOD_A>;
impl<'a, REG, const O: u8> MSTMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MSTMOD_A::SLAVE)
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MSTMOD_A::MASTER)
    }
}
#[doc = "Field `PSC` reader - Master Clock Prescaler Selection"]
pub type PSC_R = crate::FieldReader<PSC_A>;
#[doc = "Master Clock Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for PSC_A {
    type Ux = u8;
}
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
    #[doc = "PCLK / 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSC_A::DIV2
    }
    #[doc = "PCLK / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSC_A::DIV4
    }
    #[doc = "PCLK / 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSC_A::DIV8
    }
    #[doc = "PCLK / 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PSC_A::DIV16
    }
    #[doc = "PCLK / 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PSC_A::DIV32
    }
    #[doc = "PCLK / 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PSC_A::DIV64
    }
    #[doc = "PCLK / 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PSC_A::DIV128
    }
    #[doc = "PCLK / 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PSC_A::DIV256
    }
}
#[doc = "Field `PSC` writer - Master Clock Prescaler Selection"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, PSC_A>;
impl<'a, REG, const O: u8> PSC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV2)
    }
    #[doc = "PCLK / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV4)
    }
    #[doc = "PCLK / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV8)
    }
    #[doc = "PCLK / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV16)
    }
    #[doc = "PCLK / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV32)
    }
    #[doc = "PCLK / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV64)
    }
    #[doc = "PCLK / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV128)
    }
    #[doc = "PCLK / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::DIV256)
    }
}
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SPIEN_R = crate::BitReader<SPIEN_A>;
#[doc = "SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIEN_A {
        match self.bits {
            false => SPIEN_A::DISABLED,
            true => SPIEN_A::ENABLED,
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPIEN_A::DISABLED
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPIEN_A::ENABLED
    }
}
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPIEN_A>;
impl<'a, REG, const O: u8> SPIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPIEN_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPIEN_A::ENABLED)
    }
}
#[doc = "Field `LF` reader - LSB First Mode"]
pub type LF_R = crate::BitReader<LF_A>;
#[doc = "LSB First Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LF_A {
        match self.bits {
            false => LF_A::MSBFIRST,
            true => LF_A::LSBFIRST,
        }
    }
    #[doc = "Data is transmitted/received with the MSB first"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LF_A::MSBFIRST
    }
    #[doc = "Data is transmitted/received with the LSB first"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LF_A::LSBFIRST
    }
}
#[doc = "Field `LF` writer - LSB First Mode"]
pub type LF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LF_A>;
impl<'a, REG, const O: u8> LF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is transmitted/received with the MSB first"]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(LF_A::MSBFIRST)
    }
    #[doc = "Data is transmitted/received with the LSB first"]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(LF_A::LSBFIRST)
    }
}
#[doc = "Field `SWNSS` reader - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_R = crate::BitReader<SWNSS_A>;
#[doc = "NSS Pin Selection In NSS Software Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWNSS_A {
    #[doc = "0: NSS is pulled low"]
    SLAVE_SELECTED = 0,
    #[doc = "1: NSS is pulled high"]
    SLAVE_NOT_SELECTED = 1,
}
impl From<SWNSS_A> for bool {
    #[inline(always)]
    fn from(variant: SWNSS_A) -> Self {
        variant as u8 != 0
    }
}
impl SWNSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWNSS_A {
        match self.bits {
            false => SWNSS_A::SLAVE_SELECTED,
            true => SWNSS_A::SLAVE_NOT_SELECTED,
        }
    }
    #[doc = "NSS is pulled low"]
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SWNSS_A::SLAVE_SELECTED
    }
    #[doc = "NSS is pulled high"]
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SWNSS_A::SLAVE_NOT_SELECTED
    }
}
#[doc = "Field `SWNSS` writer - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWNSS_A>;
impl<'a, REG, const O: u8> SWNSS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NSS is pulled low"]
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut crate::W<REG> {
        self.variant(SWNSS_A::SLAVE_SELECTED)
    }
    #[doc = "NSS is pulled high"]
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(SWNSS_A::SLAVE_NOT_SELECTED)
    }
}
#[doc = "Field `SWNSSEN` reader - NSS Software Mode Selection"]
pub type SWNSSEN_R = crate::BitReader<SWNSSEN_A>;
#[doc = "NSS Software Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SWNSSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWNSSEN_A {
        match self.bits {
            false => SWNSSEN_A::HARDWARE,
            true => SWNSSEN_A::SOFTWARE,
        }
    }
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == SWNSSEN_A::HARDWARE
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == SWNSSEN_A::SOFTWARE
    }
}
#[doc = "Field `SWNSSEN` writer - NSS Software Mode Selection"]
pub type SWNSSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWNSSEN_A>;
impl<'a, REG, const O: u8> SWNSSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(SWNSSEN_A::HARDWARE)
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(SWNSSEN_A::SOFTWARE)
    }
}
#[doc = "Field `RO` reader - Receive only"]
pub type RO_R = crate::BitReader<RO_A>;
#[doc = "Receive only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RO_A {
    #[doc = "0: Full duplex (Transmit and receive)"]
    FULL_DUPLEX = 0,
    #[doc = "1: Output disabled (Receive-only mode)"]
    RECEIVE_ONLY = 1,
}
impl From<RO_A> for bool {
    #[inline(always)]
    fn from(variant: RO_A) -> Self {
        variant as u8 != 0
    }
}
impl RO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO_A {
        match self.bits {
            false => RO_A::FULL_DUPLEX,
            true => RO_A::RECEIVE_ONLY,
        }
    }
    #[doc = "Full duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RO_A::FULL_DUPLEX
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn is_receive_only(&self) -> bool {
        *self == RO_A::RECEIVE_ONLY
    }
}
#[doc = "Field `RO` writer - Receive only"]
pub type RO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RO_A>;
impl<'a, REG, const O: u8> RO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(RO_A::FULL_DUPLEX)
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn receive_only(self) -> &'a mut crate::W<REG> {
        self.variant(RO_A::RECEIVE_ONLY)
    }
}
#[doc = "Field `FF16` reader - Data frame format"]
pub type FF16_R = crate::BitReader<FF16_A>;
#[doc = "Data frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FF16_A {
    #[doc = "0: 8-bit data frame format is selected for transmission/reception"]
    EIGHT_BIT = 0,
    #[doc = "1: 16-bit data frame format is selected for transmission/reception"]
    SIXTEEN_BIT = 1,
}
impl From<FF16_A> for bool {
    #[inline(always)]
    fn from(variant: FF16_A) -> Self {
        variant as u8 != 0
    }
}
impl FF16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FF16_A {
        match self.bits {
            false => FF16_A::EIGHT_BIT,
            true => FF16_A::SIXTEEN_BIT,
        }
    }
    #[doc = "8-bit data frame format is selected for transmission/reception"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == FF16_A::EIGHT_BIT
    }
    #[doc = "16-bit data frame format is selected for transmission/reception"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == FF16_A::SIXTEEN_BIT
    }
}
#[doc = "Field `FF16` writer - Data frame format"]
pub type FF16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FF16_A>;
impl<'a, REG, const O: u8> FF16_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit data frame format is selected for transmission/reception"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(FF16_A::EIGHT_BIT)
    }
    #[doc = "16-bit data frame format is selected for transmission/reception"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(FF16_A::SIXTEEN_BIT)
    }
}
#[doc = "Field `CRCNT` reader - CRC transfer next"]
pub type CRCNT_R = crate::BitReader<CRCNT_A>;
#[doc = "CRC transfer next\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CRCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCNT_A {
        match self.bits {
            false => CRCNT_A::DATA,
            true => CRCNT_A::CRC,
        }
    }
    #[doc = "Next transmit value is data from Tx buffer"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == CRCNT_A::DATA
    }
    #[doc = "Next transmit value is CRC value from TCRC"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == CRCNT_A::CRC
    }
}
#[doc = "Field `CRCNT` writer - CRC transfer next"]
pub type CRCNT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CRCNT_A>;
impl<'a, REG, const O: u8> CRCNT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next transmit value is data from Tx buffer"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(CRCNT_A::DATA)
    }
    #[doc = "Next transmit value is CRC value from TCRC"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(CRCNT_A::CRC)
    }
}
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable"]
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
#[doc = "Hardware CRC calculation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::DISABLED,
            true => CRCEN_A::ENABLED,
        }
    }
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::DISABLED
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::ENABLED
    }
}
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable"]
pub type CRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CRCEN_A>;
impl<'a, REG, const O: u8> CRCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN_A::DISABLED)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN_A::ENABLED)
    }
}
#[doc = "Field `BDOEN` reader - Bidirectional Transmit output enable"]
pub type BDOEN_R = crate::BitReader<BDOEN_A>;
#[doc = "Bidirectional Transmit output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDOEN_A {
    #[doc = "0: Receive-only mode"]
    RECEIVE_ONLY = 0,
    #[doc = "1: Transmit-only mode"]
    TRANSMIT_ONLY = 1,
}
impl From<BDOEN_A> for bool {
    #[inline(always)]
    fn from(variant: BDOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BDOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDOEN_A {
        match self.bits {
            false => BDOEN_A::RECEIVE_ONLY,
            true => BDOEN_A::TRANSMIT_ONLY,
        }
    }
    #[doc = "Receive-only mode"]
    #[inline(always)]
    pub fn is_receive_only(&self) -> bool {
        *self == BDOEN_A::RECEIVE_ONLY
    }
    #[doc = "Transmit-only mode"]
    #[inline(always)]
    pub fn is_transmit_only(&self) -> bool {
        *self == BDOEN_A::TRANSMIT_ONLY
    }
}
#[doc = "Field `BDOEN` writer - Bidirectional Transmit output enable"]
pub type BDOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BDOEN_A>;
impl<'a, REG, const O: u8> BDOEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive-only mode"]
    #[inline(always)]
    pub fn receive_only(self) -> &'a mut crate::W<REG> {
        self.variant(BDOEN_A::RECEIVE_ONLY)
    }
    #[doc = "Transmit-only mode"]
    #[inline(always)]
    pub fn transmit_only(self) -> &'a mut crate::W<REG> {
        self.variant(BDOEN_A::TRANSMIT_ONLY)
    }
}
#[doc = "Field `BDEN` reader - Bidirectional enable"]
pub type BDEN_R = crate::BitReader<BDEN_A>;
#[doc = "Bidirectional enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDEN_A {
        match self.bits {
            false => BDEN_A::UNIDIRECTIONAL,
            true => BDEN_A::BIDIRECTIONAL,
        }
    }
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn is_unidirectional(&self) -> bool {
        *self == BDEN_A::UNIDIRECTIONAL
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BDEN_A::BIDIRECTIONAL
    }
}
#[doc = "Field `BDEN` writer - Bidirectional enable"]
pub type BDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BDEN_A>;
impl<'a, REG, const O: u8> BDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn unidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BDEN_A::UNIDIRECTIONAL)
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BDEN_A::BIDIRECTIONAL)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    pub fn ckph(&self) -> CKPH_R {
        CKPH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity Selection"]
    #[inline(always)]
    pub fn ckpl(&self) -> CKPL_R {
        CKPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstmod(&self) -> MSTMOD_R {
        MSTMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    pub fn lf(&self) -> LF_R {
        LF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    pub fn swnss(&self) -> SWNSS_R {
        SWNSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    pub fn swnssen(&self) -> SWNSSEN_R {
        SWNSSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn ff16(&self) -> FF16_R {
        FF16_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnt(&self) -> CRCNT_R {
        CRCNT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    pub fn bdoen(&self) -> BDOEN_R {
        BDOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    pub fn bden(&self) -> BDEN_R {
        BDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckph(&mut self) -> CKPH_W<CTL0_SPEC, 0> {
        CKPH_W::new(self)
    }
    #[doc = "Bit 1 - Clock Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckpl(&mut self) -> CKPL_W<CTL0_SPEC, 1> {
        CKPL_W::new(self)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    #[must_use]
    pub fn mstmod(&mut self) -> MSTMOD_W<CTL0_SPEC, 2> {
        MSTMOD_W::new(self)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<CTL0_SPEC, 3> {
        PSC_W::new(self)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<CTL0_SPEC, 6> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lf(&mut self) -> LF_W<CTL0_SPEC, 7> {
        LF_W::new(self)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    #[must_use]
    pub fn swnss(&mut self) -> SWNSS_W<CTL0_SPEC, 8> {
        SWNSS_W::new(self)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn swnssen(&mut self) -> SWNSSEN_W<CTL0_SPEC, 9> {
        SWNSSEN_W::new(self)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RO_W<CTL0_SPEC, 10> {
        RO_W::new(self)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff16(&mut self) -> FF16_W<CTL0_SPEC, 11> {
        FF16_W::new(self)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    #[must_use]
    pub fn crcnt(&mut self) -> CRCNT_W<CTL0_SPEC, 12> {
        CRCNT_W::new(self)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<CTL0_SPEC, 13> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    #[must_use]
    pub fn bdoen(&mut self) -> BDOEN_W<CTL0_SPEC, 14> {
        BDOEN_W::new(self)
    }
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    #[must_use]
    pub fn bden(&mut self) -> BDEN_W<CTL0_SPEC, 15> {
        BDEN_W::new(self)
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
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
