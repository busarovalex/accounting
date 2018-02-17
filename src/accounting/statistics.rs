use chrono::prelude::*;

use std::collections::HashMap;
use std::str::FromStr;

use super::{Entry, Category};
use error::{Error, ErrorKind};

#[derive(Debug)]
pub struct Statistics {
    entries: Vec<Entry>,
    categories: HashMap<String, String>,
    now: NaiveDateTime
}

#[derive(Debug)]
pub struct Report<'r> {
    pub period: (NaiveDateTime, NaiveDateTime),
    pub total_spent: i32,
    pub total_products: i32,
    pub without_category: NoCategory<'r>,
    pub by_category: Vec<ByCategory<'r>>,
    pub by_product: Vec<ByProduct<'r>>
}

#[derive(Debug)]
pub struct NoCategory<'r> {
    pub entries: Vec<&'r Entry>,
    pub total_spent: i32,
    pub total_products: i32,
    pub persent: f32
}

#[derive(Debug)]
pub struct ByCategory<'r> {
    pub category: &'r str,
    pub entries: Vec<&'r Entry>,
    pub total_spent: i32,
    pub total_products: i32,
    pub persent: f32
}

#[derive(Debug)]
pub struct ByProduct<'r> {
    pub product: &'r str,
    pub entries: Vec<&'r Entry>,
    pub total_spent: i32,
    pub total_products: i32,
    pub persent: f32
}

#[derive(Debug)]
pub enum TimePeriod {
    Today,
    ThisWeek,
    ThisMonth,
    ThisYear
}

impl FromStr for TimePeriod {
    type Err = Error;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "day" => Ok(TimePeriod::Today),
            "week" => Ok(TimePeriod::ThisWeek),
            "month" => Ok(TimePeriod::ThisMonth),
            "year" => Ok(TimePeriod::ThisYear),
            _ => Err(ErrorKind::InvalidEnumVariant.into())
        }
    }
}

impl Statistics {
    pub fn new(entries: Vec<Entry>, categories: Vec<Category>) -> Statistics {
        Statistics {
            entries,
            categories: categories.into_iter().map(|c| (c.product, c.category)).collect(),
            now: ::chrono::offset::Local::now().naive_local(),
        }
    }

    pub fn report(&self, period: TimePeriod) -> Result<Report, Error> {
        debug!("report for {:?}", &period);
        let (from, till) = self.period(period);
        let entries_in_period: Vec<&Entry> = self.entries.iter()
            .filter(|e| e.time >= from && e.time <= till)
            .collect();
        let total_spent = Self::total_spent(&entries_in_period)?;

        Ok(Report {
            period: (from, till),
            total_spent,
            total_products: entries_in_period.len() as i32,
            without_category: self.without_category(&entries_in_period, total_spent)?,
            by_category: self.by_category(&entries_in_period, total_spent)?,
            by_product: self.by_product(&entries_in_period, total_spent)?
        })
    }

    fn without_category<'r>(&'r self, entries: &[&'r Entry], total_spent: i32) -> Result<NoCategory<'r>, Error> {
        let entries_without_category: Vec<&Entry> = entries.iter()
            .filter(|entry| !self.categories.contains_key(&entry.product.name))
            .map(|e| *e)
            .collect();
        let (total_spent, total_products, persent) = 
            Self::stats(&entries_without_category, total_spent)?;

        Ok(NoCategory {
            entries: entries_without_category,
            total_spent,
            total_products,
            persent
        })
    }

    fn by_category<'r>(&'r self, entries: &[&'r Entry], total_spent: i32) -> Result<Vec<ByCategory<'r>>, Error> {
        let mut categories: HashMap<&str, Vec<&Entry>> = HashMap::new();

        for entry in entries {
            if let Some(category) = self.categories.get(&entry.product.name) {
                categories.entry(category).or_insert_with(|| Vec::new()).push(entry);
            }
        }


        let mut by_category = Vec::new();
        for (category, entries) in categories {
            let (total_spent, total_products, persent) = 
                Self::stats(&entries, total_spent)?;
            by_category.push(ByCategory {
                category,
                entries,
                total_spent,
                total_products,
                persent
            });
        }

        by_category.sort_unstable_by_key(|cat| cat.total_spent);
        by_category.reverse();

        Ok(by_category)
    }

    fn by_product<'r>(&'r self, entries: &[&'r Entry], total_spent: i32) -> Result<Vec<ByProduct<'r>>, Error> {
        let mut products: HashMap<&str, Vec<&Entry>> = HashMap::new();

        for entry in entries {
            products.entry(&entry.product.name).or_insert_with(|| Vec::new()).push(entry);
        }


        let mut by_product = Vec::new();
        for (product, entries) in products {
            let (total_spent, total_products, persent) = 
                Self::stats(&entries, total_spent)?;
            by_product.push(ByProduct {
                product,
                entries,
                total_spent,
                total_products,
                persent
            });
        }

        by_product.sort_unstable_by_key(|prod| prod.total_spent);
        by_product.reverse();

        Ok(by_product)
    }

    fn stats(entries: &[&Entry], all_total_spent: i32) -> Result<(i32, i32, f32), Error> {
        let mut total_spent = 0i32;

        for entry in entries {
            if let Some(sum) = total_spent.checked_add(entry.product.price) {
                total_spent = sum;
            } else {
                return Err(ErrorKind::Calculation("Overflow".to_owned()).into());
            }
        }
        let persent = total_spent as f32 / all_total_spent as f32 * 100.;

        Ok((total_spent, entries.len() as i32, persent))
    }

    fn total_spent(entries: &[&Entry]) -> Result<i32, Error> {
        let mut total = 0i32;
        for price in entries.iter().map(|e| e.product.price) {
            if let Some(sum) = total.checked_add(price) {
                total = sum;
            } else {
                return Err(ErrorKind::Calculation("Overflow".to_owned()).into());
            }
        }
        Ok(total)
    }

    fn period(&self, period: TimePeriod) -> (NaiveDateTime, NaiveDateTime) {
        match period {
            TimePeriod::Today => (
                self.now.date().and_time(start_of_day()), 
                self.now.date().and_time(end_of_day())
            ),
            TimePeriod::ThisWeek => (
                self.this_week(Weekday::Mon).and_time(start_of_day()),
                self.this_week(Weekday::Sun).and_time(end_of_day())
            ),
            TimePeriod::ThisMonth => (
                self.now.date().with_day(1).unwrap().and_time(start_of_day()),
                NaiveDate::from_ymd(self.now.year(), self.next_month(), 1).pred().and_time(end_of_day())
            ),
            TimePeriod::ThisYear => (
                NaiveDate::from_ymd(self.now.year(), 1, 1).and_time(start_of_day()),
                NaiveDate::from_ymd(self.now.year() + 1, 1, 1).pred().and_time(end_of_day()),
            )
        }
    }

    fn this_week(&self, day: Weekday) -> NaiveDate {
        NaiveDate::from_isoywd(self.now.year(), self.now.iso_week().week(), day)
    }

    fn next_month(&self) -> u32 {
        (self.now.month0() + 1) % 12 + 1
    }
}

fn start_of_day() -> NaiveTime {
    NaiveTime::from_hms(0, 0, 0)
}

fn end_of_day() -> NaiveTime {
    NaiveTime::from_hms(23, 59, 59)
}
