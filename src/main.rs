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
        println!("Mark for Student {}: ", count + 1);

        let mut mark: String = String::new();

        io::stdin()
            .read_line(&mut mark)
            .expect("Failed to read the mark!");

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

    let total_mark: i32 = calculate_total(&marks);
    let average_mark: f32 = calculate_average(&marks);
    println!("Here is the list of marks: {:?}", marks);
    println!("Total mark for the class: {}", total_mark);
    println!("Average mark for the class: {}", average_mark);
    display_student_marks(&marks, average_mark);
}

fn calculate_total(list_of_marks: &Vec<i32>) -> i32 {
    return list_of_marks.into_iter().fold(0, |a, b| a + b);
}

fn calculate_average(list_of_marks: &Vec<i32>) -> f32 {
    let total_marks = calculate_total(list_of_marks);
    return total_marks as f32 / list_of_marks.len() as f32;
}

fn display_student_marks(marks: &Vec<i32>, class_average: f32) {
    for (index, mark) in marks.into_iter().enumerate() {
        println!(
            "Student {}, Diference: {}",
            index + 1,
            *mark as f32 - class_average
        );
    }
}
