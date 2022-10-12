# Configure a schedule task to run Spear

## Mount a network attached drive or an external drive

Skip this step if you don't want to connect an external or a network drive.

```shell
sudo apt install cifs-utils

sudo mkdir /mnt/my_share

sudo nano /etc/fstab
# paste below in the file
//192.168.[num].[num]/my_share  /mnt/my_share  cifs  auto,username=\*\*\*,password=\*\*\*",nofail,x-systemd.automount 0       0
```

## Create a shell script

This shell script will create a new file `run.sh`. When it runs on a schedule, it configures log files output, executes `spear` and clean up old logs files at the end of a run.

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

Schedule `run.sh` using cron job

```shell
crontab -e
```

To run every 10 minutes:

```shell
*/10 * * * * /home/{user name}/spear/run.sh 2>&1 | logger -t spear
```

## View the logs of the job

```shell
grep spear /var/log/syslog
grep CRON /var/log/syslog
```

or

```shell
tail /home/spear/log/log_file_name
```
