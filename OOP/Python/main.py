class Students:
    def total_attendance(self, att, week):
        return att * week


student1 = Students()
student1.name = "Mahi"
student1.attendance = 3
student1.week = 34
print(student1.total_attendance(student1.attendance, student1.week))


student2 = Students()
student2.name = "Nidhi"
student2.attendance = 5
student2.week = 23
print(student2.total_attendance(student2.attendance, student2.week))
