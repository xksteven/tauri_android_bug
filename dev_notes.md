

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

cd src-tauri
cargo add base64 tokio warp
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

## Debugging notes


```
adb logcat

02-13 14:32:43.119  1722  4736 W PackageConfigPersister: App-specific configuration not found for packageName: com.read_anywhere.app and userId: 0
02-13 14:32:43.119 28169 28169 I AndroidIME: InputBundleManager.loadActiveInputBundleId():407 loadActiveInputBundleId: und-Latn-x-password, password
02-13 14:32:43.121 17223 17223 V Tauri/Plugin: Tauri plugin: pluginId: dialog, command: showFilePicker
02-13 14:32:43.191  1722  4736 I ActivityTaskManager: START u0 {act=android.intent.action.PICK typ=application/* (has extras)} with LAUNCH_MULTIPLE from uid 10050 result code=-91
02-13 14:32:43.192 17223 17223 E Tauri   : No Activity found to handle Intent { act=android.intent.action.PICK typ=application/* (has extras) }
```


```
02-13 15:32:51.552 28169 28169 I GoogleInputMethodService: GoogleInputMethodService.onStartInput():1568 onStartInput(EditorInfo{EditorInfo{packageName=com.read_anywhere.app, inputType=0, inputTypeString=NULL, enableLearning=false, autoCorrection=false, autoComplete=false, imeOptions=12000000, privateImeOptions=null, actionName=UNSPECIFIED, actionLabel=null, initialSelStart=-1, initialSelEnd=-1, initialCapsMode=0, label=null, fieldId=0, fieldName=null, extras=null, hintText=null, hintLocales=[]}}, false)
02-13 15:32:51.553 28169 28169 I Module  : DeviceLockedStatusModuleProvider$Module.updateDeviceLockedStatus():100 repeatCheckTimes = 1, locked = false
02-13 15:32:51.553  1722  7804 W PackageConfigPersister: App-specific configuration not found for packageName: com.read_anywhere.app and userId: 0
02-13 15:32:51.554 28169 28169 I AndroidIME: InputBundleManager.loadActiveInputBundleId():407 loadActiveInputBundleId: und-Latn-x-password, password
02-13 15:32:51.561 24928 24928 V Tauri/Plugin: Tauri plugin: pluginId: dialog, command: showFilePicker
02-13 15:32:51.633  1722  7804 I ActivityTaskManager: START u0 {act=android.intent.action.PICK typ=application/* (has extras)} with LAUNCH_MULTIPLE from uid 10050 result code=-91
02-13 15:32:51.633 24928 24928 E Tauri   : No Activity found to handle Intent { act=android.intent.action.PICK typ=application/* (has extras) }
```

```
02-13 15:23:56.124 24893 24893 I artd    : unable to open vdex file /data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/oat/arm64/base.vdex: File /data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/oat/arm64/base.vdex does not exist.
02-13 15:23:56.124 24893 24893 I artd    : OatFileAssistant test for existing oat file /data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/oat/arm64/base.vdex: File /data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/oat/arm64/base.vdex does not exist.
02-13 15:23:56.124 24893 24893 I artd    : GetBestInfo checking dm (/data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/base.dm)
02-13 15:23:56.124 24893 24893 W ziparchive: Unable to open '/data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/base.dm': No such file or directory
02-13 15:23:56.124 24893 24893 I artd    : OatFileAssistant test for existing oat file /data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/base.dm: I/O error
02-13 15:23:56.124 24893 24893 I artd    : GetBestInfo checking dm (/data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/base.dm)
02-13 15:23:56.124 24893 24893 W ziparchive: Unable to open '/data/app/~~tr69EDTHjlZuVt5O97jFPg==/com.read_anywhere.app-VyrnDC_OHtkJIHoU0CG6ig==/base.dm': No such file or director

...

02-13 15:23:56.256  1722  4735 I AppsFilter: interaction: PackageSetting{9a557df com.read_anywhere.app/10050} -> PackageSetting{a9a0795 com.google.android.overlay.permissioncontroller/10024} BLOCKED
02-13 15:23:56.255  1722  4735 I AppsFilter: interaction: PackageSetting{9a557df com.read_anywhere.app/10050} -> PackageSetting{b2d65f com.google.android.apps.nbu.files/10124} BLOCKED
```

```
02-13 17:39:13.965  5761  5761 I GoogleInputMethodService: GoogleInputMethodService.onFinishInput():2589 
02-13 17:39:13.966  5761  5761 I Module  : DeviceLockedStatusModuleProvider$Module.updateDeviceLockedStatus():100 repeatCheckTimes = 0, locked = false
02-13 17:39:13.966  5761  5761 I GoogleInputMethodService: GoogleInputMethodService.onStartInput():1568 onStartInput(EditorInfo{EditorInfo{packageName=com.read_anywhere.app, inputType=0, inputTypeString=NULL, enableLearning=false, autoCorrection=false, autoComplete=false, imeOptions=12000000, privateImeOptions=null, actionName=UNSPECIFIED, actionLabel=null, initialSelStart=-1, initialSelEnd=-1, initialCapsMode=0, label=null, fieldId=0, fieldName=null, extras=null, hintText=null, hintLocales=[]}}, false)
02-13 17:39:13.966  5761  5761 I Module  : DeviceLockedStatusModuleProvider$Module.updateDeviceLockedStatus():100 repeatCheckTimes = 1, locked = false
02-13 17:39:13.967  1722  1759 W PackageConfigPersister: App-specific configuration not found for packageName: com.read_anywhere.app and userId: 0
02-13 17:39:13.967  5761  5761 I AndroidIME: InputBundleManager.loadActiveInputBundleId():407 loadActiveInputBundleId: und-Latn-x-password, password
02-13 17:39:13.969 15059 15059 V Tauri/Plugin: Tauri plugin: pluginId: dialog, command: showFilePicker
02-13 17:39:13.972 15059 15071 I ad_anywhere.app: Background concurrent mark compact GC freed 10019KB AllocSpace bytes, 26(2248KB) LOS objects, 84% free, 4567KB/28MB, paused 189us,1.650ms total 113.834ms
02-13 17:39:13.981 15059 15073 W ad_anywhere.app: ApkAssets: Deleting an ApkAssets object '<empty> and /data/app/~~02vgaGieu_dJGP8ldaa57A==/com.google.android.webview--2CRQaFaUX48UJZyESyd0w==/base.apk' with 2 weak references
02-13 17:39:14.024  1722  1759 I ActivityTaskManager: START u0 {act=android.intent.action.PICK typ=application/* (has extras)} with LAUNCH_MULTIPLE from uid 10050 result code=-91
02-13 17:39:14.024 15059 15059 E Tauri   : No Activity found to handle Intent { act=android.intent.action.PICK typ=application/* (has extras) }
02-13 17:39:14.037  1722  1899 W ProcessStats: Tracking association SourceState{1f8d109 com.android.phone/1001 BFgs #529420} whose proc state 4 is better than process ProcessState{a1de233 com.google.android.euicc/10115 pkg=com.google.android.euicc} proc state 14 (16 skipped)
```

Adding this 

```
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE"/>
    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
```
to `gen/android/app/src/main/AndroidManifest.xml`


