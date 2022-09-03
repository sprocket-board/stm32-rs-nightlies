#[doc = "Register `ICACHE_CRR3` reader"]
pub struct R(crate::R<ICACHE_CRR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_CRR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_CRR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_CRR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_CRR3` writer"]
pub struct W(crate::W<ICACHE_CRR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_CRR3_SPEC>;
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
impl From<crate::W<ICACHE_CRR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_CRR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDR` reader - BASEADDR"]
pub type BASEADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASEADDR` writer - BASEADDR"]
pub type BASEADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICACHE_CRR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSIZE` reader - RSIZE"]
pub type RSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSIZE` writer - RSIZE"]
pub type RSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICACHE_CRR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `REN` reader - REN"]
pub type REN_R = crate::BitReader<bool>;
#[doc = "Field `REN` writer - REN"]
pub type REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CRR3_SPEC, bool, O>;
#[doc = "Field `REMAPADDR` reader - REMAPADDR"]
pub type REMAPADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REMAPADDR` writer - REMAPADDR"]
pub type REMAPADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ICACHE_CRR3_SPEC, u16, u16, 11, O>;
#[doc = "Field `MSTSEL` reader - MSTSEL"]
pub type MSTSEL_R = crate::BitReader<bool>;
#[doc = "Field `MSTSEL` writer - MSTSEL"]
pub type MSTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CRR3_SPEC, bool, O>;
#[doc = "Field `HBURST` reader - HBURST"]
pub type HBURST_R = crate::BitReader<bool>;
#[doc = "Field `HBURST` writer - HBURST"]
pub type HBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CRR3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:11 - RSIZE"]
    #[inline(always)]
    pub fn rsize(&self) -> RSIZE_R {
        RSIZE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 15 - REN"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - REMAPADDR"]
    #[inline(always)]
    pub fn remapaddr(&self) -> REMAPADDR_R {
        REMAPADDR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - MSTSEL"]
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BASEADDR_W<0> {
        BASEADDR_W::new(self)
    }
    #[doc = "Bits 9:11 - RSIZE"]
    #[inline(always)]
    pub fn rsize(&mut self) -> RSIZE_W<9> {
        RSIZE_W::new(self)
    }
    #[doc = "Bit 15 - REN"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W<15> {
        REN_W::new(self)
    }
    #[doc = "Bits 16:26 - REMAPADDR"]
    #[inline(always)]
    pub fn remapaddr(&mut self) -> REMAPADDR_W<16> {
        REMAPADDR_W::new(self)
    }
    #[doc = "Bit 28 - MSTSEL"]
    #[inline(always)]
    pub fn mstsel(&mut self) -> MSTSEL_W<28> {
        MSTSEL_W::new(self)
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    pub fn hburst(&mut self) -> HBURST_W<31> {
        HBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICACHE region configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_crr3](index.html) module"]
pub struct ICACHE_CRR3_SPEC;
impl crate::RegisterSpec for ICACHE_CRR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_crr3::R](R) reader structure"]
impl crate::Readable for ICACHE_CRR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_crr3::W](W) writer structure"]
impl crate::Writable for ICACHE_CRR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_CRR3 to value 0x0200"]
impl crate::Resettable for ICACHE_CRR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
