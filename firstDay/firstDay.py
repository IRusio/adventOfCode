path = 'C:/Users/Kuba/Desktop/adventOfCode/firstDay/sourceData.txt'

DataFile = open(path,'r')

#first task module one start
result = 0
for line in DataFile:
    if line.strip():
        result += int(line)

print(result)
#first task module one end
