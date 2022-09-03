#[doc = "Register `PMEM` reader"]
pub struct R(crate::R<PMEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMEM` writer"]
pub struct W(crate::W<PMEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMEM_SPEC>;
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
impl From<crate::W<PMEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMSET` reader - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:"]
pub type MEMSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMSET` writer - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:"]
pub type MEMSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMWAIT` reader - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
pub type MEMWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMWAIT` writer - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
pub type MEMWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMHOLD` reader - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:"]
pub type MEMHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMHOLD` writer - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:"]
pub type MEMHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMHIZ` reader - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:"]
pub type MEMHIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMHIZ` writer - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:"]
pub type MEMHIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:"]
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:"]
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:"]
    #[inline(always)]
    pub fn memset(&mut self) -> MEMSET_W<0> {
        MEMSET_W::new(self)
    }
    #[doc = "Bits 8:15 - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
    #[inline(always)]
    pub fn memwait(&mut self) -> MEMWAIT_W<8> {
        MEMWAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:"]
    #[inline(always)]
    pub fn memhold(&mut self) -> MEMHOLD_W<16> {
        MEMHOLD_W::new(self)
    }
    #[doc = "Bits 24:31 - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:"]
    #[inline(always)]
    pub fn memhiz(&mut self) -> MEMHIZ_W<24> {
        MEMHIZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem](index.html) module"]
pub struct PMEM_SPEC;
impl crate::RegisterSpec for PMEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmem::R](R) reader structure"]
impl crate::Readable for PMEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmem::W](W) writer structure"]
impl crate::Writable for PMEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMEM to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
