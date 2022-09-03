#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT1C` writer - Fault 1 Interrupt Flag Clear"]
pub type FLT1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FLT2C` writer - Fault 2 Interrupt Flag Clear"]
pub type FLT2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FLT3C` writer - Fault 3 Interrupt Flag Clear"]
pub type FLT3C_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FLT4C` writer - Fault 4 Interrupt Flag Clear"]
pub type FLT4C_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FLT5C` writer - Fault 5 Interrupt Flag Clear"]
pub type FLT5C_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `SYSFLTC` writer - System Fault Interrupt Flag Clear"]
pub type SYSFLTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FLT6C` writer - Fault 6 Interrupt Flag Clear"]
pub type FLT6C_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `DLLRDYC` writer - DLL Ready Interrupt flag Clear"]
pub type DLLRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BMPERC` writer - Burst mode period flag Clear"]
pub type BMPERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt1c(&mut self) -> FLT1C_W<0> {
        FLT1C_W::new(self)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt2c(&mut self) -> FLT2C_W<1> {
        FLT2C_W::new(self)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt3c(&mut self) -> FLT3C_W<2> {
        FLT3C_W::new(self)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt4c(&mut self) -> FLT4C_W<3> {
        FLT4C_W::new(self)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt5c(&mut self) -> FLT5C_W<4> {
        FLT5C_W::new(self)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag Clear"]
    #[inline(always)]
    pub fn sysfltc(&mut self) -> SYSFLTC_W<5> {
        SYSFLTC_W::new(self)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn flt6c(&mut self) -> FLT6C_W<6> {
        FLT6C_W::new(self)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt flag Clear"]
    #[inline(always)]
    pub fn dllrdyc(&mut self) -> DLLRDYC_W<16> {
        DLLRDYC_W::new(self)
    }
    #[doc = "Bit 17 - Burst mode period flag Clear"]
    #[inline(always)]
    pub fn bmperc(&mut self) -> BMPERC_W<17> {
        BMPERC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
