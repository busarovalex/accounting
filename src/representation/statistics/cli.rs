use std::fmt;

use accounting::statistics::Report;

#[derive(Debug)]
pub struct ReportRepresentation<'r>(Report<'r>);

impl<'r> From<Report<'r>> for ReportRepresentation<'r> {
    fn from(report: Report) -> ReportRepresentation {
        ReportRepresentation(report)
    }
}

impl<'r> fmt::Display for ReportRepresentation<'r> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stats = &self.0;
        let format = "%Y-%m-%d %H:%M";
        writeln!(f, "{} - {}", stats.period.0.format(format).to_string(), stats.period.1.format(format).to_string())?;
        writeln!(f, "Всего потрачено: {}. Вего записей: {}\n", 
            stats.total_spent, 
            stats.total_products)?;
        writeln!(f, "По категориям:                      потрачено (руб),   %,  записей")?;
        for category in &stats.by_category {
            writeln!(f, "{:36}{:15},{:4},{:9}", category.category, category.total_spent, category.persent as i32, category.total_products)?;
        }
        writeln!(f, "")
    }
}
