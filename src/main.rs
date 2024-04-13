use std::io;

fn main() {
    let mut count: i32 = 0;
    let mut marks: Vec<i32> = Vec::new();

    println!("Enter number of students: ");
    let mut num_students: String = String::new();

    io::stdin()
        .read_line(&mut num_students)
        .expect("Failed to read line");

    let num_students: i32 = num_students
        .trim()
        .parse()
        .expect("Please give me a correct string number.");

    while count < num_students {
        println!("Mark {}: ", count + 1);

        let mut mark: String = String::new();

        io::stdin()
            .read_line(&mut mark)
            .expect("Failed to read line");

        let mark: i32 = match mark.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a real mark for the student!");
                continue;
            }
        };
        marks.push(mark);
        count += 1;
    }
    let average_mark: f64 = calculate_average(&marks);
    println!("Here is the list of marks: {:?}", marks);
    println!("Average mark for the class: {}", average_mark);
    display_student_marks(&marks, average_mark);
}

fn calculate_average(list_of_marks: &Vec<i32>) -> f64 {
    let total_marks = list_of_marks.into_iter().fold(0, |a, b| a + b);
    return total_marks as f64 / list_of_marks.len() as f64;
}

fn display_student_marks(marks: &Vec<i32>, class_average: f64) {
    for (index, mark) in marks.into_iter().enumerate() {
        println!(
            "Student {}, Diference: {}",
            index + 1,
            *mark as f64 - class_average
        );
    }
}
