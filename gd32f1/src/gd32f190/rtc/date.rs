#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `DAYU` reader - Date units in BCD format"]
pub type DAYU_R = crate::FieldReader;
#[doc = "Field `DAYU` writer - Date units in BCD format"]
pub type DAYU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DAYT` reader - Date tens in BCD format"]
pub type DAYT_R = crate::FieldReader;
#[doc = "Field `DAYT` writer - Date tens in BCD format"]
pub type DAYT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MONU` reader - Month units in BCD format"]
pub type MONU_R = crate::FieldReader;
#[doc = "Field `MONU` writer - Month units in BCD format"]
pub type MONU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MONT` reader - Month tens in BCD format"]
pub type MONT_R = crate::BitReader;
#[doc = "Field `MONT` writer - Month tens in BCD format"]
pub type MONT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOW` reader - Week day units"]
pub type DOW_R = crate::FieldReader;
#[doc = "Field `DOW` writer - Week day units"]
pub type DOW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `YRU` reader - Year units in BCD format"]
pub type YRU_R = crate::FieldReader;
#[doc = "Field `YRU` writer - Year units in BCD format"]
pub type YRU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `YRT` reader - Year tens in BCD format"]
pub type YRT_R = crate::FieldReader;
#[doc = "Field `YRT` writer - Year tens in BCD format"]
pub type YRT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn monu(&self) -> MONU_R {
        MONU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn mont(&self) -> MONT_R {
        MONT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    pub fn yru(&self) -> YRU_R {
        YRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    pub fn yrt(&self) -> YRT_R {
        YRT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dayu(&mut self) -> DAYU_W<DATE_SPEC, 0> {
        DAYU_W::new(self)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dayt(&mut self) -> DAYT_W<DATE_SPEC, 4> {
        DAYT_W::new(self)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn monu(&mut self) -> MONU_W<DATE_SPEC, 8> {
        MONU_W::new(self)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mont(&mut self) -> MONT_W<DATE_SPEC, 12> {
        MONT_W::new(self)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    #[must_use]
    pub fn dow(&mut self) -> DOW_W<DATE_SPEC, 13> {
        DOW_W::new(self)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yru(&mut self) -> YRU_W<DATE_SPEC, 16> {
        YRU_W::new(self)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yrt(&mut self) -> YRT_W<DATE_SPEC, 20> {
        YRT_W::new(self)
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
#[doc = "date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATE to value 0x2101"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x2101;
}
