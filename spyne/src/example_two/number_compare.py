import sys

def main(args):
    
    numberone = int(args[0])
    numbertwo = int(args[1])
    
    if (numberone > numbertwo):
        print("The first number is greater than the second number: {} > {}".format(numberone, numbertwo))
    elif (numberone == numbertwo):
        print("The first number is equal to the second number: {} == {}".format(numberone, numbertwo))
    elif (numberone < numbertwo):
        print("The second number is greater than the first number: {} < {}".format(numberone, numbertwo))

if __name__ == "__main__":
    # Ignore the first argument, which is the script name
    arguments = sys.argv[1:]
    main(arguments)