#convert rgb value to hex - very simple in python
def rgb(r, g, b):
    list = [r, g, b]
    for n, i in enumerate(list):
        #make sure all values are between 0 and 255
        if i > 255:
            list[n] = 255
            
        if i < 0:
            list[n] = 0
    #convert list values into hexadecimal representation
    for n, i in enumerate(list):
            list[n] = '%02X'%list[n]
    #print them as a joined string
    result = ''.join(str(i) for i in list)
    return(result)
        
    pass