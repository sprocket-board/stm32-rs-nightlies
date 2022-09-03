#[doc = "Register `DDRCTRL_PCFGWQOS0_0` reader"]
pub struct R(crate::R<DDRCTRL_PCFGWQOS0_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCFGWQOS0_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCFGWQOS0_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCFGWQOS0_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCFGWQOS0_0` writer"]
pub struct W(crate::W<DDRCTRL_PCFGWQOS0_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCFGWQOS0_0_SPEC>;
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
impl From<crate::W<DDRCTRL_PCFGWQOS0_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCFGWQOS0_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WQOS_MAP_LEVEL1` reader - WQOS_MAP_LEVEL1"]
pub type WQOS_MAP_LEVEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WQOS_MAP_LEVEL1` writer - WQOS_MAP_LEVEL1"]
pub type WQOS_MAP_LEVEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS0_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `WQOS_MAP_LEVEL2` reader - WQOS_MAP_LEVEL2"]
pub type WQOS_MAP_LEVEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WQOS_MAP_LEVEL2` writer - WQOS_MAP_LEVEL2"]
pub type WQOS_MAP_LEVEL2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS0_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `WQOS_MAP_REGION0` reader - WQOS_MAP_REGION0"]
pub type WQOS_MAP_REGION0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WQOS_MAP_REGION0` writer - WQOS_MAP_REGION0"]
pub type WQOS_MAP_REGION0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS0_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WQOS_MAP_REGION1` reader - WQOS_MAP_REGION1"]
pub type WQOS_MAP_REGION1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WQOS_MAP_REGION1` writer - WQOS_MAP_REGION1"]
pub type WQOS_MAP_REGION1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS0_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WQOS_MAP_REGION2` reader - WQOS_MAP_REGION2"]
pub type WQOS_MAP_REGION2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WQOS_MAP_REGION2` writer - WQOS_MAP_REGION2"]
pub type WQOS_MAP_REGION2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS0_0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - WQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn wqos_map_level1(&self) -> WQOS_MAP_LEVEL1_R {
        WQOS_MAP_LEVEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn wqos_map_level2(&self) -> WQOS_MAP_LEVEL2_R {
        WQOS_MAP_LEVEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - WQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn wqos_map_region0(&self) -> WQOS_MAP_REGION0_R {
        WQOS_MAP_REGION0_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - WQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn wqos_map_region1(&self) -> WQOS_MAP_REGION1_R {
        WQOS_MAP_REGION1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - WQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn wqos_map_region2(&self) -> WQOS_MAP_REGION2_R {
        WQOS_MAP_REGION2_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - WQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn wqos_map_level1(&mut self) -> WQOS_MAP_LEVEL1_W<0> {
        WQOS_MAP_LEVEL1_W::new(self)
    }
    #[doc = "Bits 8:11 - WQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn wqos_map_level2(&mut self) -> WQOS_MAP_LEVEL2_W<8> {
        WQOS_MAP_LEVEL2_W::new(self)
    }
    #[doc = "Bits 16:17 - WQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn wqos_map_region0(&mut self) -> WQOS_MAP_REGION0_W<16> {
        WQOS_MAP_REGION0_W::new(self)
    }
    #[doc = "Bits 20:21 - WQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn wqos_map_region1(&mut self) -> WQOS_MAP_REGION1_W<20> {
        WQOS_MAP_REGION1_W::new(self)
    }
    #[doc = "Bits 24:25 - WQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn wqos_map_region2(&mut self) -> WQOS_MAP_REGION2_W<24> {
        WQOS_MAP_REGION2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port 0 write Q0S configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgwqos0_0](index.html) module"]
pub struct DDRCTRL_PCFGWQOS0_0_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCFGWQOS0_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pcfgwqos0_0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGWQOS0_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgwqos0_0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGWQOS0_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGWQOS0_0 to value 0x0e00"]
impl crate::Resettable for DDRCTRL_PCFGWQOS0_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e00
    }
}
