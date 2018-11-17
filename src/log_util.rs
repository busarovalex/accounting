use std::io::Write;

pub fn format_log(
    buf: &mut env_logger::fmt::Formatter,
    record: &log::Record,
) -> std::io::Result<()> {
    let level_style = buf.default_level_style(record.level());
    let now = chrono::offset::Utc::now().format("%Y-%m-%dT%H:%M:%S");
    match (record.module_path(), record.line()) {
        (Some(module_path), Some(line)) => writeln!(
            buf,
            "{} {} {}:{} {}",
            level_style.value(record.level()),
            level_style.value(now),
            level_style.value(module_path),
            level_style.value(line),
            level_style.value(record.args()),
        ),
        _ => writeln!(
            buf,
            "{} {}: {}",
            level_style.value(record.level()),
            level_style.value(now),
            level_style.value(record.args()),
        ),
    }
}
