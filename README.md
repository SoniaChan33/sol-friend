### Question
在 Rust 中，mod 语句用于告诉编译器包含某个模块的源文件。只有在模块被声明为 pub mod xxx; 后，其他文件（如 processor.rs）才能通过 use crate::xxx::... 访问其内容。

原因如下：

state.rs 只是物理存在于 src 目录下，只有在 lib.rs 里用 pub mod state; 声明后，state 模块才会被编译进 crate 并对外可见。
如果不在 lib.rs 里声明，crate::state 这个路径在其他文件里就找不到。
processor.rs 只是使用 state，但模块的“引入”必须在 crate 的根（即 lib.rs）声明。