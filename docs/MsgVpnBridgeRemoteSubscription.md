# MsgVpnBridgeRemoteSubscription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bridge_name** | **String** | The name of the Bridge. | [optional] [default to null]
**bridge_virtual_router** | **String** | Specify whether the Bridge is configured for the primary or backup Virtual Router or auto configured. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The Bridge is used for the primary Virtual Router. \&quot;backup\&quot; - The Bridge is used for the backup Virtual Router. \&quot;auto\&quot; - The Bridge is automatically assigned a Router. &lt;/pre&gt;  | [optional] [default to null]
**deliver_always_enabled** | **bool** | Flag the Subscription Topic as deliver always instead of with the deliver-to-one remote priority value for the Bridge given by \&quot;remoteDeliverToOnePriority\&quot;. A given topic may be deliver-to-one or deliver always but not both. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**remote_subscription_topic** | **String** | The Topic of the Remote Subscription. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


