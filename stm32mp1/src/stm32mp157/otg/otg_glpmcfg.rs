#[doc = "Register `OTG_GLPMCFG` reader"]
pub struct R(crate::R<OTG_GLPMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GLPMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GLPMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GLPMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GLPMCFG` writer"]
pub struct W(crate::W<OTG_GLPMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GLPMCFG_SPEC>;
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
impl From<crate::W<OTG_GLPMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GLPMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMEN` reader - LPMEN"]
pub type LPMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPMEN` writer - LPMEN"]
pub type LPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GLPMCFG_SPEC, bool, O>;
#[doc = "Field `LPMACK` reader - LPMACK"]
pub type LPMACK_R = crate::BitReader<bool>;
#[doc = "Field `LPMACK` writer - LPMACK"]
pub type LPMACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GLPMCFG_SPEC, bool, O>;
#[doc = "Field `BESL` reader - BESL"]
pub type BESL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BESL` writer - BESL"]
pub type BESL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GLPMCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `REMWAKE` reader - REMWAKE"]
pub type REMWAKE_R = crate::BitReader<bool>;
#[doc = "Field `REMWAKE` writer - REMWAKE"]
pub type REMWAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GLPMCFG_SPEC, bool, O>;
#[doc = "Field `L1SSEN` reader - L1SSEN"]
pub type L1SSEN_R = crate::BitReader<bool>;
#[doc = "Field `L1SSEN` writer - L1SSEN"]
pub type L1SSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GLPMCFG_SPEC, bool, O>;
#[doc = "Field `BESLTHRS` reader - BESLTHRS"]
pub type BESLTHRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BESLTHRS` writer - BESLTHRS"]
pub type BESLTHRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GLPMCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `L1DSEN` reader - L1DSEN"]
pub type L1DSEN_R = crate::BitReader<bool>;
#[doc = "Field `L1DSEN` writer - L1DSEN"]
pub type L1DSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GLPMCFG_SPEC, bool, O>;
#[doc = "Field `LPMRSP` reader - LPMRSP"]
pub type LPMRSP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLPSTS` reader - SLPSTS"]
pub type SLPSTS_R = crate::BitReader<bool>;
#[doc = "Field `L1RSMOK` reader - L1RSMOK"]
pub type L1RSMOK_R = crate::BitReader<bool>;
#[doc = "Field `LPMCHIDX` reader - LPMCHIDX"]
pub type LPMCHIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMCHIDX` writer - LPMCHIDX"]
pub type LPMCHIDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GLPMCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPMRCNT` reader - LPMRCNT"]
pub type LPMRCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMRCNT` writer - LPMRCNT"]
pub type LPMRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GLPMCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `SNDLPM` reader - SNDLPM"]
pub type SNDLPM_R = crate::BitReader<bool>;
#[doc = "Field `SNDLPM` writer - SNDLPM"]
pub type SNDLPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GLPMCFG_SPEC, bool, O>;
#[doc = "Field `LPMRCNTSTS` reader - LPMRCNTSTS"]
pub type LPMRCNTSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENBESL` reader - ENBESL"]
pub type ENBESL_R = crate::BitReader<bool>;
#[doc = "Field `ENBESL` writer - ENBESL"]
pub type ENBESL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GLPMCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LPMEN"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPMACK"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - BESL"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - REMWAKE"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - L1SSEN"]
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BESLTHRS"]
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - L1DSEN"]
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - LPMRSP"]
    #[inline(always)]
    pub fn lpmrsp(&self) -> LPMRSP_R {
        LPMRSP_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SLPSTS"]
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L1RSMOK"]
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - LPMCHIDX"]
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:23 - LPMRCNT"]
    #[inline(always)]
    pub fn lpmrcnt(&self) -> LPMRCNT_R {
        LPMRCNT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - SNDLPM"]
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - LPMRCNTSTS"]
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - ENBESL"]
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPMEN"]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W<0> {
        LPMEN_W::new(self)
    }
    #[doc = "Bit 1 - LPMACK"]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W<1> {
        LPMACK_W::new(self)
    }
    #[doc = "Bits 2:5 - BESL"]
    #[inline(always)]
    pub fn besl(&mut self) -> BESL_W<2> {
        BESL_W::new(self)
    }
    #[doc = "Bit 6 - REMWAKE"]
    #[inline(always)]
    pub fn remwake(&mut self) -> REMWAKE_W<6> {
        REMWAKE_W::new(self)
    }
    #[doc = "Bit 7 - L1SSEN"]
    #[inline(always)]
    pub fn l1ssen(&mut self) -> L1SSEN_W<7> {
        L1SSEN_W::new(self)
    }
    #[doc = "Bits 8:11 - BESLTHRS"]
    #[inline(always)]
    pub fn beslthrs(&mut self) -> BESLTHRS_W<8> {
        BESLTHRS_W::new(self)
    }
    #[doc = "Bit 12 - L1DSEN"]
    #[inline(always)]
    pub fn l1dsen(&mut self) -> L1DSEN_W<12> {
        L1DSEN_W::new(self)
    }
    #[doc = "Bits 17:20 - LPMCHIDX"]
    #[inline(always)]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W<17> {
        LPMCHIDX_W::new(self)
    }
    #[doc = "Bits 21:23 - LPMRCNT"]
    #[inline(always)]
    pub fn lpmrcnt(&mut self) -> LPMRCNT_W<21> {
        LPMRCNT_W::new(self)
    }
    #[doc = "Bit 24 - SNDLPM"]
    #[inline(always)]
    pub fn sndlpm(&mut self) -> SNDLPM_W<24> {
        SNDLPM_W::new(self)
    }
    #[doc = "Bit 28 - ENBESL"]
    #[inline(always)]
    pub fn enbesl(&mut self) -> ENBESL_W<28> {
        ENBESL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG core LPM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_glpmcfg](index.html) module"]
pub struct OTG_GLPMCFG_SPEC;
impl crate::RegisterSpec for OTG_GLPMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_glpmcfg::R](R) reader structure"]
impl crate::Readable for OTG_GLPMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_glpmcfg::W](W) writer structure"]
impl crate::Writable for OTG_GLPMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_GLPMCFG to value 0"]
impl crate::Resettable for OTG_GLPMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
