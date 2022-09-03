#[doc = "Register `AHB3ENR` reader"]
pub struct R(crate::R<AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3ENR` writer"]
pub struct W(crate::W<AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3ENR_SPEC>;
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
impl From<crate::W<AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCEN` reader - Flexible memory controller clock enable"]
pub type FMCEN_R = crate::BitReader<bool>;
#[doc = "Field `FMCEN` writer - Flexible memory controller clock enable"]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3ENR_SPEC, bool, O>;
#[doc = "Field `OSPI1EN` reader - OSPI1 memory interface clock enable"]
pub type OSPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `OSPI1EN` writer - OSPI1 memory interface clock enable"]
pub type OSPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3ENR_SPEC, bool, O>;
#[doc = "Field `OSPI2EN` reader - OSPI2EN memory interface clock enable"]
pub type OSPI2EN_R = crate::BitReader<bool>;
#[doc = "Field `OSPI2EN` writer - OSPI2EN memory interface clock enable"]
pub type OSPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - OSPI1 memory interface clock enable"]
    #[inline(always)]
    pub fn ospi1en(&self) -> OSPI1EN_R {
        OSPI1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OSPI2EN memory interface clock enable"]
    #[inline(always)]
    pub fn ospi2en(&self) -> OSPI2EN_R {
        OSPI2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<0> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 8 - OSPI1 memory interface clock enable"]
    #[inline(always)]
    pub fn ospi1en(&mut self) -> OSPI1EN_W<8> {
        OSPI1EN_W::new(self)
    }
    #[doc = "Bit 9 - OSPI2EN memory interface clock enable"]
    #[inline(always)]
    pub fn ospi2en(&mut self) -> OSPI2EN_W<9> {
        OSPI2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3enr](index.html) module"]
pub struct AHB3ENR_SPEC;
impl crate::RegisterSpec for AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3enr::R](R) reader structure"]
impl crate::Readable for AHB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3enr::W](W) writer structure"]
impl crate::Writable for AHB3ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for AHB3ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}