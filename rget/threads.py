# 100 Bytes
total = 100
# Split on 5 threads
threads = 5
# A single threads byte amount
single = total/threads
# The Last single value
lastsingle = 0
for a in range(threads+1):
    if a == 0:
        continue
    print("From: {}\n\tTo: {}".format(lastsingle, single*a))
    lastsingle = single*a+1
