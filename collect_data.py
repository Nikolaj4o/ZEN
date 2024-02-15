#!/usr/bin/env python  
# Python code to illustrate split() function
print("shallownet")
shallownet = {};
with open("shallownet.log", "r") as file:
    data = file.readlines()
    for line in data:
        word = line.split()
        if (len(word) > 0):
            if (word[1] == 'time'):
                shallownet[word[0]] = word[2]
            else:
                shallownet[word[0]] = word[1]
            #print (word)
                
for line in shallownet:
    print(line, shallownet[line])

print("lenet sf")
lenet_sf = {};
with open("lenet_small_face.log", "r") as file:
    data = file.readlines()
    for line in data:
        word = line.split()
        if (len(word) > 0):
            if (word[1] == 'time' or word[1] == 'size:'):
                lenet_sf[word[0]] = word[2]
            else:
                lenet_sf[word[0]] = word[1]
            #print (word)

for line in lenet_sf:
    print(line, lenet_sf[line])


print("lenet sc")
lenet_sc = {};
with open("lenet_small_cifar.log", "r") as file:
    data = file.readlines()
    for line in data:
        word = line.split()
        if (len(word) > 0):
            if (word[1] == 'time' or word[1] == 'size:'):
                lenet_sc[word[0]] = word[2]
            else:
                lenet_sc[word[0]] = word[1]
            #print (word)

for line in lenet_sc:
    print(line, lenet_sc[line])

print("lenet mc")
lenet_mc = {};
with open("lenet_medium_cifar.log", "r") as file:
    data = file.readlines()
    for line in data:
        word = line.split()
        if (len(word) > 0):
            if (word[1] == 'time' or word[1] == 'size:'):
                lenet_mc[word[0]] = word[2]
            else:
                lenet_mc[word[0]] = word[1]
            #print (word)

for line in lenet_mc:
    print(line, lenet_mc[line])