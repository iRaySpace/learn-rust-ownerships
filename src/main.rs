#[derive(Debug)]
struct Student {
    age: u8,
}

#[derive(Debug)]
struct Enemy<'a> {
    health: &'a u8,
}

fn print_student_age_readonly_ref(student: &Student) {
    println!("{:?}", student.age);
}

fn set_student_age(student: &mut Student, age: u8) {
    student.age = age;
}

fn get_older_student_by_age<'a>(sx: &'a Student, sy: &'a Student) -> &'a Student {
    if sx.age > sy.age {
        sx
    } else {
        sy
    }
}

fn print_enemy_health(e: &Enemy) {
    println!("{:?}", e.health);
}

fn set_enemy_health<'a>(e: &mut Enemy<'a>, health: &'a u8) {
    e.health = health;
}

fn main() {
    let mut ivan_student = Student { age: 25 };
    // print_student_age_readonly_ref(&ivan_student);
    // set_student_age(&mut ivan_student, 100);
    // print_student_age_readonly_ref(&ivan_student);

    let mut ray_student = Student { age: 27 };

    let oldest_student: &Student;
    oldest_student = get_older_student_by_age(&ivan_student, &ray_student);
    print_student_age_readonly_ref(oldest_student);

    let health = 100;
    let mut enemy = Enemy { health: &health };
    print_enemy_health(&enemy);

    set_enemy_health(&mut enemy, &50);
    print_enemy_health(&enemy);

    println!("{:?}", health);
}
