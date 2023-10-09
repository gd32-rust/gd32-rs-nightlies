#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `I2CEN` reader - I2C peripheral enable"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C peripheral enable"]
pub type I2CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBEN` reader - SMBus/I2C mode switch"]
pub type SMBEN_R = crate::BitReader;
#[doc = "Field `SMBEN` writer - SMBus/I2C mode switch"]
pub type SMBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBSEL` reader - SMBusType Selection"]
pub type SMBSEL_R = crate::BitReader;
#[doc = "Field `SMBSEL` writer - SMBusType Selection"]
pub type SMBSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARPEN` reader - ARP protocol in SMBus switch"]
pub type ARPEN_R = crate::BitReader;
#[doc = "Field `ARPEN` writer - ARP protocol in SMBus switch"]
pub type ARPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECEN` reader - PEC Calculation Switch"]
pub type PECEN_R = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC Calculation Switch"]
pub type PECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCEN` reader - Whether or not to response to a General Call (0x00)"]
pub type GCEN_R = crate::BitReader;
#[doc = "Field `GCEN` writer - Whether or not to response to a General Call (0x00)"]
pub type GCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SS` reader - Whether to stretch SCL low when data is not ready in slave mode"]
pub type SS_R = crate::BitReader;
#[doc = "Field `SS` writer - Whether to stretch SCL low when data is not ready in slave mode"]
pub type SS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START` reader - Generate a START condition on I2C bus"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Generate a START condition on I2C bus"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` reader - Generate a STOP condition on I2C bus"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Generate a STOP condition on I2C bus"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKEN` reader - Whether or not to send an ACK"]
pub type ACKEN_R = crate::BitReader;
#[doc = "Field `ACKEN` writer - Whether or not to send an ACK"]
pub type ACKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POAP` reader - Position of ACK and PEC when receiving"]
pub type POAP_R = crate::BitReader;
#[doc = "Field `POAP` writer - Position of ACK and PEC when receiving"]
pub type POAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECTRANS` reader - PEC Transfer"]
pub type PECTRANS_R = crate::BitReader;
#[doc = "Field `PECTRANS` writer - PEC Transfer"]
pub type PECTRANS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SALT` reader - SMBus alert"]
pub type SALT_R = crate::BitReader;
#[doc = "Field `SALT` writer - SMBus alert"]
pub type SALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRESET` reader - Software reset"]
pub type SRESET_R = crate::BitReader;
#[doc = "Field `SRESET` writer - Software reset"]
pub type SRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C mode switch"]
    #[inline(always)]
    pub fn smben(&self) -> SMBEN_R {
        SMBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBusType Selection"]
    #[inline(always)]
    pub fn smbsel(&self) -> SMBSEL_R {
        SMBSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARP protocol in SMBus switch"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC Calculation Switch"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Whether or not to response to a General Call (0x00)"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generate a START condition on I2C bus"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generate a STOP condition on I2C bus"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Whether or not to send an ACK"]
    #[inline(always)]
    pub fn acken(&self) -> ACKEN_R {
        ACKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Position of ACK and PEC when receiving"]
    #[inline(always)]
    pub fn poap(&self) -> POAP_R {
        POAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Transfer"]
    #[inline(always)]
    pub fn pectrans(&self) -> PECTRANS_R {
        PECTRANS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn salt(&self) -> SALT_R {
        SALT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn sreset(&self) -> SRESET_R {
        SRESET_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<CTL0_SPEC, 0> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 1 - SMBus/I2C mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn smben(&mut self) -> SMBEN_W<CTL0_SPEC, 1> {
        SMBEN_W::new(self)
    }
    #[doc = "Bit 3 - SMBusType Selection"]
    #[inline(always)]
    #[must_use]
    pub fn smbsel(&mut self) -> SMBSEL_W<CTL0_SPEC, 3> {
        SMBSEL_W::new(self)
    }
    #[doc = "Bit 4 - ARP protocol in SMBus switch"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<CTL0_SPEC, 4> {
        ARPEN_W::new(self)
    }
    #[doc = "Bit 5 - PEC Calculation Switch"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CTL0_SPEC, 5> {
        PECEN_W::new(self)
    }
    #[doc = "Bit 6 - Whether or not to response to a General Call (0x00)"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<CTL0_SPEC, 6> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 7 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<CTL0_SPEC, 7> {
        SS_W::new(self)
    }
    #[doc = "Bit 8 - Generate a START condition on I2C bus"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTL0_SPEC, 8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - Generate a STOP condition on I2C bus"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CTL0_SPEC, 9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - Whether or not to send an ACK"]
    #[inline(always)]
    #[must_use]
    pub fn acken(&mut self) -> ACKEN_W<CTL0_SPEC, 10> {
        ACKEN_W::new(self)
    }
    #[doc = "Bit 11 - Position of ACK and PEC when receiving"]
    #[inline(always)]
    #[must_use]
    pub fn poap(&mut self) -> POAP_W<CTL0_SPEC, 11> {
        POAP_W::new(self)
    }
    #[doc = "Bit 12 - PEC Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn pectrans(&mut self) -> PECTRANS_W<CTL0_SPEC, 12> {
        PECTRANS_W::new(self)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn salt(&mut self) -> SALT_W<CTL0_SPEC, 13> {
        SALT_W::new(self)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sreset(&mut self) -> SRESET_W<CTL0_SPEC, 15> {
        SRESET_W::new(self)
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
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
