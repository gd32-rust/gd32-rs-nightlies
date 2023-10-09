#[doc = "Register `CS0` reader"]
pub type R = crate::R<CS0_SPEC>;
#[doc = "Register `CS0` writer"]
pub type W = crate::W<CS0_SPEC>;
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `STBF` reader - Standby flag"]
pub type STBF_R = crate::BitReader;
#[doc = "Field `LVDF` reader - Low Voltage Detector Status Flag"]
pub type LVDF_R = crate::BitReader;
#[doc = "Field `WUPEN6` reader - Enable WKUP pin6"]
pub type WUPEN6_R = crate::BitReader;
#[doc = "Field `WUPEN6` writer - Enable WKUP pin6"]
pub type WUPEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUPEN0` reader - Enable WKUP pin0"]
pub type WUPEN0_R = crate::BitReader;
#[doc = "Field `WUPEN0` writer - Enable WKUP pin0"]
pub type WUPEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUPEN1` reader - Enable WKUP pin1"]
pub type WUPEN1_R = crate::BitReader;
#[doc = "Field `WUPEN1` writer - Enable WKUP pin1"]
pub type WUPEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUPEN2` reader - Enable WKUP pin2"]
pub type WUPEN2_R = crate::BitReader;
#[doc = "Field `WUPEN2` writer - Enable WKUP pin2"]
pub type WUPEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUPEN3` reader - Enable WKUP pin3"]
pub type WUPEN3_R = crate::BitReader;
#[doc = "Field `WUPEN3` writer - Enable WKUP pin3"]
pub type WUPEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUPEN4` reader - Enable WKUP pin4"]
pub type WUPEN4_R = crate::BitReader;
#[doc = "Field `WUPEN4` writer - Enable WKUP pin4"]
pub type WUPEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUPEN5` reader - Enable WKUP pin5"]
pub type WUPEN5_R = crate::BitReader;
#[doc = "Field `WUPEN5` writer - Enable WKUP pin5"]
pub type WUPEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUPEN7` reader - Enable WKUP Pin7(PF8)"]
pub type WUPEN7_R = crate::BitReader;
#[doc = "Field `WUPEN7` writer - Enable WKUP Pin7(PF8)"]
pub type WUPEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDRF` reader - High-driver ready flag"]
pub type HDRF_R = crate::BitReader;
#[doc = "Field `HDSRF` reader - High-driver switch ready flag"]
pub type HDSRF_R = crate::BitReader;
#[doc = "Field `LDRF` reader - Low-driver mode ready flag"]
pub type LDRF_R = crate::FieldReader;
#[doc = "Field `LDRF` writer - Low-driver mode ready flag"]
pub type LDRF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn stbf(&self) -> STBF_R {
        STBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Voltage Detector Status Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable WKUP pin6"]
    #[inline(always)]
    pub fn wupen6(&self) -> WUPEN6_R {
        WUPEN6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin0"]
    #[inline(always)]
    pub fn wupen0(&self) -> WUPEN0_R {
        WUPEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable WKUP pin1"]
    #[inline(always)]
    pub fn wupen1(&self) -> WUPEN1_R {
        WUPEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable WKUP pin2"]
    #[inline(always)]
    pub fn wupen2(&self) -> WUPEN2_R {
        WUPEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable WKUP pin3"]
    #[inline(always)]
    pub fn wupen3(&self) -> WUPEN3_R {
        WUPEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable WKUP pin4"]
    #[inline(always)]
    pub fn wupen4(&self) -> WUPEN4_R {
        WUPEN4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable WKUP pin5"]
    #[inline(always)]
    pub fn wupen5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable WKUP Pin7(PF8)"]
    #[inline(always)]
    pub fn wupen7(&self) -> WUPEN7_R {
        WUPEN7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - High-driver ready flag"]
    #[inline(always)]
    pub fn hdrf(&self) -> HDRF_R {
        HDRF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High-driver switch ready flag"]
    #[inline(always)]
    pub fn hdsrf(&self) -> HDSRF_R {
        HDSRF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Low-driver mode ready flag"]
    #[inline(always)]
    pub fn ldrf(&self) -> LDRF_R {
        LDRF_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Enable WKUP pin6"]
    #[inline(always)]
    #[must_use]
    pub fn wupen6(&mut self) -> WUPEN6_W<CS0_SPEC, 7> {
        WUPEN6_W::new(self)
    }
    #[doc = "Bit 8 - Enable WKUP pin0"]
    #[inline(always)]
    #[must_use]
    pub fn wupen0(&mut self) -> WUPEN0_W<CS0_SPEC, 8> {
        WUPEN0_W::new(self)
    }
    #[doc = "Bit 9 - Enable WKUP pin1"]
    #[inline(always)]
    #[must_use]
    pub fn wupen1(&mut self) -> WUPEN1_W<CS0_SPEC, 9> {
        WUPEN1_W::new(self)
    }
    #[doc = "Bit 10 - Enable WKUP pin2"]
    #[inline(always)]
    #[must_use]
    pub fn wupen2(&mut self) -> WUPEN2_W<CS0_SPEC, 10> {
        WUPEN2_W::new(self)
    }
    #[doc = "Bit 11 - Enable WKUP pin3"]
    #[inline(always)]
    #[must_use]
    pub fn wupen3(&mut self) -> WUPEN3_W<CS0_SPEC, 11> {
        WUPEN3_W::new(self)
    }
    #[doc = "Bit 12 - Enable WKUP pin4"]
    #[inline(always)]
    #[must_use]
    pub fn wupen4(&mut self) -> WUPEN4_W<CS0_SPEC, 12> {
        WUPEN4_W::new(self)
    }
    #[doc = "Bit 13 - Enable WKUP pin5"]
    #[inline(always)]
    #[must_use]
    pub fn wupen5(&mut self) -> WUPEN5_W<CS0_SPEC, 13> {
        WUPEN5_W::new(self)
    }
    #[doc = "Bit 15 - Enable WKUP Pin7(PF8)"]
    #[inline(always)]
    #[must_use]
    pub fn wupen7(&mut self) -> WUPEN7_W<CS0_SPEC, 15> {
        WUPEN7_W::new(self)
    }
    #[doc = "Bits 18:19 - Low-driver mode ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn ldrf(&mut self) -> LDRF_W<CS0_SPEC, 18> {
        LDRF_W::new(self)
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
#[doc = "power control/status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS0_SPEC;
impl crate::RegisterSpec for CS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs0::R`](R) reader structure"]
impl crate::Readable for CS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs0::W`](W) writer structure"]
impl crate::Writable for CS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS0 to value 0"]
impl crate::Resettable for CS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
