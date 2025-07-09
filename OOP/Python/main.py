class Students:
    def __init__(self, name:str, attendance: int, week = 0):

        #Validate the values received with Custom Exception Errors
        assert attendance >= 0, f"Attendance must be more than or equa 0 days! Attendance Received: {attendance}"
        assert week >= 0, f"Must be more than or equal 0 Week! Week Received: {week}"

        #Assign Values
        self.name = name
        self.attendance = attendance
        self.week = week

        print(f"Name: {self.name}\nAttendace per Week: {self.attendance}\nWeeks Attended: {self.week}")
        
        print(f"Total Days Attended: {self.total_attendance()}")

    def total_attendance(self) -> int:
        return self.attendance * self.week


student1 = Students("Mahi", -3, 34)
print()
student2 = Students("Nidhi", 5, 23)
