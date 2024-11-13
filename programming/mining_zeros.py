import hashlib
import random

how_many_zeros_needed = 7

mining_round = 0

test_string = str(random.randint(0,256))

while(True):
    hashed_string = hashlib.sha256(test_string.encode('utf-8')).hexdigest()
    mining_round +=1
    if(hashed_string[0:how_many_zeros_needed ] =='0'*how_many_zeros_needed ):
        break
    test_string=hashed_string 

print('A random value '+test_string+' produced a SHA256 hash with at least '+str(how_many_zeros_needed)+' leading zeros . Winning hash is = '+hashed_string+' | This process took '+str(mining_round)+' rounds to complete.')
