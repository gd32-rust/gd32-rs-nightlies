#[doc = "Register `CS1` reader"]
pub type R = crate::R<CS1_SPEC>;
#[doc = "Register `CS1` writer"]
pub type W = crate::W<CS1_SPEC>;
#[doc = "Field `DPF1` reader - Deep-sleep1 mode status flag"]
pub type DPF1_R = crate::BitReader;
#[doc = "Field `DPF1` writer - Deep-sleep1 mode status flag"]
pub type DPF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPF2` reader - Deep-sleep 2 mode status flag"]
pub type DPF2_R = crate::BitReader;
#[doc = "Field `DPF2` writer - Deep-sleep 2 mode status flag"]
pub type DPF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Deep-sleep1 mode status flag"]
    #[inline(always)]
    pub fn dpf1(&self) -> DPF1_R {
        DPF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Deep-sleep 2 mode status flag"]
    #[inline(always)]
    pub fn dpf2(&self) -> DPF2_R {
        DPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deep-sleep1 mode status flag"]
    #[inline(always)]
    #[must_use]
    pub fn dpf1(&mut self) -> DPF1_W<CS1_SPEC, 0> {
        DPF1_W::new(self)
    }
    #[doc = "Bit 1 - Deep-sleep 2 mode status flag"]
    #[inline(always)]
    #[must_use]
    pub fn dpf2(&mut self) -> DPF2_W<CS1_SPEC, 1> {
        DPF2_W::new(self)
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
#[doc = "power control and status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS1_SPEC;
impl crate::RegisterSpec for CS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs1::R`](R) reader structure"]
impl crate::Readable for CS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs1::W`](W) writer structure"]
impl crate::Writable for CS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS1 to value 0"]
impl crate::Resettable for CS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
