#[doc = "Register `RCC_TZCR` reader"]
pub struct R(crate::R<RCC_TZCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_TZCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_TZCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_TZCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_TZCR` writer"]
pub struct W(crate::W<RCC_TZCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_TZCR_SPEC>;
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
impl From<crate::W<RCC_TZCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_TZCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZEN` reader - TZEN"]
pub type TZEN_R = crate::BitReader<bool>;
#[doc = "Field `TZEN` writer - TZEN"]
pub type TZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_TZCR_SPEC, bool, O>;
#[doc = "Field `MCKPROT` reader - MCKPROT"]
pub type MCKPROT_R = crate::BitReader<bool>;
#[doc = "Field `MCKPROT` writer - MCKPROT"]
pub type MCKPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_TZCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TZEN"]
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCKPROT"]
    #[inline(always)]
    pub fn mckprot(&self) -> MCKPROT_R {
        MCKPROT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZEN"]
    #[inline(always)]
    pub fn tzen(&mut self) -> TZEN_W<0> {
        TZEN_W::new(self)
    }
    #[doc = "Bit 1 - MCKPROT"]
    #[inline(always)]
    pub fn mckprot(&mut self) -> MCKPROT_W<1> {
        MCKPROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_tzcr](index.html) module"]
pub struct RCC_TZCR_SPEC;
impl crate::RegisterSpec for RCC_TZCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_tzcr::R](R) reader structure"]
impl crate::Readable for RCC_TZCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_tzcr::W](W) writer structure"]
impl crate::Writable for RCC_TZCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_TZCR to value 0x03"]
impl crate::Resettable for RCC_TZCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
