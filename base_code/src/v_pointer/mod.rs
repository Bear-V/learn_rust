mod example_box;
mod example_drop;
mod example_rc;
mod example_rc_and_ref_cell;
mod example_ref_cell;

// 类似于 Rc<T>，RefCell<T> 只能用于单线程场景。
// 如果尝试在多线程上下文中使用RefCell<T>，会得到一个编译错误。
// 第十六章会介绍如何在多线程程序中使用 RefCell<T> 的功能。
//
// 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
//
// Rc<T> 允许相同数据有多个所有者；
// Box<T> 和 RefCell<T> 有单一所有者。
//
// Box<T> 允许在编译时执行不可变或可变借用检查；
// Rc<T>仅允许在编译时执行不可变借用检查；
// RefCell<T> 允许在运行时执行不可变或可变借用检查。
//
// 因为 RefCell<T> 允许在运行时执行可变借用检查，
// 所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
//
// 在不可变值内部改变值就是 内部可变性 模式。让我们看看何时内部可变性是有用的，并讨论这是如何成为可能的。
