#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `BUSY` reader - The flash is busy bit"]
pub type BUSY_R = crate::BitReader<BUSYR_A>;
#[doc = "The flash is busy bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSYR_A {
    #[doc = "0: No operation is in progress"]
    INACTIVE = 0,
    #[doc = "1: An operation is in progress"]
    ACTIVE = 1,
}
impl From<BUSYR_A> for bool {
    #[inline(always)]
    fn from(variant: BUSYR_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSYR_A {
        match self.bits {
            false => BUSYR_A::INACTIVE,
            true => BUSYR_A::ACTIVE,
        }
    }
    #[doc = "No operation is in progress"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BUSYR_A::INACTIVE
    }
    #[doc = "An operation is in progress"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BUSYR_A::ACTIVE
    }
}
#[doc = "Field `PGERR` reader - Program error flag bit"]
pub type PGERR_R = crate::BitReader<PGERRR_A>;
#[doc = "Program error flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERRR_A {
    #[doc = "0: There was no error"]
    NO_ERROR = 0,
    #[doc = "1: There was an error programming flash"]
    ERROR = 1,
}
impl From<PGERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PGERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGERRR_A {
        match self.bits {
            false => PGERRR_A::NO_ERROR,
            true => PGERRR_A::ERROR,
        }
    }
    #[doc = "There was no error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERRR_A::NO_ERROR
    }
    #[doc = "There was an error programming flash"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERRR_A::ERROR
    }
}
#[doc = "Program error flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERRW_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<PGERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PGERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGERR` writer - Program error flag bit"]
pub type PGERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PGERRW_AW>;
impl<'a, REG, const O: u8> PGERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGERRW_AW::CLEAR)
    }
}
#[doc = "Field `PGAERR` reader - Program alignment error flag bit"]
pub type PGAERR_R = crate::BitReader;
#[doc = "Field `PGAERR` writer - Program alignment error flag bit"]
pub type PGAERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPERR` reader - Erase/Program protection error flag bit"]
pub type WPERR_R = crate::BitReader<WPERRR_A>;
#[doc = "Erase/Program protection error flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPERRR_A {
    #[doc = "0: There was no error"]
    NO_ERROR = 0,
    #[doc = "1: There was an error erasing/programming protected pages"]
    ERROR = 1,
}
impl From<WPERRR_A> for bool {
    #[inline(always)]
    fn from(variant: WPERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl WPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPERRR_A {
        match self.bits {
            false => WPERRR_A::NO_ERROR,
            true => WPERRR_A::ERROR,
        }
    }
    #[doc = "There was no error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WPERRR_A::NO_ERROR
    }
    #[doc = "There was an error erasing/programming protected pages"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WPERRR_A::ERROR
    }
}
#[doc = "Erase/Program protection error flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPERRW_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<WPERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: WPERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPERR` writer - Erase/Program protection error flag bit"]
pub type WPERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WPERRW_AW>;
impl<'a, REG, const O: u8> WPERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WPERRW_AW::CLEAR)
    }
}
#[doc = "Field `ENDF` reader - End of operation flag bit"]
pub type ENDF_R = crate::BitReader<ENDFR_A>;
#[doc = "End of operation flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDFR_A {
    #[doc = "0: No end of operation occurred"]
    NO_EVENT = 0,
    #[doc = "1: An end of operation event occurred"]
    EVENT = 1,
}
impl From<ENDFR_A> for bool {
    #[inline(always)]
    fn from(variant: ENDFR_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDFR_A {
        match self.bits {
            false => ENDFR_A::NO_EVENT,
            true => ENDFR_A::EVENT,
        }
    }
    #[doc = "No end of operation occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ENDFR_A::NO_EVENT
    }
    #[doc = "An end of operation event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == ENDFR_A::EVENT
    }
}
#[doc = "End of operation flag bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDFW_AW {
    #[doc = "1: Clears flag"]
    CLEAR = 1,
}
impl From<ENDFW_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDF` writer - End of operation flag bit"]
pub type ENDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENDFW_AW>;
impl<'a, REG, const O: u8> ENDF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ENDFW_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - The flash is busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Program alignment error flag bit"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WPERR_R {
        WPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    pub fn endf(&self) -> ENDF_R {
        ENDF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgerr(&mut self) -> PGERR_W<STAT_SPEC, 2> {
        PGERR_W::new(self)
    }
    #[doc = "Bit 3 - Program alignment error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<STAT_SPEC, 3> {
        PGAERR_W::new(self)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn wperr(&mut self) -> WPERR_W<STAT_SPEC, 4> {
        WPERR_W::new(self)
    }
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn endf(&mut self) -> ENDF_W<STAT_SPEC, 5> {
        ENDF_W::new(self)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
