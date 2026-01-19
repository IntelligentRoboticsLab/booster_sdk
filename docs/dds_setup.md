## DDS Setup

The Rust SDK communicates directly over DDS using RustDDS. The Booster robot uses
FastDDS, so you only need to ensure the DDS network settings match.

### Network Requirements

- **Same Subnet:** DDS participants should be on the same network (192.168.10.x recommended).
- **Multicast Enabled:** UDP multicast must be allowed.
- **Open Ports:** UDP 7400-7500 should be accessible.
- **Low Latency:** Wired connection recommended.

### Domain ID

The default DDS domain ID is `0`. You can override the domain ID when creating a
client or by setting `BOOSTER_DOMAIN_ID` in examples.

### Troubleshooting

- If no data appears, confirm multicast is enabled on the interface.
- Ensure firewalls allow UDP traffic on the DDS port range.
- Verify the robot and your machine share the same domain ID.
