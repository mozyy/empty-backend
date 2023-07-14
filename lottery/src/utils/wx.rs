use serde::Deserialize;

#[derive(Deserialize)]
enum ErrCode {
    /// js_code无效
    CodeInvalid = 40029,
    ///API 调用太频繁，请稍候再试
    ApiLimit = 45011,
    /// 高风险等级用户，小程序登录拦截 。风险等级详见[用户安全解方案](https://developers.weixin.qq.com/miniprogram/dev/framework/operation.html)
    CodeBlocked = 40226,
    /// 系统繁忙，此时请开发者稍候再试
    SystemError = -1,
}

#[derive(Deserialize)]
pub struct Response<T> {
    #[serde(flatten)]
    value: T,
    errcode: ErrCode, // i32
    errmsg: String,
}
