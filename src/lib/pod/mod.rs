/// 将 ruby 版本的 pod dep/rdep/search 给搬过来
/// 需要做的工作：
///   - 解析 Podfile.lock
///

pub(crate) mod lockfile;
pub(crate) mod dep;
