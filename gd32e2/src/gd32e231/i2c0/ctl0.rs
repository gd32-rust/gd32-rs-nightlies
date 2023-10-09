#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `I2CEN` reader - Peripheral enable"]
pub type I2CEN_R = crate::BitReader<I2CEN_A>;
#[doc = "Peripheral enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl I2CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CEN_A {
        match self.bits {
            false => I2CEN_A::DISABLED,
            true => I2CEN_A::ENABLED,
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2CEN_A::DISABLED
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2CEN_A::ENABLED
    }
}
#[doc = "Field `I2CEN` writer - Peripheral enable"]
pub type I2CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2CEN_A>;
impl<'a, REG, const O: u8> I2CEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2CEN_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2CEN_A::ENABLED)
    }
}
#[doc = "Field `SMBEN` reader - SMBus mode"]
pub type SMBEN_R = crate::BitReader<SMBEN_A>;
#[doc = "SMBus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SMBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBEN_A {
        match self.bits {
            false => SMBEN_A::I2C,
            true => SMBEN_A::SMBUS,
        }
    }
    #[doc = "I2C Mode"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == SMBEN_A::I2C
    }
    #[doc = "SMBus"]
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == SMBEN_A::SMBUS
    }
}
#[doc = "Field `SMBEN` writer - SMBus mode"]
pub type SMBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMBEN_A>;
impl<'a, REG, const O: u8> SMBEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C Mode"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(SMBEN_A::I2C)
    }
    #[doc = "SMBus"]
    #[inline(always)]
    pub fn smbus(self) -> &'a mut crate::W<REG> {
        self.variant(SMBEN_A::SMBUS)
    }
}
#[doc = "Field `SMBSEL` reader - SMBus type"]
pub type SMBSEL_R = crate::BitReader<SMBSEL_A>;
#[doc = "SMBus type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SMBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBSEL_A {
        match self.bits {
            false => SMBSEL_A::DEVICE,
            true => SMBSEL_A::HOST,
        }
    }
    #[doc = "SMBus Device"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == SMBSEL_A::DEVICE
    }
    #[doc = "SMBus Host"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == SMBSEL_A::HOST
    }
}
#[doc = "Field `SMBSEL` writer - SMBus type"]
pub type SMBSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMBSEL_A>;
impl<'a, REG, const O: u8> SMBSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBus Device"]
    #[inline(always)]
    pub fn device(self) -> &'a mut crate::W<REG> {
        self.variant(SMBSEL_A::DEVICE)
    }
    #[doc = "SMBus Host"]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(SMBSEL_A::HOST)
    }
}
#[doc = "Field `ARPEN` reader - ARP enable"]
pub type ARPEN_R = crate::BitReader<ARPEN_A>;
#[doc = "ARP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ARPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARPEN_A {
        match self.bits {
            false => ARPEN_A::DISABLED,
            true => ARPEN_A::ENABLED,
        }
    }
    #[doc = "ARP disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARPEN_A::DISABLED
    }
    #[doc = "ARP enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARPEN_A::ENABLED
    }
}
#[doc = "Field `ARPEN` writer - ARP enable"]
pub type ARPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ARPEN_A>;
impl<'a, REG, const O: u8> ARPEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARP disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARPEN_A::DISABLED)
    }
    #[doc = "ARP enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARPEN_A::ENABLED)
    }
}
#[doc = "Field `PECEN` reader - PEC enable"]
pub type PECEN_R = crate::BitReader<PECEN_A>;
#[doc = "PEC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PECEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECEN_A {
        match self.bits {
            false => PECEN_A::DISABLED,
            true => PECEN_A::ENABLED,
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PECEN_A::DISABLED
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PECEN_A::ENABLED
    }
}
#[doc = "Field `PECEN` writer - PEC enable"]
pub type PECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PECEN_A>;
impl<'a, REG, const O: u8> PECEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PECEN_A::DISABLED)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PECEN_A::ENABLED)
    }
}
#[doc = "Field `GCEN` reader - General call enable"]
pub type GCEN_R = crate::BitReader<GCEN_A>;
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCEN_A {
    #[doc = "0: Slave won't respond to General Call"]
    NOT_RESPOND = 0,
    #[doc = "1: Slave will respond to General Call"]
    RESPOND = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::NOT_RESPOND,
            true => GCEN_A::RESPOND,
        }
    }
    #[doc = "Slave won't respond to General Call"]
    #[inline(always)]
    pub fn is_not_respond(&self) -> bool {
        *self == GCEN_A::NOT_RESPOND
    }
    #[doc = "Slave will respond to General Call"]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == GCEN_A::RESPOND
    }
}
#[doc = "Field `GCEN` writer - General call enable"]
pub type GCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GCEN_A>;
impl<'a, REG, const O: u8> GCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave won't respond to General Call"]
    #[inline(always)]
    pub fn not_respond(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN_A::NOT_RESPOND)
    }
    #[doc = "Slave will respond to General Call"]
    #[inline(always)]
    pub fn respond(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN_A::RESPOND)
    }
}
#[doc = "Field `SS` reader - SCL Stretching(Slave mode)"]
pub type SS_R = crate::BitReader<SS_A>;
#[doc = "SCL Stretching(Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_A {
        match self.bits {
            false => SS_A::ENABLED,
            true => SS_A::DISABLED,
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SS_A::ENABLED
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SS_A::DISABLED
    }
}
#[doc = "Field `SS` writer - SCL Stretching(Slave mode)"]
pub type SS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SS_A>;
impl<'a, REG, const O: u8> SS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SS_A::ENABLED)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SS_A::DISABLED)
    }
}
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: START will not be sent"]
    NO_START = 0,
    #[doc = "1: START will be sent"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NO_START,
            true => START_A::START,
        }
    }
    #[doc = "START will not be sent"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NO_START
    }
    #[doc = "START will be sent"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, START_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "START will not be sent"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::NO_START)
    }
    #[doc = "START will be sent"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::START)
    }
}
#[doc = "Field `STOP` reader - Stop condition"]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Stop condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: STOP will not be sent"]
    NO_STOP = 0,
    #[doc = "1: STOP will be sent"]
    STOP = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::NO_STOP,
            true => STOP_A::STOP,
        }
    }
    #[doc = "STOP will not be sent"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP_A::NO_STOP
    }
    #[doc = "STOP will be sent"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP_A::STOP
    }
}
#[doc = "Field `STOP` writer - Stop condition"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STOP_A>;
impl<'a, REG, const O: u8> STOP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOP will not be sent"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::NO_STOP)
    }
    #[doc = "STOP will be sent"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::STOP)
    }
}
#[doc = "Field `ACKEN` reader - Acknowledge enable"]
pub type ACKEN_R = crate::BitReader<ACKEN_A>;
#[doc = "Acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ACKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKEN_A {
        match self.bits {
            false => ACKEN_A::NAK,
            true => ACKEN_A::ACK,
        }
    }
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == ACKEN_A::NAK
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACKEN_A::ACK
    }
}
#[doc = "Field `ACKEN` writer - Acknowledge enable"]
pub type ACKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ACKEN_A>;
impl<'a, REG, const O: u8> ACKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(ACKEN_A::NAK)
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(ACKEN_A::ACK)
    }
}
#[doc = "Field `POAP` reader - Acknowledge/PEC Position (for data reception)"]
pub type POAP_R = crate::BitReader<POAP_A>;
#[doc = "Acknowledge/PEC Position (for data reception)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl POAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POAP_A {
        match self.bits {
            false => POAP_A::CURRENT,
            true => POAP_A::NEXT,
        }
    }
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == POAP_A::CURRENT
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == POAP_A::NEXT
    }
}
#[doc = "Field `POAP` writer - Acknowledge/PEC Position (for data reception)"]
pub type POAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, POAP_A>;
impl<'a, REG, const O: u8> POAP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline(always)]
    pub fn current(self) -> &'a mut crate::W<REG> {
        self.variant(POAP_A::CURRENT)
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline(always)]
    pub fn next(self) -> &'a mut crate::W<REG> {
        self.variant(POAP_A::NEXT)
    }
}
#[doc = "Field `PECTRANS` reader - Packet error checking"]
pub type PECTRANS_R = crate::BitReader<PECTRANS_A>;
#[doc = "Packet error checking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PECTRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECTRANS_A {
        match self.bits {
            false => PECTRANS_A::DISABLED,
            true => PECTRANS_A::ENABLED,
        }
    }
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PECTRANS_A::DISABLED
    }
    #[doc = "PEC transfer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PECTRANS_A::ENABLED
    }
}
#[doc = "Field `PECTRANS` writer - Packet error checking"]
pub type PECTRANS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PECTRANS_A>;
impl<'a, REG, const O: u8> PECTRANS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PECTRANS_A::DISABLED)
    }
    #[doc = "PEC transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PECTRANS_A::ENABLED)
    }
}
#[doc = "Field `SALT` reader - SMBus alert"]
pub type SALT_R = crate::BitReader<SALT_A>;
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SALT_A {
        match self.bits {
            false => SALT_A::RELEASE,
            true => SALT_A::DRIVE,
        }
    }
    #[doc = "SMBA pin released high"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == SALT_A::RELEASE
    }
    #[doc = "SMBA pin driven low"]
    #[inline(always)]
    pub fn is_drive(&self) -> bool {
        *self == SALT_A::DRIVE
    }
}
#[doc = "Field `SALT` writer - SMBus alert"]
pub type SALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SALT_A>;
impl<'a, REG, const O: u8> SALT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBA pin released high"]
    #[inline(always)]
    pub fn release(self) -> &'a mut crate::W<REG> {
        self.variant(SALT_A::RELEASE)
    }
    #[doc = "SMBA pin driven low"]
    #[inline(always)]
    pub fn drive(self) -> &'a mut crate::W<REG> {
        self.variant(SALT_A::DRIVE)
    }
}
#[doc = "Field `SRESET` reader - Software reset"]
pub type SRESET_R = crate::BitReader<SRESET_A>;
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRESET_A {
    #[doc = "0: I2C peripheral not under reset"]
    NOT_RESET = 0,
    #[doc = "1: I2C peripheral under reset"]
    RESET = 1,
}
impl From<SRESET_A> for bool {
    #[inline(always)]
    fn from(variant: SRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl SRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRESET_A {
        match self.bits {
            false => SRESET_A::NOT_RESET,
            true => SRESET_A::RESET,
        }
    }
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SRESET_A::NOT_RESET
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SRESET_A::RESET
    }
}
#[doc = "Field `SRESET` writer - Software reset"]
pub type SRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRESET_A>;
impl<'a, REG, const O: u8> SRESET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SRESET_A::NOT_RESET)
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SRESET_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smben(&self) -> SMBEN_R {
        SMBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbsel(&self) -> SMBSEL_R {
        SMBSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCL Stretching(Slave mode)"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop condition"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn acken(&self) -> ACKEN_R {
        ACKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn poap(&self) -> POAP_R {
        POAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pectrans(&self) -> PECTRANS_R {
        PECTRANS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn salt(&self) -> SALT_R {
        SALT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn sreset(&self) -> SRESET_R {
        SRESET_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<CTL0_SPEC, 0> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    #[must_use]
    pub fn smben(&mut self) -> SMBEN_W<CTL0_SPEC, 1> {
        SMBEN_W::new(self)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    #[must_use]
    pub fn smbsel(&mut self) -> SMBSEL_W<CTL0_SPEC, 3> {
        SMBSEL_W::new(self)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<CTL0_SPEC, 4> {
        ARPEN_W::new(self)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CTL0_SPEC, 5> {
        PECEN_W::new(self)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<CTL0_SPEC, 6> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 7 - SCL Stretching(Slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<CTL0_SPEC, 7> {
        SS_W::new(self)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTL0_SPEC, 8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - Stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CTL0_SPEC, 9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn acken(&mut self) -> ACKEN_W<CTL0_SPEC, 10> {
        ACKEN_W::new(self)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    #[must_use]
    pub fn poap(&mut self) -> POAP_W<CTL0_SPEC, 11> {
        POAP_W::new(self)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    #[must_use]
    pub fn pectrans(&mut self) -> PECTRANS_W<CTL0_SPEC, 12> {
        PECTRANS_W::new(self)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn salt(&mut self) -> SALT_W<CTL0_SPEC, 13> {
        SALT_W::new(self)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sreset(&mut self) -> SRESET_W<CTL0_SPEC, 15> {
        SRESET_W::new(self)
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
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
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
