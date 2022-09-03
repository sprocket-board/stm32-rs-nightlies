#[doc = "Register `ECCR` reader"]
pub struct R(crate::R<ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCR` writer"]
pub struct W(crate::W<ECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCR_SPEC>;
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
impl From<crate::W<ECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub type ADDR_ECC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYSF_ECC` reader - System Flash ECC fail"]
pub type SYSF_ECC_R = crate::BitReader<bool>;
#[doc = "Field `ECCCIE` reader - ECC correction interrupt enable"]
pub type ECCCIE_R = crate::BitReader<bool>;
#[doc = "Field `ECCCIE` writer - ECC correction interrupt enable"]
pub type ECCCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, bool, O>;
#[doc = "Field `CPUID` reader - CPU identification"]
pub type CPUID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCC` reader - ECC correction"]
pub type ECCC_R = crate::BitReader<bool>;
#[doc = "Field `ECCC` writer - ECC correction"]
pub type ECCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, bool, O>;
#[doc = "Field `ECCD` reader - ECC detection"]
pub type ECCD_R = crate::BitReader<bool>;
#[doc = "Field `ECCD` writer - ECC detection"]
pub type ECCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:16 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 20 - System Flash ECC fail"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 26:28 - CPU identification"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ecccie(&mut self) -> ECCCIE_W<24> {
        ECCCIE_W::new(self)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W<30> {
        ECCC_W::new(self)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W<31> {
        ECCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash ECC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr](index.html) module"]
pub struct ECCR_SPEC;
impl crate::RegisterSpec for ECCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccr::R](R) reader structure"]
impl crate::Readable for ECCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccr::W](W) writer structure"]
impl crate::Writable for ECCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for ECCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
