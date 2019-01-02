use chrono::prelude::*;
use failure::Error as FailureError;

use std::collections::HashMap;
use std::str::FromStr;

use super::{Category, Entry};
use dates::{end_of_day, last_day_of_month, start_of_day};
use error::AppError;

#[derive(Debug)]
pub struct Statistics {
    entries: Vec<Entry>,
    categories: HashMap<String, String>,
    now: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Report<'r> {
    pub period: (NaiveDateTime, NaiveDateTime),
    pub total_spent: i32,
    pub total_products: i32,
    pub by_category: Vec<ByCategory<'r>>,
    stats: &'r Statistics,
}

#[derive(Debug, Clone)]
pub struct ByCategory<'r> {
    pub category: &'r str,
    pub entries: Vec<&'r Entry>,
    pub total_spent: i32,
    pub total_products: i32,
    pub persent: f32,
}

#[derive(Debug)]
pub enum TimePeriod {
    Today,
    ThisWeek,
    ThisMonth,
    ThisYear,
    Any(NaiveDate, NaiveDate),
}

impl FromStr for TimePeriod {
    type Err = FailureError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut split = raw.split('-');
        let (dimension, value) = (split.next(), split.next());
        match (dimension, value) {
            (Some("day"), None) => Ok(TimePeriod::Today),
            (Some("week"), None) => Ok(TimePeriod::ThisWeek),
            (Some("month"), None) => Ok(TimePeriod::ThisMonth),
            (Some("year"), None) => Ok(TimePeriod::ThisYear),
            (Some("year"), Some(year_number)) => parse_year(year_number),
            _ => Err(AppError::InvalidEnumVariant.into()),
        }
    }
}

fn parse_year(year_number: &str) -> Result<TimePeriod, FailureError> {
    let year:  i32 = year_number.parse()?;
    let start = NaiveDate::from_yo(year, 1);
    let end = NaiveDate::from_yo(year + 1, 1).pred();
    Ok(TimePeriod::Any(start, end))
}

impl<'r> Report<'r> {
    pub fn subreports(&self) -> Result<Option<Vec<Report<'r>>>, FailureError> {
        if let Some(periods) = subperiods(self.period.0.date(), self.period.1.date()) {
            let mut subreports = Vec::new();
            for (from, to) in periods {
                if let Some(subreport) = self.stats.report(TimePeriod::Any(from, to))? {
                    subreports.push(subreport);
                }
            }
            return Ok(Some(subreports));
        }
        Ok(None)
    }
}

impl Statistics {
    pub fn new(entries: Vec<Entry>, categories: Vec<Category>) -> Statistics {
        Statistics {
            entries,
            categories: categories
                .into_iter()
                .map(|c| (c.product, c.category))
                .collect(),
            now: ::chrono::offset::Local::now().naive_local(),
        }
    }

    pub fn report(&self, period: TimePeriod) -> Result<Option<Report>, FailureError> {
        debug!("report for {:?}", &period);
        let (from, till) = self.period(period);
        let entries_in_period: Vec<&Entry> = self
            .entries
            .iter()
            .filter(|e| e.time >= from && e.time <= till)
            .collect();
        if entries_in_period.is_empty() {
            return Ok(None);
        }
        let total_spent = Self::total_spent(&entries_in_period)?;

        Ok(Some(Report {
            period: (from, till),
            total_spent,
            total_products: entries_in_period.len() as i32,
            by_category: self.by_category(&entries_in_period, total_spent)?,
            stats: &self,
        }))
    }

    fn by_category<'r>(
        &'r self,
        entries: &[&'r Entry],
        total_spent: i32,
    ) -> Result<Vec<ByCategory<'r>>, FailureError> {
        let mut categories: HashMap<&str, Vec<&Entry>> = HashMap::new();

        for entry in entries {
            let category_name = self
                .categories
                .get(&entry.product.name)
                .unwrap_or(&entry.product.name);
            categories
                .entry(category_name)
                .or_insert_with(|| Vec::new())
                .push(entry);
        }

        let mut by_category = Vec::new();
        for (category, entries) in categories {
            let (total_spent, total_products, persent) = Self::stats(&entries, total_spent)?;
            by_category.push(ByCategory {
                category,
                entries,
                total_spent,
                total_products,
                persent,
            });
        }

        by_category.sort_unstable_by_key(|cat| cat.total_spent);
        by_category.reverse();

        Ok(by_category)
    }

    fn stats(entries: &[&Entry], all_total_spent: i32) -> Result<(i32, i32, f32), FailureError> {
        let mut total_spent = 0i32;

        for entry in entries {
            if let Some(sum) = total_spent.checked_add(entry.product.price) {
                total_spent = sum;
            } else {
                return Err(AppError::Calculation {
                    reason: "Overflow".to_owned(),
                }.into());
            }
        }
        let persent = total_spent as f32 / all_total_spent as f32 * 100.;

        Ok((total_spent, entries.len() as i32, persent))
    }

    fn total_spent(entries: &[&Entry]) -> Result<i32, FailureError> {
        let mut total = 0i32;
        for price in entries.iter().map(|e| e.product.price) {
            if let Some(sum) = total.checked_add(price) {
                total = sum;
            } else {
                return Err(AppError::Calculation {
                    reason: "Overflow".to_owned(),
                }.into());
            }
        }
        Ok(total)
    }

    fn period(&self, period: TimePeriod) -> (NaiveDateTime, NaiveDateTime) {
        match period {
            TimePeriod::Today => (
                self.now.date().and_time(start_of_day()),
                self.now.date().and_time(end_of_day()),
            ),
            TimePeriod::ThisWeek => (
                self.this_week(Weekday::Mon).and_time(start_of_day()),
                self.this_week(Weekday::Sun).and_time(end_of_day()),
            ),
            TimePeriod::ThisMonth => (
                self.now
                    .date()
                    .with_day(1)
                    .unwrap()
                    .and_time(start_of_day()),
                last_day_of_month(self.now.date()).and_time(end_of_day()),
            ),
            TimePeriod::ThisYear => (
                NaiveDate::from_ymd(self.now.year(), 1, 1).and_time(start_of_day()),
                NaiveDate::from_ymd(self.now.year() + 1, 1, 1)
                    .pred()
                    .and_time(end_of_day()),
            ),
            TimePeriod::Any(from, to) => (from.and_time(start_of_day()), to.and_time(end_of_day())),
        }
    }

    fn this_week(&self, day: Weekday) -> NaiveDate {
        NaiveDate::from_isoywd(self.now.year(), self.now.iso_week().week(), day)
    }
}

fn subperiods(from: NaiveDate, to: NaiveDate) -> Option<Vec<(NaiveDate, NaiveDate)>> {
    debug!("{}, {}", from, to);
    if (from.month0() >= to.month0() && from.year() == to.year()) || (from.year() > to.year()) {
        return None;
    }

    let mut periods = Vec::new();
    let mut next_period_start = from;
    let mut next_period_end = last_day_of_month(from);
    while next_period_end < to {
        periods.push((next_period_start, next_period_end));
        next_period_start = next_period_end.succ();
        next_period_end = last_day_of_month(next_period_start);
    }
    periods.push((next_period_start, to));
    Some(periods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_subperiods() {
        assert_eq!(
            subperiods(
                NaiveDate::from_ymd(2018, 1, 1),
                NaiveDate::from_ymd(2018, 1, 30)
            ),
            None
        );
        assert_eq!(
            subperiods(
                NaiveDate::from_ymd(2018, 1, 1),
                NaiveDate::from_ymd(2018, 2, 28)
            ),
            Some(vec![
                (
                    NaiveDate::from_ymd(2018, 1, 1),
                    NaiveDate::from_ymd(2018, 1, 31),
                ),
                (
                    NaiveDate::from_ymd(2018, 2, 1),
                    NaiveDate::from_ymd(2018, 2, 28),
                ),
            ])
        );
    }
}
