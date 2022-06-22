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
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBEIE_A {
    #[doc = "0: TBE interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: TBE interrupt enabled"]
    ENABLED = 1,
}
impl From<TBEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBEIE` reader - Tx buffer empty interrupt enable"]
pub type TBEIE_R = crate::BitReader<TBEIE_A>;
impl TBEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBEIE_A {
        match self.bits {
            false => TBEIE_A::DISABLED,
            true => TBEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBEIE_A::ENABLED
    }
}
#[doc = "Field `TBEIE` writer - Tx buffer empty interrupt enable"]
pub type TBEIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, TBEIE_A, 7>;
impl<'a> TBEIE_W<'a> {
    #[doc = "TBE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TBEIE_A::DISABLED)
    }
    #[doc = "TBE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TBEIE_A::ENABLED)
    }
}
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBNEIE_A {
    #[doc = "0: RBNE interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: RBNE interrupt enabled"]
    ENABLED = 1,
}
impl From<RBNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBNEIE` reader - RX buffer not empty interrupt enable"]
pub type RBNEIE_R = crate::BitReader<RBNEIE_A>;
impl RBNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBNEIE_A {
        match self.bits {
            false => RBNEIE_A::DISABLED,
            true => RBNEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RBNEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RBNEIE_A::ENABLED
    }
}
#[doc = "Field `RBNEIE` writer - RX buffer not empty interrupt enable"]
pub type RBNEIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, RBNEIE_A, 6>;
impl<'a> RBNEIE_W<'a> {
    #[doc = "RBNE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RBNEIE_A::DISABLED)
    }
    #[doc = "RBNE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RBNEIE_A::ENABLED)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Error interrupt enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, ERRIE_A, 5>;
impl<'a> ERRIE_W<'a> {
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Field `TMOD` reader - SPI TI Mode Enable"]
pub type TMOD_R = crate::BitReader<bool>;
#[doc = "Field `TMOD` writer - SPI TI Mode Enable"]
pub type TMOD_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 4>;
#[doc = "Field `NSSP` reader - SPI NSS pulse mode Enable"]
pub type NSSP_R = crate::BitReader<bool>;
#[doc = "Field `NSSP` writer - SPI NSS pulse mode Enable"]
pub type NSSP_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, 3>;
#[doc = "Drive NSS Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSDRV_A {
    #[doc = "0: NSS output is disabled in master mode"]
    DISABLED = 0,
    #[doc = "1: NSS output is enabled in master mode"]
    ENABLED = 1,
}
impl From<NSSDRV_A> for bool {
    #[inline(always)]
    fn from(variant: NSSDRV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSSDRV` reader - Drive NSS Output"]
pub type NSSDRV_R = crate::BitReader<NSSDRV_A>;
impl NSSDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSDRV_A {
        match self.bits {
            false => NSSDRV_A::DISABLED,
            true => NSSDRV_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NSSDRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NSSDRV_A::ENABLED
    }
}
#[doc = "Field `NSSDRV` writer - Drive NSS Output"]
pub type NSSDRV_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, NSSDRV_A, 2>;
impl<'a> NSSDRV_W<'a> {
    #[doc = "NSS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NSSDRV_A::DISABLED)
    }
    #[doc = "NSS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NSSDRV_A::ENABLED)
    }
}
#[doc = "Transmit Buffer DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATEN_A {
    #[doc = "0: Tx buffer DMA disabled"]
    DISABLED = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    ENABLED = 1,
}
impl From<DMATEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMATEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATEN` reader - Transmit Buffer DMA Enable"]
pub type DMATEN_R = crate::BitReader<DMATEN_A>;
impl DMATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMATEN_A {
        match self.bits {
            false => DMATEN_A::DISABLED,
            true => DMATEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATEN_A::ENABLED
    }
}
#[doc = "Field `DMATEN` writer - Transmit Buffer DMA Enable"]
pub type DMATEN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, DMATEN_A, 1>;
impl<'a> DMATEN_W<'a> {
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATEN_A::DISABLED)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATEN_A::ENABLED)
    }
}
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAREN_A {
    #[doc = "0: Rx buffer DMA disabled"]
    DISABLED = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    ENABLED = 1,
}
impl From<DMAREN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` reader - Rx buffer DMA enable"]
pub type DMAREN_R = crate::BitReader<DMAREN_A>;
impl DMAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAREN_A {
        match self.bits {
            false => DMAREN_A::DISABLED,
            true => DMAREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAREN_A::ENABLED
    }
}
#[doc = "Field `DMAREN` writer - Rx buffer DMA enable"]
pub type DMAREN_W<'a> = crate::BitWriter<'a, u32, CTL1_SPEC, DMAREN_A, 0>;
impl<'a> DMAREN_W<'a> {
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAREN_A::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAREN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI NSS pulse mode Enable"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive NSS Output"]
    #[inline(always)]
    pub fn nssdrv(&self) -> NSSDRV_R {
        NSSDRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer DMA Enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&mut self) -> TBEIE_W {
        TBEIE_W::new(self)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&mut self) -> RBNEIE_W {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W::new(self)
    }
    #[doc = "Bit 3 - SPI NSS pulse mode Enable"]
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W {
        NSSP_W::new(self)
    }
    #[doc = "Bit 2 - Drive NSS Output"]
    #[inline(always)]
    pub fn nssdrv(&mut self) -> NSSDRV_W {
        NSSDRV_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Buffer DMA Enable"]
    #[inline(always)]
    pub fn dmaten(&mut self) -> DMATEN_W {
        DMATEN_W::new(self)
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W {
        DMAREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
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
