//! Tests for ports responses

use tilt_sdk_cloudengine::client::responses::PortWrapper;
use tilt_sdk_cloudengine::models::Ports;

#[test]
fn test_port_state_deserialization() {
    let json = r#"{
        "item_id": "00000000-0000-0000-0000-000000000001",
        "state": "on",
        "data": {
            "config": {
                "name": "00000000-0000-0000-0000-000000000001",
                "fixed_ips": [{"ip_address": "10.0.0.1", "id": "00000000-0000-0000-0000-000000000004", "subnet_id": "00000000-0000-0000-0000-000000000005"}],
                "subnet": {"id": "00000000-0000-0000-0000-000000000005", "name": "ru-central2"},
                "network": {"id": "00000000-0000-0000-0000-000000000006", "name": "srazmi"},
                "security_groups": []
            },
            "parent": "00000000-0000-0000-0000-000000000003"
        },
        "parent": "00000000-0000-0000-0000-000000000003"
    }"#;

    let port: PortWrapper = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(port.state, "on");
    assert_eq!(port.item_id, "00000000-0000-0000-0000-000000000001");
    assert!(port.data.parent.is_some());
}

#[test]
fn test_port_detached_state_in_data() {
    let json = r#"{
        "item_id": "00000000-0000-0000-0000-000000000002",
        "parent": null,
        "data": {
            "config": {
                "name": "00000000-0000-0000-0000-000000000002",
                "fixed_ips": [{"ip_address": "10.0.0.2"}],
                "security_groups": []
            },
            "state": "on",
            "parent": null
        },
        "state": "on"
    }"#;

    let port: PortWrapper = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(port.state, "on");
    assert_eq!(port.data.state, "on");
    assert!(port.data.parent.is_none());
}

#[test]
fn test_port_wrapper_to_ports_model() {
    let json = r#"{
        "item_id": "00000000-0000-0000-0000-000000000001",
        "state": "on",
        "data": {
            "config": {
                "name": "test-port",
                "mac_address": "02:00:00:00:00:01",
                "fixed_ips": [{"ip_address": "10.0.0.1"}],
                "subnet": {"id": "00000000-0000-0000-0000-000000000005", "name": "ru-central2"},
                "network": {"id": "00000000-0000-0000-0000-000000000006", "name": "srazmi"},
                "security_groups": [
                    {"id": "00000000-0000-0000-0000-000000000007", "name": "sg1"},
                    {"id": "00000000-0000-0000-0000-000000000008", "name": "default"}
                ]
            }
        },
        "parent": "00000000-0000-0000-0000-000000000003"
    }"#;

    let wrapper: PortWrapper = serde_json::from_str(json).expect("Failed to deserialize");
    let port: Ports = wrapper.into();

    assert_eq!(port.state, "on");
    assert_eq!(port.name, "test-port");
    assert_eq!(port.mac_address, "02:00:00:00:00:01");
    assert_eq!(port.ip_address, Some("10.0.0.1".to_string()));
    assert_eq!(port.network_name, "srazmi");
    assert_eq!(port.security_group_names.len(), 2);
}

#[test]
fn test_port_wrapper_to_ports_model_detached() {
    let json = r#"{
        "item_id": "00000000-0000-0000-0000-000000000002",
        "data": {
            "config": {
                "name": "detached-port",
                "mac_address": "02:00:00:00:00:02",
                "fixed_ips": [{"ip_address": "10.0.0.2"}],
                "security_groups": []
            },
            "state": "on"
        },
        "parent": null
    }"#;

    let wrapper: PortWrapper = serde_json::from_str(json).expect("Failed to deserialize");
    let port: Ports = wrapper.into();

    assert_eq!(port.state, "on");
    assert_eq!(port.name, "detached-port");
    assert!(port.instance_id.is_none());
    assert!(port.instance_name.is_none());
}

#[test]
fn test_parse_single_port_with_parent_item() {
    let json = r#"{
        "item_id": "00000000-0000-0000-0000-000000000001",
        "type": "nic",
        "parent": "00000000-0000-0000-0000-000000000003",
        "created_row_dt": "2026-01-13T19:33:45.133236",
        "data": {
            "config": {
                "name": "test-port",
                "fixed_ips": [{"ip_address": "10.0.0.1"}],
                "security_groups": []
            },
            "parent_item": {
                "item_id": "00000000-0000-0000-0000-000000000003",
                "type": "instance",
                "data": {
                    "state": "on",
                    "config": {
                        "id": "00000000-0000-0000-0000-000000000003",
                        "name": "test-instance",
                        "flavor": {"id": "00000000-0000-0000-0000-000000000009", "name": "b5.large.2"},
                        "addresses": {"network": [{"addr": "10.0.0.1", "version": 4}]},
                        "source_image": {"id": "00000000-0000-0000-0000-00000000000a", "name": "ubuntu_22_04"},
                        "availability_zone": {"id": "az-001", "name": "ru-central2-a"}
                    }
                },
                "state": "on"
            },
            "state": "on"
        },
        "state": "on"
    }"#;

    let port: PortWrapper = serde_json::from_str(json).expect("Failed to deserialize");
    println!("Parsed port: {:?}", port);
    println!("type_field: '{}'", port.type_field);
    println!("parent_item: {:?}", port.data.parent_item);

    assert_eq!(port.type_field, "nic");
    assert!(port.data.parent_item.is_some());
    if let Some(ref parent) = port.data.parent_item {
        println!("parent.type_field: '{}'", parent.type_field);
        assert_eq!(parent.type_field, "instance");
    }
}
