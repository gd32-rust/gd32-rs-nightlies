#[doc = "Register `I2SCTL` reader"]
pub struct R(crate::R<I2SCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCTL` writer"]
pub struct W(crate::W<I2SCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCTL_SPEC>;
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
impl From<crate::W<I2SCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2S mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `I2SSEL` reader - I2S mode selection"]
pub type I2SSEL_R = crate::BitReader<I2SSEL_A>;
impl I2SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSEL_A {
        match self.bits {
            false => I2SSEL_A::SPIMODE,
            true => I2SSEL_A::I2SMODE,
        }
    }
    #[doc = "Checks if the value of the field is `SPIMODE`"]
    #[inline(always)]
    pub fn is_spimode(&self) -> bool {
        *self == I2SSEL_A::SPIMODE
    }
    #[doc = "Checks if the value of the field is `I2SMODE`"]
    #[inline(always)]
    pub fn is_i2smode(&self) -> bool {
        *self == I2SSEL_A::I2SMODE
    }
}
#[doc = "Field `I2SSEL` writer - I2S mode selection"]
pub type I2SSEL_W<'a> = crate::BitWriter<'a, u16, I2SCTL_SPEC, I2SSEL_A, 11>;
impl<'a> I2SSEL_W<'a> {
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn spimode(self) -> &'a mut W {
        self.variant(I2SSEL_A::SPIMODE)
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn i2smode(self) -> &'a mut W {
        self.variant(I2SSEL_A::I2SMODE)
    }
}
#[doc = "I2S Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `I2SEN` reader - I2S Enable"]
pub type I2SEN_R = crate::BitReader<I2SEN_A>;
impl I2SEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SEN_A {
        match self.bits {
            false => I2SEN_A::DISABLED,
            true => I2SEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2SEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2SEN_A::ENABLED
    }
}
#[doc = "Field `I2SEN` writer - I2S Enable"]
pub type I2SEN_W<'a> = crate::BitWriter<'a, u16, I2SCTL_SPEC, I2SEN_A, 10>;
impl<'a> I2SEN_W<'a> {
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2SEN_A::DISABLED)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2SEN_A::ENABLED)
    }
}
#[doc = "I2S configuration mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SOPMOD_A {
    #[doc = "0: Slave - transmit"]
    SLAVETX = 0,
    #[doc = "1: Slave - receive"]
    SLAVERX = 1,
    #[doc = "2: Master - transmit"]
    MASTERTX = 2,
    #[doc = "3: Master - receive"]
    MASTERRX = 3,
}
impl From<I2SOPMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SOPMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2SOPMOD` reader - I2S configuration mode"]
pub type I2SOPMOD_R = crate::FieldReader<u8, I2SOPMOD_A>;
impl I2SOPMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SOPMOD_A {
        match self.bits {
            0 => I2SOPMOD_A::SLAVETX,
            1 => I2SOPMOD_A::SLAVERX,
            2 => I2SOPMOD_A::MASTERTX,
            3 => I2SOPMOD_A::MASTERRX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVETX`"]
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == I2SOPMOD_A::SLAVETX
    }
    #[doc = "Checks if the value of the field is `SLAVERX`"]
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == I2SOPMOD_A::SLAVERX
    }
    #[doc = "Checks if the value of the field is `MASTERTX`"]
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == I2SOPMOD_A::MASTERTX
    }
    #[doc = "Checks if the value of the field is `MASTERRX`"]
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == I2SOPMOD_A::MASTERRX
    }
}
#[doc = "Field `I2SOPMOD` writer - I2S configuration mode"]
pub type I2SOPMOD_W<'a> = crate::FieldWriterSafe<'a, u16, I2SCTL_SPEC, u8, I2SOPMOD_A, 2, 8>;
impl<'a> I2SOPMOD_W<'a> {
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut W {
        self.variant(I2SOPMOD_A::SLAVETX)
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut W {
        self.variant(I2SOPMOD_A::SLAVERX)
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut W {
        self.variant(I2SOPMOD_A::MASTERTX)
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut W {
        self.variant(I2SOPMOD_A::MASTERRX)
    }
}
#[doc = "PCM frame synchronization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PCMSMOD` reader - PCM frame synchronization mode"]
pub type PCMSMOD_R = crate::BitReader<PCMSMOD_A>;
impl PCMSMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCMSMOD_A {
        match self.bits {
            false => PCMSMOD_A::SHORT,
            true => PCMSMOD_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSMOD_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSMOD_A::LONG
    }
}
#[doc = "Field `PCMSMOD` writer - PCM frame synchronization mode"]
pub type PCMSMOD_W<'a> = crate::BitWriter<'a, u16, I2SCTL_SPEC, PCMSMOD_A, 7>;
impl<'a> PCMSMOD_W<'a> {
    #[doc = "Short frame synchronisation"]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSMOD_A::SHORT)
    }
    #[doc = "Long frame synchronisation"]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(PCMSMOD_A::LONG)
    }
}
#[doc = "I2S standard selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2SSTD_R = crate::FieldReader<u8, I2SSTD_A>;
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
    #[doc = "Checks if the value of the field is `PHILIPS`"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD_A::PHILIPS
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == I2SSTD_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == I2SSTD_A::LSB
    }
    #[doc = "Checks if the value of the field is `PCM`"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD_A::PCM
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2SSTD_W<'a> = crate::FieldWriterSafe<'a, u16, I2SCTL_SPEC, u8, I2SSTD_A, 2, 4>;
impl<'a> I2SSTD_W<'a> {
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn philips(self) -> &'a mut W {
        self.variant(I2SSTD_A::PHILIPS)
    }
    #[doc = "MSB justified standard"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(I2SSTD_A::MSB)
    }
    #[doc = "LSB justified standard"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(I2SSTD_A::LSB)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut W {
        self.variant(I2SSTD_A::PCM)
    }
}
#[doc = "Idle state clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPL_A {
    #[doc = "0: I2S clock inactive state is low level"]
    IDLELOW = 0,
    #[doc = "1: I2S clock inactive state is high level"]
    IDLEHIGH = 1,
}
impl From<CKPL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPL` reader - Idle state clock polarity"]
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
#[doc = "Field `CKPL` writer - Idle state clock polarity"]
pub type CKPL_W<'a> = crate::BitWriter<'a, u16, I2SCTL_SPEC, CKPL_A, 3>;
impl<'a> CKPL_W<'a> {
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CKPL_A::IDLELOW)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CKPL_A::IDLEHIGH)
    }
}
#[doc = "Data length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTLEN_A {
    #[doc = "0: 16-bit data length"]
    SIXTEENBIT = 0,
    #[doc = "1: 24-bit data length"]
    TWENTYFOURBIT = 1,
    #[doc = "2: 32-bit data length"]
    THIRTYTWOBIT = 2,
}
impl From<DTLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DTLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTLEN` reader - Data length"]
pub type DTLEN_R = crate::FieldReader<u8, DTLEN_A>;
impl DTLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTLEN_A> {
        match self.bits {
            0 => Some(DTLEN_A::SIXTEENBIT),
            1 => Some(DTLEN_A::TWENTYFOURBIT),
            2 => Some(DTLEN_A::THIRTYTWOBIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DTLEN_A::SIXTEENBIT
    }
    #[doc = "Checks if the value of the field is `TWENTYFOURBIT`"]
    #[inline(always)]
    pub fn is_twenty_four_bit(&self) -> bool {
        *self == DTLEN_A::TWENTYFOURBIT
    }
    #[doc = "Checks if the value of the field is `THIRTYTWOBIT`"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == DTLEN_A::THIRTYTWOBIT
    }
}
#[doc = "Field `DTLEN` writer - Data length"]
pub type DTLEN_W<'a> = crate::FieldWriter<'a, u16, I2SCTL_SPEC, u8, DTLEN_A, 2, 1>;
impl<'a> DTLEN_W<'a> {
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DTLEN_A::SIXTEENBIT)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn twenty_four_bit(self) -> &'a mut W {
        self.variant(DTLEN_A::TWENTYFOURBIT)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(DTLEN_A::THIRTYTWOBIT)
    }
}
#[doc = "Channel length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHLEN_A {
    #[doc = "0: 16-bit wide"]
    SIXTEENBIT = 0,
    #[doc = "1: 32-bit wide"]
    THIRTYTWOBIT = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHLEN` reader - Channel length"]
pub type CHLEN_R = crate::BitReader<CHLEN_A>;
impl CHLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::SIXTEENBIT,
            true => CHLEN_A::THIRTYTWOBIT,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CHLEN_A::SIXTEENBIT
    }
    #[doc = "Checks if the value of the field is `THIRTYTWOBIT`"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == CHLEN_A::THIRTYTWOBIT
    }
}
#[doc = "Field `CHLEN` writer - Channel length"]
pub type CHLEN_W<'a> = crate::BitWriter<'a, u16, I2SCTL_SPEC, CHLEN_A, 0>;
impl<'a> CHLEN_W<'a> {
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::SIXTEENBIT)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::THIRTYTWOBIT)
    }
}
impl R {
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2ssel(&self) -> I2SSEL_R {
        I2SSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2sopmod(&self) -> I2SOPMOD_R {
        I2SOPMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization mode"]
    #[inline(always)]
    pub fn pcmsmod(&self) -> PCMSMOD_R {
        PCMSMOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    pub fn ckpl(&self) -> CKPL_R {
        CKPL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length"]
    #[inline(always)]
    pub fn dtlen(&self) -> DTLEN_R {
        DTLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 0 - Channel length"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2ssel(&mut self) -> I2SSEL_W {
        I2SSEL_W::new(self)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2sen(&mut self) -> I2SEN_W {
        I2SEN_W::new(self)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2sopmod(&mut self) -> I2SOPMOD_W {
        I2SOPMOD_W::new(self)
    }
    #[doc = "Bit 7 - PCM frame synchronization mode"]
    #[inline(always)]
    pub fn pcmsmod(&mut self) -> PCMSMOD_W {
        PCMSMOD_W::new(self)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 3 - Idle state clock polarity"]
    #[inline(always)]
    pub fn ckpl(&mut self) -> CKPL_W {
        CKPL_W::new(self)
    }
    #[doc = "Bits 1:2 - Data length"]
    #[inline(always)]
    pub fn dtlen(&mut self) -> DTLEN_W {
        DTLEN_W::new(self)
    }
    #[doc = "Bit 0 - Channel length"]
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sctl](index.html) module"]
pub struct I2SCTL_SPEC;
impl crate::RegisterSpec for I2SCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2sctl::R](R) reader structure"]
impl crate::Readable for I2SCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sctl::W](W) writer structure"]
impl crate::Writable for I2SCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SCTL to value 0"]
impl crate::Resettable for I2SCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
