//! Операции со временем и датами.

use chrono::{DateTime, Local, Utc};

/// Преобразовать часовую отметку UTC в текстовое представление даты.
///
/// Функция не может проверить создан ли timestamp в зоне UTC. Если
/// подтверждения нет, это может привести к искажённому результату.
///
/// ## Args
///
/// - `utc_secs` — количество секунд с начала эпохи UNIX во временной зоне UTC
/// - `local_time` — если `true`, время будет приведено к локальному часовому
///   поясу
///
/// ## Returns
///
/// При успешном преобразовании вернётся отформатированная строка. Если вывести
/// время не удалось, вернётся `None`.
pub fn string_from_timestamp(utc_secs: i64, local_time: bool) -> Option<String> {
    let datetime_utc = DateTime::from_timestamp_secs(utc_secs)?;
    
    string_from_datetime(datetime_utc, local_time)
}

/// Преобразовать [`DateTime`] в отформатированную строку с датой и временем.
pub fn string_from_datetime(dt: DateTime<Utc>, local_time: bool) -> Option<String> {
    let datetime_local = if local_time {
        dt
    } else {
        DateTime::from(dt.with_timezone(&Local))
    };

    Some(datetime_local.format("%H:%M, %d.%m.%Y").to_string())
}
