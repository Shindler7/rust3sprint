//! Поддерживающие методы для взаимодействия с внешними сервисами.
use web_sys::{window, Document};

/// Предоставить ссылку на HTTP-сервис API.
///
/// ## Returns
///
/// Не дело функции паниковать, если ссылки нет, с этим должен разбираться
/// заказчик. Поэтому вернёт `None`, если есть проблемы.
pub(super) fn get_base_api_url() -> Option<String> {
    get_dom()?
        .query_selector(r#"meta[name="api-base-url"]"#)
        .ok()
        .flatten()
        .and_then(|e1| e1.get_attribute("content"))
}

/// Предоставить доступ к объекту DOM для извлечения данных.
fn get_dom() -> Option<Document> {
    window().and_then(|d| d.document())
}
