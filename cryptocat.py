import sys
import argparse
import secrets
import pathlib

def caesar(inputString: str, key: int) -> str:
    return "CEASAR"

parser = argparse.ArgumentParser(prog="cryptocat", description="A simple unsafe encryption program", epilog=":3")
parser.add_argument("-d", "--data", help="Data to be encrypted")
parser.add_argument("-m", "--method", choices=range(3), type=int, required=True, help="Encryption method. 0-Caesar")
parser.add_argument("-k", "--key", type=int, help="Encryption Key to be used. Will be created randomly if not specified. Depending on the encryption method different keys are accepted.")
parser.add_argument("-o", "--output", help="Stores encrypted data as file")
parser.add_argument("-i", "--input", help="filename of input file")
args = parser.parse_args()

data1 = args.data
data2 = args.input
method = args.method
key = args.key
if (data1 is None and data2 is None) or ((not data1 is None) and (not data2 is None)):
    raise Exception("Exactly one source of input data is required")
data = data1 if data2 is None else data2
if data2 is None:
    data = data1
    readingFromFile = False
else: 
    data = data2
    readingFromFile = True

if method is None:
    raise Exception("Must specify an encryption method")

saveToFile = not args.output is None
if saveToFile:
    location = args.output

match method:
    case 0:
        if key is None:
            key = secrets.randbelow(26)
        elif key < 0 or key > 25: 
            raise Exception("For Caesar encryption, key must be an integer between 0 and 25 inclusive")
        if readingFromFile:
            sourceFile = open(data, "r")
            data = sourceFile.read()
            sourceFile.close()
        encrypted = caesar(data, key)
    case 1:
        ##encryptionTwo()
        print("2")
    case 2:
        ##encryptionThree()
        print("3")

if saveToFile:
    print("Writing encrypted Data to", location)
    file = open(location, "w")
    file.write(encrypted)
    file.close()
else:    
    print(encrypted)
