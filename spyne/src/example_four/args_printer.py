import sys

def main(args):
    print(f"Received arguments: {args}")

if __name__ == "__main__":
    # Ignore the first argument, which is the script name
    arguments = sys.argv[1:]
    main(arguments)