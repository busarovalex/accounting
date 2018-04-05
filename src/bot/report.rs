use chrono::prelude::*;

use std::str::FromStr;

use registry::Registry;
use accounting::UserId;
use accounting::statistics::{Statistics, TimePeriod};
use error::{Error, ErrorKind};
use bot::email::EmailSender;
use config::Config;

pub fn report<'a, I>(
    commands: &mut I,
    config: &Config,
    registry: &Registry,
    user: UserId,
) -> Result<String, Error>
where
    I: Iterator<Item = &'a str>,
{
    let reports = ReportFactory { registry, user };
    let time_period = commands.next();
    let email = commands.next();
    let last = commands.next();
    match (time_period, email, last) {
        (None, None, None) => reports.print_week_report(),
        (Some(time_period), None, _) => reports.try_print_report(time_period),
        (Some(time_period), Some(email), None) => {
            reports.try_send_report(config, time_period, email)
        }
        (Some(_), Some(_), Some(_)) => Err(wrong_bot_usage()),
        (None, _, _) => unreachable!(),
    }
}

struct ReportFactory<'r> {
    registry: &'r Registry,
    user: UserId,
}

impl<'r> ReportFactory<'r> {
    fn print_week_report(&self) -> Result<String, Error> {
        self.print_report(TimePeriod::ThisWeek)
    }

    fn try_print_report(&self, time_period: &str) -> Result<String, Error> {
        let time_period = parse_time_period(time_period)?;
        self.print_report(time_period)
    }

    fn try_send_report(
        &self,
        config: &Config,
        time_period: &str,
        email: &str,
    ) -> Result<String, Error> {
        let sender = EmailSender::from_config(config)?;
        let time_period = parse_time_period(time_period)?;
        let statistics = self.statistics()?;
        let report = statistics.report(time_period)?;
        match report {
            Some(actual_report) => {
                let react_report = format!(
                    "{}",
                    ::representation::ReactReportRepresentation::from(actual_report)
                );
                sender.send(react_report, email)?;
                Ok(format!("Отчет выслан на {}", email))
            }
            None => Ok(format!("нет данных за этот период")),
        }
    }

    fn print_report(&self, time_period: TimePeriod) -> Result<String, Error> {
        let statistics = self.statistics()?;
        let report = statistics.report(time_period)?;
        match report {
            Some(actual_report) => Ok(format!(
                "{}",
                ::representation::BotReportRepresentation::from(actual_report)
            )),
            None => Ok(format!("нет данных за этот период")),
        }
    }

    fn statistics(&self) -> Result<Statistics, Error> {
        let entries = self.registry.list(self.user.clone())?;
        let categories = self.registry.categories(self.user.clone())?;
        Ok(Statistics::new(entries, categories))
    }
}

fn parse_time_period(time_period: &str) -> Result<TimePeriod, Error> {
    let now = ::chrono::offset::Local::now().naive_local().date();
    match time_period {
        "январь" => Ok(month(now, 1)),
        "февраль" => Ok(month(now, 2)),
        "март" => Ok(month(now, 3)),
        "апрель" => Ok(month(now, 4)),
        "май" => Ok(month(now, 5)),
        "июнь" => Ok(month(now, 6)),
        "июль" => Ok(month(now, 7)),
        "август" => Ok(month(now, 8)),
        "сентябрь" => Ok(month(now, 9)),
        "октябрь" => Ok(month(now, 10)),
        "ноябрь" => Ok(month(now, 11)),
        "декабрь" => Ok(month(now, 12)),
        year @ _ => parse_year(year),
    }
}

fn parse_year(time_period: &str) -> Result<TimePeriod, Error> {
    let year = i32::from_str(time_period)?;
    let invalid_date: Error = ErrorKind::InvalidDate.into();
    let start_of_year = NaiveDate::from_ymd_opt(year, 1, 1).ok_or(invalid_date)?;
    let invalid_date: Error = ErrorKind::InvalidDate.into();
    let end_of_year =
        ::dates::last_day_of_month(NaiveDate::from_ymd_opt(year, 12, 1).ok_or(invalid_date)?);
    Ok(TimePeriod::Any(start_of_year, end_of_year))
}

fn month(now: NaiveDate, month: u32) -> TimePeriod {
    TimePeriod::Any(start_of(now, month), end_of(now, month))
}

fn start_of(now: NaiveDate, month: u32) -> NaiveDate {
    NaiveDate::from_ymd(now.year(), month, 1)
}

fn end_of(now: NaiveDate, month: u32) -> NaiveDate {
    ::dates::last_day_of_month(start_of(now, month))
}

fn wrong_bot_usage() -> Error {
    ErrorKind::BotUsage("ожидается два аргумента".to_owned()).into()
}
