class Students:
    avg_rate = 0.8 #On average students show up 80% of the time of the submiited time
    everyone = []

    def __init__(self, name:str, attendance: int, week = 0):

        #Validate the values received with Custom Exception Errors
        assert attendance >= 0, f"Attendance must be more than or equal to 0 days! Attendance Received: {attendance}"
        assert week >= 0, f"Must be more than or equal 0 Week! Week Received: {week}"

        #Assign Values
        self.name = name
        self.attendance = attendance
        self.week = week

        #Adding everyone
        Students.everyone.append(self)

        print(f"Name: {self.name}\nAttendace per Week: {self.attendance}\nWeeks Attended: {self.week}")

    def total_attendance(self) -> int:
        self.total = self.attendance * self.week
        return self.total
    
    def apply_avg_rate(self) -> float:
        return round(self.total_attendance() * self.avg_rate, 0)
    
    # Representing the values as close to as it was given to us
    def __repr__(self) -> str:
        return f"Students('{self.name}', {self.attendance}, {self.week})"


student1 = Students("Mahi", 3, 34)
print(f"Total days attended: {student1.apply_avg_rate()}")

print()

student2 = Students("Nidhi", 5, 23)
student2.avg_rate = 0.5
print(f"Total days attended: {student2.apply_avg_rate()}")

print()

# Looking at the all the values
print(Students.everyone)

# # Different Attributes
# print(Students.avg_rate)
# print(Students.__dict__) # All the class attributes

# print(student1.avg_rate)
# print(student1.__dict__) # All the instance attributes