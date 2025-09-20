import os

root_dir = r'D:\svcs'
skip_dirs = {r'D:\svcs\target', r'D:\svcs\docs', r'D:\svcs\scripts', r'D:\svcs\systemverilog\LRM'}
output_file = '.\out\output.txt'

with open(output_file, 'w', encoding='utf-8') as out:
    for subdir, dirs, files in os.walk(root_dir):
        # Skip directories in skip_dirs
        if any(os.path.commonpath([subdir, skip_dir]) == skip_dir for skip_dir in skip_dirs):
            continue
        for file in files:
            file_path = os.path.join(subdir, file)
            # Write file path
            out.write(file_path + '\n')
            # Write file content
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    out.write(f.read())
            except Exception as e:
                out.write(f'\n[ERROR READING FILE: {e}]\n')
            # Write a newline after each file
            out.write('\n\n')

print('Done writing files to output.txt')
