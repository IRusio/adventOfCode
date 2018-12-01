path = 'C:/Users/Kuba/Desktop/adventOfCode/firstDay/sourceData1.txt'

DataFile = open(path,'r')

listOfNumbers = []
listOfResults = []
#first task module one start
result = 0
for line in DataFile:
    if line.strip():
        result += int(line)
        listOfNumbers.append(int(line))
        listOfResults.append(result)

print(result)
#first task module one end
DataFile.close()


#first task module two start
DataFile = open(path, 'r')
list = list()

one = 1
while one == 1:
    for number in listOfNumbers:
        result += number
        if result in listOfResults:
            print(result)
            one = 0
            break
#first task module two end