use std::fmt;

use accounting::statistics::Report;

#[derive(Debug)]
pub struct BotReportRepresentation<'r>(Report<'r>);

impl<'r> From<Report<'r>> for BotReportRepresentation<'r> {
    fn from(report: Report) -> BotReportRepresentation {
        BotReportRepresentation(report)
    }
}

impl<'r> fmt::Display for BotReportRepresentation<'r> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stats = &self.0;
        let format = "%Y-%m-%d %H:%M";
        writeln!(
            f,
            "{} - {}",
            stats.period.0.format(format).to_string(),
            stats.period.1.format(format).to_string()
        )?;
        writeln!(
            f,
            "Всего потрачено: {}. Вего записей: {}\n",
            stats.total_spent, stats.total_products
        )?;
        for category in &stats.by_category {
            writeln!(
                f,
                "{} - {} ({}%), {} ед.",
                category.category,
                category.total_spent,
                category.persent as i32,
                category.total_products
            )?;
        }
        writeln!(f, "")
    }
}
