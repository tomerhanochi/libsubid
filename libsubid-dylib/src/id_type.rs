#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum IdType {
    Uid = 1,
    Gid = 2,
}
