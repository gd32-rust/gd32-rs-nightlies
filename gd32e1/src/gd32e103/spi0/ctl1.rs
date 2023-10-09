#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `DMAREN` reader - Rx buffer DMA enable"]
pub type DMAREN_R = crate::BitReader<DMAREN_A>;
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DMAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAREN_A {
        match self.bits {
            false => DMAREN_A::DISABLED,
            true => DMAREN_A::ENABLED,
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAREN_A::DISABLED
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAREN_A::ENABLED
    }
}
#[doc = "Field `DMAREN` writer - Rx buffer DMA enable"]
pub type DMAREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMAREN_A>;
impl<'a, REG, const O: u8> DMAREN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREN_A::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREN_A::ENABLED)
    }
}
#[doc = "Field `DMATEN` reader - Transmit Buffer DMA Enable"]
pub type DMATEN_R = crate::BitReader<DMATEN_A>;
#[doc = "Transmit Buffer DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DMATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMATEN_A {
        match self.bits {
            false => DMATEN_A::DISABLED,
            true => DMATEN_A::ENABLED,
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATEN_A::DISABLED
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATEN_A::ENABLED
    }
}
#[doc = "Field `DMATEN` writer - Transmit Buffer DMA Enable"]
pub type DMATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMATEN_A>;
impl<'a, REG, const O: u8> DMATEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMATEN_A::DISABLED)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMATEN_A::ENABLED)
    }
}
#[doc = "Field `NSSDRV` reader - Drive NSS Output"]
pub type NSSDRV_R = crate::BitReader<NSSDRV_A>;
#[doc = "Drive NSS Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl NSSDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSDRV_A {
        match self.bits {
            false => NSSDRV_A::DISABLED,
            true => NSSDRV_A::ENABLED,
        }
    }
    #[doc = "NSS output is disabled in master mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NSSDRV_A::DISABLED
    }
    #[doc = "NSS output is enabled in master mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NSSDRV_A::ENABLED
    }
}
#[doc = "Field `NSSDRV` writer - Drive NSS Output"]
pub type NSSDRV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NSSDRV_A>;
impl<'a, REG, const O: u8> NSSDRV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NSS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NSSDRV_A::DISABLED)
    }
    #[doc = "NSS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NSSDRV_A::ENABLED)
    }
}
#[doc = "Field `NSSP` reader - SPI NSS pulse mode Enable"]
pub type NSSP_R = crate::BitReader<NSSP_A>;
#[doc = "SPI NSS pulse mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSP_A {
    #[doc = "0: NSSP Mode disabled"]
    DISABLED = 0,
    #[doc = "1: NSSP Mode enabled"]
    ENABLED = 1,
}
impl From<NSSP_A> for bool {
    #[inline(always)]
    fn from(variant: NSSP_A) -> Self {
        variant as u8 != 0
    }
}
impl NSSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSP_A {
        match self.bits {
            false => NSSP_A::DISABLED,
            true => NSSP_A::ENABLED,
        }
    }
    #[doc = "NSSP Mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NSSP_A::DISABLED
    }
    #[doc = "NSSP Mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NSSP_A::ENABLED
    }
}
#[doc = "Field `NSSP` writer - SPI NSS pulse mode Enable"]
pub type NSSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NSSP_A>;
impl<'a, REG, const O: u8> NSSP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NSSP Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP_A::DISABLED)
    }
    #[doc = "NSSP Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP_A::ENABLED)
    }
}
#[doc = "Field `TMOD` reader - SPI TI Mode Enable"]
pub type TMOD_R = crate::BitReader<TMOD_A>;
#[doc = "SPI TI Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOD_A {
    #[doc = "0: SPI TI Mode disabled"]
    DISABLED = 0,
    #[doc = "1: SPI TI Mode enabled"]
    ENABLED = 1,
}
impl From<TMOD_A> for bool {
    #[inline(always)]
    fn from(variant: TMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMOD_A {
        match self.bits {
            false => TMOD_A::DISABLED,
            true => TMOD_A::ENABLED,
        }
    }
    #[doc = "SPI TI Mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TMOD_A::DISABLED
    }
    #[doc = "SPI TI Mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TMOD_A::ENABLED
    }
}
#[doc = "Field `TMOD` writer - SPI TI Mode Enable"]
pub type TMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMOD_A>;
impl<'a, REG, const O: u8> TMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI TI Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::DISABLED)
    }
    #[doc = "SPI TI Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::ENABLED)
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIE_A>;
impl<'a, REG, const O: u8> ERRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Field `RBNEIE` reader - RX buffer not empty interrupt enable"]
pub type RBNEIE_R = crate::BitReader<RBNEIE_A>;
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RBNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBNEIE_A {
        match self.bits {
            false => RBNEIE_A::DISABLED,
            true => RBNEIE_A::ENABLED,
        }
    }
    #[doc = "RBNE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RBNEIE_A::DISABLED
    }
    #[doc = "RBNE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RBNEIE_A::ENABLED
    }
}
#[doc = "Field `RBNEIE` writer - RX buffer not empty interrupt enable"]
pub type RBNEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RBNEIE_A>;
impl<'a, REG, const O: u8> RBNEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RBNE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RBNEIE_A::DISABLED)
    }
    #[doc = "RBNE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RBNEIE_A::ENABLED)
    }
}
#[doc = "Field `TBEIE` reader - Tx buffer empty interrupt enable"]
pub type TBEIE_R = crate::BitReader<TBEIE_A>;
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TBEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBEIE_A {
        match self.bits {
            false => TBEIE_A::DISABLED,
            true => TBEIE_A::ENABLED,
        }
    }
    #[doc = "TBE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBEIE_A::DISABLED
    }
    #[doc = "TBE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBEIE_A::ENABLED
    }
}
#[doc = "Field `TBEIE` writer - Tx buffer empty interrupt enable"]
pub type TBEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TBEIE_A>;
impl<'a, REG, const O: u8> TBEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TBE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBEIE_A::DISABLED)
    }
    #[doc = "TBE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBEIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer DMA Enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive NSS Output"]
    #[inline(always)]
    pub fn nssdrv(&self) -> NSSDRV_R {
        NSSDRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI NSS pulse mode Enable"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTL1_SPEC, 0> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Buffer DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTL1_SPEC, 1> {
        DMATEN_W::new(self)
    }
    #[doc = "Bit 2 - Drive NSS Output"]
    #[inline(always)]
    #[must_use]
    pub fn nssdrv(&mut self) -> NSSDRV_W<CTL1_SPEC, 2> {
        NSSDRV_W::new(self)
    }
    #[doc = "Bit 3 - SPI NSS pulse mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NSSP_W<CTL1_SPEC, 3> {
        NSSP_W::new(self)
    }
    #[doc = "Bit 4 - SPI TI Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TMOD_W<CTL1_SPEC, 4> {
        TMOD_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL1_SPEC, 5> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbneie(&mut self) -> RBNEIE_W<CTL1_SPEC, 6> {
        RBNEIE_W::new(self)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbeie(&mut self) -> TBEIE_W<CTL1_SPEC, 7> {
        TBEIE_W::new(self)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
