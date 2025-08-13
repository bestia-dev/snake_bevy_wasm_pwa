#!/bin/sh

printf "\033[0;33m    RUN ON WEB SERVER: Bash script to publish web site \033[0m\n"
printf "\n"
printf "\033[0;33m    First the development files are copied over SSH to the folder 'transfer_folder'. \033[0m\n"
printf "\033[0;33m    Then copy the files from 'transfer_folder' to the web server folder. \033[0m\n"
printf "\033[0;33m rsync -avz --delete-after /var/www/transfer_folder/hello_world /var/www/bestia.dev/hello_world \033[0m\n"
rsync -avz --delete-after rsync -avz --delete-after /var/www/transfer_folder/hello_world/ /var/www/bestia.dev/hello_world/

printf "\033[0;33m    Completed. \033[0m\n"
printf "\n"
