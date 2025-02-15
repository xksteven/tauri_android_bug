

https://developer.android.com/studio/install#linux

download android sdk

```bash
sudo apt-get update -y && sudo apt-get upgrade -y;\
sudo apt-get install -y curl git unzip xz-utils zip libglu1-mesa
sudo apt install curl git unzip xz-utils zip libglu1-mesa clang cmake ninja-build pkg-config libgtk-3-dev

mv ./google-chrome-stable_current_amd64.deb /tmp
sudo apt install ./google-chrome-stable_current_amd64.deb

ln -s /usr/bin/ninja /usr/bin/ninja-build

cargo install tauri-cli
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```


```bash
npx create-tauri-app read_anywhere --template svelte
cd read_anywhere
npm install

# For android
# npm run tauri android init
npm run tauri dev


npm run tauri add fs
npm install @tauri-apps/plugin-dialog

npm install pdfjs-dist epubjs jszip

npm install r2-navigator-js
```

On Mac

Installation

```
# Install brew
brew install npm
# Install Xcode (pre req for tauri)

# Install rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# For ios
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
brew install cocoapods

cd 
npm install
```

Run it

```
npm run tauri dev
```

For Android

```
npm run tauri android init 

sudo apt install openjdk-21-jdk
export JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
java -version

export NDK_HOME=$(ls -d /home/steven/Android/Sdk/ndk/*/ | sort -V | tail -n 1)

#Make sure the app and computer are running on the same network

sudo /home/steven/Android/Sdk/platform-tools/adb root

adb reverse tcp:1420 tcp:1420
export TAURI_DEV_HOST="localhost"
npm run tauri android dev
```




# Debugging notes:


## Useful debugging commmands


adb logcat -v color | egrep -C 5 "(com.read_anywhere.app|TAURI|dialog|pdf)"

Go to browser and visit chrome://inspect

