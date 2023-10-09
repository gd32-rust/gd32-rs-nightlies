#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `PG` reader - Main flash program for bank0 command bit"]
pub type PG_R = crate::BitReader<PG_A>;
#[doc = "Main flash program for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG_A {
    #[doc = "1: Flash programming activated"]
    PROGRAM = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
impl PG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG_A> {
        match self.bits {
            true => Some(PG_A::PROGRAM),
            _ => None,
        }
    }
    #[doc = "Flash programming activated"]
    #[inline(always)]
    pub fn is_program(&self) -> bool {
        *self == PG_A::PROGRAM
    }
}
#[doc = "Field `PG` writer - Main flash program for bank0 command bit"]
pub type PG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PG_A>;
impl<'a, REG, const O: u8> PG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash programming activated"]
    #[inline(always)]
    pub fn program(self) -> &'a mut crate::W<REG> {
        self.variant(PG_A::PROGRAM)
    }
}
#[doc = "Field `PER` reader - Main flash page erase for bank0 command bit"]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "Main flash page erase for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "1: Erase activated for selected page"]
    PAGE_ERASE = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PER_A> {
        match self.bits {
            true => Some(PER_A::PAGE_ERASE),
            _ => None,
        }
    }
    #[doc = "Erase activated for selected page"]
    #[inline(always)]
    pub fn is_page_erase(&self) -> bool {
        *self == PER_A::PAGE_ERASE
    }
}
#[doc = "Field `PER` writer - Main flash page erase for bank0 command bit"]
pub type PER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PER_A>;
impl<'a, REG, const O: u8> PER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase activated for selected page"]
    #[inline(always)]
    pub fn page_erase(self) -> &'a mut crate::W<REG> {
        self.variant(PER_A::PAGE_ERASE)
    }
}
#[doc = "Field `MER` reader - Main flash mass erase for bank0 command bit"]
pub type MER_R = crate::BitReader<MER_A>;
#[doc = "Main flash mass erase for bank0 command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER_A {
    #[doc = "1: Erase activated for all user sectors"]
    MASS_ERASE = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
impl MER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MER_A> {
        match self.bits {
            true => Some(MER_A::MASS_ERASE),
            _ => None,
        }
    }
    #[doc = "Erase activated for all user sectors"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER_A::MASS_ERASE
    }
}
#[doc = "Field `MER` writer - Main flash mass erase for bank0 command bit"]
pub type MER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MER_A>;
impl<'a, REG, const O: u8> MER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase activated for all user sectors"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER_A::MASS_ERASE)
    }
}
#[doc = "Field `OBPG` reader - Option bytes program command bit"]
pub type OBPG_R = crate::BitReader<OBPG_A>;
#[doc = "Option bytes program command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBPG_A {
    #[doc = "1: Program option byte activated"]
    OPTION_BYTE_PROGRAMMING = 1,
}
impl From<OBPG_A> for bool {
    #[inline(always)]
    fn from(variant: OBPG_A) -> Self {
        variant as u8 != 0
    }
}
impl OBPG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OBPG_A> {
        match self.bits {
            true => Some(OBPG_A::OPTION_BYTE_PROGRAMMING),
            _ => None,
        }
    }
    #[doc = "Program option byte activated"]
    #[inline(always)]
    pub fn is_option_byte_programming(&self) -> bool {
        *self == OBPG_A::OPTION_BYTE_PROGRAMMING
    }
}
#[doc = "Field `OBPG` writer - Option bytes program command bit"]
pub type OBPG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OBPG_A>;
impl<'a, REG, const O: u8> OBPG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Program option byte activated"]
    #[inline(always)]
    pub fn option_byte_programming(self) -> &'a mut crate::W<REG> {
        self.variant(OBPG_A::OPTION_BYTE_PROGRAMMING)
    }
}
#[doc = "Field `OBER` reader - Option bytes erase command bit"]
pub type OBER_R = crate::BitReader<OBER_A>;
#[doc = "Option bytes erase command bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBER_A {
    #[doc = "1: Erase option byte activated"]
    OPTION_BYTE_ERASE = 1,
}
impl From<OBER_A> for bool {
    #[inline(always)]
    fn from(variant: OBER_A) -> Self {
        variant as u8 != 0
    }
}
impl OBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OBER_A> {
        match self.bits {
            true => Some(OBER_A::OPTION_BYTE_ERASE),
            _ => None,
        }
    }
    #[doc = "Erase option byte activated"]
    #[inline(always)]
    pub fn is_option_byte_erase(&self) -> bool {
        *self == OBER_A::OPTION_BYTE_ERASE
    }
}
#[doc = "Field `OBER` writer - Option bytes erase command bit"]
pub type OBER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OBER_A>;
impl<'a, REG, const O: u8> OBER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase option byte activated"]
    #[inline(always)]
    pub fn option_byte_erase(self) -> &'a mut crate::W<REG> {
        self.variant(OBER_A::OPTION_BYTE_ERASE)
    }
}
#[doc = "Field `START` reader - Send erase command to FMC bit"]
pub type START_R = crate::BitReader<STARTW_A>;
#[doc = "Send erase command to FMC bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW_A {
    #[doc = "1: Trigger an erase operation"]
    START = 1,
}
impl From<STARTW_A> for bool {
    #[inline(always)]
    fn from(variant: STARTW_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTW_A> {
        match self.bits {
            true => Some(STARTW_A::START),
            _ => None,
        }
    }
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTW_A::START
    }
}
#[doc = "Field `START` writer - Send erase command to FMC bit"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STARTW_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger an erase operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STARTW_A::START)
    }
}
#[doc = "Field `LK` reader - FMC_CTL0 lock bit"]
pub type LK_R = crate::BitReader<LKR_A>;
#[doc = "FMC_CTL0 lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LKR_A {
    #[doc = "0: CTL register is unlocked"]
    UNLOCKED = 0,
    #[doc = "1: CTL register is locked"]
    LOCKED = 1,
}
impl From<LKR_A> for bool {
    #[inline(always)]
    fn from(variant: LKR_A) -> Self {
        variant as u8 != 0
    }
}
impl LK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LKR_A {
        match self.bits {
            false => LKR_A::UNLOCKED,
            true => LKR_A::LOCKED,
        }
    }
    #[doc = "CTL register is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LKR_A::UNLOCKED
    }
    #[doc = "CTL register is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LKR_A::LOCKED
    }
}
#[doc = "FMC_CTL0 lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LKW_AW {
    #[doc = "1: Lock CTL register"]
    LOCK = 1,
}
impl From<LKW_AW> for bool {
    #[inline(always)]
    fn from(variant: LKW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` writer - FMC_CTL0 lock bit"]
pub type LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LKW_AW>;
impl<'a, REG, const O: u8> LK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock CTL register"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(LKW_AW::LOCK)
    }
}
#[doc = "Field `OBWEN` reader - Option byte erase/program enable bit"]
pub type OBWEN_R = crate::BitReader<OBWENR_A>;
#[doc = "Option byte erase/program enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBWENR_A {
    #[doc = "0: Option byte write disabled"]
    DISABLED = 0,
    #[doc = "1: Option byte write enabled"]
    ENABLED = 1,
}
impl From<OBWENR_A> for bool {
    #[inline(always)]
    fn from(variant: OBWENR_A) -> Self {
        variant as u8 != 0
    }
}
impl OBWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBWENR_A {
        match self.bits {
            false => OBWENR_A::DISABLED,
            true => OBWENR_A::ENABLED,
        }
    }
    #[doc = "Option byte write disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OBWENR_A::DISABLED
    }
    #[doc = "Option byte write enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OBWENR_A::ENABLED
    }
}
#[doc = "Option byte erase/program enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBWENW_AW {
    #[doc = "0: Disable option byte write"]
    DISABLE = 0,
}
impl From<OBWENW_AW> for bool {
    #[inline(always)]
    fn from(variant: OBWENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBWEN` writer - Option byte erase/program enable bit"]
pub type OBWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OBWENW_AW>;
impl<'a, REG, const O: u8> OBWEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable option byte write"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OBWENW_AW::DISABLE)
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable bit"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
#[doc = "Error interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt generation disabled"]
    DISABLED = 0,
    #[doc = "1: Error interrupt generation enabled"]
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
    #[doc = "Error interrupt generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Error interrupt generation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable bit"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIE_A>;
impl<'a, REG, const O: u8> ERRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Field `ENDIE` reader - End of operation interrupt enable bit"]
pub type ENDIE_R = crate::BitReader<ENDIE_A>;
#[doc = "End of operation interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIE_A {
    #[doc = "0: End of operation interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: End of operation interrupt enabled"]
    ENABLED = 1,
}
impl From<ENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIE_A {
        match self.bits {
            false => ENDIE_A::DISABLED,
            true => ENDIE_A::ENABLED,
        }
    }
    #[doc = "End of operation interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDIE_A::DISABLED
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDIE_A::ENABLED
    }
}
#[doc = "Field `ENDIE` writer - End of operation interrupt enable bit"]
pub type ENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENDIE_A>;
impl<'a, REG, const O: u8> ENDIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of operation interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIE_A::DISABLED)
    }
    #[doc = "End of operation interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Option bytes program command bit"]
    #[inline(always)]
    pub fn obpg(&self) -> OBPG_R {
        OBPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Option bytes erase command bit"]
    #[inline(always)]
    pub fn ober(&self) -> OBER_R {
        OBER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Option byte erase/program enable bit"]
    #[inline(always)]
    pub fn obwen(&self) -> OBWEN_R {
        OBWEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&self) -> ENDIE_R {
        ENDIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<CTL0_SPEC, 0> {
        PG_W::new(self)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<CTL0_SPEC, 1> {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<CTL0_SPEC, 2> {
        MER_W::new(self)
    }
    #[doc = "Bit 4 - Option bytes program command bit"]
    #[inline(always)]
    #[must_use]
    pub fn obpg(&mut self) -> OBPG_W<CTL0_SPEC, 4> {
        OBPG_W::new(self)
    }
    #[doc = "Bit 5 - Option bytes erase command bit"]
    #[inline(always)]
    #[must_use]
    pub fn ober(&mut self) -> OBER_W<CTL0_SPEC, 5> {
        OBER_W::new(self)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTL0_SPEC, 6> {
        START_W::new(self)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lk(&mut self) -> LK_W<CTL0_SPEC, 7> {
        LK_W::new(self)
    }
    #[doc = "Bit 9 - Option byte erase/program enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn obwen(&mut self) -> OBWEN_W<CTL0_SPEC, 9> {
        OBWEN_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL0_SPEC, 10> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn endie(&mut self) -> ENDIE_W<CTL0_SPEC, 12> {
        ENDIE_W::new(self)
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
#[doc = "`reset()` method sets CTL0 to value 0x80"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
