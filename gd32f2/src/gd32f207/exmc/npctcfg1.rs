#[doc = "Register `NPCTCFG1` reader"]
pub type R = crate::R<NPCTCFG1_SPEC>;
#[doc = "Register `NPCTCFG1` writer"]
pub type W = crate::W<NPCTCFG1_SPEC>;
#[doc = "Field `COMSET` reader - Common memory setup time"]
pub type COMSET_R = crate::FieldReader;
#[doc = "Field `COMSET` writer - Common memory setup time"]
pub type COMSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `COMWAIT` reader - Common memory wait time"]
pub type COMWAIT_R = crate::FieldReader;
#[doc = "Field `COMWAIT` writer - Common memory wait time"]
pub type COMWAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `COMHLD` reader - Common memory hold time"]
pub type COMHLD_R = crate::FieldReader;
#[doc = "Field `COMHLD` writer - Common memory hold time"]
pub type COMHLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `COMHIZ` reader - Common memory data bus HiZ time"]
pub type COMHIZ_R = crate::FieldReader;
#[doc = "Field `COMHIZ` writer - Common memory data bus HiZ time"]
pub type COMHIZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Common memory setup time"]
    #[inline(always)]
    pub fn comset(&self) -> COMSET_R {
        COMSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Common memory wait time"]
    #[inline(always)]
    pub fn comwait(&self) -> COMWAIT_R {
        COMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Common memory hold time"]
    #[inline(always)]
    pub fn comhld(&self) -> COMHLD_R {
        COMHLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Common memory data bus HiZ time"]
    #[inline(always)]
    pub fn comhiz(&self) -> COMHIZ_R {
        COMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn comset(&mut self) -> COMSET_W<NPCTCFG1_SPEC, 0> {
        COMSET_W::new(self)
    }
    #[doc = "Bits 8:15 - Common memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn comwait(&mut self) -> COMWAIT_W<NPCTCFG1_SPEC, 8> {
        COMWAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - Common memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn comhld(&mut self) -> COMHLD_W<NPCTCFG1_SPEC, 16> {
        COMHLD_W::new(self)
    }
    #[doc = "Bits 24:31 - Common memory data bus HiZ time"]
    #[inline(always)]
    #[must_use]
    pub fn comhiz(&mut self) -> COMHIZ_W<NPCTCFG1_SPEC, 24> {
        COMHIZ_W::new(self)
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
#[doc = "NAND flash/PC card common space timing configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npctcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npctcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NPCTCFG1_SPEC;
impl crate::RegisterSpec for NPCTCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npctcfg1::R`](R) reader structure"]
impl crate::Readable for NPCTCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`npctcfg1::W`](W) writer structure"]
impl crate::Writable for NPCTCFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NPCTCFG1 to value 0xfcfc_fcfc"]
impl crate::Resettable for NPCTCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
