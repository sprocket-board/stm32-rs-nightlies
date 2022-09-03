#[doc = "Register `CSTAT` reader"]
pub struct R(crate::R<CSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSTAT` writer"]
pub struct W(crate::W<CSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSTAT_SPEC>;
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
impl From<crate::W<CSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCPC` reader - Oscillator Clock Period Counter"]
pub type OCPC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OCPC` writer - Oscillator Clock Period Counter"]
pub type OCPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSTAT_SPEC, u32, u32, 18, O>;
#[doc = "Field `TQC` reader - Time Quanta Counter"]
pub type TQC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TQC` writer - Time Quanta Counter"]
pub type TQC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSTAT_SPEC, u16, u16, 11, O>;
#[doc = "Field `CALS` reader - Calibration State"]
pub type CALS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALS` writer - Calibration State"]
pub type CALS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSTAT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&mut self) -> OCPC_W<0> {
        OCPC_W::new(self)
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&mut self) -> TQC_W<18> {
        TQC_W::new(self)
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&mut self) -> CALS_W<30> {
        CALS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cstat](index.html) module"]
pub struct CSTAT_SPEC;
impl crate::RegisterSpec for CSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cstat::R](R) reader structure"]
impl crate::Readable for CSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cstat::W](W) writer structure"]
impl crate::Writable for CSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSTAT to value 0"]
impl crate::Resettable for CSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
