#[test]
fn test() {
    // 生命周期由借用检查来检查，一旦出现引用的生命周期比被引用值的生命周期长的情况，就报错。
    // 借用检查在编译器执行。

    // 用这个函数来作为例子，它返回两个字符串中，最长的一个。
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    // 这个函数无法通过编译的原因是，编译器无法推断两个实参引用的值的生命周期，所以无法进行借用检查，比如如下的代码就可能导致出错，如果还是用上面这个函数的话。
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    // 显然 reuslt 的声明周期是比 string2 长的，但若 longest 返回的 string2 比 string1，那么运行到 println! 时，string2 就已经被释放了，此时 result 显然是悬垂指针，这显然是 Rust 所不允许的。

    // 所以生命周期应运而生，在函数名之后，它像泛型参数一样定义，在参数上，它在借用符号后面给出。
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // 生命周期符号 'a 代表，参数中所有标注了生命周期的参数的生命周期的最小值，然后将 'a 标注给返回值时，表示返回值的周期不能比 'a 长。
    // 所以下面这段代码，string2.as_str() 会被报错，因为它活得太短了，或者可以认为是 result 活得太长了.
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // 下面这样就不会报错了。
    let string1 = String::from("long string is long");
    {
        let result;
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // 静态生命周期是可以无视生命周期标注的存在。
    fn _longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else if x.len() == y.len() {
            "它俩一样长" // 声明周期为 'static
        } else {
            y
        }
    }

    // 结构体的引用类型字段要标注生命周期，表示其声明周期不小于实例化的结构体的生命周期。
}
