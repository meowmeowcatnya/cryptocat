import sys
import argparse
import secrets
import pathlib

def caesar(inputString: str, key: int) -> str:
    return "CEASAR"

parser = argparse.ArgumentParser(prog="cryptocat", description="A simple (not safe for actual usage) encryption program", epilog=":3")
parser.add_argument("-d", "--data", required=True, help="Data to be encrypted")
parser.add_argument("-m", "--method", choices=range(3), type=int, required=True, help="Encryption method. 1-Caesar")
parser.add_argument("-k", "--key", type=int, help="Encryption Key to be used. Will be created randomly if not specified. Depending on the encryption method different keys are accepted.")
parser.add_argument("-w", "--write", type=pathlib.Path, help="Not required. Writes encrypted data to this location")
args = parser.parse_args()

clearData = args.data
method = args.method
key = args.key
writeData = not args.write is None
if writeData:
    location = args.write

match method:
    case 1:
        if key is None:
            key = secrets.randbelow(26)
        elif key < 0 or key > 25: 
            raise Exception("For Caesar encryption, key must be an integer between 0 and 25 inclusive")
        encrypted = caesar(clearData, key)
    case 2:
        ##encryptionTwo()
        print("2")
    case 3:
        ##encryptionThree()
        print("3")

if writeData:
    print("Writes Data to", location)
else:    
    print(encrypted)
