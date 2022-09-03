#[doc = "Register `HASH_SR` reader"]
pub struct R(crate::R<HASH_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_SR` writer"]
pub struct W(crate::W<HASH_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_SR_SPEC>;
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
impl From<crate::W<HASH_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DINIS` reader - DINIS"]
pub type DINIS_R = crate::BitReader<bool>;
#[doc = "Field `DINIS` writer - DINIS"]
pub type DINIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_SR_SPEC, bool, O>;
#[doc = "Field `DCIS` reader - DCIS"]
pub type DCIS_R = crate::BitReader<bool>;
#[doc = "Field `DCIS` writer - DCIS"]
pub type DCIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_SR_SPEC, bool, O>;
#[doc = "Field `DMAS` reader - DMAS"]
pub type DMAS_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DINIS"]
    #[inline(always)]
    pub fn dinis(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCIS"]
    #[inline(always)]
    pub fn dcis(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAS"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DINIS"]
    #[inline(always)]
    pub fn dinis(&mut self) -> DINIS_W<0> {
        DINIS_W::new(self)
    }
    #[doc = "Bit 1 - DCIS"]
    #[inline(always)]
    pub fn dcis(&mut self) -> DCIS_W<1> {
        DCIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_sr](index.html) module"]
pub struct HASH_SR_SPEC;
impl crate::RegisterSpec for HASH_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_sr::R](R) reader structure"]
impl crate::Readable for HASH_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_sr::W](W) writer structure"]
impl crate::Writable for HASH_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_SR to value 0x01"]
impl crate::Resettable for HASH_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
