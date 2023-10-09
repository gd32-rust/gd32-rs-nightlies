#[doc = "Register `MAC_INTMSK` reader"]
pub type R = crate::R<MAC_INTMSK_SPEC>;
#[doc = "Register `MAC_INTMSK` writer"]
pub type W = crate::W<MAC_INTMSK_SPEC>;
#[doc = "Field `WUMIM` reader - WUM interrupt mask"]
pub type WUMIM_R = crate::BitReader;
#[doc = "Field `WUMIM` writer - WUM interrupt mask"]
pub type WUMIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMSTIM` reader - Time stamp trigger interrupt mask"]
pub type TMSTIM_R = crate::BitReader;
#[doc = "Field `TMSTIM` writer - Time stamp trigger interrupt mask"]
pub type TMSTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - WUM interrupt mask"]
    #[inline(always)]
    pub fn wumim(&self) -> WUMIM_R {
        WUMIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tmstim(&self) -> TMSTIM_R {
        TMSTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - WUM interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wumim(&mut self) -> WUMIM_W<MAC_INTMSK_SPEC, 3> {
        WUMIM_W::new(self)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tmstim(&mut self) -> TMSTIM_W<MAC_INTMSK_SPEC, 9> {
        TMSTIM_W::new(self)
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
#[doc = "Ethernet MAC interrupt mask register (MAC_INTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_intmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_intmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_INTMSK_SPEC;
impl crate::RegisterSpec for MAC_INTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_intmsk::R`](R) reader structure"]
impl crate::Readable for MAC_INTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_intmsk::W`](W) writer structure"]
impl crate::Writable for MAC_INTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_INTMSK to value 0"]
impl crate::Resettable for MAC_INTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
