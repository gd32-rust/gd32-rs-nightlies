#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `CECEN` reader - Enable/disable HDMI-CEC controller"]
pub type CECEN_R = crate::BitReader;
#[doc = "Field `CECEN` writer - Enable/disable HDMI-CEC controller"]
pub type CECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOM` reader - Start of sending a message"]
pub type SOM_R = crate::BitReader;
#[doc = "Field `SOM` writer - Start of sending a message"]
pub type SOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENDOM` reader - ENDOM bit value in the next frame in TX mode"]
pub type ENDOM_R = crate::BitReader;
#[doc = "Field `ENDOM` writer - ENDOM bit value in the next frame in TX mode"]
pub type ENDOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable/disable HDMI-CEC controller"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of sending a message"]
    #[inline(always)]
    pub fn som(&self) -> SOM_R {
        SOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ENDOM bit value in the next frame in TX mode"]
    #[inline(always)]
    pub fn endom(&self) -> ENDOM_R {
        ENDOM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable/disable HDMI-CEC controller"]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<CTL_SPEC, 0> {
        CECEN_W::new(self)
    }
    #[doc = "Bit 1 - Start of sending a message"]
    #[inline(always)]
    #[must_use]
    pub fn som(&mut self) -> SOM_W<CTL_SPEC, 1> {
        SOM_W::new(self)
    }
    #[doc = "Bit 2 - ENDOM bit value in the next frame in TX mode"]
    #[inline(always)]
    #[must_use]
    pub fn endom(&mut self) -> ENDOM_W<CTL_SPEC, 2> {
        ENDOM_W::new(self)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
