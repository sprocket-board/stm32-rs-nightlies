#[doc = "Register `M5SR` reader"]
pub struct R(crate::R<M5SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M5SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M5SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M5SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M5SR` writer"]
pub struct W(crate::W<M5SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M5SR_SPEC>;
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
impl From<crate::W<M5SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M5SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEDCF` reader - ECC single error detected flag"]
pub type SEDCF_R = crate::BitReader<bool>;
#[doc = "Field `SEDCF` writer - ECC single error detected flag"]
pub type SEDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, M5SR_SPEC, bool, O>;
#[doc = "Field `DEDF` reader - ECC double error detected flag"]
pub type DEDF_R = crate::BitReader<bool>;
#[doc = "Field `DEDF` writer - ECC double error detected flag"]
pub type DEDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, M5SR_SPEC, bool, O>;
#[doc = "Field `DEBWDF` reader - ECC double error on byte write flag"]
pub type DEBWDF_R = crate::BitReader<bool>;
#[doc = "Field `DEBWDF` writer - ECC double error on byte write flag"]
pub type DEBWDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, M5SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ECC single error detected flag"]
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC double error on byte write flag"]
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error detected flag"]
    #[inline(always)]
    pub fn sedcf(&mut self) -> SEDCF_W<0> {
        SEDCF_W::new(self)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&mut self) -> DEDF_W<1> {
        DEDF_W::new(self)
    }
    #[doc = "Bit 2 - ECC double error on byte write flag"]
    #[inline(always)]
    pub fn debwdf(&mut self) -> DEBWDF_W<2> {
        DEBWDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor 5 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5sr](index.html) module"]
pub struct M5SR_SPEC;
impl crate::RegisterSpec for M5SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m5sr::R](R) reader structure"]
impl crate::Readable for M5SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m5sr::W](W) writer structure"]
impl crate::Writable for M5SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M5SR to value 0"]
impl crate::Resettable for M5SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
