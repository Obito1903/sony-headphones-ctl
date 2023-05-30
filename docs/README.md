# Sony Headphone protocol

Basic communication protocol : `RFCOMM`

## RFCOMM Payload

| Start marker - 1B | Data type - 1B       | Sequence number - 4B     | Payload size - 1B | Payload      | Checksum - 1B                                                                  | End marker - 1B |
| ----------------- | -------------------- | ------------------------ | ----------------- | ------------ | ------------------------------------------------------------------------------ | --------------- |
| Always `>`        | One of the following | Almost always `0` or `1` |                   | Escaped data | sum of data type, seq number, payload size and payload. wrapping around itself | Always `<`      |


### Data types

| Name         | Value  |
| ------------ | :----: |
| `Data`       | `0x00` |
| `Ack   `     | `0x01` |
| `DataMc1`    | `0x02` |
| `DataIcd`    | `0x09` |
| `DataEv`     | `0x0a` |
| `DataMdr`    | `0x0c` |
| `DataCommon` | `0x0d` |
| `DataMdr2`   | `0x0e` |
| `Shot`       | `0x10` |
| `ShotMc1`    | `0x12` |
| `ShotIcd`    | `0x19` |
| `ShotEv`     | `0x1a` |
| `ShotMdr`    | `0x1c` |
| `ShotCommon` | `0x1d` |
| `ShotMdr2`   | `0x1e` |
| `LargerData` | `0x2d` |

## Payloads for WF-1000XM4

### DataMdr

This data type is used to set or read the state of the headphone.

The first 2 bytes are used to specify the kind of state you want to query or change.
|           |              |
| --------- | ------------ |
| Code - 2B | Payload - NB |

#### Noise cancelling

Codes :
- `0x6815` : Write
- `0x6915` : Read

Payload :

| Continous mode - 1B                                           | NC enable - 1B | NC mode - 1B | NC Wind? - 1B         | NC voice passthrough - 1B | NC level - 1B        |
| ------------------------------------------------------------- | -------------- | ------------ | --------------------- | ------------------------- | -------------------- |
| Only  `0` when continously changing the NC level otherwie `1` | `0` or `1`     | `0` or `1`   | See [Wind](#####Wind) | `0` or `1`                | `0` to `20` (`0x0a`) |

##### Wind

- `0x02` : Disabled
- `0x03` : Enabled without voice passthrough
- `0x05` : Enabled with voice passthrough


#### Speak-To-Chat

Codes :

- `0xf802` : Write

Payload :

| Speak-To-Chat enable - 1B | ? - 1B     |
| ------------------------- | ---------- |
| `0` or `1`                | Always `1` |

#### DSEE Extreme 

Codes :
- `0xe801` : Write

Payload :

| DSEE Extreme enable - 1B |
| ------------------------ |
| `0` or `1`               |

#### Equalizer

Codes :

- `0x5800` : Write
- `0x5900` : Read

##### Set Equalizer


Payload :

| Command - 2B | Profile - 1B | Number of bands - 1B | Bass - 1B | 400 - 1B | 1k - 1B | 2.5k - 1B | 6.3k - 1B | 16k - 1B |
| ------------ | ------------ | -------------------- | --------- | -------- | ------- | --------- | --------- | -------- |
| `5800`       | `a1`         | `0x06`               |           |          |         |           |           |          |

Each band can go from -10db (`0x00`) to +10db (`0x14`), +0 being `0x04`

##### Set Profile

| Command - 2B | Profile - 1B | Number of bands - 1B |
| ------------ | ------------ | -------------------- |
| `0x5800`     |              | `0x00`               |

| Profile  | Code   |
| -------- | ------ |
| Off      | `0x00` |
| Custom 1 | `0xa1` |
| Custom 2 | `0xa2` |
| ...      |        |

Headset will respond with Ack then send back the current state of the equalizer with command `5900`

#### Automatic Power Off

| Command - 2B | Auto Off - 1B                | ?? - 1B |
| ------------ | ---------------------------- | ------- |
| `0x2805`     | `0x11` => Off , `0x10` => On | `0x00`  |


Headset will respond with Ack then send back the current state of this option with command `0x2905`

#### Pause when removed

| Command - 2B | Pause - 1B                  |
| ------------ | --------------------------- |
| `f801`       | `0x00` => On, `0x01` => Off |


Headset will respond with Ack then send back the current state of this option with command `0xf901`

#### Notification & Voice Guide

| Command - 2B | Notif - 1B                  |
| ------------ | --------------------------- |
| `4801`       | `0x00` => On, `0x01` => Off |


Headset will respond with Ack then send back the current state of this option with command `0x4901`