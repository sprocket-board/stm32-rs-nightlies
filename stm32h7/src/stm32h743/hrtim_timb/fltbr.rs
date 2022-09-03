#[doc = "Register `FLTBR` reader"]
pub struct R(crate::R<FLTBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTBR` writer"]
pub struct W(crate::W<FLTBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTBR_SPEC>;
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
impl From<crate::W<FLTBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub type FLT1EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub type FLT1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTBR_SPEC, bool, O>;
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub type FLT2EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub type FLT2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTBR_SPEC, bool, O>;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub type FLT3EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub type FLT3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTBR_SPEC, bool, O>;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub type FLT4EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub type FLT4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTBR_SPEC, bool, O>;
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub type FLT5EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub type FLT5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTBR_SPEC, bool, O>;
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub type FLTLCK_R = crate::BitReader<bool>;
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub type FLTLCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTBR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> FLT1EN_W<0> {
        FLT1EN_W::new(self)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&mut self) -> FLT2EN_W<1> {
        FLT2EN_W::new(self)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&mut self) -> FLT3EN_W<2> {
        FLT3EN_W::new(self)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&mut self) -> FLT4EN_W<3> {
        FLT4EN_W::new(self)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&mut self) -> FLT5EN_W<4> {
        FLT5EN_W::new(self)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&mut self) -> FLTLCK_W<31> {
        FLTLCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltbr](index.html) module"]
pub struct FLTBR_SPEC;
impl crate::RegisterSpec for FLTBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltbr::R](R) reader structure"]
impl crate::Readable for FLTBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltbr::W](W) writer structure"]
impl crate::Writable for FLTBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTBR to value 0"]
impl crate::Resettable for FLTBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
