#[test]
fn test() {
    // 基本类型的转换通常使用 as 操作符，但可能出现值溢出的问题
    let a = 128;
    let b = a as i8;
    println!("{}", b); // -127
    println!("i8 的最大值为 {}，最小值为 {}", i8::MAX, i8::MIN);

    // 如果担心出现值溢出的异常，可以使用 try_into 方法，其返回值是 Result 枚举值
    // let b2: i8 = a.try_into().unwrap();

    let c = [1, 2, 3];
    // 数组可以用 as_mut_prt 获取首地址，指针转变为地址（usize）。
    let d = c.as_ptr() as usize; // 其实就算是 as_ptr 也没事，毕竟我们最后都是在 unsafe 中进行操作，可变性会被无视。
    unsafe {
        // 可在 unsafe 块中对地址进行解引用
        *((d + 4) as *mut i32) = 5;
        // 因为是在 unsafe 中修改的数组 c，所以就算 c 没有被声明为可变变量也能通过编译，且运行符合预期。
    }
    println!("{:?}", c); // 数组已被成功改变

    fn foo() {
        println!("this is foo");
    }

    // 获取函数的裸指针，const 代表这个指针变量 a 不可变，const 之后的 () 表示 foo 的返回值为 ().
    let e = foo as *const ();
    // 如果后期指针变量 e 可能被修改以指向其他位置，则可是使用 as *mut T，或声明为这个.

    unsafe {
        // 将裸指针转变为函数指针并调用
        std::mem::transmute::<*const (), fn() -> ()>(e)();
    }

    // 获取数组的指针并切改其中的元素的代码。
    let f = [1, 2, 3];
    let f2 = &f as *const [i32; 3] as usize; // 大概 &f 就相当于获取首地址，as *const [i32;3] 之后，还残留的有类型，再 as usize 之后就没有了。
    unsafe {
        *((f2 + (i32::BITS as usize) / 8) as *mut i32) = 5;
        let f3 = &f as *const [i32; 3];
        let _f4 = *f3;
        // Rust 数组裸指针可以解引用为指针，然后就可以作为普通的 Rust 数组使用了，不过需要只能在 unsafe 块中对 Rust 数组指针进行解引用。
        // 我估计提供 dll 接口时，就会用到这个操作。
    }
    println!("{:?}", f);
}
