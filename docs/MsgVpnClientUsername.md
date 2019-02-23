# MsgVpnClientUsername

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acl_profile_name** | **String** | The ACL Profile of the Client Username. The default value is &#x60;\&quot;default\&quot;&#x60;. | [optional] [default to null]
**client_profile_name** | **String** | The Client Profile of the Client Username. The default value is &#x60;\&quot;default\&quot;&#x60;. | [optional] [default to null]
**client_username** | **String** | The value of the Client Username. | [optional] [default to null]
**enabled** | **bool** | Enables or disables the Client Username. When disabled all clients currently connected as the Client Username are disconnected. The default value is &#x60;false&#x60;. | [optional] [default to null]
**guaranteed_endpoint_permission_override_enabled** | **bool** | Enables or disables guaranteed endpoint permission override for the Client Username. When enabled all guaranteed endpoints may be accessed, modified or deleted with the same permission as the owner. The default value is &#x60;false&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**password** | **String** | The password of this Client Username for internal Authentication. The default is to have no password. The default is to have no &#x60;password&#x60;. | [optional] [default to null]
**subscription_manager_enabled** | **bool** | Enables or disables the subscription management capability of the Client Username. This is the ability to manage subscriptions on behalf of other Client Usernames. The default value is &#x60;false&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


