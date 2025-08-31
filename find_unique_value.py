#given an array of numbers, all of them are the same, except one that is unique. to find the unique value:
def find_uniq(arr):
    #if the first and second values are the same, the target is somewhere else
    if arr[0] == arr[1]:
        #in that case iterate and find the one not matching arr[0]
        for i in arr:
            if i != arr[0]:
                n = i
    #if the first and second values are not the same, the 3rd value decides which one is the unique one
    elif arr[0] == arr[2]:
        n = arr[1]
    else:
        n = arr[0]
        
    return n   # n: unique integer in the array