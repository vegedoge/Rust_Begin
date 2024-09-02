def collatz(limit: int) -> int :
    '''realize collatz sequence
    limit: upper limit for input number
    return: longest number
    '''
    longest_num = 1;
    longest_len = 1;
    for n in range(limit + 1):
        x = n
        counter = 1
        
        while x > 1:
            counter += 1
            if x % 2 == 0:
                x = x / 2
            else:
                x = 3 * x + 1
        if counter > longest_len:
            longest_len = counter
            longest_num = n
    print(f"{longest_num}") 
    return longest_num
    
if __name__ == "__main__":
    limit = 1000000
    collatz(limit)
    

