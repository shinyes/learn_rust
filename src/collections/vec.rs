#[test]
fn test() {
    // Vec 是有序的变长的数组，由于其是变长的，所以其肯定是存储在堆中的。
    let v1 = vec![1, 2, 3];
    println!("{:?}", v1); // Vec<i32>没有实现 Display 所以只能使用 Debug（`{:?}`）来输出。

    println!("----");

    // 不用 tubor fish 语法也就是 `::<T>` 显式的给出这个变量的类型，编译器也可以在下面通过第一次给这个引用添加的元素的类型来推测。但一定要记住，将引用声明为 mut 类型
    let mut v2 = Vec::<i32>::new();
    // 可以使用 push 来在 Vec 的最后添加元素。
    v2.push(1);
    println!("{:?}", v2);
    println!("----");

    println!("{}", v2[0]);
    // 可以用索引的这种方式来获取或修改 Vec 中的元素。
    v2[0] = 2;
    println!("{}", v2[0]);

    println!("----");
    // 可以用 get 和 get_mut 来获取 Vec 中的元素，但返回的并不是元素，而是包裹元素的 Option 枚举，通常需要使用 if let 来解构和判断，这种方式可以避免索引超出范围。
    *v2.get_mut(0).unwrap() = 1; // get_mut 获取的是包裹元素的可变引用的 Option 枚举，所以在这个位置使用 unwrap 拆开之后，还需要使用解引用符号。
    println!("{:?}", v2.get(0));

    println!("----");
    // 遍历 Vec 中的元素， v1.iter() 可以改为 &v1，后者是前者的语法糖，类似的，&mut v1 是 v1.iter_mut() 的语法糖，如果直接写 v1，那么 v 的所有权将会转移。
    for e in v1.iter() {
        println!("{}", e);
    }
}
