
a = []
with open('input.txt', 'r') as f:
    for l in f:
        l = l.strip().replace(' ', '')
        b = []
        for c in l:
            b.append(c)
        a.append(b)


def printboard(a):
    for i in a:
        print(''.join(i))


printboard(a)

while True:
    try:
        k = input("> ")
    except EOFError:
        break
    if k.strip() == 'done':
        break
    ii, jj = [int(i) for i in k.split(',')]
    print(f"Inverting {ii}, {jj}")
    for i, j in [[ii, jj], [ii+1, jj], [ii-1, jj], [ii, jj+1], [ii, jj-1]]:
        if i>=0 and j>=0:
            try:
                if a[i][j] == '0':
                    a[i][j] = '1'
                elif a[i][j] == '1':
                    a[i][j] = '0'
            except IndexError:
                pass
    #printboard(a)
print()
printboard(a)
