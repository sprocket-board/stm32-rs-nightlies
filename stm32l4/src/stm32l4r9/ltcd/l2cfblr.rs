#[doc = "Register `L2CFBLR` reader"]
pub struct R(crate::R<L2CFBLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2CFBLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2CFBLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2CFBLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L2CFBLR` writer"]
pub struct W(crate::W<L2CFBLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2CFBLR_SPEC>;
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
impl From<crate::W<L2CFBLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2CFBLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBLL` reader - Color Frame Buffer Line Length"]
pub type CFBLL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFBLL` writer - Color Frame Buffer Line Length"]
pub type CFBLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L2CFBLR_SPEC, u16, u16, 13, O>;
#[doc = "Field `CFBP` reader - Color Frame Buffer Pitch in bytes"]
pub type CFBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFBP` writer - Color Frame Buffer Pitch in bytes"]
pub type CFBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L2CFBLR_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Color Frame Buffer Line Length"]
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes"]
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Color Frame Buffer Line Length"]
    #[inline(always)]
    pub fn cfbll(&mut self) -> CFBLL_W<0> {
        CFBLL_W::new(self)
    }
    #[doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes"]
    #[inline(always)]
    pub fn cfbp(&mut self) -> CFBP_W<16> {
        CFBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layer Color Frame Buffer Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2cfblr](index.html) module"]
pub struct L2CFBLR_SPEC;
impl crate::RegisterSpec for L2CFBLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2cfblr::R](R) reader structure"]
impl crate::Readable for L2CFBLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2cfblr::W](W) writer structure"]
impl crate::Writable for L2CFBLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L2CFBLR to value 0"]
impl crate::Resettable for L2CFBLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
