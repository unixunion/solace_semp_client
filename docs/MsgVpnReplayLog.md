# MsgVpnReplayLog

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**egress_enabled** | **bool** | Enable or disable the egress flow of messages from the Replay Log. The default value is &#x60;false&#x60;. | [optional] [default to null]
**ingress_enabled** | **bool** | Enable or disable the ingress flow of messages to the Replay Log. The default value is &#x60;false&#x60;. | [optional] [default to null]
**max_spool_usage** | **i64** | The maximum spool usage in megabytes (MB) allowed by the Replay Log. If this limit is exceeded, old messages will be trimmed. The default value is &#x60;0&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**replay_log_name** | **String** | The name of the Replay Log. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


