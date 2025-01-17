#[doc = "Register `DDRPHYC_DTPR2` reader"]
pub struct R(crate::R<DDRPHYC_DTPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTPR2` writer"]
pub struct W(crate::W<DDRPHYC_DTPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTPR2_SPEC>;
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
impl From<crate::W<DDRPHYC_DTPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXS` reader - TXS"]
pub type TXS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXS` writer - TXS"]
pub type TXS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR2_SPEC, u16, u16, 10, O>;
#[doc = "Field `TXP` reader - TXP"]
pub type TXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXP` writer - TXP"]
pub type TXP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `TCKE` reader - TCKE"]
pub type TCKE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCKE` writer - TCKE"]
pub type TCKE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `TDLLK` reader - TDLLK"]
pub type TDLLK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDLLK` writer - TDLLK"]
pub type TDLLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DTPR2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - TXS"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:14 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:18 - TCKE"]
    #[inline(always)]
    pub fn tcke(&self) -> TCKE_R {
        TCKE_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:28 - TDLLK"]
    #[inline(always)]
    pub fn tdllk(&self) -> TDLLK_R {
        TDLLK_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TXS"]
    #[inline(always)]
    pub fn txs(&mut self) -> TXS_W<0> {
        TXS_W::new(self)
    }
    #[doc = "Bits 10:14 - TXP"]
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W<10> {
        TXP_W::new(self)
    }
    #[doc = "Bits 15:18 - TCKE"]
    #[inline(always)]
    pub fn tcke(&mut self) -> TCKE_W<15> {
        TCKE_W::new(self)
    }
    #[doc = "Bits 19:28 - TDLLK"]
    #[inline(always)]
    pub fn tdllk(&mut self) -> TDLLK_W<19> {
        TDLLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTP register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr2](index.html) module"]
pub struct DDRPHYC_DTPR2_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtpr2::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr2::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DTPR2 to value 0x2004_0d84"]
impl crate::Resettable for DDRPHYC_DTPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2004_0d84
    }
}
