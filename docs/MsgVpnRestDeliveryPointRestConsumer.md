# MsgVpnRestDeliveryPointRestConsumer

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_client_cert_content** | **String** | The PEM formatted content for the client certificate that the REST Consumer will present to the REST host. It must consist of a private key and between one and three certificates comprising the certificate trust chain. The default value is &#x60;\&quot;\&quot;&#x60;. Available since 2.9. | [optional] [default to null]
**authentication_client_cert_password** | **String** | The password for the client certificate that the REST Consumer will present to the REST host. The default value is &#x60;\&quot;\&quot;&#x60;. Available since 2.9. | [optional] [default to null]
**authentication_http_basic_password** | **String** | The password that the REST Consumer will use to login to the REST host. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**authentication_http_basic_username** | **String** | The username that the REST Consumer will use to login to the REST host. Normally a username is only configured when basic authentication is selected for the REST Consumer. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**authentication_scheme** | **String** | The authentication scheme used by the REST Consumer to login to the REST host. The default value is &#x60;\&quot;none\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;none\&quot; - Login with no authentication. This may be useful for anonymous connections or when a REST Consumer does not require authentication. \&quot;http-basic\&quot; - Login with a username and optional password according to HTTP Basic authentication as per RFC2616. \&quot;client-certificate\&quot; - Login with a client TLS certificate as per RFC5246. Client certificate authentication is only available on TLS connections. &lt;/pre&gt;  | [optional] [default to null]
**enabled** | **bool** | Enable or disable the REST Consumer. When disabled, no connections are initiated or messages delivered to this particular REST Consumer. The default value is &#x60;false&#x60;. | [optional] [default to null]
**local_interface** | **String** | The interface that will be used for all outgoing connections associated with the REST Consumer. When unspecified, an interface is automatically chosen. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**max_post_wait_time** | **i32** | The maximum amount of time (in seconds) to wait for an HTTP POST response from the REST Consumer. Once this time is exceeded, the TCP connection is reset. If the POST request is for a direct message, then the message is discarded. If for a persistent message, then message redelivery is attempted on another available outgoing connection for the REST Delivery Point. The default value is &#x60;30&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**outgoing_connection_count** | **i32** | The total number of concurrent TCP connections open to the REST Consumer. Multiple connections to a single REST Consumer increase throughput via concurrency. The default value is &#x60;3&#x60;. | [optional] [default to null]
**remote_host** | **String** | The IP address or DNS name to which the router is to connect to deliver messages for this REST Consumer. If the REST Consumer is enabled while the host value is not configured then the REST Consumer has an operational Down state due to the empty host configuration until a usable host value is configured. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**remote_port** | **i64** | The port associated with the host of the REST Consumer. The port can only be changed when the REST Consumer is disabled. The default value is &#x60;8080&#x60;. | [optional] [default to null]
**rest_consumer_name** | **String** | The name of the REST Consumer. | [optional] [default to null]
**rest_delivery_point_name** | **String** | The name of the REST Delivery Point. | [optional] [default to null]
**retry_delay** | **i32** | The number of seconds that must pass before retrying the remote REST Consumer connection. The default value is &#x60;3&#x60;. | [optional] [default to null]
**tls_cipher_suite_list** | **String** | The colon-separated list of cipher-suites the REST Consumer uses in its encrypted connection. All supported suites are included by default, from most-secure to least-secure. The REST Consumer should choose the first suite from this list that it supports. The cipher-suite list can only be changed when the REST Consumer is disabled. The default value is &#x60;\&quot;default\&quot;&#x60;. | [optional] [default to null]
**tls_enabled** | **bool** | Enable or disable TLS for the REST Consumer. This may only be done when the REST Consumer is disabled. The default value is &#x60;false&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


