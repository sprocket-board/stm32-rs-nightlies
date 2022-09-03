#[doc = "Register `I2C_ICR` writer"]
pub struct W(crate::W<I2C_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<I2C_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRCF` writer - ADDRCF"]
pub type ADDRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
#[doc = "Field `NACKCF` writer - NACKCF"]
pub type NACKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
#[doc = "Field `STOPCF` writer - STOPCF"]
pub type STOPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
#[doc = "Field `BERRCF` writer - BERRCF"]
pub type BERRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
#[doc = "Field `ARLOCF` writer - ARLOCF"]
pub type ARLOCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
#[doc = "Field `OVRCF` writer - OVRCF"]
pub type OVRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
#[doc = "Field `PECCF` writer - PECCF"]
pub type PECCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
#[doc = "Field `TIMOUTCF` writer - TIMOUTCF"]
pub type TIMOUTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
#[doc = "Field `ALERTCF` writer - ALERTCF"]
pub type ALERTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 3 - ADDRCF"]
    #[inline(always)]
    pub fn addrcf(&mut self) -> ADDRCF_W<3> {
        ADDRCF_W::new(self)
    }
    #[doc = "Bit 4 - NACKCF"]
    #[inline(always)]
    pub fn nackcf(&mut self) -> NACKCF_W<4> {
        NACKCF_W::new(self)
    }
    #[doc = "Bit 5 - STOPCF"]
    #[inline(always)]
    pub fn stopcf(&mut self) -> STOPCF_W<5> {
        STOPCF_W::new(self)
    }
    #[doc = "Bit 8 - BERRCF"]
    #[inline(always)]
    pub fn berrcf(&mut self) -> BERRCF_W<8> {
        BERRCF_W::new(self)
    }
    #[doc = "Bit 9 - ARLOCF"]
    #[inline(always)]
    pub fn arlocf(&mut self) -> ARLOCF_W<9> {
        ARLOCF_W::new(self)
    }
    #[doc = "Bit 10 - OVRCF"]
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W<10> {
        OVRCF_W::new(self)
    }
    #[doc = "Bit 11 - PECCF"]
    #[inline(always)]
    pub fn peccf(&mut self) -> PECCF_W<11> {
        PECCF_W::new(self)
    }
    #[doc = "Bit 12 - TIMOUTCF"]
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<12> {
        TIMOUTCF_W::new(self)
    }
    #[doc = "Bit 13 - ALERTCF"]
    #[inline(always)]
    pub fn alertcf(&mut self) -> ALERTCF_W<13> {
        ALERTCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_icr](index.html) module"]
pub struct I2C_ICR_SPEC;
impl crate::RegisterSpec for I2C_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i2c_icr::W](W) writer structure"]
impl crate::Writable for I2C_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_ICR to value 0"]
impl crate::Resettable for I2C_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
