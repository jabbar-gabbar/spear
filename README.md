# Spear

Spear can automate the backup of media files to AWS S3. The primary use case is to automate long-term data archival offsite in case onsite media files are lost. 

AWS S3 offers cheap storage when you don't need to retrieve the files for daily use. For example, S3 Glacier Deep Archive could cost roughly $1 per month for 1 TB of storage.

Spear scans source folders for new files from the last backup and uploads them to a specified S3 bucket. S3 provides lifecycle rules that automatically move recently uploaded files to a cheaper tier, such as S3 Glacier, based on your need.

## Configuration

You can use a pre-built binary of the choice of your platform from the Releases page [https://github.com/jabbar-gabbar/spear/releases](https://github.com/jabbar-gabbar/spear/releases). If you wish to build the binary yourself, follow the steps in [Build](#build).

### Download

```Shell
mkdir spear
cd spear
mkdir inventory

wget https://github.com/jabbar-gabbar/spear/releases/download/v0.1.0-alpha.2/spear-v0.1.0-alpha.2-armv7-unknown-linux-musleabihf.tar.gz

tar -xf spear-v0.1.0-alpha.2-armv7-unknown-linux-musleabihf.tar.gz
```

### Settings.toml

Settings.toml files stores configuration for your file source and destination.

You will see an empty backup toml file, as shown below.

```Toml
[[backup]]
source_directory_path  = ""
s3_bucket = ""
inventory_file_path = ""
excluded_extensions = ""
```

You can specify more than one source and destination in the settings file. The one below performs a backup from `/home/Pictures` directory to `your_aws_s3_bucket_name` S3 bucket and stores uploaded inventory file names in `inventory/inventory_file_name` file. You will need to specify appropriate names for inventory files in the settings. Spear will create inventory files if they don't exist the first time.

```Toml
[[backup]]
source_directory_path  = "/home/Pictures"
s3_bucket = "your_aws_s3_bucket_name"
inventory_file_path = "inventory/inventory_file_name"
excluded_extensions = "png,pdf,mov"

[[backup]]
source_directory_path  = "/mnt/my_share/iphone"
s3_bucket = "s3-bucket-2"
inventory_file_path = "inventory/videos_inv"
excluded_extensions = "pdf"
```

Spear uses AWS SDK to communicate with S3 bucket. You will need to use access keys and store them appropriately in your environment. Read AWS documentation for more information [https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html)

Spear can run on a scheduled basis. You can find more information [here.](docs/schedule-task.md)

## Build

Install Rust from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

```Shell
git clone git@github.com:jabbar-gabbar/spear.git
cd spear
cargo build
```
