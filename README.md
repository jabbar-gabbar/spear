# Spear

Spear can be used to automate the backup of media files to AWS S3. AWS S3 offers cheap storage especially if it is used purely for archival purposes and when you don't need to retrieve the files for day-to-day use.

AWS S3 Glacier cost less than 1 penny per GB per month, so it could cost approximately $10 a month for 1 TB of S3 Glacier Instant Retrieval. S3 Glacier Deep Archive could cost approximately $1 per month for 1 TB of storage.

The main use case is to automate long term data archival offsite.

Spear scans specified folders for new files that were added after the last backup, and uploads them in a next run. Spear uploads them to a specified S3 bucket. A lifecycle rule can be configured on a S3 bucket to move the uploaded files to an appropriate S3 Glacier storage tier based on your need.

## Configuration

You can use a pre-built binary of your choice of your platform from the Releases page [https://github.com/jabbar-gabbar/spear/releases](https://github.com/jabbar-gabbar/spear/releases). If you wish to build the binary yourself, follow the steps in [Build](#build).

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

```Shell
# Use editor of your choice , but I use nano.
nano Settings.toml
```

You will see an empty backup toml file as shown below.

```Toml
[[backup]]
source_directory_path  = ""
s3_bucket = ""
inventory_file_path = ""
```

You can specify more than one source and destination in the settings file. The one shown below performs a backup from `/home/Pictures` directory to `your_aws_s3_bucket_name_goes_here` S3 bucket and stores uploaded inventory files names in `inventory/inventory_file_name` file. You will need to specify appropriate names for inventory files in the settings. Spear will create inventory files if they don't exist the first time.

```Toml
[[backup]]
source_directory_path  = "/home/Pictures"
s3_bucket = "your_aws_s3_bucket_name_goes_here"
inventory_file_path = "inventory/inventory_file_name"

[[backup]]
source_directory_path  = "/home/Videos"
s3_bucket = "s3-bucket-2"
inventory_file_path = "inventory/videos_inv"

[[backup]]
source_directory_path  = "inventory"
s3_bucket = "s3-bucket-3"
inventory_file_path = "inventory/inventory_inv"
```

In case the configuration gets wiped out for some reason, it is recommended to backup inventory files to S3 as shown in the third backup configuration above.

Spear uses AWS SDK to communicate with S3 bucket. You will need to choose access keys and store them appropriately in your environment. Please see AWS documentation for more information [https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html)

Spear can run on schedule basis, you can find more information [here.](docs/schedule-task.md)

## Build

Install Rust from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

```Shell
git clone git@github.com:jabbar-gabbar/spear.git
cd spear
cargo build
```
