#[doc = "Register `PDCRI` reader"]
pub struct R(crate::R<PDCRI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRI` writer"]
pub struct W(crate::W<PDCRI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRI_SPEC>;
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
impl From<crate::W<PDCRI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD0` reader - Port I pull-down bit 0"]
pub type PD0_R = crate::BitReader<bool>;
#[doc = "Field `PD0` writer - Port I pull-down bit 0"]
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD1` reader - Port I pull-down bit 1"]
pub type PD1_R = crate::BitReader<bool>;
#[doc = "Field `PD1` writer - Port I pull-down bit 1"]
pub type PD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD2` reader - Port I pull-down bit 2"]
pub type PD2_R = crate::BitReader<bool>;
#[doc = "Field `PD2` writer - Port I pull-down bit 2"]
pub type PD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD3` reader - Port I pull-down bit 3"]
pub type PD3_R = crate::BitReader<bool>;
#[doc = "Field `PD3` writer - Port I pull-down bit 3"]
pub type PD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD4` reader - Port I pull-down bit 4"]
pub type PD4_R = crate::BitReader<bool>;
#[doc = "Field `PD4` writer - Port I pull-down bit 4"]
pub type PD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD5` reader - Port I pull-down bit 5"]
pub type PD5_R = crate::BitReader<bool>;
#[doc = "Field `PD5` writer - Port I pull-down bit 5"]
pub type PD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD6` reader - Port I pull-down bit 6"]
pub type PD6_R = crate::BitReader<bool>;
#[doc = "Field `PD6` writer - Port I pull-down bit 6"]
pub type PD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD7` reader - Port I pull-down bit 7"]
pub type PD7_R = crate::BitReader<bool>;
#[doc = "Field `PD7` writer - Port I pull-down bit 7"]
pub type PD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD8` reader - Port I pull-down bit 8"]
pub type PD8_R = crate::BitReader<bool>;
#[doc = "Field `PD8` writer - Port I pull-down bit 8"]
pub type PD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD9` reader - Port I pull-down bit 9"]
pub type PD9_R = crate::BitReader<bool>;
#[doc = "Field `PD9` writer - Port I pull-down bit 9"]
pub type PD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD10` reader - Port I pull-down bit 10"]
pub type PD10_R = crate::BitReader<bool>;
#[doc = "Field `PD10` writer - Port I pull-down bit 10"]
pub type PD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
#[doc = "Field `PD11` reader - Port I pull-down bit 11"]
pub type PD11_R = crate::BitReader<bool>;
#[doc = "Field `PD11` writer - Port I pull-down bit 11"]
pub type PD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRI_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port I pull-down bit 0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port I pull-down bit 1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port I pull-down bit 2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port I pull-down bit 3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port I pull-down bit 4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port I pull-down bit 5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port I pull-down bit 6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port I pull-down bit 7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port I pull-down bit 8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port I pull-down bit 9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port I pull-down bit 10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port I pull-down bit 11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port I pull-down bit 0"]
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    #[doc = "Bit 1 - Port I pull-down bit 1"]
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    #[doc = "Bit 2 - Port I pull-down bit 2"]
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    #[doc = "Bit 3 - Port I pull-down bit 3"]
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    #[doc = "Bit 4 - Port I pull-down bit 4"]
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    #[doc = "Bit 5 - Port I pull-down bit 5"]
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W<5> {
        PD5_W::new(self)
    }
    #[doc = "Bit 6 - Port I pull-down bit 6"]
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    #[doc = "Bit 7 - Port I pull-down bit 7"]
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    #[doc = "Bit 8 - Port I pull-down bit 8"]
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W<8> {
        PD8_W::new(self)
    }
    #[doc = "Bit 9 - Port I pull-down bit 9"]
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W<9> {
        PD9_W::new(self)
    }
    #[doc = "Bit 10 - Port I pull-down bit 10"]
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W<10> {
        PD10_W::new(self)
    }
    #[doc = "Bit 11 - Port I pull-down bit 11"]
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W<11> {
        PD11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port I pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcri](index.html) module"]
pub struct PDCRI_SPEC;
impl crate::RegisterSpec for PDCRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcri::R](R) reader structure"]
impl crate::Readable for PDCRI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcri::W](W) writer structure"]
impl crate::Writable for PDCRI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDCRI to value 0"]
impl crate::Resettable for PDCRI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}