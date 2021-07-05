use std::fs::File;

mod a_variable;
mod b_data_type;
mod c_function;
mod d_control_flow;
mod e_loops;
mod f_ownership;
mod g_quote_borrow;
mod h_slice;
mod i_struct;
mod j_enums;
mod k_match;
mod l_if_let;
mod m_collections;
mod n_string;
mod o_hashmap;
mod p_panic;

fn main() {
    variable();
    println!("===================");
    data_type();
    println!("===================");
    function();
    println!("===============");
    control_flow();
    println!("==============");
    loops_fun();
    println!("==============");
    ownership_fun();
    println!("==============");
    quote_and_borrow();
    println!("==============");
    slice_fun();
    println!("==============");
    struct_fun();
    println!("==============");
    enums_fun();
    println!("==============");

    println!("the last step is main.rs");
    // let _f = File::open("hello.txt").unwrap();
}

// enums
fn enums_fun() {
    j_enums::example()
}

// 结构体 struct
fn struct_fun() {
    i_struct::init_struct_fun()
}

// 切片类型 slice
fn slice_fun() {
    let abc = 'a';
    let str = String::from("this is slice function of test");
    let len = h_slice::slice_string(&str);
    println!("this len is :{},{}", len, abc);

    h_slice::slice_1st();
    h_slice::string_slice();
}

// 引用借用
fn quote_and_borrow() {
    g_quote_borrow::mutable_quote()
}

// 所有权
fn ownership_fun() {
    f_ownership::change_string();
    f_ownership::clone_string();
    f_ownership::copy_fun();
    f_ownership::change_string();
    f_ownership::move_string_fun();
    f_ownership::assignment_fun();
    f_ownership::move_ownership();
    f_ownership::move_tup();
    f_ownership::new_move_tup();
    f_ownership::quote();
}

// 循环
fn loops_fun() {
    e_loops::loop_fun();
    e_loops::while_fun();
    e_loops::for_in_fun();
    e_loops::for_in_rev_fun();
    let num = 10;
    let result = e_loops::fb(num);
    println!("斐波那契:{}:{}", num, result)
}

// 控制流 if 循环
fn control_flow() {
    d_control_flow::if_function();
    d_control_flow::let_if_fun()
}

// 函数
fn function() {
    c_function::lambda();
    println!("function five is :{:?}", c_function::five(127));
    println!("function five is :{:?}", c_function::six(1234))
}

// 数据类型
fn data_type() {
    b_data_type::int();
    b_data_type::math();
    b_data_type::different_string();
    b_data_type::tup_type();
    b_data_type::array_type();
    b_data_type::same_array()
}

// 可变变量和不可变变量
fn variable() {
    a_variable::immutable_var();
    a_variable::var();
    a_variable::constant();
    a_variable::shadowing();
}
