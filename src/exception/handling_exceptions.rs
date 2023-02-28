#[test]
fn test() {
    // Rust 使用枚举 Result<T,E> 和 panic! 宏（通常使用）来处理异常。

    // panic!("测试panic")
    // panic! 宏可以以一个字符串作为参数，当调用执行该宏时，程序会打印出这个字符串（错误信息）以及调用栈，然后强制退出程序。

    // 在异常可以处理的时候，通常以 Result 枚举来作为函数返回值来表示，表示这个函数是否发生了异常情况。
    fn _foo() -> Result<i32, String> {
        // Result 枚举有两种情况，Ok(T) 或 Err(E)，返回 Ok(T) 时表示该函数运行正常，返回 Err(E) 时表示该函数运行出现异常。
        Result::Err(String::from("发生错误"))
    }

    fn _bar() {
        // Result 通常搭配模式匹配、方法 unwrap、方法 expect 和 ? 来处理以及使用。
        _foo().unwrap();
        // 调用 Result 枚举的 unwrap 方法会在枚举值为 Ok(T) 时进行解包，返回 T，当返回值为 Err 时，会调用 panic!
        // expect 和 unwrap 非常相似，但是 expect 可以有一个字符串参数，作为错误信息
        // foo().expect("测试错误");
    }

    // 操作符 ? 其实是一个宏，它的功能是在左边的值为 Err 时，直接返回这个值结束这个函数，如果是 Ok，就不执行任何操作，操作符有一个特点就是，方便链式调用时处理异常。
    fn _bar2() -> Result<i32, String> {
        let _a = Err(String::from("发生错误2"))?;
        Ok(1)
    }
}
