fn call<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn call_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

#[derive(Debug)]
struct V(i32);

#[test]
fn test_fn() {
    let v = 0;

    let f = || {
        // `v` 通过引用操作，闭包为 `Fn`
        println!("{}", v);
    };

    // 可调用多次
    call(f);
    call(f);
}

#[test]
fn test_fn_mut() {
    let mut v = 0;

    let mut f = || {
        // `v` 通过可变引用操作，闭包为 `FnMut`，所有权移动至闭包内
        v += 1;
        println!("{}", v);
    };

    // 可调用多次
    call_mut(&mut f);
    call_mut(&mut f);
}

#[test]
fn test_fn_once() {
    let v = V(0);

    // `v` 未实现 `Copy` trait 时：
    //   `v` 所有权移动至闭包内，此闭包只可调用一次。
    //
    // `v` 实现 `Copy` trait 时：
    //   将 `v` 复制一份至闭包内，此闭包可调用多次。
    let f = || {
        // `v` 通过值操作，闭包为 `FnOnce`，所有权移动至闭包内
        drop(v);
    };

    call_once(f);
}

#[test]
fn test_move() {
    let v = V(0);

    // move：
    //   `v` 未实现 `Copy` trait 时：
    //     强行将变量所有权移至闭包内，闭包类型不会发生改变，但此闭包只可调用一次。
    //
    //   `v` 实现 `Copy` trait 时：
    //     将变量复制一份至闭包内，此闭包可调用多次。
    let f = move || {
        println!("{:?}", v);
    };

    // call(f);
    // call_mut(f);
    call_once(f);
}
