// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



struct Wrapper<T> {
    value: T,
}

/*
pub fn new(value: T) -> Wrapper<T> 定义了一个名为 new 的关联函数（使用 pub 关键字使其在结构体外部可访问）。关联函数是通过结构体类型直接调用的函数，而不是通过结构体实例调用。
参数 value：该函数接受一个类型为 T 的参数 value。
返回值：返回一个 Wrapper<T> 类型的实例。在函数体 Wrapper { value } 中，使用传入的 value 创建并返回一个新的 Wrapper 结构体实例，其中 value 字段被初始化为传入的 value。
*/
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Wrapper<T> {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
        //（使用 pub 关键字使其在结构体外部可访问）。关联函数是通过结构体类型直接调用的函数，而不是通过结构体实例调用。
    }

    //Wrapper::new("Foo").value:这是结构体取value值，Wrapper::new("Foo")会返回一个Wrapper结构体，“.value”就是取值
}
