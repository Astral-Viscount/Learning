class Students:
    def __init__(self, name, attendance, week):
        self.name = name
        self.attendance = attendance
        self.week = week

        print(f"Name: {self.name}\nAttendace per Week: {self.attendance}\nWeeks Attended: {self.week}")
        
        print(f"Total Days Attended: {self.total_attendance(self.attendance, self.week)}")

    def total_attendance(self, att, week):
        return att * week


student1 = Students("Mahi", 3, 34)
print()
student2 = Students("Nidhi", 5, 23)
