

def read_file(filename):
    file1 = open(filename, 'r')
    for line in file1.readlines():
        yield line.strip()
    file1.close()
