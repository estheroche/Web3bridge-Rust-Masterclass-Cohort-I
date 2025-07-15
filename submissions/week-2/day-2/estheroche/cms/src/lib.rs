#[derive(Debug, Clone, PartialEq)]
enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    grade: u8,
    status: Status,
}

struct ClassManagement {
    students: Vec<Student>,
}

impl ClassManagement {
    fn new() -> Self {
        ClassManagement {
            students: Vec::new(),
        }
    }

    fn register_student(&mut self, name: String, grade: u8) {
        let student = Student {
            name,
            grade,
            status: Status::Active,
        };
        self.students.push(student);
    }

    fn edit_student(&mut self, index: usize, new_name: Option<String>, new_grade: Option<u8>) {
        if let Some(student) = self.students.get_mut(index) {
            if let Some(name) = new_name {
                student.name = name;
            }
            if let Some(grade) = new_grade {
                student.grade = grade;
            }
        }
    }

    fn update_status(&mut self, index: usize, new_status: Status) {
        if let Some(student) = self.students.get_mut(index) {
            student.status = new_status;
        }
    }

    fn delete_student(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        }
    }

    fn view_students(&self) -> &Vec<Student> {
        &self.students
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut cms = ClassManagement::new();
        cms.register_student("Alice".to_string(), 10);
        assert_eq!(cms.students.len(), 1);
        assert_eq!(cms.students[0].name, "Alice");
        assert_eq!(cms.students[0].grade, 10);
        assert_eq!(cms.students[0].status, Status::Active);
    }

    #[test]
    fn test_edit_student() {
        let mut cms = ClassManagement::new();
        cms.register_student("Bob".to_string(), 9);
        cms.edit_student(0, Some("Bobby".to_string()), Some(10));
        assert_eq!(cms.students[0].name, "Bobby");
        assert_eq!(cms.students[0].grade, 10);
    }

    #[test]
    fn test_update_status() {
        let mut cms = ClassManagement::new();
        cms.register_student("Carol".to_string(), 8);
        cms.update_status(0, Status::Inactive);
        assert_eq!(cms.students[0].status, Status::Inactive);
    }

    #[test]
    fn test_delete_student() {
        let mut cms = ClassManagement::new();
        cms.register_student("Dan".to_string(), 7);
        assert_eq!(cms.students.len(), 1);
        cms.delete_student(0);
        assert_eq!(cms.students.len(), 0);
    }

    #[test]
    fn test_view_students() {
        let mut cms = ClassManagement::new();
        cms.register_student("Eve".to_string(), 6);
        let students = cms.view_students();
        assert_eq!(students.len(), 1);
        assert_eq!(students[0].name, "Eve");
    }
}
