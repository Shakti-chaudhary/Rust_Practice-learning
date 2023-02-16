#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hour: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hour: 40,
    };
    print_employee(me);
    print_employee(me);
    println!("{:?}", me.position);
    println!("{:?}", me);
}
