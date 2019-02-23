# MsgVpnBridgeRemoteMsgVpn

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bridge_name** | **String** | The name of the Bridge. | [optional] [default to null]
**bridge_virtual_router** | **String** | Specify whether the Bridge is configured for the primary or backup Virtual Router or auto configured. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The Bridge is used for the primary Virtual Router. \&quot;backup\&quot; - The Bridge is used for the backup Virtual Router. \&quot;auto\&quot; - The Bridge is automatically assigned a Router. &lt;/pre&gt;  | [optional] [default to null]
**client_username** | **String** | The Client Username the Bridge uses to login to the Remote Message VPN. This per Remote Message VPN value overrides the value provided for the bridge overall. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**compressed_data_enabled** | **bool** | Enable or disable data compression for the Remote Message VPN. The default value is &#x60;false&#x60;. | [optional] [default to null]
**connect_order** | **i32** | The order in which attempts to connect to different Message VPN hosts are attempted, or the preference given to incoming connections from remote routers, from 1 (highest priority) to 4 (lowest priority). The default value is &#x60;4&#x60;. | [optional] [default to null]
**egress_flow_window_size** | **i64** | Indicates how many outstanding guaranteed messages can be sent over the Remote Message VPN connection before acknowledgement is received by the sender. The default value is &#x60;255&#x60;. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Remote Message VPN. The default value is &#x60;false&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**password** | **String** | The password for the Client Username that the Bridge uses to login to the Remote Message VPN. The default is to have no &#x60;password&#x60;. | [optional] [default to null]
**queue_binding** | **String** | The queue binding of the Bridge for the Remote Message VPN. The Bridge attempts to bind to that queue over the Bridge link once the link has been established, or immediately if it already is established. The queue must be configured on the remote router when the Bridge connection is established. If the bind fails an event log is generated which includes the reason for the failure. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**remote_msg_vpn_interface** | **String** | The interface on the local router through which to access the Remote Message VPN. If not provided (recommended) then an interface will be chosen automatically based on routing tables. If an interface is provided, \&quot;remoteMsgVpnLocation\&quot; must be either a hostname or IP Address, not a virtual router-name. | [optional] [default to null]
**remote_msg_vpn_location** | **String** | The location of the Remote Message VPN. This may be given as either an FQDN (resolvable via DNS), IP Address, or virtual router-name (starts with &#39;v:&#39;). If specified as a FQDN or IP Address, a port must be specified as well. | [optional] [default to null]
**remote_msg_vpn_name** | **String** | The name of the Remote Message VPN. | [optional] [default to null]
**tls_enabled** | **bool** | Enable or disable TLS for the Remote Message VPN. The default value is &#x60;false&#x60;. | [optional] [default to null]
**unidirectional_client_profile** | **String** | The Client Profile for the unidirectional Bridge for the Remote Message VPN. The Client Profile must exist in the local Message VPN, and it is used only for the TCP parameters. The default value is &#x60;\&quot;#client-profile\&quot;&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


