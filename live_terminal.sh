#!/bin/bash

# refresh the command line
#
# @param $1 main file for the check
# @param $2 directory to monitor
__refresh() {
   echo -en "\ec"
   ls -la --color=auto "$2"
   ls -la "$2" > "$1"
}

# check if the exists an argument for the script and if it's a directory
__check_arguments() {
   if [[ -z "$1" ]]
   then
      echo "Error. No argument. Exit..."
      exit 1
   fi

   if [[ ! -d "$1" ]]
   then
      echo "Error. Argument isn't a directory. Exit..."
      exit 1
   fi
}

# main function
main() {
   __check_arguments "$1"

   local file=".f.txt"
   local result=""

   __refresh "$file" "$1"

   while true; do

      result=$(ls -la "$1" | diff "$file" -)

      if [[ ! -z "$result" ]]
      then
         __refresh "$file" "$1"
      fi

      result=""
      sleep 0.1s

   done
}

# call the main function
main "$1"
