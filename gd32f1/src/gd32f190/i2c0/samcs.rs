#[doc = "Register `SAMCS` reader"]
pub type R = crate::R<SAMCS_SPEC>;
#[doc = "Register `SAMCS` writer"]
pub type W = crate::W<SAMCS_SPEC>;
#[doc = "Field `SAMEN` reader - SAM_V interface enable"]
pub type SAMEN_R = crate::BitReader<SAMEN_A>;
#[doc = "SAM_V interface enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMEN_A {
    #[doc = "0: SAM_V interface disabled"]
    DISABLED = 0,
    #[doc = "1: SAM_V interface enabled"]
    ENABLED = 1,
}
impl From<SAMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMEN_A {
        match self.bits {
            false => SAMEN_A::DISABLED,
            true => SAMEN_A::ENABLED,
        }
    }
    #[doc = "SAM_V interface disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAMEN_A::DISABLED
    }
    #[doc = "SAM_V interface enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAMEN_A::ENABLED
    }
}
#[doc = "Field `SAMEN` writer - SAM_V interface enable"]
pub type SAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SAMEN_A>;
impl<'a, REG, const O: u8> SAMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAM_V interface disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAMEN_A::DISABLED)
    }
    #[doc = "SAM_V interface enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAMEN_A::ENABLED)
    }
}
#[doc = "Field `STOEN` reader - SAM_V interface timeout detect enable"]
pub type STOEN_R = crate::BitReader<STOEN_A>;
#[doc = "SAM_V interface timeout detect enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOEN_A {
    #[doc = "0: SAM_V interface timeout detect disabled"]
    DISABLED = 0,
    #[doc = "1: SAM_V interface timeout detect enabled"]
    ENABLED = 1,
}
impl From<STOEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOEN_A {
        match self.bits {
            false => STOEN_A::DISABLED,
            true => STOEN_A::ENABLED,
        }
    }
    #[doc = "SAM_V interface timeout detect disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOEN_A::DISABLED
    }
    #[doc = "SAM_V interface timeout detect enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOEN_A::ENABLED
    }
}
#[doc = "Field `STOEN` writer - SAM_V interface timeout detect enable"]
pub type STOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STOEN_A>;
impl<'a, REG, const O: u8> STOEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAM_V interface timeout detect disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOEN_A::DISABLED)
    }
    #[doc = "SAM_V interface timeout detect enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOEN_A::ENABLED)
    }
}
#[doc = "Field `TFFIE` reader - Txframe fall interrupt enable"]
pub type TFFIE_R = crate::BitReader<TFFIE_A>;
#[doc = "Txframe fall interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFFIE_A {
    #[doc = "0: Txframe fall interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Txframe fall interrupt enabled"]
    ENABLED = 1,
}
impl From<TFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TFFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFIE_A {
        match self.bits {
            false => TFFIE_A::DISABLED,
            true => TFFIE_A::ENABLED,
        }
    }
    #[doc = "Txframe fall interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFFIE_A::DISABLED
    }
    #[doc = "Txframe fall interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFFIE_A::ENABLED
    }
}
#[doc = "Field `TFFIE` writer - Txframe fall interrupt enable"]
pub type TFFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TFFIE_A>;
impl<'a, REG, const O: u8> TFFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Txframe fall interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFFIE_A::DISABLED)
    }
    #[doc = "Txframe fall interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFFIE_A::ENABLED)
    }
}
#[doc = "Field `TFRIE` reader - Txframe rise interrupt enable"]
pub type TFRIE_R = crate::BitReader<TFRIE_A>;
#[doc = "Txframe rise interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRIE_A {
    #[doc = "0: Txframe rise interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Txframe rise interrupt enabled"]
    ENABLED = 1,
}
impl From<TFRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TFRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFRIE_A {
        match self.bits {
            false => TFRIE_A::DISABLED,
            true => TFRIE_A::ENABLED,
        }
    }
    #[doc = "Txframe rise interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFRIE_A::DISABLED
    }
    #[doc = "Txframe rise interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFRIE_A::ENABLED
    }
}
#[doc = "Field `TFRIE` writer - Txframe rise interrupt enable"]
pub type TFRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TFRIE_A>;
impl<'a, REG, const O: u8> TFRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Txframe rise interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFRIE_A::DISABLED)
    }
    #[doc = "Txframe rise interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFRIE_A::ENABLED)
    }
}
#[doc = "Field `RFFIE` reader - Rxframe fall interrupt enable"]
pub type RFFIE_R = crate::BitReader<RFFIE_A>;
#[doc = "Rxframe fall interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFFIE_A {
    #[doc = "0: Rxframe fall interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Rxframe fall interrupt enabled"]
    ENABLED = 1,
}
impl From<RFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RFFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFFIE_A {
        match self.bits {
            false => RFFIE_A::DISABLED,
            true => RFFIE_A::ENABLED,
        }
    }
    #[doc = "Rxframe fall interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFFIE_A::DISABLED
    }
    #[doc = "Rxframe fall interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFFIE_A::ENABLED
    }
}
#[doc = "Field `RFFIE` writer - Rxframe fall interrupt enable"]
pub type RFFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFFIE_A>;
impl<'a, REG, const O: u8> RFFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rxframe fall interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFFIE_A::DISABLED)
    }
    #[doc = "Rxframe fall interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFFIE_A::ENABLED)
    }
}
#[doc = "Field `RFRIE` reader - Rxframe rise interrupt enable"]
pub type RFRIE_R = crate::BitReader<RFRIE_A>;
#[doc = "Rxframe rise interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRIE_A {
    #[doc = "0: Rxframe rise interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Rxframe rise interrupt enabled"]
    ENABLED = 1,
}
impl From<RFRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RFRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRIE_A {
        match self.bits {
            false => RFRIE_A::DISABLED,
            true => RFRIE_A::ENABLED,
        }
    }
    #[doc = "Rxframe rise interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFRIE_A::DISABLED
    }
    #[doc = "Rxframe rise interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFRIE_A::ENABLED
    }
}
#[doc = "Field `RFRIE` writer - Rxframe rise interrupt enable"]
pub type RFRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFRIE_A>;
impl<'a, REG, const O: u8> RFRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rxframe rise interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFRIE_A::DISABLED)
    }
    #[doc = "Rxframe rise interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RFRIE_A::ENABLED)
    }
}
#[doc = "Field `TXF` reader - Level of Txframe signal"]
pub type TXF_R = crate::BitReader;
#[doc = "Field `TXF` writer - Level of Txframe signal"]
pub type TXF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXF` reader - Level of Rxframe signal"]
pub type RXF_R = crate::BitReader;
#[doc = "Field `RXF` writer - Level of Rxframe signal"]
pub type RXF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFF` reader - Txframe fall flag"]
pub type TFF_R = crate::BitReader;
#[doc = "Field `TFF` writer - Txframe fall flag"]
pub type TFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFR` reader - Txframe rise flag"]
pub type TFR_R = crate::BitReader;
#[doc = "Field `TFR` writer - Txframe rise flag"]
pub type TFR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFF` reader - Rxframe fall flag"]
pub type RFF_R = crate::BitReader;
#[doc = "Field `RFF` writer - Rxframe fall flag"]
pub type RFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFR` reader - Rxframe rise flag"]
pub type RFR_R = crate::BitReader;
#[doc = "Field `RFR` writer - Rxframe rise flag"]
pub type RFR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    pub fn samen(&self) -> SAMEN_R {
        SAMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    pub fn stoen(&self) -> STOEN_R {
        STOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Txframe fall interrupt enable"]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Txframe rise interrupt enable"]
    #[inline(always)]
    pub fn tfrie(&self) -> TFRIE_R {
        TFRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rxframe fall interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rxframe rise interrupt enable"]
    #[inline(always)]
    pub fn rfrie(&self) -> RFRIE_R {
        RFRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of Txframe signal"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of Rxframe signal"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    pub fn rfr(&self) -> RFR_R {
        RFR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAM_V interface enable"]
    #[inline(always)]
    #[must_use]
    pub fn samen(&mut self) -> SAMEN_W<SAMCS_SPEC, 0> {
        SAMEN_W::new(self)
    }
    #[doc = "Bit 1 - SAM_V interface timeout detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn stoen(&mut self) -> STOEN_W<SAMCS_SPEC, 1> {
        STOEN_W::new(self)
    }
    #[doc = "Bit 4 - Txframe fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tffie(&mut self) -> TFFIE_W<SAMCS_SPEC, 4> {
        TFFIE_W::new(self)
    }
    #[doc = "Bit 5 - Txframe rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfrie(&mut self) -> TFRIE_W<SAMCS_SPEC, 5> {
        TFRIE_W::new(self)
    }
    #[doc = "Bit 6 - Rxframe fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie(&mut self) -> RFFIE_W<SAMCS_SPEC, 6> {
        RFFIE_W::new(self)
    }
    #[doc = "Bit 7 - Rxframe rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfrie(&mut self) -> RFRIE_W<SAMCS_SPEC, 7> {
        RFRIE_W::new(self)
    }
    #[doc = "Bit 8 - Level of Txframe signal"]
    #[inline(always)]
    #[must_use]
    pub fn txf(&mut self) -> TXF_W<SAMCS_SPEC, 8> {
        TXF_W::new(self)
    }
    #[doc = "Bit 9 - Level of Rxframe signal"]
    #[inline(always)]
    #[must_use]
    pub fn rxf(&mut self) -> RXF_W<SAMCS_SPEC, 9> {
        RXF_W::new(self)
    }
    #[doc = "Bit 12 - Txframe fall flag"]
    #[inline(always)]
    #[must_use]
    pub fn tff(&mut self) -> TFF_W<SAMCS_SPEC, 12> {
        TFF_W::new(self)
    }
    #[doc = "Bit 13 - Txframe rise flag"]
    #[inline(always)]
    #[must_use]
    pub fn tfr(&mut self) -> TFR_W<SAMCS_SPEC, 13> {
        TFR_W::new(self)
    }
    #[doc = "Bit 14 - Rxframe fall flag"]
    #[inline(always)]
    #[must_use]
    pub fn rff(&mut self) -> RFF_W<SAMCS_SPEC, 14> {
        RFF_W::new(self)
    }
    #[doc = "Bit 15 - Rxframe rise flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfr(&mut self) -> RFR_W<SAMCS_SPEC, 15> {
        RFR_W::new(self)
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
#[doc = "SAM Controland status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMCS_SPEC;
impl crate::RegisterSpec for SAMCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samcs::R`](R) reader structure"]
impl crate::Readable for SAMCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`samcs::W`](W) writer structure"]
impl crate::Writable for SAMCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMCS to value 0"]
impl crate::Resettable for SAMCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
