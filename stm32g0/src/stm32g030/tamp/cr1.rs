#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1E` reader - TAMP1E"]
pub type TAMP1E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1E` writer - TAMP1E"]
pub type TAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TAMP2E` reader - TAMP2E"]
pub type TAMP2E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2E` writer - TAMP2E"]
pub type TAMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ITAMP1E` reader - ITAMP1E"]
pub type ITAMP1E_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP1E` writer - ITAMP1E"]
pub type ITAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ITAMP3E` reader - ITAMP3E"]
pub type ITAMP3E_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP3E` writer - ITAMP3E"]
pub type ITAMP3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ITAMP4E` reader - ITAMP4E"]
pub type ITAMP4E_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP4E` writer - ITAMP4E"]
pub type ITAMP4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ITAMP5E` reader - ITAMP5E"]
pub type ITAMP5E_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP5E` writer - ITAMP5E"]
pub type ITAMP5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ITAMP6E` reader - ITAMP6E"]
pub type ITAMP6E_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP6E` writer - ITAMP6E"]
pub type ITAMP6E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TAMP1E"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2E"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - ITAMP1E"]
    #[inline(always)]
    pub fn itamp1e(&self) -> ITAMP1E_R {
        ITAMP1E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3E"]
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ITAMP4E"]
    #[inline(always)]
    pub fn itamp4e(&self) -> ITAMP4E_R {
        ITAMP4E_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6E"]
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1E"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<0> {
        TAMP1E_W::new(self)
    }
    #[doc = "Bit 1 - TAMP2E"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<1> {
        TAMP2E_W::new(self)
    }
    #[doc = "Bit 16 - ITAMP1E"]
    #[inline(always)]
    pub fn itamp1e(&mut self) -> ITAMP1E_W<16> {
        ITAMP1E_W::new(self)
    }
    #[doc = "Bit 18 - ITAMP3E"]
    #[inline(always)]
    pub fn itamp3e(&mut self) -> ITAMP3E_W<18> {
        ITAMP3E_W::new(self)
    }
    #[doc = "Bit 19 - ITAMP4E"]
    #[inline(always)]
    pub fn itamp4e(&mut self) -> ITAMP4E_W<19> {
        ITAMP4E_W::new(self)
    }
    #[doc = "Bit 20 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp5e(&mut self) -> ITAMP5E_W<20> {
        ITAMP5E_W::new(self)
    }
    #[doc = "Bit 21 - ITAMP6E"]
    #[inline(always)]
    pub fn itamp6e(&mut self) -> ITAMP6E_W<21> {
        ITAMP6E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0xffff_0000"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
