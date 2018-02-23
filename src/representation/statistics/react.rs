use chrono::prelude::*;

use std::fmt;

use accounting::statistics::Report as DomainReport;
use accounting::statistics::ByCategory;
use accounting::Entry as DomainEntry;

#[derive(Debug)]
pub struct ReactReportRepresentation<'r>(DomainReport<'r>);

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
struct Report {
    title: String,
    timePeriod: TimePeriod,
    main: Vec<ReportEntry>,
    entries: Vec<Entry>
}

#[derive(Debug, Serialize)]
struct ReportEntry {
    category: String,
    total: i32,
    persent: i32
}

#[derive(Debug, Serialize)]
struct Entry {
    product: String,
    price: i32,
    time: NaiveDateTime,
    category: String
}

#[derive(Debug, Serialize, Clone, Copy)]
struct TimePeriod {
    from: NaiveDateTime,
    to: NaiveDateTime
}

impl<'r> From<DomainReport<'r>> for ReactReportRepresentation<'r> {
    fn from(report: DomainReport) -> ReactReportRepresentation {
        ReactReportRepresentation(report)
    }
}

impl<'r> fmt::Display for ReactReportRepresentation<'r> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let head = include_str!("head.html");
        let tail = include_str!("tail.html");
        let mut main_report = Report::from(&self.0);
        main_report.title = "Всего".to_owned();
        let mut reports = vec![main_report];
        if let Some(subreports) = self.0.subreports().map_err(|_| fmt::Error)? {
            subreports.iter()
                      .map(Report::from)
                      .for_each(|r| reports.push(r));
        }
        let app_data = ::serde_json::to_string(&reports).map_err(|_| fmt::Error)?;
        write!(f, "{}{}{}", head, app_data, tail)
    }
}

impl<'a, 'r> From<&'a DomainReport<'r>> for Report {
    fn from(report: &'a DomainReport<'r>) -> Report {
        let time_period = TimePeriod {
            from: report.period.0,
            to: report.period.1
        };
        Report {
            title: report_name(time_period),
            timePeriod: time_period,
            main: report.by_category.iter()
                                    .map(ReportEntry::from)
                                    .collect(),
            entries: report.by_category.iter()
                                       .flat_map(|c| c.entries.iter())
                                       .map(|e| Entry::from(*e))
                                       .collect()
        }
    }
}

impl<'a, 'r> From<&'a ByCategory<'r>> for ReportEntry {
    fn from(entry: &'a ByCategory<'r>) -> ReportEntry {
        ReportEntry {
            category: entry.category.to_owned(),
            total: entry.total_spent,
            persent: entry.persent as i32
        }
    }
}

impl<'r> From<&'r DomainEntry> for Entry {
    fn from(entry: &'r DomainEntry) -> Entry {
        Entry {
            product: entry.product.name.to_owned(),
            price: entry.product.price,
            time: entry.time,
            category: "N/A".to_owned()
        }
    }
}

fn report_name(time_period: TimePeriod) -> String {
    if time_period.from.month() != time_period.to.month() && 
       time_period.from.year() == time_period.to.year() {
        return format!("{}: {} - {}", 
            time_period.from.year(), 
            month_name(time_period.from), 
            month_name(time_period.to));
    }
    if time_period.from.month() != time_period.to.month() && 
       time_period.from.year() != time_period.to.year() {
        return format!("{} {} - {} {}", 
            month_name(time_period.from),
            time_period.from.year(),  
            month_name(time_period.to),
            time_period.to.year());
    }
    format!("{} {}", month_name(time_period.from), time_period.from.year())
}

fn month_name(date: NaiveDateTime) -> &'static str {
    match date.month() {
        1 => "январь",
        2 => "февраль",
        3 => "март",
        4 => "апрель",
        5 => "май",
        6 => "июнь",
        7 => "июль",
        8 => "август",
        9 => "сентябрь",
        10 => "октябрь",
        11 => "ноябрь",
        12 => "декабрь",
        _ => unreachable!()
    }
}
