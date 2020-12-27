import sys
import z3

INIT = 0xcbf29ce484222325
MULT = 0x00000100000001b3
FINI = 0x5bac903ba7d81967
MASK = 0xffffffffffffffff

valid_chars = 'abcdefghijklmnopqrstuvwxyz0123456789-_.'

def name(t, i):
    return f'{t}{i}'


def solve(k, length):
    """Solve k."""

    s = z3.Solver()

    hash_vars = {}
    x_vars = {}

    x = 0
    hash_vars[name('hash', x)] = z3.BitVec(name('hash', x), 64)
    s.add(hash_vars[name('hash', x)] == INIT)

    # Loop length times
    for _ in range(length):
        cur_xvar_name = name('x', x)
        cur_xvar = z3.BitVec(cur_xvar_name, 8)
        x_vars[cur_xvar_name] = cur_xvar
        s.add(cur_xvar < 0x7b)
        s.add(cur_xvar > 0x2c)

        # for v in range(0x2c, 0x7b):
        #     if chr(v) not in valid_chars:
        #         s.add(cur_xvar != v)

        x += 1

        # Get the current name of the var for the current state
        cur_name = name('hash', x)
        # Gen a new BitVec for the current var
        hash_vars[cur_name] = z3.BitVec(cur_name, 64)
        # Add constraints for the current state
        s.add(hash_vars[cur_name] ==
              (hash_vars[name('hash', x-1)] ^ z3.ZeroExt(56, cur_xvar)) * MULT)
    
    hash_vars[name('hash', x)] = z3.BitVec(name('hash', x), 64)
    s.add(hash_vars[name('hash', x)] ^ FINI == k)

    check = s.check()
    if check.r == 1:
        solved_str = ''.join(
            [chr(s.model()[v].as_long()) for v in x_vars.values()])
        print(f'[+] Found > {solved_str}:{k}')
        with open('log', 'a+') as f:
            f.write(f'{solved_str}:{k}\n')
        return True

    return False


def main(p):
    with open(p, 'r') as f:
        hashes = [hash.strip() for hash in f.readlines()]
    
    for hash in hashes:
        print(f'[*] Attempting to solve {hash}')
        found = False
        for l in range(7):
            found = solve(hash, l)

            if found:
                break

        if not found:
            print(f'[!] Failed to solve {hash}')


if __name__ == '__main__':
    if len(sys.argv) == 2:
        main(sys.argv[1])
    else:
        print('Usage: {sys.argv[0]} <hash file>')
