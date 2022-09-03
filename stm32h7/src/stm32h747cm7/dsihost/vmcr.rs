#[doc = "Register `VMCR` reader"]
pub struct R(crate::R<VMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VMCR` writer"]
pub struct W(crate::W<VMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VMCR_SPEC>;
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
impl From<crate::W<VMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VMT` reader - Video mode type"]
pub type VMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VMT` writer - Video mode type"]
pub type VMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VMCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPVSAE` reader - Low-power vertical sync active enable"]
pub type LPVSAE_R = crate::BitReader<bool>;
#[doc = "Field `LPVSAE` writer - Low-power vertical sync active enable"]
pub type LPVSAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `LPVBPE` reader - Low-power vertical back-porch enable"]
pub type LPVBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVBPE` writer - Low-power vertical back-porch enable"]
pub type LPVBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `LPVFPE` reader - Low-power vertical front-porch enable"]
pub type LPVFPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVFPE` writer - Low-power vertical front-porch enable"]
pub type LPVFPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `LPVAE` reader - Low-power vertical active enable"]
pub type LPVAE_R = crate::BitReader<bool>;
#[doc = "Field `LPVAE` writer - Low-power vertical active enable"]
pub type LPVAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `LPHBPE` reader - Low-power horizontal back-porch enable"]
pub type LPHBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPHBPE` writer - Low-power horizontal back-porch enable"]
pub type LPHBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `LPHFPE` reader - Low-power horizontal front-porch enable"]
pub type LPHFPE_R = crate::BitReader<bool>;
#[doc = "Field `LPHFPE` writer - Low-power horizontal front-porch enable"]
pub type LPHFPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `FBTAAE` reader - Frame bus-turn-around acknowledge enable"]
pub type FBTAAE_R = crate::BitReader<bool>;
#[doc = "Field `FBTAAE` writer - Frame bus-turn-around acknowledge enable"]
pub type FBTAAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `LPCE` reader - Low-power command enable"]
pub type LPCE_R = crate::BitReader<bool>;
#[doc = "Field `LPCE` writer - Low-power command enable"]
pub type LPCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `PGE` reader - Pattern generator enable"]
pub type PGE_R = crate::BitReader<bool>;
#[doc = "Field `PGE` writer - Pattern generator enable"]
pub type PGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `PGM` reader - Pattern generator mode"]
pub type PGM_R = crate::BitReader<bool>;
#[doc = "Field `PGM` writer - Pattern generator mode"]
pub type PGM_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `PGO` reader - Pattern generator orientation"]
pub type PGO_R = crate::BitReader<bool>;
#[doc = "Field `PGO` writer - Pattern generator orientation"]
pub type PGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Video mode type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Low-power vertical sync active enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power vertical back-porch enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low-power vertical front-porch enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low-power vertical active enable"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Low-power horizontal back-porch enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Low-power horizontal front-porch enable"]
    #[inline(always)]
    pub fn lphfpe(&self) -> LPHFPE_R {
        LPHFPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Frame bus-turn-around acknowledge enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Low-power command enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pattern generator enable"]
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Pattern generator mode"]
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Pattern generator orientation"]
    #[inline(always)]
    pub fn pgo(&self) -> PGO_R {
        PGO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Video mode type"]
    #[inline(always)]
    pub fn vmt(&mut self) -> VMT_W<0> {
        VMT_W::new(self)
    }
    #[doc = "Bit 8 - Low-power vertical sync active enable"]
    #[inline(always)]
    pub fn lpvsae(&mut self) -> LPVSAE_W<8> {
        LPVSAE_W::new(self)
    }
    #[doc = "Bit 9 - Low-power vertical back-porch enable"]
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<9> {
        LPVBPE_W::new(self)
    }
    #[doc = "Bit 10 - Low-power vertical front-porch enable"]
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<10> {
        LPVFPE_W::new(self)
    }
    #[doc = "Bit 11 - Low-power vertical active enable"]
    #[inline(always)]
    pub fn lpvae(&mut self) -> LPVAE_W<11> {
        LPVAE_W::new(self)
    }
    #[doc = "Bit 12 - Low-power horizontal back-porch enable"]
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LPHBPE_W<12> {
        LPHBPE_W::new(self)
    }
    #[doc = "Bit 13 - Low-power horizontal front-porch enable"]
    #[inline(always)]
    pub fn lphfpe(&mut self) -> LPHFPE_W<13> {
        LPHFPE_W::new(self)
    }
    #[doc = "Bit 14 - Frame bus-turn-around acknowledge enable"]
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FBTAAE_W<14> {
        FBTAAE_W::new(self)
    }
    #[doc = "Bit 15 - Low-power command enable"]
    #[inline(always)]
    pub fn lpce(&mut self) -> LPCE_W<15> {
        LPCE_W::new(self)
    }
    #[doc = "Bit 16 - Pattern generator enable"]
    #[inline(always)]
    pub fn pge(&mut self) -> PGE_W<16> {
        PGE_W::new(self)
    }
    #[doc = "Bit 20 - Pattern generator mode"]
    #[inline(always)]
    pub fn pgm(&mut self) -> PGM_W<20> {
        PGM_W::new(self)
    }
    #[doc = "Bit 24 - Pattern generator orientation"]
    #[inline(always)]
    pub fn pgo(&mut self) -> PGO_W<24> {
        PGO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host video mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmcr](index.html) module"]
pub struct VMCR_SPEC;
impl crate::RegisterSpec for VMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmcr::R](R) reader structure"]
impl crate::Readable for VMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vmcr::W](W) writer structure"]
impl crate::Writable for VMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VMCR to value 0"]
impl crate::Resettable for VMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
