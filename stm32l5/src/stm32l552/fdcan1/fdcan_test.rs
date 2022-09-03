#[doc = "Register `FDCAN_TEST` reader"]
pub struct R(crate::R<FDCAN_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TEST` writer"]
pub struct W(crate::W<FDCAN_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TEST_SPEC>;
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
impl From<crate::W<FDCAN_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBCK` reader - Loop Back mode"]
pub type LBCK_R = crate::BitReader<bool>;
#[doc = "Field `LBCK` writer - Loop Back mode"]
pub type LBCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TEST_SPEC, bool, O>;
#[doc = "Field `TX` reader - Loop Back mode"]
pub type TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX` writer - Loop Back mode"]
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX` reader - Control of Transmit Pin"]
pub type RX_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - Loop Back mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Loop Back mode"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back mode"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LBCK_W<4> {
        LBCK_W::new(self)
    }
    #[doc = "Bits 5:6 - Loop Back mode"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W<5> {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_test](index.html) module"]
pub struct FDCAN_TEST_SPEC;
impl crate::RegisterSpec for FDCAN_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_test::R](R) reader structure"]
impl crate::Readable for FDCAN_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_test::W](W) writer structure"]
impl crate::Writable for FDCAN_TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TEST to value 0x10"]
impl crate::Resettable for FDCAN_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
