#[doc = "Register `I2SCTL` reader"]
pub type R = crate::R<I2SCTL_SPEC>;
#[doc = "Register `I2SCTL` writer"]
pub type W = crate::W<I2SCTL_SPEC>;
#[doc = "Field `CHLEN` reader - Channel length"]
pub type CHLEN_R = crate::BitReader<CHLEN_A>;
#[doc = "Channel length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN_A {
    #[doc = "0: 16-bit wide"]
    SIXTEEN_BIT = 0,
    #[doc = "1: 32-bit wide"]
    THIRTY_TWO_BIT = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::SIXTEEN_BIT,
            true => CHLEN_A::THIRTY_TWO_BIT,
        }
    }
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CHLEN_A::SIXTEEN_BIT
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == CHLEN_A::THIRTY_TWO_BIT
    }
}
#[doc = "Field `CHLEN` writer - Channel length"]
pub type CHLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHLEN_A>;
impl<'a, REG, const O: u8> CHLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN_A::SIXTEEN_BIT)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN_A::THIRTY_TWO_BIT)
    }
}
#[doc = "Field `DTLEN` reader - Data length"]
pub type DTLEN_R = crate::FieldReader<DTLEN_A>;
#[doc = "Data length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTLEN_A {
    #[doc = "0: 16-bit data length"]
    SIXTEEN_BIT = 0,
    #[doc = "1: 24-bit data length"]
    TWENTY_FOUR_BIT = 1,
    #[doc = "2: 32-bit data length"]
    THIRTY_TWO_BIT = 2,
}
impl From<DTLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DTLEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTLEN_A {
    type Ux = u8;
}
impl DTLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTLEN_A> {
        match self.bits {
            0 => Some(DTLEN_A::SIXTEEN_BIT),
            1 => Some(DTLEN_A::TWENTY_FOUR_BIT),
            2 => Some(DTLEN_A::THIRTY_TWO_BIT),
            _ => None,
        }
    }
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DTLEN_A::SIXTEEN_BIT
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn is_twenty_four_bit(&self) -> bool {
        *self == DTLEN_A::TWENTY_FOUR_BIT
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == DTLEN_A::THIRTY_TWO_BIT
    }
}
#[doc = "Field `DTLEN` writer - Data length"]
pub type DTLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DTLEN_A>;
impl<'a, REG, const O: u8> DTLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DTLEN_A::SIXTEEN_BIT)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn twenty_four_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DTLEN_A::TWENTY_FOUR_BIT)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DTLEN_A::THIRTY_TWO_BIT)
    }
}
#[doc = "Field `CKPL` reader - Idle state clock polarity"]
pub type CKPL_R = crate::BitReader<CKPL_A>;
#[doc = "Idle state clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPL_A {
    #[doc = "0: I2S clock inactive state is low level"]
    IDLE_LOW = 0,
    #[doc = "1: I2S clock inactive state is high level"]
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
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPL_A::IDLE_LOW
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPL_A::IDLE_HIGH
    }
}
#[doc = "Field `CKPL` writer - Idle state clock polarity"]
pub type CKPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CKPL_A>;
impl<'a, REG, const O: u8> CKPL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CKPL_A::IDLE_LOW)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(CKPL_A::IDLE_HIGH)
    }
}
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2SSTD_R = crate::FieldReader<I2SSTD_A>;
#[doc = "I2S standard selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD_A {
    #[doc = "0: I2S Philips standard"]
    PHILIPS = 0,
    #[doc = "1: MSB justified standard"]
    MSB = 1,
    #[doc = "2: LSB justified standard"]
    LSB = 2,
    #[doc = "3: PCM standard"]
    PCM = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SSTD_A {
    type Ux = u8;
}
impl I2SSTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::PHILIPS,
            1 => I2SSTD_A::MSB,
            2 => I2SSTD_A::LSB,
            3 => I2SSTD_A::PCM,
            _ => unreachable!(),
        }
    }
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD_A::PHILIPS
    }
    #[doc = "MSB justified standard"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == I2SSTD_A::MSB
    }
    #[doc = "LSB justified standard"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == I2SSTD_A::LSB
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD_A::PCM
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2SSTD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, I2SSTD_A>;
impl<'a, REG, const O: u8> I2SSTD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn philips(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::PHILIPS)
    }
    #[doc = "MSB justified standard"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::MSB)
    }
    #[doc = "LSB justified standard"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::LSB)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::PCM)
    }
}
#[doc = "Field `PCMSMOD` reader - PCM frame synchronization mode"]
pub type PCMSMOD_R = crate::BitReader<PCMSMOD_A>;
#[doc = "PCM frame synchronization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSMOD_A {
    #[doc = "0: Short frame synchronisation"]
    SHORT = 0,
    #[doc = "1: Long frame synchronisation"]
    LONG = 1,
}
impl From<PCMSMOD_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl PCMSMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCMSMOD_A {
        match self.bits {
            false => PCMSMOD_A::SHORT,
            true => PCMSMOD_A::LONG,
        }
    }
    #[doc = "Short frame synchronisation"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSMOD_A::SHORT
    }
    #[doc = "Long frame synchronisation"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSMOD_A::LONG
    }
}
#[doc = "Field `PCMSMOD` writer - PCM frame synchronization mode"]
pub type PCMSMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PCMSMOD_A>;
impl<'a, REG, const O: u8> PCMSMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short frame synchronisation"]
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSMOD_A::SHORT)
    }
    #[doc = "Long frame synchronisation"]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSMOD_A::LONG)
    }
}
#[doc = "Field `I2SOPMOD` reader - I2S configuration mode"]
pub type I2SOPMOD_R = crate::FieldReader<I2SOPMOD_A>;
#[doc = "I2S configuration mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SOPMOD_A {
    #[doc = "0: Slave - transmit"]
    SLAVE_TX = 0,
    #[doc = "1: Slave - receive"]
    SLAVE_RX = 1,
    #[doc = "2: Master - transmit"]
    MASTER_TX = 2,
    #[doc = "3: Master - receive"]
    MASTER_RX = 3,
}
impl From<I2SOPMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SOPMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SOPMOD_A {
    type Ux = u8;
}
impl I2SOPMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SOPMOD_A {
        match self.bits {
            0 => I2SOPMOD_A::SLAVE_TX,
            1 => I2SOPMOD_A::SLAVE_RX,
            2 => I2SOPMOD_A::MASTER_TX,
            3 => I2SOPMOD_A::MASTER_RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == I2SOPMOD_A::SLAVE_TX
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == I2SOPMOD_A::SLAVE_RX
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == I2SOPMOD_A::MASTER_TX
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == I2SOPMOD_A::MASTER_RX
    }
}
#[doc = "Field `I2SOPMOD` writer - I2S configuration mode"]
pub type I2SOPMOD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, I2SOPMOD_A>;
impl<'a, REG, const O: u8> I2SOPMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SOPMOD_A::SLAVE_TX)
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SOPMOD_A::SLAVE_RX)
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SOPMOD_A::MASTER_TX)
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut crate::W<REG> {
        self.variant(I2SOPMOD_A::MASTER_RX)
    }
}
#[doc = "Field `I2SEN` reader - I2S Enable"]
pub type I2SEN_R = crate::BitReader<I2SEN_A>;
#[doc = "I2S Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SEN_A {
    #[doc = "0: I2S peripheral is disabled"]
    DISABLED = 0,
    #[doc = "1: I2S peripheral is enabled"]
    ENABLED = 1,
}
impl From<I2SEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2SEN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SEN_A {
        match self.bits {
            false => I2SEN_A::DISABLED,
            true => I2SEN_A::ENABLED,
        }
    }
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2SEN_A::DISABLED
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2SEN_A::ENABLED
    }
}
#[doc = "Field `I2SEN` writer - I2S Enable"]
pub type I2SEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2SEN_A>;
impl<'a, REG, const O: u8> I2SEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2SEN_A::DISABLED)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2SEN_A::ENABLED)
    }
}
#[doc = "Field `I2SSEL` reader - I2S mode selection"]
pub type I2SSEL_R = crate::BitReader<I2SSEL_A>;
#[doc = "I2S mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SSEL_A {
    #[doc = "0: SPI mode is selected"]
    SPIMODE = 0,
    #[doc = "1: I2S mode is selected"]
    I2SMODE = 1,
}
impl From<I2SSEL_A> for bool {
    #[inline(always)]
    fn from(variant: I2SSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSEL_A {
        match self.bits {
            false => I2SSEL_A::SPIMODE,
            true => I2SSEL_A::I2SMODE,
        }
    }
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn is_spimode(&self) -> bool {
        *self == I2SSEL_A::SPIMODE
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn is_i2smode(&self) -> bool {
        *self == I2SSEL_A::I2SMODE
    }
}
#[doc = "Field `I2SSEL` writer - I2S mode selection"]
pub type I2SSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2SSEL_A>;
impl<'a, REG, const O: u8> I2SSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn spimode(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSEL_A::SPIMODE)
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn i2smode(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSEL_A::I2SMODE)
    }
}
impl R {
    #[doc = "Bit 0 - Channel length"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length"]
    #[inline(always)]
    pub fn dtlen(&self) -> DTLEN_R {
        DTLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    pub fn ckpl(&self) -> CKPL_R {
        CKPL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization mode"]
    #[inline(always)]
    pub fn pcmsmod(&self) -> PCMSMOD_R {
        PCMSMOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2sopmod(&self) -> I2SOPMOD_R {
        I2SOPMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2ssel(&self) -> I2SSEL_R {
        I2SSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<I2SCTL_SPEC, 0> {
        CHLEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Data length"]
    #[inline(always)]
    #[must_use]
    pub fn dtlen(&mut self) -> DTLEN_W<I2SCTL_SPEC, 1> {
        DTLEN_W::new(self)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpl(&mut self) -> CKPL_W<I2SCTL_SPEC, 3> {
        CKPL_W::new(self)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<I2SCTL_SPEC, 4> {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 7 - PCM frame synchronization mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsmod(&mut self) -> PCMSMOD_W<I2SCTL_SPEC, 7> {
        PCMSMOD_W::new(self)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2sopmod(&mut self) -> I2SOPMOD_W<I2SCTL_SPEC, 8> {
        I2SOPMOD_W::new(self)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2sen(&mut self) -> I2SEN_W<I2SCTL_SPEC, 10> {
        I2SEN_W::new(self)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2ssel(&mut self) -> I2SSEL_W<I2SCTL_SPEC, 11> {
        I2SSEL_W::new(self)
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
#[doc = "I2S control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCTL_SPEC;
impl crate::RegisterSpec for I2SCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`i2sctl::R`](R) reader structure"]
impl crate::Readable for I2SCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sctl::W`](W) writer structure"]
impl crate::Writable for I2SCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCTL to value 0"]
impl crate::Resettable for I2SCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
