//! 原句复习：每页练三句，先看中文，按需揭晓译文。
use super::parts::pager;
use crate::i18n::{tr, tr_args};
use crate::message::Message;
use crate::view::Page;
use crate::view::parts::controls::toggle_info;
use crate::vocabulary::Vocabulary;
use bamboo_slip::taffy::NodeId;
use bamboo_slip::{Button, RenderError, SettingRow, Ui, group, paragraph};

pub fn sentences(
    ui: &mut Ui<Message>,
    page: &Page,
    book: &Vocabulary,
) -> Result<Vec<NodeId>, RenderError> {
    let theme = &page.theme;
    let mut out = Vec::new();
    if crate::support::Feature::SentenceReview.here() {
        let toggle = toggle_info(
            ui,
            page,
            tr("vocabulary.save_sentences"),
            Some(tr("vocabulary.save_sentences_info")),
            ("general", "record_sentences"),
            page.settings.config.general.record_sentences,
            false,
        )?;
        out.push(group(ui, theme, &[toggle])?);
    }
    if book.sentences.is_empty() {
        out.push(paragraph(
            ui,
            theme,
            tr("vocabulary.sentences_empty"),
            None,
        )?);
        return Ok(out);
    }
    let pages = book.sentences.len().div_ceil(3);
    let current = book.page.min(pages - 1);
    let mut rows = Vec::new();
    for (i, (original, count, translation)) in
        book.sentences.iter().enumerate().skip(current * 3).take(3)
    {
        let count_label = tr_args("vocabulary.sentence_count", &[("n", count)]);
        let mut row = SettingRow::new(original);
        let detail;
        if let Some(answer) = translation {
            let revealed = book.revealed_sentence == Some(i);
            detail = if revealed {
                format!("{count_label} · {answer}")
            } else {
                count_label
            };
            let button = Button::new(if revealed {
                tr("vocabulary.hide")
            } else {
                tr("vocabulary.reveal")
            })
            .build(
                ui,
                theme,
                ("review-sentence", i),
                Message::VocabularyReveal(i),
            )?;
            row = row.control(button);
        } else {
            detail = format!("{} · {}", count_label, tr("vocabulary.untranslated"));
            if page.settings.config.predict.enabled {
                let button = Button::new(tr("vocabulary.translate_one"))
                    .disabled(page.settings.sentence_translation.is_some())
                    .build(
                        ui,
                        theme,
                        ("translate-sentence", i),
                        Message::VocabularyTranslate(i),
                    )?;
                row = row.control(button);
            }
        }
        rows.push(row.detail(&detail).build(ui, theme)?);
    }
    if pages > 1 {
        rows.push(pager(ui, page, current, pages)?);
    }
    out.push(group(ui, theme, &rows)?);
    if page.settings.sentence_translation.is_some() {
        out.push(paragraph(ui, theme, tr("vocabulary.translating"), None)?);
    } else if let Some(("sentence-translation", message)) = page
        .settings
        .notice
        .as_ref()
        .map(|(slot, msg)| (*slot, msg))
    {
        out.push(paragraph(ui, theme, message, None)?);
    }
    Ok(out)
}
