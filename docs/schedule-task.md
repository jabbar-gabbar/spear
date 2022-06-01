# Configure a schedule task to run Spear

## Mount a network attached drive or an external drive

```shell
sudo apt install cifs-utils

sudo mkdir /mnt/my_share

sudo nano /etc/fstab
# paste below in the file
//192.168.[num].[num]/my_share  /mnt/my_share  cifs  auto,username=\*\*\*,password=\*\*\*",nofail,x-systemd.automount 0       0
```

## Create a shell script

This shell script will run when the schedule task runs

```shell
sudo cat >>run.sh <<EOF
#!/bin/bash
cd /home/{user name}/spear
file_name=$(date +'%Y_%m_%d')
RUST_LOG=spear=info ./spear &>> log/log_$file_name
find /home/{user name}/spear/log -mindepth 1 -mtime +1 -type f -delete
EOF
```

## Create a cron job

Cron job runs the above shell script on the schedule

```shell
crontab -e
# schedule to run every 30 minutes
# add the line below in the file
*/10 * * * * /home/{user name}/spear/run.sh 2>&1 | logger -t spear
```

## View the logs of new job

```shell
grep spear /var/log/syslog
grep CRON /var/log/syslog
```
