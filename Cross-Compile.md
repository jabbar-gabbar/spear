# Cross-compiling Rust on Ubuntu and macOS

Cross compiling produces a binary that can run on a different platform other than one on which the compiler is running.

Cross compiling helps programmers in situations where it is difficult or impossible to run a compiler e.g. in embedded devices where the resources are extremely limited to run a compiler. It is also sometimes helpful in running a release pipeline which generates binaries to target different computer platforms.

As part of the Spear project, I’ve generated the binary for targeted platforms such as Win, Linux and Raspberry-pi (Linux). The first two platforms are straightforward and do not require cross compiling as I can run Windows and Ubuntu runners in the release pipeline. It gets tricky to generate binary for Raspberry-Pi since the underlying Raspberry-Pi hardware is ARM architecture and I wasn’t able to find a release pipeline runner that supports ARM and Debian. I ended up cross compiling for Raspberry-Pi binary on Ubuntu runner in the pipeline and generated a statically linked binary. 

I generated Spear binary using statically linked because it is easier to set up using musl.cc. It gets tricky to generate dynamically linked cross compiled binary for a Raspberry-Pi. There are a couple of disadvantages, one the resulting binary is comparatively large in size as it needs to package all the required dependencies, two it can quickly become outdated as all the dependencies are packaged with it. Since I’ve a release pipeline, it can overcome the second disadvantage to update dependencies by running a new release pipeline. The advantage is that it can run independently on supported platforms even though a dependency is not installed.

## Ubuntu host

In this instance, I’ve used Ubuntu as host system and armv7-unknown-linux-musleabihf (Raspberry Pi 3 or higher) as target system. You can test this by running a docker container running a Ubuntu image.

```console
apt-get update
apt-get install -y --no-install-recommends git curl file wget xz-utils build-essential

mkdir /usr/local/opt
cd /
wget https://musl.cc/arm-linux-musleabihf-cross.tgz
tar -xf arm-linux-musleabihf-cross.tgz
ln -s /arm-linux-musleabihf-cross/bin/arm-linux-musleabihf-gcc /usr/local/bin/arm-linux-musleabihf-gcc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

exit
# log back again to the shell to load rustup after install

rustup target add armv7-unknown-linux-musleabihf
cat >>~/.cargo/config.toml <<EOF
[target.armv7-unknown-linux-musleabihf]
linker = "arm-linux-musleabihf-gcc"
EOF

git clone https://github.com/jabbar-gabbar/spear.git
cd spear
cargo release --target=armv7-unknown-linux-musleabihf
```

## macOS host

In the second instance, I've used macOS as host and armv7-unknown-linux-musleabihf (Raspberry Pi 3 or higher) as target system.

```console
brew install FiloSottile/musl-cross/musl-cross
brew reinstall FiloSottile/musl-cross/musl-cross --without-x86_--with-arm-hf
# if the last steps fails, run below to install musl-cross:
    sudo rm -rf /Library/Developer/CommandLineTools
    xcode-select --install
    brew reinstall FiloSottile/musl-cross/musl-cross --without-x86_--with-arm-hf
# end if

rustup target add armv7-unknown-linux-musleabihf
[target.armv7-unknown-linux-musleabihf]
linker = "arm-linux-musleabihf-gcc"

git clone https://github.com/jabbar-gabbar/spear.git
cd spear
cargo release --target=armv7-unknown-linux-musleabihf
```
