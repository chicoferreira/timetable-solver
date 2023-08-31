use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use itertools::Itertools;
use toml::map::Map;
use toml::Value;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl Day {
    const DAYS: [Day; 5] = [
        Day::Monday,
        Day::Tuesday,
        Day::Wednesday,
        Day::Thursday,
        Day::Friday,
    ];
}

struct ParseError(&'static str);

impl FromStr for Day {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monday" => Ok(Day::Monday),
            "Tuesday" => Ok(Day::Tuesday),
            "Wednesday" => Ok(Day::Wednesday),
            "Thursday" => Ok(Day::Thursday),
            "Friday" => Ok(Day::Friday),
            _ => Err(ParseError("Invalid day")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Hour {
    hour: u16,
    minute: u16,
}

impl Hour {
    fn to_minutes(self) -> u16 {
        self.hour * 60 + self.minute
    }
}

impl FromStr for Hour {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hour, minute) = s.split(':').collect_tuple().unwrap_or((s, "0"));

        let hour = hour.parse().map_err(|_| ParseError("Invalid hour"))?;
        let minute = minute.parse().map_err(|_| ParseError("Invalid minute"))?;

        Ok(Hour { hour, minute })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Duration {
    start: Hour,
    end: Hour,
}

impl Duration {
    fn minutes(&self) -> u16 {
        self.end.to_minutes() - self.start.to_minutes()
    }

    fn merge(&self, duration: &Duration) -> Duration {
        Duration {
            start: Hour {
                hour: self.start.hour.min(duration.start.hour),
                minute: self.start.minute.min(duration.start.minute),
            },
            end: Hour {
                hour: self.end.hour.max(duration.end.hour),
                minute: self.end.minute.max(duration.end.minute),
            },
        }
    }

    fn is_overlapping(&self, duration: &Duration) -> bool {
        self.start.to_minutes() < duration.end.to_minutes()
            && self.end.to_minutes() > duration.start.to_minutes()
    }
}

impl FromStr for Duration {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split("->")
            .collect_tuple()
            .ok_or(ParseError("Invalid duration format"))?;

        let start = start.parse().map_err(|_| ParseError("Invalid start"))?;
        let end = end.parse().map_err(|_| ParseError("Invalid end"))?;

        Ok(Duration { start, end })
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Shift {
    name: String,
    day: Day,
    duration: Duration,
}

impl Shift {
    fn is_overlapping(&self, shift: &Shift) -> bool {
        self.day == shift.day && self.duration.is_overlapping(&shift.duration)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Subject {
    name: String,
    available_shifts: Vec<Shift>,
}

#[derive(Debug)]
struct ChosenTimetable<'a>(Vec<(&'a Subject, &'a Shift)>);

impl<'a> ChosenTimetable<'a> {
    fn prettify(&self) -> String {
        self.0
            .iter()
            .map(|(subject, shift)| format!("{} {}", subject.name, shift.name))
            .join(", ")
    }
}

impl<'a> ChosenTimetable<'a> {
    fn get_total_duration_at_day(&self, day: Day) -> Option<Duration> {
        self.0
            .iter()
            .map(|(_, shift)| shift)
            .filter(|shift| shift.day == day)
            .map(|shift| shift.duration)
            .reduce(|duration, next_duration| duration.merge(&next_duration))
    }

    fn get_minutes_in_classes(&self) -> u16 {
        self.0
            .iter()
            .map(|(_, shift)| shift)
            .map(|shift| shift.duration.minutes())
            .sum()
    }

    fn get_wait_time_in_minutes_at_day(&self) -> u16 {
        let total_duration = self.get_total_duration();
        let minutes_in_classes = self.get_minutes_in_classes();

        total_duration - minutes_in_classes
    }

    fn get_total_duration(&self) -> u16 {
        Day::DAYS
            .iter()
            .filter_map(|day| self.get_total_duration_at_day(*day))
            .map(|duration| duration.minutes())
            .sum()
    }

    fn has_classes_at_day(&self, day: Day) -> bool {
        self.get_total_duration_at_day(day).is_some()
    }

    fn count_days_with_classes(&self) -> usize {
        Day::DAYS
            .iter()
            .filter(|day| self.has_classes_at_day(**day))
            .count()
    }

    fn is_overlapping(&self) -> bool {
        self.0
            .iter()
            .map(|(_, shift)| shift)
            .tuple_combinations()
            .any(|(shift1, shift2)| shift1.is_overlapping(shift2))
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.get_total_duration().cmp(&other.get_total_duration())
    }
}

fn solve(subjects: Vec<Subject>) {
    let result = subjects
        .iter()
        .map(|subject| {
            subject
                .available_shifts
                .iter()
                .map(move |shift| (subject, shift))
        })
        .multi_cartesian_product();

    let result: Vec<ChosenTimetable> = result
        .map(|combination| ChosenTimetable(combination.to_vec()))
        .filter(|timetable| !timetable.is_overlapping())
        .collect();

    println!("Total possible timetables: {}", result.len());

    fn generate_results(results: &[ChosenTimetable], days: usize) {
        let results = results
            .iter()
            .filter(|timetable| timetable.count_days_with_classes() == days)
            .min_set_by(|a, b| a.cmp(b));

        for (i, result) in (1..).zip(results) {
            fn get_hours_at_day(result: &ChosenTimetable, day: Day) -> u16 {
                result
                    .get_total_duration_at_day(day)
                    .map(|duration| duration.minutes())
                    .unwrap_or(0)
                    / 60
            }

            println!(
                "{}. {:?} - {} hours ({}) with {} wait hours",
                i,
                result.prettify(),
                result.get_total_duration() / 60,
                Day::DAYS
                    .iter()
                    .map(|day| get_hours_at_day(result, *day))
                    .join("+"),
                result.get_wait_time_in_minutes_at_day() / 60
            );
        }
    }
    (1..=5).for_each(|days| {
        println!();
        println!("Best timetables with {} days of classes:", days);
        generate_results(&result, days);
    });
}

fn handle_shifts_array(shift_table: Map<String, Value>) -> Result<Vec<Shift>, ParseError> {
    let mut shifts = Vec::new();
    for (shift_name, shift_data) in shift_table {
        if let Value::String(shift_data) = shift_data {
            let (day, duration) =
                shift_data
                    .split_whitespace()
                    .collect_tuple()
                    .ok_or(ParseError(
                        "Invalid shift data format. Expected: <day> <start>-><end>",
                    ))?;
            let day = day.parse()?;
            let duration = duration.parse()?;

            shifts.push(Shift {
                name: shift_name.clone(),
                day,
                duration,
            })
        }
    }
    Ok(shifts)
}

fn load_schedule_file(file_name: &str) -> Result<Vec<Subject>, ParseError> {
    let content = fs::read_to_string(file_name).map_err(|_| ParseError("File not found"))?;
    let data: Map<String, Value> =
        toml::from_str(&content).map_err(|_| ParseError("Invalid TOML file"))?;

    let mut result = Vec::new();

    for (subject_name, shifts_tables) in data {
        if let Value::Array(shifts_tables) = shifts_tables {
            for shift_table in shifts_tables {
                if let Value::Table(shift_table) = shift_table {
                    let shifts = handle_shifts_array(shift_table)?;
                    result.push(Subject {
                        name: subject_name.clone(),
                        available_shifts: shifts,
                    });
                }
            }
        }
    }

    Ok(result)
}

fn main() {
    let vec = load_schedule_file("schedule.toml").unwrap_or_else(|err| {
        eprintln!("Error parsing schedule file: {}", err.0);
        std::process::exit(1);
    });

    let before = Instant::now();
    solve(vec);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
