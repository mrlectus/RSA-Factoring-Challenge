#!/usr/bin/python3
from sys import argv
from math import sqrt
from multiprocessing import Pool

def find_factor(x):
    if x % 2 == 0:
        return (x//2, 2)
    for i in range(3, int(sqrt(x)) + 1, 2):
        if x % i == 0:
            return (x//i, i)
    return (-1, -1)
