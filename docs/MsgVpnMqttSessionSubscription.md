# MsgVpnMqttSessionSubscription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mqtt_session_client_id** | **String** | The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | [optional] [default to null]
**mqtt_session_virtual_router** | **String** | The Virtual Router of the MQTT Session. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The MQTT Session belongs to the primary Virtual Router. \&quot;backup\&quot; - The MQTT Session belongs to the backup Virtual Router. &lt;/pre&gt;  | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**subscription_qos** | **i64** | The quality of service (QoS) for the subscription as either 0 (deliver at most once) or 1 (deliver at least once). QoS 2 is not supported, but QoS 2 messages attracted by QoS 0 or QoS 1 subscriptions are accepted and delivered accordingly. The default value is &#x60;0&#x60;. | [optional] [default to null]
**subscription_topic** | **String** | The MQTT subscription topic. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


