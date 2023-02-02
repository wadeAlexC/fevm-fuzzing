# bin python3
# python3 gobuild.py

import os
import argparse

parser = argparse.ArgumentParser(description="of argparse")
parser.add_argument('-n','--name', default='lib.a')
parser.add_argument('-p','--path',default="./")
args = parser.parse_args()
# print(args)
name = args.name
path = args.path
print('build info {}  {}'.format(name,path))

command = 'go build -o {}/{} -buildmode=c-archive calculate.go'.format(path,name)

os.popen(command)
