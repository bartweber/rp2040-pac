#[doc = "Register `CH9_DBG_CTDREQ` reader"]
pub type R = crate::R<CH9_DBG_CTDREQ_SPEC>;
#[doc = "Register `CH9_DBG_CTDREQ` writer"]
pub type W = crate::W<CH9_DBG_CTDREQ_SPEC>;
#[doc = "Field `CH9_DBG_CTDREQ` reader - "]
pub type CH9_DBG_CTDREQ_R = crate::FieldReader;
#[doc = "Field `CH9_DBG_CTDREQ` writer - "]
pub type CH9_DBG_CTDREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ch9_dbg_ctdreq(&self) -> CH9_DBG_CTDREQ_R {
        CH9_DBG_CTDREQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn ch9_dbg_ctdreq(&mut self) -> CH9_DBG_CTDREQ_W<CH9_DBG_CTDREQ_SPEC> {
        CH9_DBG_CTDREQ_W::new(self, 0)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake.  

You can [`read`](crate::generic::Reg::read) this register and get [`ch9_dbg_ctdreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_dbg_ctdreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH9_DBG_CTDREQ_SPEC;
impl crate::RegisterSpec for CH9_DBG_CTDREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch9_dbg_ctdreq::R`](R) reader structure"]
impl crate::Readable for CH9_DBG_CTDREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch9_dbg_ctdreq::W`](W) writer structure"]
impl crate::Writable for CH9_DBG_CTDREQ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets CH9_DBG_CTDREQ to value 0"]
impl crate::Resettable for CH9_DBG_CTDREQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
