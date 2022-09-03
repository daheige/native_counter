use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn count_words(mut cx: FunctionContext) ->JsResult<JsNumber>{
    let txt = cx.argument::<JsString>(0)?.value();
    let word = cx.argument::<JsString>(1)?.value();

    // 按照空格进行分割，得到的是一个字符串数组，然后调用filter进行回调处理
    // 统计word出现的次数，强制转换为f64格式，因为js number都是float64类型
    Ok(cx.number(txt.split(" ").filter(|s|{
        s == &word
    }).count() as f64))
}

// 通过宏的方式将函数导出
register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("count_words", count_words)?;
    Ok(())
});


