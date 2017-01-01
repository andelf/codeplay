#!/usr/bin/env python2


# handle perf script format

from __future__ import print_function

import sys
import re


def is_rust_hash(s):
    try:
        return s.startswith('h') and int(s[1:], 16)
    except:
        return False


def demangle_element(s):
    ret = []
    if s.startswith('_$'):
        s = s[1:]

    s = s.replace('..', '::')
    s = s.replace("$SP$", "@") \
         .replace("$BP$", "*") \
         .replace("$RF$", "&") \
         .replace("$LT$", "<") \
         .replace("$GT$", ">") \
         .replace("$LP$", "(") \
         .replace("$RP$", ")") \
         .replace("$C$", ",") \
         .replace("$u7e$", "~") \
         .replace("$u20$", " ") \
         .replace("$u27$", "'") \
         .replace("$u5b$", "[") \
         .replace("$u5d$", "]") \
         .replace("$u7b$", "{") \
         .replace("$u7d$", "}") \
         .replace("$u3b$", ";") \
         .replace("$u2b$", "+") \
         .replace("$u22$", "\"")

    return s

class Demangle(object):
    def __init__(self, original, inner, valid=True, elements=[]):
        self.original = original
        self.inner = inner
        self.valid = valid
        self.elements = elements


    def __repr__(self):
        return self.original

    def __str__(self):
        if not self.valid:
            return self.original

        segs = list(map(demangle_element, self.elements))

        if is_rust_hash(segs[-1]):
            segs = segs[:-1]

        return '::'.join(segs)



def demangle(s):
    valid = True
    inner = s
    if len(s) > 4 and s.startswith('_ZN') and s.endswith('E'):
        inner = s[3:len(s)-1]
    elif len(s) > 3 and s.startswith('ZN') and s.endswith('E'):
        inner = s[2:len(s)-1]
    else:
        valid = False

    rest = inner
    elements = []

    while valid:
        i = 0
        for idx, c in enumerate(rest):
            if c.isdigit():
                i = i*10 + int(c)
            else:
                break

        if i == 0:
            valid = rest == ""
            break

        if len(rest[idx:idx+i]) != i:
            valid = False
        else:
            elements.append(rest[idx:idx+i])
            rest = rest[idx+i:]

    return Demangle(s, inner, valid, elements)


def test():
    print(demangle("_ZN7rocksdb9BlockIter4SeekERKNS_5SliceE"))
    print(demangle("_ZN12test$RF$test4foobE"))
    print(demangle("_ZN35Bar$LT$$u5b$u32$u3b$$u20$4$u5d$$GT$E"))
    print(demangle("_ZN28_$u7b$$u7b$closure$u7d$$u7d$E"))
    print(demangle("_ZN71_$LT$Test$u20$$u2b$$u20$$u27$static$u20$as$u20$foo..Bar$LT$Test$GT$$GT$3barE"))
    print(demangle("_ZN3foo17h05af221e174051e9E"))
    print(str(demangle("_ZN4tikv7storage4mvcc6reader10MvccReader4seek17h8518c19d47619655E")))
    print(demangle("_ZN4tikv7storage6engine6raftkv137_$LT$impl$u20$tikv..storage..engine..Iterator$u20$for$u20$tikv..raftstore..coprocessor..region_snapshot..RegionIterator$LT$$u27$a$GT$$GT$4seek17h1c0795529ab57299E"))


def demangle_str(match):
    s = match.group(0).strip()
    return ' ' + str(demangle(s)) + ' '

def main():
    for line in sys.stdin:
        if len(line) > 10:
            print(re.sub(r'\s(_ZN[^\s]*E)\s', demangle_str, line), end='')
        else:
            print(line, end='')



if __name__ == '__main__':
    main()
    #test()
