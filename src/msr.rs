use crate::{Register, Uart16550IO, MSR};

impl<R: Register> MSR<R> {
    /// 读取调制解调器状态。
    #[inline]
    pub fn read(&self, io_region: &dyn Uart16550IO<R>) -> ModemStatus {
        let val = io_region.read_at(self.offset).val();
        ModemStatus(val)
    }
}

/// 调制解调器状态。
///
/// TODO: 一般用不到，未实现方法，可以取出值操作。
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct ModemStatus(pub u8);
