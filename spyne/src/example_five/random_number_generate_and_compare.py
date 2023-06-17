import random

def main():
    
    numberone = random.randint(1, 100)
    numbertwo = random.randint(1, 100)
    
    if (numberone > numbertwo):
        print("The first number is greater than the second number: {} > {}".format(numberone, numbertwo))
    elif (numberone == numbertwo):
        print("The first number is equal to the second number: {} = {}".format(numberone, numbertwo))
    elif (numberone < numbertwo):
        print("The second number is greater than the first number: {} < {}".format(numberone, numbertwo))

if __name__ == "__main__":
    main()