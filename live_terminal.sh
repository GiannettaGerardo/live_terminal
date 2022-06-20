#!/bin/bash

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

   echo -en "\ec"
   ls -la --color=auto "$1"
   ls -la "$1" > "$file"

   while true; do

      result=$(ls -la "$1" | diff "$file" -)

      if [[ ! -z "$result" ]]
      then
         echo -en "\ec"
         ls -la --color=auto "$1"
         ls -la "$1" > "$file"
      fi

      result=""
      sleep 0.1s

   done
}

# call the main function
main "$1"
