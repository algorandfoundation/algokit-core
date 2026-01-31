# algokit_transact

## Building

```sh
cargo pkg transact kt
```

## Testing

### Host

```sh
./gradlew test
```

### Android

First you need a connected android device. You can either use a physical device or start an emulator.

To see available emulators:

```sh
~/Library/Android/sdk/emulator/emulator -list-avds
```

Then start the emulator:

```sh
~/Library/Android/sdk/emulator/emulator -avd <emulator_name>
```

Or to start the first one:

```sh
~/Library/Android/sdk/emulator/emulator -avd `~/Library/Android/sdk/emulator/emulator -list-avds | head -n 1`
```

Then run the tests:

```sh
./gradlew connectedAndroidTest
```
