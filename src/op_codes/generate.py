print("// GENERATED: DO NOT EDIT")

with open('codes.txt') as f:
    print("pub enum OpCodes {")
    lines = [line for line in f]
    for line in lines:
        name, token = line.rstrip().split(" ")
        print(f"\t{name.upper()} = {token},")
    print("}")
