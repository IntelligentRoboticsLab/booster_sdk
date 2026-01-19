# Booster Robot DDS Communication Reference

Complete reference for real-time communication with the Booster K1 humanoid robot using Data Distribution Service (DDS).

**Protocol:** DDS (Data Distribution Service)
**Implementation:** FastDDS (eProsima)
**Transport:** UDPv4 with Multicast
**Data Format:** CDR (Common Data Representation)

---

## Table of Contents

- [DDS Configuration](#dds-configuration)
- [Topic Overview](#topic-overview)
- [Command Topics](#command-topics)
- [State Topics](#state-topics)
- [Message Schemas](#message-schemas)
- [Connection Guide](#connection-guide)
- [Implementation Examples](#implementation-examples)
- [Quality of Service](#quality-of-service)
- [Safety Guidelines](#safety-guidelines)

---

## DDS Configuration

### Domain Configuration

```yaml
Domain ID: 0
Transport: UDPv4
Discovery: Simple Discovery Protocol (multicast)
Multicast Address: 239.255.0.1
Discovery Port: 7400 (default)
User Data Ports: 7401-7500 (typical range)
```

### Network Requirements

- **Same Subnet:** All DDS participants must be on same network (192.168.10.x recommended)
- **Multicast Enabled:** Network must support UDP multicast
- **No Firewalls:** UDP ports 7400-7500 should be accessible
- **Low Latency:** Direct wired connection recommended for best performance

### DDS Implementation

The robot uses **FastDDS (eProsima)** implementation. Compatible DDS implementations:
- FastDDS (eProsima) - **Recommended**
- CycloneDDS (Eclipse)
- RTI Connext DDS
- OpenDDS

### Quality of Service (QoS)

Default QoS profiles:
```yaml
Reliability: BEST_EFFORT  (for most sensor data)
Durability: VOLATILE
History: KEEP_LAST (depth=1)
```

For commands:
```yaml
Reliability: RELIABLE
Durability: VOLATILE
History: KEEP_LAST (depth=10)
```

---

## Topic Overview

### Topic Naming Convention

All robot topics use the `rt/` prefix (robot topics).

### Topic Categories

| Category | Topics | Direction | Purpose |
|----------|--------|-----------|---------|
| **Commands** | `LocoApiTopicReq`, `LocoApiTopicResp` | Bidirectional | Motion control commands |
| **Sensor Data** | `rt/device_gateway` | Subscribe | IMU, motors, battery state |
| **Motion State** | `rt/motion_state` | Subscribe | Motion controller state |
| **Process Health** | `rt/booster_process_state` | Subscribe | System health monitoring |
| **User Input** | `rt/button_event`, `rt/remote_controller_state` | Subscribe | Physical input events |
| **Media** | `rt/booster/video_stream` | Subscribe | Camera feeds |
| **Control** | `rt/gripper_control`, `rt/light_control` | Publish | Device control |

---

## Command Topics

### LocoApiTopicReq (Commands)

**Direction:** Publish
**Message Type:** `RpcReqMsg`
**QoS:** Reliable, Keep Last (10)
**Purpose:** Send motion control commands to robot

#### Message Schema

```cpp
struct RpcReqMsg {
    string uuid;     // UUID v4 string
    string header;   // JSON: {"api_id": <int>}
    string body;     // JSON: command payload
};
```

#### Supported Commands (LocoApiId)

```cpp
enum LocoApiId {
    CHANGE_MODE = 2000,  // 0x7d0
    MOVE = 2001,         // 0x7d1
    // Additional commands discovered through reverse engineering
};
```

#### Example: Move Command

```json
{
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "header": "{\"api_id\":2001}",
    "body": "{\"vx\":0.5,\"vy\":0.0,\"vyaw\":0.0}"
}
```

**Body Fields for MOVE (2001):**
- `vx`: Forward/backward velocity in m/s (-1.0 to 1.0)
- `vy`: Left/right velocity in m/s (-1.0 to 1.0)
- `vyaw`: Rotation velocity in rad/s (-2.0 to 2.0)

#### Example: Change Mode Command

```json
{
    "uuid": "550e8400-e29b-41d4-a716-446655440001",
    "header": "{\"api_id\":2000}",
    "body": "{\"mode\":2}"
}
```

**Body Fields for CHANGE_MODE (2000):**
- `mode`: Robot mode (0=DAMPING, 1=PREPARE, 2=WALKING, 3=CUSTOM, 4=SOCCER)

### LocoApiTopicResp (Responses)

**Direction:** Subscribe
**Message Type:** `RpcRespMsg`
**QoS:** Reliable, Keep Last (10)
**Purpose:** Receive command acknowledgments

#### Message Schema

```cpp
struct RpcRespMsg {
    string uuid;         // Matching request UUID
    string header;       // JSON: response metadata
    string body;         // JSON: response data
    int32 status_code;   // 0=success, 100=timeout, -1=pending
};
```

#### Status Codes

- `0`: Success
- `100`: Timeout
- `-1`: Pending (internal state, not final)
- Non-zero: Error codes (varies by command)

---

## State Topics

### rt/device_gateway (Primary Sensor Data)

**Direction:** Subscribe
**Message Type:** `RobotStatusDdsMsg`
**Frequency:** ~100 Hz
**QoS:** Best Effort, Keep Last (1)
**Purpose:** Real-time sensor data (IMU, motors, battery)

#### Message Schema

```cpp
struct RobotDdsJointStatus {
    string name;                    // Joint/motor name (e.g., "left_hip_pitch")
    int32 index;                    // Joint index (0-based)
    bool is_connected;              // Motor connection status
    int32 temperature;              // Temperature in Celsius
    bool is_limited;                // True if at mechanical limit
    int32 status_code;              // 0=OK, non-zero=error
    int32 temperature_level;        // 0=NORMAL, 1=WARNING, 2=FATAL
};

struct RobotDdsImuStatus {
    string name;                    // IMU name (e.g., "body_imu")
    int32 index;                    // IMU index
    bool is_connected;              // Connection status
    int32 status_code;              // 0=OK, non-zero=error
};

struct RobotDdsBatteryStatus {
    string name;                    // Battery name
    int32 index;                    // Battery index
    float soc;                      // State of charge (0.0-100.0%)
    int32 status_code;              // Status code
    int32 soc_level;                // 0=NORMAL, 1=WARNING, 2=FATAL
};

struct RobotStatusDdsMsg {
    sequence<RobotDdsJointStatus> joint_vec;
    sequence<RobotDdsImuStatus> imu_vec;
    sequence<RobotDdsBatteryStatus> battery_vec;
};
```

#### Common Joint Names

**Lower Body:**
- `left_hip_pitch`, `left_hip_roll`, `left_hip_yaw`
- `left_knee`, `left_ankle_pitch`, `left_ankle_roll`
- `right_hip_pitch`, `right_hip_roll`, `right_hip_yaw`
- `right_knee`, `right_ankle_pitch`, `right_ankle_roll`

**Upper Body:**
- `torso_yaw`, `torso_pitch`
- `left_shoulder_pitch`, `left_shoulder_roll`, `left_shoulder_yaw`
- `left_elbow`, `left_wrist_pitch`, `left_wrist_roll`
- `right_shoulder_pitch`, `right_shoulder_roll`, `right_shoulder_yaw`
- `right_elbow`, `right_wrist_pitch`, `right_wrist_roll`

**Head:**
- `head_pitch`, `head_yaw`

### rt/motion_state (Motion Controller State)

**Direction:** Subscribe
**Message Type:** `MotionState`
**Frequency:** ~100 Hz
**QoS:** Best Effort, Keep Last (1)
**Purpose:** Current motion mode and transitions

#### Message Schema

```cpp
struct MotionState {
    int32 current_mode;      // Current RobotMode
    int32 target_mode;       // Target RobotMode (during transition)
    bool is_transitioning;   // True if mode change in progress
};

enum RobotMode {
    DAMPING = 0,     // Motors relaxed
    PREPARE = 1,     // Standing preparation
    WALKING = 2,     // Walking mode
    CUSTOM = 3,      // Custom control
    SOCCER = 4,      // Soccer mode
};
```

### rt/battery_state (Battery Details)

**Direction:** Subscribe
**Message Type:** `BatteryState`
**Frequency:** ~1 Hz
**QoS:** Reliable, Keep Last (1)
**Purpose:** Detailed battery information

#### Message Schema (Inferred)

```cpp
struct BatteryState {
    float voltage;           // Battery voltage (V)
    float current;           // Battery current (A)
    float temperature;       // Battery temperature (°C)
    float soc;               // State of charge (0.0-100.0%)
    int32 health;            // Battery health percentage
    int32 status_code;       // Status code
};
```

### rt/button_event (Physical Buttons)

**Direction:** Subscribe
**Message Type:** `ButtonEventMsg`
**Frequency:** Event-driven
**QoS:** Reliable, Keep All
**Purpose:** Physical button presses on robot

#### Message Schema

```cpp
struct ButtonEventMsg {
    uint8 event_type;        // 0=PRESSED, 1=RELEASED, 2=LONG_PRESS
    uint32 button_id;        // Button identifier
    int64 timestamp;         // Unix timestamp (ms)
    string data;             // Additional data
};
```

### rt/remote_controller_state (Remote Controller)

**Direction:** Subscribe
**Message Type:** `RemoteControllerState`
**Frequency:** ~50 Hz
**QoS:** Best Effort, Keep Last (1)
**Purpose:** Remote controller input

#### Message Schema

```cpp
struct RemoteControllerState {
    uint32 event;            // SDL event type

    // Analog sticks (-1.0 to 1.0)
    float lx;                // Left stick X (left=-1, right=1)
    float ly;                // Left stick Y (forward=-1, back=1)
    float rx;                // Right stick X
    float ry;                // Right stick Y

    // Face buttons
    bool a;
    bool b;
    bool x;
    bool y;

    // Shoulder buttons
    bool lb;                 // Left bumper
    bool rb;                 // Right bumper
    bool lt;                 // Left trigger
    bool rt;                 // Right trigger

    // Stick clicks
    bool ls;                 // Left stick click
    bool rs;                 // Right stick click

    // System buttons
    bool back;
    bool start;

    // D-pad (hat)
    bool hat_c;              // Centered
    bool hat_u;              // Up
    bool hat_d;              // Down
    bool hat_l;              // Left
    bool hat_r;              // Right
    bool hat_lu;             // Left-up
    bool hat_ld;             // Left-down
    bool hat_ru;             // Right-up
    bool hat_rd;             // Right-down
    uint8 hat_pos;           // Hat position value
};
```

### rt/booster_process_state (Process Health)

**Direction:** Subscribe
**Message Type:** `RobotProcessStateMsg`
**Frequency:** ~1 Hz
**QoS:** Reliable, Keep Last (1)
**Purpose:** Monitor system process health

#### Message Schema

```cpp
struct RobotProcessStatus {
    string name;             // Process name
    int32 index;             // Process index
    int32 pid;               // Process ID
    int32 status;            // 0=RUNNING, 1=NOT_RUNNING
    int32 status_level;      // 0=NORMAL, 1=WARNING, 2=FATAL
    bool can_restart;        // True if can be restarted
};

struct RobotProcessStateMsg {
    sequence<RobotProcessStatus> process_vec;
};
```

**Common Process Names:**
- `device-gateway` - Hardware interface
- `booster-motion` - Motion control daemon

### rt/booster/video_stream (Camera Feed)

**Direction:** Subscribe
**Message Type:** `BinaryData`
**Frequency:** ~30 Hz
**QoS:** Best Effort, Keep Last (1)
**Purpose:** Video stream from cameras

#### Message Schema (Inferred)

```cpp
struct BinaryData {
    sequence<uint8> data;    // Raw binary data (H.264, JPEG, etc.)
    int64 timestamp;         // Timestamp in microseconds
    uint32 sequence_num;     // Sequence number
    string encoding;         // Encoding type
};
```

---

## Control Topics (Publishing)

### rt/gripper_control (Gripper Commands)

**Direction:** Publish
**Message Type:** `GripperControl`
**QoS:** Reliable, Keep Last (10)
**Purpose:** Control gripper position/force

#### Message Schema (Inferred)

```cpp
struct GripperControl {
    uint8 hand_index;        // 0=LEFT, 1=RIGHT
    int32 position;          // 0-1000 (0=open, 1000=closed)
    int32 force;             // Force in arbitrary units
    int32 speed;             // Speed in arbitrary units
};
```

### rt/light_control (LED Control)

**Direction:** Publish
**Message Type:** `LightControlMsg`
**QoS:** Reliable, Keep Last (10)
**Purpose:** Control LED lights on robot

### rt/enter_safe_mode (Emergency Stop)

**Direction:** Publish
**Message Type:** `SafeMode`
**QoS:** Reliable, Keep All
**Purpose:** Trigger emergency safe mode

---

## Connection Guide

### Python Implementation (CycloneDDS)

#### Step 1: Install Dependencies

```bash
pip install cyclonedds
```

#### Step 2: Define Message Types

Create Python classes matching IDL schemas (or compile from IDL files).

```python
from dataclasses import dataclass
from typing import List

@dataclass
class RobotDdsJointStatus:
    name: str
    index: int
    is_connected: bool
    temperature: int
    is_limited: bool
    status_code: int
    temperature_level: int

@dataclass
class RobotDdsImuStatus:
    name: str
    index: int
    is_connected: bool
    status_code: int

@dataclass
class RobotDdsBatteryStatus:
    name: str
    index: int
    soc: float
    status_code: int
    soc_level: int

@dataclass
class RobotStatusDdsMsg:
    joint_vec: List[RobotDdsJointStatus]
    imu_vec: List[RobotDdsImuStatus]
    battery_vec: List[RobotDdsBatteryStatus]

@dataclass
class RpcReqMsg:
    uuid: str
    header: str
    body: str
```

#### Step 3: Subscribe to State Data

```python
from cyclonedds.domain import DomainParticipant
from cyclonedds.topic import Topic
from cyclonedds.sub import DataReader
from cyclonedds.qos import Qos, Policy

# Create participant (domain 0)
participant = DomainParticipant(0)

# Create topic
topic = Topic(participant, "rt/device_gateway", RobotStatusDdsMsg)

# Create reader with default QoS
reader = DataReader(participant, topic)

# Read data in loop
while True:
    samples = reader.take(10)  # Take up to 10 samples

    for sample in samples:
        if sample.data:
            msg = sample.data

            # Access motor data
            for joint in msg.joint_vec:
                print(f"Motor {joint.name}: {joint.temperature}°C")

            # Access IMU data
            for imu in msg.imu_vec:
                print(f"IMU {imu.name}: connected={imu.is_connected}")

            # Access battery
            for bat in msg.battery_vec:
                print(f"Battery: {bat.soc}%")
```

#### Step 4: Publish Commands

```python
from cyclonedds.pub import DataWriter
import json
import uuid

# Create topic for commands
cmd_topic = Topic(participant, "LocoApiTopicReq", RpcReqMsg)

# Create writer
writer = DataWriter(participant, cmd_topic)

# Send move command
def send_move(vx, vy, vyaw):
    msg = RpcReqMsg(
        uuid=str(uuid.uuid4()),
        header=json.dumps({"api_id": 2001}),  # MOVE
        body=json.dumps({"vx": vx, "vy": vy, "vyaw": vyaw})
    )

    writer.write(msg)
    print(f"Sent move: vx={vx}, vy={vy}, vyaw={vyaw}")

# Use it
send_move(0.5, 0.0, 0.0)  # Move forward
```

---

### Rust Implementation (CycloneDDS-rs)

#### Step 1: Add Dependencies

```toml
[dependencies]
cyclonedds-rs = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1", features = ["v4"] }
```

#### Step 2: Define Message Types

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RobotDdsJointStatus {
    name: String,
    index: i32,
    is_connected: bool,
    temperature: i32,
    is_limited: bool,
    status_code: i32,
    temperature_level: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RobotDdsImuStatus {
    name: String,
    index: i32,
    is_connected: bool,
    status_code: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RobotDdsBatteryStatus {
    name: String,
    index: i32,
    soc: f32,
    status_code: i32,
    soc_level: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RobotStatusDdsMsg {
    joint_vec: Vec<RobotDdsJointStatus>,
    imu_vec: Vec<RobotDdsImuStatus>,
    battery_vec: Vec<RobotDdsBatteryStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RpcReqMsg {
    uuid: String,
    header: String,
    body: String,
}
```

#### Step 3: Subscribe to State

```rust
use cyclonedds_rs::*;

fn main() {
    // Create participant
    let participant = DomainParticipant::new(0).unwrap();

    // Create topic
    let topic = Topic::<RobotStatusDdsMsg>::new(
        &participant,
        "rt/device_gateway"
    ).unwrap();

    // Create reader
    let reader = DataReader::new(&participant, &topic).unwrap();

    // Read loop
    loop {
        let samples = reader.take::<RobotStatusDdsMsg>(10).unwrap();

        for sample in samples {
            if let Some(msg) = sample.data {
                // Process motor data
                for joint in &msg.joint_vec {
                    println!("Motor {}: {}°C", joint.name, joint.temperature);
                }

                // Process IMU data
                for imu in &msg.imu_vec {
                    println!("IMU {}: connected={}", imu.name, imu.is_connected);
                }

                // Process battery
                for bat in &msg.battery_vec {
                    println!("Battery: {}%", bat.soc);
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
```

#### Step 4: Publish Commands

```rust
use uuid::Uuid;

fn send_move_command(writer: &DataWriter<RpcReqMsg>, vx: f32, vy: f32, vyaw: f32) {
    let msg = RpcReqMsg {
        uuid: Uuid::new_v4().to_string(),
        header: serde_json::json!({"api_id": 2001}).to_string(),
        body: serde_json::json!({
            "vx": vx,
            "vy": vy,
            "vyaw": vyaw
        }).to_string(),
    };

    writer.write(&msg).unwrap();
    println!("Sent move: vx={}, vy={}, vyaw={}", vx, vy, vyaw);
}
```

---

### C++ Implementation (FastDDS)

#### Message Definition (IDL)

Create `RobotMessages.idl`:

```idl
module booster {
    module msg {
        struct RobotDdsJointStatus {
            string name;
            long index;
            boolean is_connected;
            long temperature;
            boolean is_limited;
            long status_code;
            long temperature_level;
        };

        struct RobotDdsImuStatus {
            string name;
            long index;
            boolean is_connected;
            long status_code;
        };

        struct RobotDdsBatteryStatus {
            string name;
            long index;
            float soc;
            long status_code;
            long soc_level;
        };

        struct RobotStatusDdsMsg {
            sequence<RobotDdsJointStatus> joint_vec;
            sequence<RobotDdsImuStatus> imu_vec;
            sequence<RobotDdsBatteryStatus> battery_vec;
        };

        struct RpcReqMsg {
            string uuid;
            string header;
            string body;
        };
    };
};
```

#### Compile IDL

```bash
fastddsgen RobotMessages.idl
```

#### Subscribe Example

```cpp
#include <fastdds/dds/domain/DomainParticipantFactory.hpp>
#include <fastdds/dds/domain/DomainParticipant.hpp>
#include <fastdds/dds/subscriber/Subscriber.hpp>
#include <fastdds/dds/subscriber/DataReader.hpp>
#include "RobotMessages.h"

using namespace eprosima::fastdds::dds;
using namespace booster::msg;

class StateListener : public DataReaderListener {
public:
    void on_data_available(DataReader* reader) override {
        RobotStatusDdsMsg msg;
        SampleInfo info;

        if (reader->take_next_sample(&msg, &info) == ReturnCode_t::RETCODE_OK) {
            if (info.valid_data) {
                // Process motor data
                for (const auto& joint : msg.joint_vec()) {
                    std::cout << "Motor " << joint.name()
                             << ": " << joint.temperature() << "°C\n";
                }

                // Process IMU data
                for (const auto& imu : msg.imu_vec()) {
                    std::cout << "IMU " << imu.name()
                             << ": connected=" << imu.is_connected() << "\n";
                }

                // Process battery
                for (const auto& bat : msg.battery_vec()) {
                    std::cout << "Battery: " << bat.soc() << "%\n";
                }
            }
        }
    }
};

int main() {
    // Create participant
    DomainParticipant* participant =
        DomainParticipantFactory::get_instance()->create_participant(
            0, PARTICIPANT_QOS_DEFAULT);

    // Create subscriber
    Subscriber* subscriber = participant->create_subscriber(SUBSCRIBER_QOS_DEFAULT);

    // Create topic
    TypeSupport type(new RobotStatusDdsMsgPubSubType());
    participant->register_type(type);

    Topic* topic = participant->create_topic(
        "rt/device_gateway",
        type.get_type_name(),
        TOPIC_QOS_DEFAULT);

    // Create reader with listener
    StateListener listener;
    DataReader* reader = subscriber->create_datareader(
        topic,
        DATAREADER_QOS_DEFAULT,
        &listener);

    // Wait for data (listener handles callbacks)
    std::this_thread::sleep_for(std::chrono::seconds(60));

    return 0;
}
```

#### Publish Example

```cpp
#include <fastdds/dds/publisher/Publisher.hpp>
#include <fastdds/dds/publisher/DataWriter.hpp>
#include <nlohmann/json.hpp>

void send_move_command(DataWriter* writer, float vx, float vy, float vyaw) {
    RpcReqMsg msg;

    // Generate UUID (simplified)
    msg.uuid("550e8400-e29b-41d4-a716-446655440000");

    // Set header
    nlohmann::json header;
    header["api_id"] = 2001;  // MOVE
    msg.header(header.dump());

    // Set body
    nlohmann::json body;
    body["vx"] = vx;
    body["vy"] = vy;
    body["vyaw"] = vyaw;
    msg.body(body.dump());

    // Publish
    writer->write(&msg);
}
```

---

## Quality of Service (QoS) Configuration

### Recommended QoS Profiles

#### For Sensor Data (Subscribing)

```python
from cyclonedds.qos import Qos, Policy

sensor_qos = Qos(
    Policy.Reliability.BestEffort(),
    Policy.History.KeepLast(1),
    Policy.Durability.Volatile()
)

reader = DataReader(participant, topic, qos=sensor_qos)
```

#### For Commands (Publishing)

```python
command_qos = Qos(
    Policy.Reliability.Reliable(),
    Policy.History.KeepLast(10),
    Policy.Durability.Volatile()
)

writer = DataWriter(participant, topic, qos=command_qos)
```

#### For Critical Events

```python
event_qos = Qos(
    Policy.Reliability.Reliable(),
    Policy.History.KeepAll(),
    Policy.Durability.TransientLocal()
)
```

---

## Safety Guidelines

### Critical Safety Measures

1. **Monitor sensor data continuously** - Subscribe to `rt/device_gateway`
2. **Check battery level** - Stop if SoC < 15%
3. **Monitor motor temperatures** - Emergency stop if > 85°C
4. **Implement watchdog** - Detect communication loss
5. **Emergency stop ready** - Can immediately switch to DAMPING mode

### Safe Startup Sequence

```python
import time

def safe_startup(cmd_writer):
    # 1. Wait for sensor data
    print("Waiting for sensor data...")
    wait_for_topic("rt/device_gateway", timeout=5.0)

    # 2. Check battery
    battery_ok = check_battery_level(min_soc=20.0)
    if not battery_ok:
        raise Exception("Battery too low")

    # 3. Send PREPARE mode
    send_mode_change(cmd_writer, mode=1)  # PREPARE
    time.sleep(3)

    # 4. Send WALKING mode
    send_mode_change(cmd_writer, mode=2)  # WALKING
    time.sleep(1)

    print("Robot ready")
```

### Emergency Stop

```python
def emergency_stop(cmd_writer):
    """Immediately stop robot"""
    msg = RpcReqMsg(
        uuid=str(uuid.uuid4()),
        header=json.dumps({"api_id": 2000}),  # CHANGE_MODE
        body=json.dumps({"mode": 0})  # DAMPING
    )
    cmd_writer.write(msg)
    print("EMERGENCY STOP ACTIVATED")
```

### Watchdog Implementation

```python
import threading
import time

class RobotWatchdog:
    def __init__(self, timeout=1.0):
        self.timeout = timeout
        self.last_data_time = time.time()
        self.running = True
        self.thread = threading.Thread(target=self._watch)
        self.thread.start()

    def feed(self):
        """Call when data received"""
        self.last_data_time = time.time()

    def _watch(self):
        while self.running:
            if time.time() - self.last_data_time > self.timeout:
                print("WARNING: No data received for {} seconds!".format(self.timeout))
                # Trigger emergency stop
                emergency_stop(cmd_writer)
                break
            time.sleep(0.1)

# Usage
watchdog = RobotWatchdog(timeout=2.0)

# In data callback
def on_data_received(sample):
    watchdog.feed()
    # Process data...
```

---

## Complete Topic Reference

| Topic Name | Type | Direction | Frequency | QoS | Purpose |
|------------|------|-----------|-----------|-----|---------|
| `LocoApiTopicReq` | `RpcReqMsg` | Publish | On-demand | Reliable | Send commands |
| `LocoApiTopicResp` | `RpcRespMsg` | Subscribe | Response | Reliable | Command responses |
| `rt/device_gateway` | `RobotStatusDdsMsg` | Subscribe | ~100 Hz | Best Effort | Motor/IMU/Battery |
| `rt/motion_state` | `MotionState` | Subscribe | ~100 Hz | Best Effort | Motion mode |
| `rt/battery_state` | `BatteryState` | Subscribe | ~1 Hz | Reliable | Battery details |
| `rt/button_event` | `ButtonEventMsg` | Subscribe | Event | Reliable | Button presses |
| `rt/remote_controller_state` | `RemoteControllerState` | Subscribe | ~50 Hz | Best Effort | Controller input |
| `rt/booster_process_state` | `RobotProcessStateMsg` | Subscribe | ~1 Hz | Reliable | Process health |
| `rt/booster/video_stream` | `BinaryData` | Subscribe | ~30 Hz | Best Effort | Video feed |
| `rt/gripper_control` | `GripperControl` | Publish | On-demand | Reliable | Gripper commands |
| `rt/light_control` | `LightControlMsg` | Publish | On-demand | Reliable | LED control |
| `rt/enter_safe_mode` | `SafeMode` | Publish | Event | Reliable | Emergency stop |

---

## Troubleshooting

### Discovery Issues

**Symptoms:** No topics discovered, no data received

**Solutions:**
1. Verify domain ID is 0
2. Check multicast is enabled on network
3. Ensure no firewall blocking UDP
4. Try disabling VPN
5. Use `dds spy` tool to debug discovery

### Data Not Received

**Symptoms:** Topic discovered but no samples

**Solutions:**
1. Check QoS compatibility (must match publisher)
2. Verify data type definition matches exactly
3. Check network bandwidth
4. Monitor DDS logs for errors

### High Latency

**Symptoms:** Delayed data reception

**Solutions:**
1. Use wired connection instead of WiFi
2. Use BEST_EFFORT reliability for real-time data
3. Reduce History depth (KeepLast 1)
4. Check for network congestion

---

## Best Practices

1. **Use domain 0** - Always
2. **Subscribe before publishing** - Ensure reader ready
3. **Handle missing data** - Best effort may drop samples
4. **UUID for commands** - Always use UUID v4
5. **JSON in strings** - Header and body are JSON strings, not objects
6. **Monitor health** - Subscribe to process state and device gateway
7. **Implement timeouts** - Detect communication loss
8. **Emergency stop ready** - Quick mode change to DAMPING
9. **Test discovery** - Verify topics found before sending commands
10. **Match QoS** - Reader/writer QoS must be compatible

---

**Document Version:** 1.0
**Last Updated:** 2026-01-19
**Protocol:** DDS (Data Distribution Service)
**Implementation:** FastDDS compatible
**Domain:** 0
