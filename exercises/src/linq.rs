#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    name: String,
    grade: u32,
}
#[allow(dead_code)]
fn top_students(students: &mut [Student]) -> Vec<String> {
    students.sort_by(|a, b| b.grade.cmp(&a.grade)); // sort descending
    students
        .iter()
        .filter(|s| s.grade >= 90)
        .take(3)
        .map(|s| format!("{}: {}", s.name, s.grade))
        .collect()
}
#[allow(dead_code)]
pub fn test() {
    let mut students = vec![
        Student {
            name: "Alice".into(),
            grade: 95,
        },
        Student {
            name: "Bob".into(),
            grade: 88,
        },
        Student {
            name: "Carol".into(),
            grade: 92,
        },
        Student {
            name: "Dave".into(),
            grade: 97,
        },
        Student {
            name: "Eve".into(),
            grade: 91,
        },
    ];
    let result = top_students(&mut students);
    assert_eq!(result, vec!["Dave: 97", "Alice: 95", "Carol: 92"]);
    println!("{result:?}");
}
