import concurrent.futures
import os
import subprocess

# Define the directory where .btor2 files are located
directory = "/data/guangyuh/coding_env/eda_2_mc/xepic_testcase_for_participant/"
# Define the log directory
log_directory = "./log"
# Ensure the log directory exists
os.makedirs(log_directory, exist_ok=True)
# Define the command to run
command = "./mc"

# Function to run the command for a single file
def run_command(file_path):
    # Extract the base filename without the extension for the log file name
    base_name = os.path.basename(file_path)
    log_filename = os.path.splitext(base_name)[0] + ".log"
    # Construct the full path for the log file within the log directory
    full_log_path = os.path.join(log_directory, log_filename)
    # Construct the full command with the file path
    full_command = f"{command} {file_path}"
    # Run the command and capture the output
    result = subprocess.run(full_command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    # Save the output to the log file in the log directory
    with open(full_log_path, 'wb') as log_file:
        log_file.write(result.stdout)
    # Return the result and the full path to the log file for further processing if needed
    return result, full_log_path

# Function to find all .btor2 files in the directory
def find_btor2_files(directory):
    unique_files = {}
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(".btor2") and file not in unique_files:
                unique_files[file] = os.path.join(root, file)
    # Get the unique file paths
    btor2_files = list(unique_files.values())
    return btor2_files

def main():
    # Find all .btor2 files
    btor2_files = find_btor2_files(directory)
    
    # Run the commands in parallel using a process pool
    with concurrent.futures.ProcessPoolExecutor(max_workers=64) as executor:
        # Map the run_command function to all the files
        results = list(executor.map(run_command, btor2_files))
        
        # Process the results
        for result, full_log_path in results:
            if result.returncode == 0:
                print(f"Command for {result.args} completed successfully. Output saved to {full_log_path}.")
            else:
                print(f"Command for {result.args} failed with return code {result.returncode}. Check {full_log_path} for details.")

if __name__ == "__main__":
    main()