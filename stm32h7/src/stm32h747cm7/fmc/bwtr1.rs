#[doc = "Register `BWTR1` reader"]
pub struct R(crate::R<BWTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BWTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BWTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BWTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BWTR1` writer"]
pub struct W(crate::W<BWTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BWTR1_SPEC>;
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
impl From<crate::W<BWTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BWTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDSET` reader - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in KCK_FMC cycles (refer to Figure81 to Figure93), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
pub type ADDSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDSET` writer - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in KCK_FMC cycles (refer to Figure81 to Figure93), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
pub type ADDSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDHLD` reader - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
pub type ADDHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDHLD` writer - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
pub type ADDHLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATAST` reader - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:"]
pub type DATAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAST` writer - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:"]
pub type DATAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (tEHEL from ENx high to ENx low): (BUSTRUN + 1) KCK_FMC period &#8805; tEHELmin. The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed mode and mode D. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank A synchronous write transfer ((in Burst or Single mode) and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write transfer (in Burst or Single mode) and a synchronous read from the same or a different bank. ..."]
pub type BUSTURN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (tEHEL from ENx high to ENx low): (BUSTRUN + 1) KCK_FMC period &#8805; tEHELmin. The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed mode and mode D. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank A synchronous write transfer ((in Burst or Single mode) and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write transfer (in Burst or Single mode) and a synchronous read from the same or a different bank. ..."]
pub type BUSTURN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACCMOD` reader - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACCMOD` writer - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in KCK_FMC cycles (refer to Figure81 to Figure93), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:"]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (tEHEL from ENx high to ENx low): (BUSTRUN + 1) KCK_FMC period &#8805; tEHELmin. The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed mode and mode D. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank A synchronous write transfer ((in Burst or Single mode) and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write transfer (in Burst or Single mode) and a synchronous read from the same or a different bank. ..."]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in KCK_FMC cycles (refer to Figure81 to Figure93), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W<0> {
        ADDSET_W::new(self)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W<4> {
        ADDHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:"]
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W<8> {
        DATAST_W::new(self)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (tEHEL from ENx high to ENx low): (BUSTRUN + 1) KCK_FMC period &#8805; tEHELmin. The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed mode and mode D. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank A synchronous write transfer ((in Burst or Single mode) and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write transfer (in Burst or Single mode) and a synchronous read from the same or a different bank. ..."]
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W<16> {
        BUSTURN_W::new(self)
    }
    #[doc = "Bits 28:29 - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W<28> {
        ACCMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bwtr1](index.html) module"]
pub struct BWTR1_SPEC;
impl crate::RegisterSpec for BWTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bwtr1::R](R) reader structure"]
impl crate::Readable for BWTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bwtr1::W](W) writer structure"]
impl crate::Writable for BWTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BWTR1 to value 0x0fff_ffff"]
impl crate::Resettable for BWTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_ffff
    }
}
