from a import b
from a import c, d
from a import e as f
from calc1 import Calc as FakeCalc ,Calc1 as FakeCalc1

def main():
    c = FakeCalc()
    print(c.add(1, 2))
