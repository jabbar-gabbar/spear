# Spear

Spear can be used to automate the backup of media files to AWS S3. AWS S3 offers cheap storage especially if it is used purely for archival purposes and when you don't need to retrieve the media files for day-to-day use.

AWS S3 Glacier cost less than 1 penny per Gb per month, so it could cost approximately $10 a month for 1 Tb for S3 Glacier Instant Retrieval. 1 Tb stored on S3 Glacier Deep Archive could cost approximately $1 per month.

The main use case of the Spear is to automate long term data backup offsite.

Spear scans the specified folders for any new files that were not included in the backup, and uploads them in the next run. Spear uploads them to a specified S3 bucket but does not move them to Glacier. A lifecycle rule can be configured on a S3 bucket to move the uploaded files to an appropriate S3 storage tier based on your need.

## Configuration

You can use a pre-built binary for choice of your platform from the Releases page [https://github.com/jabbar-gabbar/spear/releases](https://github.com/jabbar-gabbar/spear/releases).

```Shell
mkdir spear

wget https://github.com/jabbar-gabbar/spear/releases/download/v0.1.0-alpha.2/spear-v0.1.0-alpha.2-armv7-unknown-linux-musleabihf.tar.gz

tar -xf spear-v0.1.0-alpha.2-armv7-unknown-linux-musleabihf.tar.gz
mkdir inventory
```

If you wish to build the binary follow the steps in [Build](#build). Continue to the next step for the prebuilt one.

### Settings.toml

```Shell
# edit the file using choice of your editor
nano Settings.toml
```

You can specify more than one source and destination in the settings file. The one below performs a backup of `/home/Pictures` to `your_aws_s3_bucket_name` S3 bucket and stores uploaded inventory files names in `inventory/pictures_inv` file. Spear will create inventory files if they don't exist the first time based on the name specified in the settings below.

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

Spear uses AWS SDK to communicate with S3 bucket. You will need to choose access keys and store them appropriately in your environment, please see AWS documentation for more information [https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html)

You can setup to run Spear on schedule basis, you can find more information [here.](docs/schedule-task.md)

## Build

Install Rust from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

```Shell
git clone git@github.com:jabbar-gabbar/spear.git
cd spear
cargo build
```
