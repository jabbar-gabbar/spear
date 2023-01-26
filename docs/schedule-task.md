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

## Configure a schedule job

### Create a shell script

Create a new shell script `run.sh` or whatever name you like. When it runs on a schedule, it configures log files output, runs `spear` and clean up old logs files at the end of a run.

```shell
#!/bin/bash
cd /home/{user name}/spear
file_name=$(date +'%Y_%m_%d')
RUST_LOG=spear=info ./spear &>> log/log_$file_name
find /home/{user name}/spear/log -mindepth 1 -mtime +1 -type f -delete
```

### Create a trigger

In order to run only one instance of `run.sh` at a time for backup that may run longer than next scheduled run, you can use a trigger script that checks if the previous `run.sh` job is completed. You can find more information about this approach at [SimpleIt.Rocks](https://simpleit.rocks/linux/shell/prevent-running-of-duplicate-cron-jobs/) blog.

`caller.sh`

```shell
#!/bin/bash
if pidof -o %PPID -x "run.sh">/dev/null; then
echo "Process is already running"
exit 1
else
/home/{user}/spear/run.sh
fi
```

### Configure a cron job

Schedule using cron job:

```shell
crontab -e
```

Add the following line to the cron editor to schedule `caller.sh` every 12 hours:

```shell
0 */12 * * * /home/{user}/spear/caller.sh 2>&1 | logger -t spear
```

View the logs of the job:

```shell
grep spear /var/log/syslog
grep CRON /var/log/syslog
```

View spear logs:

```shell
tail /home/spear/log/log_file_name
```
