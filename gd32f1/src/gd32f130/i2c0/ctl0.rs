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
#[doc = "Software reset I2C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRESET_A {
    #[doc = "0: I2C peripheral not under reset"]
    NOTRESET = 0,
    #[doc = "1: I2C peripheral under reset"]
    RESET = 1,
}
impl From<SRESET_A> for bool {
    #[inline(always)]
    fn from(variant: SRESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRESET` reader - Software reset I2C"]
pub type SRESET_R = crate::BitReader<SRESET_A>;
impl SRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRESET_A {
        match self.bits {
            false => SRESET_A::NOTRESET,
            true => SRESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SRESET_A::NOTRESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SRESET_A::RESET
    }
}
#[doc = "Field `SRESET` writer - Software reset I2C"]
pub type SRESET_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, SRESET_A, 15>;
impl<'a> SRESET_W<'a> {
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(SRESET_A::NOTRESET)
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SRESET_A::RESET)
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SALT_A {
    #[doc = "0: SMBA pin released high"]
    RELEASE = 0,
    #[doc = "1: SMBA pin driven low"]
    DRIVE = 1,
}
impl From<SALT_A> for bool {
    #[inline(always)]
    fn from(variant: SALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SALT` reader - SMBus alert"]
pub type SALT_R = crate::BitReader<SALT_A>;
impl SALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SALT_A {
        match self.bits {
            false => SALT_A::RELEASE,
            true => SALT_A::DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASE`"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == SALT_A::RELEASE
    }
    #[doc = "Checks if the value of the field is `DRIVE`"]
    #[inline(always)]
    pub fn is_drive(&self) -> bool {
        *self == SALT_A::DRIVE
    }
}
#[doc = "Field `SALT` writer - SMBus alert"]
pub type SALT_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, SALT_A, 13>;
impl<'a> SALT_W<'a> {
    #[doc = "SMBA pin released high"]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(SALT_A::RELEASE)
    }
    #[doc = "SMBA pin driven low"]
    #[inline(always)]
    pub fn drive(self) -> &'a mut W {
        self.variant(SALT_A::DRIVE)
    }
}
#[doc = "PEC Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECTRANS_A {
    #[doc = "0: No PEC transfer"]
    DISABLED = 0,
    #[doc = "1: PEC transfer"]
    ENABLED = 1,
}
impl From<PECTRANS_A> for bool {
    #[inline(always)]
    fn from(variant: PECTRANS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECTRANS` reader - PEC Transfer"]
pub type PECTRANS_R = crate::BitReader<PECTRANS_A>;
impl PECTRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECTRANS_A {
        match self.bits {
            false => PECTRANS_A::DISABLED,
            true => PECTRANS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PECTRANS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PECTRANS_A::ENABLED
    }
}
#[doc = "Field `PECTRANS` writer - PEC Transfer"]
pub type PECTRANS_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, PECTRANS_A, 12>;
impl<'a> PECTRANS_W<'a> {
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PECTRANS_A::DISABLED)
    }
    #[doc = "PEC transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PECTRANS_A::ENABLED)
    }
}
#[doc = "Position of ACK meaning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POAP_A {
    #[doc = "0: ACK bit controls the (N)ACK of the current byte being received"]
    CURRENT = 0,
    #[doc = "1: ACK bit controls the (N)ACK of the next byte to be received"]
    NEXT = 1,
}
impl From<POAP_A> for bool {
    #[inline(always)]
    fn from(variant: POAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POAP` reader - Position of ACK meaning"]
pub type POAP_R = crate::BitReader<POAP_A>;
impl POAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POAP_A {
        match self.bits {
            false => POAP_A::CURRENT,
            true => POAP_A::NEXT,
        }
    }
    #[doc = "Checks if the value of the field is `CURRENT`"]
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == POAP_A::CURRENT
    }
    #[doc = "Checks if the value of the field is `NEXT`"]
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == POAP_A::NEXT
    }
}
#[doc = "Field `POAP` writer - Position of ACK meaning"]
pub type POAP_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, POAP_A, 11>;
impl<'a> POAP_W<'a> {
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline(always)]
    pub fn current(self) -> &'a mut W {
        self.variant(POAP_A::CURRENT)
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline(always)]
    pub fn next(self) -> &'a mut W {
        self.variant(POAP_A::NEXT)
    }
}
#[doc = "Whether or not to send an ACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKEN_A {
    #[doc = "0: No acknowledge returned"]
    NAK = 0,
    #[doc = "1: Acknowledge returned after a byte is received"]
    ACK = 1,
}
impl From<ACKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKEN` reader - Whether or not to send an ACK"]
pub type ACKEN_R = crate::BitReader<ACKEN_A>;
impl ACKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKEN_A {
        match self.bits {
            false => ACKEN_A::NAK,
            true => ACKEN_A::ACK,
        }
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == ACKEN_A::NAK
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACKEN_A::ACK
    }
}
#[doc = "Field `ACKEN` writer - Whether or not to send an ACK"]
pub type ACKEN_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, ACKEN_A, 10>;
impl<'a> ACKEN_W<'a> {
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(ACKEN_A::NAK)
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(ACKEN_A::ACK)
    }
}
#[doc = "Generate a STOP condition on I2C bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: STOP will not be sent"]
    NOSTOP = 0,
    #[doc = "1: STOP will be sent"]
    STOP = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Generate a STOP condition on I2C bus"]
pub type STOP_R = crate::BitReader<STOP_A>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::NOSTOP,
            true => STOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP_A::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP_A::STOP
    }
}
#[doc = "Field `STOP` writer - Generate a STOP condition on I2C bus"]
pub type STOP_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, STOP_A, 9>;
impl<'a> STOP_W<'a> {
    #[doc = "STOP will not be sent"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut W {
        self.variant(STOP_A::NOSTOP)
    }
    #[doc = "STOP will be sent"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_A::STOP)
    }
}
#[doc = "Generate a START condition on I2C bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "0: START will not be sent"]
    NOSTART = 0,
    #[doc = "1: START will be sent"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Generate a START condition on I2C bus"]
pub type START_R = crate::BitReader<START_A>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NOSTART,
            true => START_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTART`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NOSTART
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Field `START` writer - Generate a START condition on I2C bus"]
pub type START_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, START_A, 8>;
impl<'a> START_W<'a> {
    #[doc = "START will not be sent"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut W {
        self.variant(START_A::NOSTART)
    }
    #[doc = "START will be sent"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
    }
}
#[doc = "Whether to stretch SCL low when data is not ready in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_A {
    #[doc = "0: Clock stretching enabled"]
    ENABLED = 0,
    #[doc = "1: Clock stretching disabled"]
    DISABLED = 1,
}
impl From<SS_A> for bool {
    #[inline(always)]
    fn from(variant: SS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS` reader - Whether to stretch SCL low when data is not ready in slave mode"]
pub type SS_R = crate::BitReader<SS_A>;
impl SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_A {
        match self.bits {
            false => SS_A::ENABLED,
            true => SS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SS_A::DISABLED
    }
}
#[doc = "Field `SS` writer - Whether to stretch SCL low when data is not ready in slave mode"]
pub type SS_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, SS_A, 7>;
impl<'a> SS_W<'a> {
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SS_A::ENABLED)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SS_A::DISABLED)
    }
}
#[doc = "Whether or not to response to a General Call (0x00)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCEN_A {
    #[doc = "0: Slave won't respond to General Call"]
    NOTRESPOND = 0,
    #[doc = "1: Slave will respond to General Call"]
    RESPOND = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - Whether or not to response to a General Call (0x00)"]
pub type GCEN_R = crate::BitReader<GCEN_A>;
impl GCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::NOTRESPOND,
            true => GCEN_A::RESPOND,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRESPOND`"]
    #[inline(always)]
    pub fn is_not_respond(&self) -> bool {
        *self == GCEN_A::NOTRESPOND
    }
    #[doc = "Checks if the value of the field is `RESPOND`"]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == GCEN_A::RESPOND
    }
}
#[doc = "Field `GCEN` writer - Whether or not to response to a General Call (0x00)"]
pub type GCEN_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, GCEN_A, 6>;
impl<'a> GCEN_W<'a> {
    #[doc = "Slave won't respond to General Call"]
    #[inline(always)]
    pub fn not_respond(self) -> &'a mut W {
        self.variant(GCEN_A::NOTRESPOND)
    }
    #[doc = "Slave will respond to General Call"]
    #[inline(always)]
    pub fn respond(self) -> &'a mut W {
        self.variant(GCEN_A::RESPOND)
    }
}
#[doc = "PEC Calculation Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECEN_A {
    #[doc = "0: PEC calculation disabled"]
    DISABLED = 0,
    #[doc = "1: PEC calculation enabled"]
    ENABLED = 1,
}
impl From<PECEN_A> for bool {
    #[inline(always)]
    fn from(variant: PECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC Calculation Switch"]
pub type PECEN_R = crate::BitReader<PECEN_A>;
impl PECEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECEN_A {
        match self.bits {
            false => PECEN_A::DISABLED,
            true => PECEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PECEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PECEN_A::ENABLED
    }
}
#[doc = "Field `PECEN` writer - PEC Calculation Switch"]
pub type PECEN_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, PECEN_A, 5>;
impl<'a> PECEN_W<'a> {
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PECEN_A::DISABLED)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PECEN_A::ENABLED)
    }
}
#[doc = "ARP protocol in SMBus switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARPEN_A {
    #[doc = "0: ARP disabled"]
    DISABLED = 0,
    #[doc = "1: ARP enabled"]
    ENABLED = 1,
}
impl From<ARPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARPEN` reader - ARP protocol in SMBus switch"]
pub type ARPEN_R = crate::BitReader<ARPEN_A>;
impl ARPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARPEN_A {
        match self.bits {
            false => ARPEN_A::DISABLED,
            true => ARPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARPEN_A::ENABLED
    }
}
#[doc = "Field `ARPEN` writer - ARP protocol in SMBus switch"]
pub type ARPEN_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, ARPEN_A, 4>;
impl<'a> ARPEN_W<'a> {
    #[doc = "ARP disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARPEN_A::DISABLED)
    }
    #[doc = "ARP enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARPEN_A::ENABLED)
    }
}
#[doc = "SMBusType Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBSEL_A {
    #[doc = "0: SMBus Device"]
    DEVICE = 0,
    #[doc = "1: SMBus Host"]
    HOST = 1,
}
impl From<SMBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SMBSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBSEL` reader - SMBusType Selection"]
pub type SMBSEL_R = crate::BitReader<SMBSEL_A>;
impl SMBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBSEL_A {
        match self.bits {
            false => SMBSEL_A::DEVICE,
            true => SMBSEL_A::HOST,
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == SMBSEL_A::DEVICE
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == SMBSEL_A::HOST
    }
}
#[doc = "Field `SMBSEL` writer - SMBusType Selection"]
pub type SMBSEL_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, SMBSEL_A, 3>;
impl<'a> SMBSEL_W<'a> {
    #[doc = "SMBus Device"]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(SMBSEL_A::DEVICE)
    }
    #[doc = "SMBus Host"]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(SMBSEL_A::HOST)
    }
}
#[doc = "SMBus/I2C mode switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBEN_A {
    #[doc = "0: I2C Mode"]
    I2C = 0,
    #[doc = "1: SMBus"]
    SMBUS = 1,
}
impl From<SMBEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBEN` reader - SMBus/I2C mode switch"]
pub type SMBEN_R = crate::BitReader<SMBEN_A>;
impl SMBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBEN_A {
        match self.bits {
            false => SMBEN_A::I2C,
            true => SMBEN_A::SMBUS,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == SMBEN_A::I2C
    }
    #[doc = "Checks if the value of the field is `SMBUS`"]
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == SMBEN_A::SMBUS
    }
}
#[doc = "Field `SMBEN` writer - SMBus/I2C mode switch"]
pub type SMBEN_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, SMBEN_A, 1>;
impl<'a> SMBEN_W<'a> {
    #[doc = "I2C Mode"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(SMBEN_A::I2C)
    }
    #[doc = "SMBus"]
    #[inline(always)]
    pub fn smbus(self) -> &'a mut W {
        self.variant(SMBEN_A::SMBUS)
    }
}
#[doc = "I2C peripheral enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CEN_A {
    #[doc = "0: Peripheral disabled"]
    DISABLED = 0,
    #[doc = "1: Peripheral enabled"]
    ENABLED = 1,
}
impl From<I2CEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CEN` reader - I2C peripheral enable"]
pub type I2CEN_R = crate::BitReader<I2CEN_A>;
impl I2CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CEN_A {
        match self.bits {
            false => I2CEN_A::DISABLED,
            true => I2CEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2CEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2CEN_A::ENABLED
    }
}
#[doc = "Field `I2CEN` writer - I2C peripheral enable"]
pub type I2CEN_W<'a> = crate::BitWriter<'a, u16, CTL0_SPEC, I2CEN_A, 0>;
impl<'a> I2CEN_W<'a> {
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2CEN_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2CEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 15 - Software reset I2C"]
    #[inline(always)]
    pub fn sreset(&self) -> SRESET_R {
        SRESET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn salt(&self) -> SALT_R {
        SALT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Transfer"]
    #[inline(always)]
    pub fn pectrans(&self) -> PECTRANS_R {
        PECTRANS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Position of ACK meaning"]
    #[inline(always)]
    pub fn poap(&self) -> POAP_R {
        POAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Whether or not to send an ACK"]
    #[inline(always)]
    pub fn acken(&self) -> ACKEN_R {
        ACKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Generate a STOP condition on I2C bus"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Generate a START condition on I2C bus"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Whether or not to response to a General Call (0x00)"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC Calculation Switch"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - ARP protocol in SMBus switch"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBusType Selection"]
    #[inline(always)]
    pub fn smbsel(&self) -> SMBSEL_R {
        SMBSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C mode switch"]
    #[inline(always)]
    pub fn smben(&self) -> SMBEN_R {
        SMBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Software reset I2C"]
    #[inline(always)]
    pub fn sreset(&mut self) -> SRESET_W {
        SRESET_W::new(self)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn salt(&mut self) -> SALT_W {
        SALT_W::new(self)
    }
    #[doc = "Bit 12 - PEC Transfer"]
    #[inline(always)]
    pub fn pectrans(&mut self) -> PECTRANS_W {
        PECTRANS_W::new(self)
    }
    #[doc = "Bit 11 - Position of ACK meaning"]
    #[inline(always)]
    pub fn poap(&mut self) -> POAP_W {
        POAP_W::new(self)
    }
    #[doc = "Bit 10 - Whether or not to send an ACK"]
    #[inline(always)]
    pub fn acken(&mut self) -> ACKEN_W {
        ACKEN_W::new(self)
    }
    #[doc = "Bit 9 - Generate a STOP condition on I2C bus"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W::new(self)
    }
    #[doc = "Bit 8 - Generate a START condition on I2C bus"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 7 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W::new(self)
    }
    #[doc = "Bit 6 - Whether or not to response to a General Call (0x00)"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W::new(self)
    }
    #[doc = "Bit 5 - PEC Calculation Switch"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W::new(self)
    }
    #[doc = "Bit 4 - ARP protocol in SMBus switch"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W {
        ARPEN_W::new(self)
    }
    #[doc = "Bit 3 - SMBusType Selection"]
    #[inline(always)]
    pub fn smbsel(&mut self) -> SMBSEL_W {
        SMBSEL_W::new(self)
    }
    #[doc = "Bit 1 - SMBus/I2C mode switch"]
    #[inline(always)]
    pub fn smben(&mut self) -> SMBEN_W {
        SMBEN_W::new(self)
    }
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W {
        I2CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u16;
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
