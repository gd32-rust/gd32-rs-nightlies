#[doc = "Register `TIME` reader"]
pub type R = crate::R<TIME_SPEC>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TIME_SPEC>;
#[doc = "Field `SCU` reader - Second units in BCD code"]
pub type SCU_R = crate::FieldReader;
#[doc = "Field `SCU` writer - Second units in BCD code"]
pub type SCU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SCT` reader - Second tens in BCD code"]
pub type SCT_R = crate::FieldReader;
#[doc = "Field `SCT` writer - Second tens in BCD code"]
pub type SCT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MNU` reader - Minute units in BCD code"]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD code"]
pub type MNU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MNT` reader - Minute tens in BCD code"]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD code"]
pub type MNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `HRU` reader - Hour units in BCD format"]
pub type HRU_R = crate::FieldReader;
#[doc = "Field `HRU` writer - Hour units in BCD format"]
pub type HRU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HRT` reader - Hour tens in BCD code"]
pub type HRT_R = crate::FieldReader;
#[doc = "Field `HRT` writer - Hour tens in BCD code"]
pub type HRT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PM` reader - AM/PM mark"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM mark"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD code"]
    #[inline(always)]
    pub fn scu(&self) -> SCU_R {
        SCU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code"]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD code"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD code"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hru(&self) -> HRU_R {
        HRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code"]
    #[inline(always)]
    pub fn hrt(&self) -> HRT_R {
        HRT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM mark"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn scu(&mut self) -> SCU_W<TIME_SPEC, 0> {
        SCU_W::new(self)
    }
    #[doc = "Bits 4:6 - Second tens in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SCT_W<TIME_SPEC, 4> {
        SCT_W::new(self)
    }
    #[doc = "Bits 8:11 - Minute units in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<TIME_SPEC, 8> {
        MNU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<TIME_SPEC, 12> {
        MNT_W::new(self)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn hru(&mut self) -> HRU_W<TIME_SPEC, 16> {
        HRU_W::new(self)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD code"]
    #[inline(always)]
    #[must_use]
    pub fn hrt(&mut self) -> HRT_W<TIME_SPEC, 20> {
        HRT_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM mark"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<TIME_SPEC, 22> {
        PM_W::new(self)
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
#[doc = "time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
