#[test]
fn test() {
    // hashmap 用于存储键值对，就像 python 中的字典一样，
    // 不过 Rust 不像 python，Rust 不是动态类型，所以键和键的类型必须相同，值和值的类型必须相同，且是固定的，
    // 所以如果想实现键和值的多类型的情况，就像是 python 那样，那就可以使用 Rust 强大的枚举类型。

    // HashMap 不像是 Vec，必须手动导入符号，因为他并没有被 prelude.
    use std::collections::HashMap;

    // 可以通过 new 这个关联方法来获取一个 HashMap 变量，并将其绑定给一个引用。
    let mut a_map = HashMap::new(); // 这里既没有使用 turbo fish，也没有显式的给出 HashMap 中的键和值的类型，而是让编译器根据之后的代码去猜。

    // instert 方法用于添加键值对，但是要注意这可能会导致变量的所有权转移，如果这个变量没有实现 copy trait，而实现了 copy trait 的话，就会复制一份，然后再添加进去了。
    a_map.insert("hello", "你好");
    println!("{:?}", a_map);

    // 可以在创建变量时就为其设定好空间，这样就能避免频繁的内存分配和内存拷贝，Vec 也是这样。
    let mut a_map = HashMap::with_capacity(5);
    a_map.insert("hello", "你好");

    // 可以将一个 Vec 转变为 HashMap，前提是 Vec 中的元素是元组，且符合组成 HashMap 的要求。
    let v = vec![("hello", "你好"), ("world", "世界")];
    // 迭代器的 collect 方法需要我们告诉它，要转变为什么类型，可以用 turbo fish 也可以直接给引用声明类型，并且我们可以用下划线来让编译器自己去推断键和值的类型。
    let mut a_map = v.into_iter().collect::<HashMap<_, _>>();

    // HashMap 有一系列的 get 方法来获取各种各样的值，不过由于 HashMap 的键的类型很可能不是实现了 copy 的类型，
    // 比如这几个例子中，所以这些方法的参数的类型都是键的引用类型，如果不这样，那肯定会导致参数的所有权被转移
    let _a_v = a_map.get("hello").unwrap_or(&"default");

    // insert 可以用于更新已有键的值。
    a_map.insert("hello", "konnichiha");
    println!("{:?}", a_map);

    // 可以用下面这种方式来判断是否已有某个键，并且在没有这个键时添加这个键值对，
    // 不过我搞不懂，为什么 entry 方法返回的是 Entry 结构体，却可以调用 or_inster 为原本的 HashMap 添加键值对，大概是这个 Entry 中存储了 HashMap 的引用。
    a_map.entry("你好世界").or_insert("hello world");
    println!("{:?}", a_map);
}
