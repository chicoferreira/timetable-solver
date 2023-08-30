use std::cmp::Ordering;
use std::time::Instant;

use itertools::Itertools;

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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Duration {
    start: Hour,
    end: Hour,
}

macro_rules! shift {
    ($name:tt, $day:ident, $start_hour:literal:$start_minutes:literal->$end_hour:literal:$end_minutes:literal) => {
        Shift {
            name: $name,
            day: Day::$day,
            duration: Duration {
                start: Hour {
                    hour: $start_hour,
                    minute: $start_minutes,
                },
                end: Hour {
                    hour: $end_hour,
                    minute: $end_minutes,
                },
            }
        }
    };
    ($name:tt, $day:ident, $start_hour:literal->$end_hour:literal) => {
        shift!($name, $day, $start_hour:0->$end_hour:0)
    };
}

impl Duration {
    fn duration(&self) -> u16 {
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

#[derive(Debug, Eq, PartialEq, Hash)]
struct Shift {
    name: &'static str,
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
    name: &'static str,
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
    fn get_duration_at_day(&self, day: Day) -> Option<Duration> {
        self.0
            .iter()
            .map(|(_, shift)| shift)
            .filter(|shift| shift.day == day)
            .map(|shift| shift.duration)
            .reduce(|duration, next_duration| duration.merge(&next_duration))
    }

    fn get_total_duration(&self) -> u16 {
        Day::DAYS
            .iter()
            .filter_map(|day| self.get_duration_at_day(*day))
            .map(|duration| duration.duration())
            .sum()
    }

    fn has_classes_at_day(&self, day: Day) -> bool {
        self.get_duration_at_day(day).is_some()
    }

    fn count_days_with_classes(&self) -> usize {
        Day::DAYS
            .iter()
            .filter(|day| self.has_classes_at_day(**day))
            .count()
    }

    fn is_overlapping(&self) -> bool {
        for (_, x) in self.0.iter() {
            for (_, y) in self.0.iter() {
                if x != y && x.is_overlapping(y) {
                    return true;
                }
            }
        }
        false
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

    fn generate_results(results: &[ChosenTimetable], days: usize) {
        let results = results
            .iter()
            .filter(|timetable| timetable.count_days_with_classes() == days)
            .min_set_by(|a, b| a.cmp(b));

        for (i, result) in (1..).zip(results) {
            fn get_hours_at_day(result: &ChosenTimetable, day: Day) -> u16 {
                result
                    .get_duration_at_day(day)
                    .map(|duration| duration.duration())
                    .unwrap_or(0)
                    / 60
            }

            println!(
                "{}. {:?} - {} hours ({})",
                i,
                result.prettify(),
                result.get_total_duration() / 60,
                Day::DAYS
                    .iter()
                    .map(|day| get_hours_at_day(result, *day))
                    .join("+")
            );
        }
    }
    (1..=5).for_each(|days| {
        println!();
        println!("Best timetables with {} days with classes:", days);
        generate_results(&result, days);
    });
}

fn main() {
    let subjects = vec![
        Subject {
            name: "DSS",
            available_shifts: vec![
                shift!("PL1", Monday, 9->11),
                shift!("PL2/PL3/PL5", Thursday, 9->11),
                shift!("PL4", Tuesday, 9->11),
                shift!("PL6", Thursday, 11->13),
            ],
        },
        Subject {
            name: "DSS",
            available_shifts: vec![shift!("T1", Wednesday, 9->11)],
        },
        Subject {
            name: "IA",
            available_shifts: vec![
                shift!("PL1", Wednesday, 11->13),
                shift!("PL2", Tuesday, 14->16),
                shift!("PL3", Tuesday, 16->18),
                shift!("PL4/PL5", Friday, 9->11),
                shift!("PL6", Monday, 9->11),
            ],
        },
        Subject {
            name: "IA",
            available_shifts: vec![shift!("T1", Tuesday, 11->13)],
        },
        Subject {
            name: "CC",
            available_shifts: vec![
                shift!("PL1/PL4", Thursday, 9->11),
                shift!("PL2", Monday, 11->13),
                shift!("PL3", Tuesday, 9->11),
                shift!("PL5/PL7", Monday, 9->11),
                shift!("PL6", Friday, 9->11),
            ],
        },
        Subject {
            name: "CC",
            available_shifts: vec![
                shift!("T1", Friday, 11->13),
                shift!("T2", Wednesday, 14->16),
            ],
        },
        Subject {
            name: "CP",
            available_shifts: vec![
                shift!("TP1", Tuesday, 9->11),
                shift!("TP2", Wednesday, 11->13),
                shift!("TP3/TP5", Thursday, 11->13),
                shift!("TP4", Monday, 9->11),
            ],
        },
        Subject {
            name: "CP",
            available_shifts: vec![
                shift!("T1", Wednesday, 14->16),
                shift!("T2", Monday, 11->13),
            ],
        },
        Subject {
            name: "SD",
            available_shifts: vec![
                shift!("PL1", Tuesday, 14->16),
                shift!("PL2", Tuesday, 16->18),
                shift!("PL3", Monday, 11->13),
                shift!("PL4/PL5", Wednesday, 11->13),
                shift!("PL6/PL7", Thursday, 11->13),
            ],
        },
        Subject {
            name: "SD",
            available_shifts: vec![shift!("T1", Friday, 14->16)],
        },
    ];

    let before = Instant::now();
    solve(subjects);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
