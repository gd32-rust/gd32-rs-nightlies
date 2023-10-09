#[doc = "Register `PIOTCFG3` reader"]
pub type R = crate::R<PIOTCFG3_SPEC>;
#[doc = "Register `PIOTCFG3` writer"]
pub type W = crate::W<PIOTCFG3_SPEC>;
#[doc = "Field `IOSET` reader - IO space setup time"]
pub type IOSET_R = crate::FieldReader;
#[doc = "Field `IOSET` writer - IO space setup time"]
pub type IOSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `IOWAIT` reader - IO space wait time"]
pub type IOWAIT_R = crate::FieldReader;
#[doc = "Field `IOWAIT` writer - IO space wait time"]
pub type IOWAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `IOHLD` reader - IO space hold time"]
pub type IOHLD_R = crate::FieldReader;
#[doc = "Field `IOHLD` writer - IO space hold time"]
pub type IOHLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `IOHIZ` reader - IO space data bus HiZ time"]
pub type IOHIZ_R = crate::FieldReader;
#[doc = "Field `IOHIZ` writer - IO space data bus HiZ time"]
pub type IOHIZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IO space setup time"]
    #[inline(always)]
    pub fn ioset(&self) -> IOSET_R {
        IOSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IO space wait time"]
    #[inline(always)]
    pub fn iowait(&self) -> IOWAIT_R {
        IOWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IO space hold time"]
    #[inline(always)]
    pub fn iohld(&self) -> IOHLD_R {
        IOHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - IO space data bus HiZ time"]
    #[inline(always)]
    pub fn iohiz(&self) -> IOHIZ_R {
        IOHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IO space setup time"]
    #[inline(always)]
    #[must_use]
    pub fn ioset(&mut self) -> IOSET_W<PIOTCFG3_SPEC, 0> {
        IOSET_W::new(self)
    }
    #[doc = "Bits 8:15 - IO space wait time"]
    #[inline(always)]
    #[must_use]
    pub fn iowait(&mut self) -> IOWAIT_W<PIOTCFG3_SPEC, 8> {
        IOWAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - IO space hold time"]
    #[inline(always)]
    #[must_use]
    pub fn iohld(&mut self) -> IOHLD_W<PIOTCFG3_SPEC, 16> {
        IOHLD_W::new(self)
    }
    #[doc = "Bits 24:31 - IO space data bus HiZ time"]
    #[inline(always)]
    #[must_use]
    pub fn iohiz(&mut self) -> IOHIZ_W<PIOTCFG3_SPEC, 24> {
        IOHIZ_W::new(self)
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
#[doc = "PC card I/O space timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`piotcfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`piotcfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIOTCFG3_SPEC;
impl crate::RegisterSpec for PIOTCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`piotcfg3::R`](R) reader structure"]
impl crate::Readable for PIOTCFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`piotcfg3::W`](W) writer structure"]
impl crate::Writable for PIOTCFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIOTCFG3 to value 0xffff_ffff"]
impl crate::Resettable for PIOTCFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
