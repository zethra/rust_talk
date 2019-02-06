import sys

def main():
    buf = ""
    args = sys.argv[1:]
    length = len(args) - 1
    for i, arg in enumerate(args):
        buf += arg
        if i < length:
            buf += ' '
    print(buf)

if __name__ == '__main__':
    main()
