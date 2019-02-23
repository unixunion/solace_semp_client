# MsgVpnMqttSession

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | Enable or disable the MQTT Session. When disabled, the client is disconnected, new messages matching QoS 0 subscriptions are discarded, and new messages matching QoS 1 subscriptions are stored for future delivery. The default value is &#x60;false&#x60;. | [optional] [default to null]
**mqtt_session_client_id** | **String** | The Client ID of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | [optional] [default to null]
**mqtt_session_virtual_router** | **String** | The Virtual Router of the MQTT Session. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The MQTT Session belongs to the primary Virtual Router. \&quot;backup\&quot; - The MQTT Session belongs to the backup Virtual Router. &lt;/pre&gt;  | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**owner** | **String** | The owner of the MQTT Session. For externally-created sessions this defaults to the Client Username of the connecting client. For management-created sessions this defaults to empty. Before configuring, the MQTT Session must be disabled. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


