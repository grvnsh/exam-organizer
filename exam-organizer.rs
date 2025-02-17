


use std::collections::BinaryHeap;
use std::cmp::Ordering;
use chrono::{NaiveDate, Local};
use std::io;

// Define a struct to hold exam/study session details
#[derive(Eq)]
struct StudyTask {
    subject: String,
    priority: u8, // Lower = Higher priority (e.g., 1 = Most Difficult)
    exam_date: NaiveDate,
}

// Implement ordering for sorting in BinaryHeap
impl Ord for StudyTask {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Compare by priority first
            .then_with(|| self.exam_date.cmp(&other.exam_date)) // Earlier exam date comes first
    }
}

impl PartialOrd for StudyTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for StudyTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.exam_date == other.exam_date
    }
}

fn main() {
    let mut study_queue = BinaryHeap::new();
    let mut input = String::new();

    println!("Enter the number of exams/study sessions:");
    io::stdin().read_line(&mut input).unwrap();
    let num_tasks: usize = input.trim().parse().unwrap();
    input.clear();

    for _ in 0..num_tasks {
        println!("Enter subject name:");
        io::stdin().read_line(&mut input).unwrap();
        let subject = input.trim().to_string();
        input.clear();

        println!("Enter priority (1 = Hardest, 5 = Easiest):");
        io::stdin().read_line(&mut input).unwrap();
        let priority: u8 = input.trim().parse().unwrap();
        input.clear();

        println!("Enter exam date (YYYY-MM-DD):");
        io::stdin().read_line(&mut input).unwrap();
        let exam_date = NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d").unwrap();
        input.clear();

        study_queue.push(StudyTask {
            subject,
            priority,
            exam_date,
        });
    }

    println!("\n**Study Plan Sorted by Priority & Exam Date:**");
    while let Some(task) = study_queue.pop() {
        println!(
            "{} (Priority: {}, Exam Date: {})",
            task.subject, task.priority, task.exam_date
        );
    }
}
