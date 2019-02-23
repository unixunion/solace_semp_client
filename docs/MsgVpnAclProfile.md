# MsgVpnAclProfile

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acl_profile_name** | **String** | The name of the ACL Profile. | [optional] [default to null]
**client_connect_default_action** | **String** | The default action when a Client connects to the Message VPN. The default value is &#x60;\&quot;disallow\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;allow\&quot; - Allow client connection unless an exception is found for it. \&quot;disallow\&quot; - Disallow client connection unless an exception is found for it. &lt;/pre&gt;  | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**publish_topic_default_action** | **String** | The default action to take when a Client publishes to a Topic in the Message VPN. The default value is &#x60;\&quot;disallow\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;allow\&quot; - Allow topic unless an exception is found for it. \&quot;disallow\&quot; - Disallow topic unless an exception is found for it. &lt;/pre&gt;  | [optional] [default to null]
**subscribe_topic_default_action** | **String** | The default action to take when a Client subscribes to a Topic. The default value is &#x60;\&quot;disallow\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;allow\&quot; - Allow topic unless an exception is found for it. \&quot;disallow\&quot; - Disallow topic unless an exception is found for it. &lt;/pre&gt;  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


