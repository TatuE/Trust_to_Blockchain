import os
import glob
from datetime import datetime

output_file_name ='ter_password_list'

output_file_name_timestamped = f'{output_file_name}_{datetime.now().strftime("%d-%m-%Y")}.txt'
target_file_type = '*.txt'

target_files_l = []
words_processed = set()
word_count = 0
working_directory = os.path.dirname(os.path.realpath(__file__))

sys_output_file_name = f'{working_directory}/{output_file_name_timestamped}'

output_file = open(sys_output_file_name ,"w")
print('')
print('*********************')
print('Program started!')
print('*********************')
print('')


for f in glob.glob("*.txt"):
    if f.find(output_file_name) < 0:
        target_files_l.append(f)

print('Target files found:')
for f in target_files_l:
    print(f)

for file in target_files_l:

    sys_input_file_name = f'{working_directory}/{file}'
    input_file = open(sys_input_file_name,'r')

    print('')
    print('*********************')
    print(f'Processing file : {file}')
    print('*********************')
    print('')

    for word in input_file:
        word_count += 1

        if word not in words_processed and word != '':
            output_file.write(word)
            words_processed.add(word)

input_file.close()
output_file.close()

print('')
print('*********************')
print('Program complete!')
print('*********************')
print('')
print(f'Number of words processed : {word_count}')
print(f'Number of unique words found : {str(len(words_processed))}')
print(f'New word list saved, file name : {output_file_name_timestamped}')