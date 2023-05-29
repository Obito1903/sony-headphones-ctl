# Sony Headphone Ctl

A small cli tool to control the Sony WF-1000XM4 (Earbuds) 

## Features

### Report

- [ ] Battery
- [ ] Device Info
- [ ] Registered Devices


### Config

- [X] Ambient sound control
- [ ] Equalizer
- [ ] Connection Quality
- [ ] DSEE Extreme
- [ ] Speak-To-Chat
- [ ] Automatic Power Off
- [ ] Change Touch sensor function
- [ ] On device ASC settings
- [ ] BT Multipoint

## Usage


```bash
sony-headphone-ctl --help
```

Set Noise Canceling to wind noise reduction

```bash
sony-headphone-ctl config anc nc --wind
```

Set Noise Canceling to Ambient sound with level of 5 and voice passthrough

```bash
sony-headphone-ctl config anc ambient --level 5 --voice
```

## Protocol Documentation

[docs/README.md](docs/README.md)

## Disclaimer

WIP tool and library, don't excpect it to work all the time.

I am not responsible for any damage done to your headset, use at your own risk.

## Thanks

- [Plutoberth](https://github.com/Plutoberth/SonyHeadphonesClient) - For the first step in understanding the protocol