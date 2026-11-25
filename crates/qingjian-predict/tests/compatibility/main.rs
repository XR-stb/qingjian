//! 用本地 HTTP 服务验证按接口报错降级，不读取密钥或调用外部服务。

mod cache;
mod failures;
mod helpers;
mod parameters;

#[test]
fn translates_only_the_selected_saved_sentence() {
    let (status, mut response) = helpers::success();
    response["choices"][0]["message"]["content"] =
        serde_json::json!(r#"{"items":[{"w":"回头聊。","t":"Talk to you later."}]}"#);
    let (translated, requests) = helpers::with_server(vec![(status, response)], |config| {
        qingjian_predict::translate_saved_sentence(
            config,
            None,
            qingjian_core::Language::English,
            "回头聊。".into(),
        )
        .unwrap()
    });
    assert_eq!(translated.translation, "Talk to you later.");
    assert_eq!(requests.len(), 1);
    let input: serde_json::Value =
        serde_json::from_str(requests[0]["messages"][1]["content"].as_str().unwrap()).unwrap();
    assert_eq!(input, serde_json::json!({"texts":["回头聊。"]}));
}
