use crate::{Register, Uart16550IO, RBR_THR};

impl<R: Register> RBR_THR<R> {
    /// 从接收缓冲寄存器读取字符。
    #[inline]
    pub fn rx_data(&self, io_region: &dyn Uart16550IO<R>) -> u8 {
        io_region.read_at(self.offset).val()
    }

    /// 向发送缓冲寄存器写入字符。
    #[inline]
    pub fn tx_data(&self, io_region: &dyn Uart16550IO<R>, val: u8) {
        io_region.write_at(self.offset, val.into());
    }

    pub(crate) fn write(&self, io_region: &dyn Uart16550IO<R>, val: R) {
        io_region.write_at(self.offset, val);
    }
}
