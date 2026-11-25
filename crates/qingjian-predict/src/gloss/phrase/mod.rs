//! 整段翻译：攒一批上屏的中文，一次请求翻成学习语言（生词本的时间线用）。

mod prompt;
mod worker;

pub(super) use worker::PhraseWorker;

/// 在后台线程里补译用户选中的一条原句；沿用整句翻译的提示、解析与用量统计。
pub fn translate_saved_sentence(
    config: &crate::PredictConfig,
    usage: Option<std::path::PathBuf>,
    language: qingjian_core::Language,
    text: String,
) -> Result<qingjian_core::Encounter, crate::PredictError> {
    let key = config
        .resolve_api_key()
        .ok_or_else(|| crate::PredictError::MissingApiKey(config.api_key_env.clone()))?;
    let client = crate::chat_client::ChatClient::new(config, key).with_usage(usage);
    let texts = [text];
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let reply = runtime.block_on(client.chat(
        &prompt::system_prompt(language),
        &prompt::user_prompt(&texts),
        600,
        &prompt::reply_schema(language),
    ))?;
    prompt::parse_reply(&reply, language, &texts)
        .into_iter()
        .next()
        .ok_or(crate::PredictError::EmptyReply)
}
