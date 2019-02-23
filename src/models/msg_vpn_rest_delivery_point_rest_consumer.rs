/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see [note 1](#notes)) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+  broker. Resources are either individual **objects**, or **collections** of  objects. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See [note 2](#notes)    Resources are always nouns, with individual objects being singular and  collections being plural. Objects within a collection are identified by an  `obj-id`, which follows the collection name with the form  `collection-name/obj-id`. Some examples:  <pre> /SEMP/v2/config/msgVpns                       ; MsgVpn collection /SEMP/v2/config/msgVpns/finance               ; MsgVpn object named \"finance\" /SEMP/v2/config/msgVpns/finance/queues        ; Queue collection within MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ ; Queue object named \"orderQ\" within MsgVpn \"finance\" </pre>  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and  are described by JSON arrays. Each item in the array represents an object in  the same manner as the individual object would normally be represented. The creation of a new object is done through its collection  resource.   ## Object Resources  Objects are composed of attributes and collections, and are described by JSON  content as name/value pairs. The collections of an object are not contained  directly in the object's JSON content, rather the content includes a URI  attribute which points to the collection. This contained collection resource  must be managed as a separate resource through this URI.  At a minimum, every object has 1 or more identifying attributes, and its own  `uri` attribute which contains the URI to itself. Attributes may have any  (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See [note 3](#notes) Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in  certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request     ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these  general principles:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see [note 4](#notes)) PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many  method/URI combinations. Individual URIs may document additional parameters.  Note that multiple query parameters can be used together in a single URI,  separated by the ampersand character. For example:  <pre> ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 </pre>  ### select  Include in the response only selected attributes of the object, or exclude  from the response selected attributes of the object. Use this query parameter  to limit the size of the returned data for each returned object, return only  those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the  list contains attribute names that are not prefaced by `-`, only those  attributes are included in the response. If the list contains attribute names  that are prefaced by `-`, those attributes are excluded from the response. If  the list contains both types, then the difference of the first set of  attributes and the second set of attributes is returned. If the list is  empty (i.e. `select=`), no attributes are returned  All attributes that are prefaced by `-` must follow all attributes that are  not prefaced by `-`. In addition, each attribute name in the list must match  at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute  names are supported using periods (e.g. `parentName.childName`).  Some examples:  <pre> ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName  ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName  ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication*  ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication*  ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission </pre>  ### where  Include in the response only objects where certain conditions are true. Use  this query parameter to limit which objects are returned to those whose  attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions  must be true for the object to be included in the response. Each expression  takes the form:  <pre> expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' </pre>  `value` may be a number, string, `true`, or `false`, as appropriate for the  type of `attribute-name`. Greater-than and less-than comparisons only work for  numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more  characters). Some examples:  <pre> ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true  ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap  ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100  ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* </pre>  ### count  Limit the count of objects in the response. This can be useful to limit the  size of the response for large collections. The minimum value for `count` is  `1` and the default is `10`. There is a hidden maximum  as to prevent overloading the system. For example:  <pre> ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 </pre>  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data  that should not be created or interpreted by SEMP clients, and should only be  used as described below.  When a request is made for a collection and there may be additional objects  available for retrieval that are not included in the initial response, the  response will include a `cursorQuery` field containing a cursor. The value  of this field can be specified in the `cursor` query parameter of a  subsequent request to retrieve the next page of objects. For convenience,  an appropriate URI is constructed automatically by the broker and included  in the `nextPageUri` field of the response. This URI can be used directly  to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first. 5|For DELETE, the body of the request currently serves no purpose and will cause an error if not empty.    
 *
 * OpenAPI spec version: 2.10
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgVpnRestDeliveryPointRestConsumer {
  /// The PEM formatted content for the client certificate that the REST Consumer will present to the REST host. It must consist of a private key and between one and three certificates comprising the certificate trust chain. The default value is `\"\"`. Available since 2.9.
  #[serde(rename = "authenticationClientCertContent")]
  authentication_client_cert_content: Option<String>,
  /// The password for the client certificate that the REST Consumer will present to the REST host. The default value is `\"\"`. Available since 2.9.
  #[serde(rename = "authenticationClientCertPassword")]
  authentication_client_cert_password: Option<String>,
  /// The password that the REST Consumer will use to login to the REST host. The default value is `\"\"`.
  #[serde(rename = "authenticationHttpBasicPassword")]
  authentication_http_basic_password: Option<String>,
  /// The username that the REST Consumer will use to login to the REST host. Normally a username is only configured when basic authentication is selected for the REST Consumer. The default value is `\"\"`.
  #[serde(rename = "authenticationHttpBasicUsername")]
  authentication_http_basic_username: Option<String>,
  /// The authentication scheme used by the REST Consumer to login to the REST host. The default value is `\"none\"`. The allowed values and their meaning are:  <pre> \"none\" - Login with no authentication. This may be useful for anonymous connections or when a REST Consumer does not require authentication. \"http-basic\" - Login with a username and optional password according to HTTP Basic authentication as per RFC2616. \"client-certificate\" - Login with a client TLS certificate as per RFC5246. Client certificate authentication is only available on TLS connections. </pre> 
  #[serde(rename = "authenticationScheme")]
  authentication_scheme: Option<String>,
  /// Enable or disable the REST Consumer. When disabled, no connections are initiated or messages delivered to this particular REST Consumer. The default value is `false`.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// The interface that will be used for all outgoing connections associated with the REST Consumer. When unspecified, an interface is automatically chosen. The default value is `\"\"`.
  #[serde(rename = "localInterface")]
  local_interface: Option<String>,
  /// The maximum amount of time (in seconds) to wait for an HTTP POST response from the REST Consumer. Once this time is exceeded, the TCP connection is reset. If the POST request is for a direct message, then the message is discarded. If for a persistent message, then message redelivery is attempted on another available outgoing connection for the REST Delivery Point. The default value is `30`.
  #[serde(rename = "maxPostWaitTime")]
  max_post_wait_time: Option<i32>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName")]
  msg_vpn_name: Option<String>,
  /// The total number of concurrent TCP connections open to the REST Consumer. Multiple connections to a single REST Consumer increase throughput via concurrency. The default value is `3`.
  #[serde(rename = "outgoingConnectionCount")]
  outgoing_connection_count: Option<i32>,
  /// The IP address or DNS name to which the router is to connect to deliver messages for this REST Consumer. If the REST Consumer is enabled while the host value is not configured then the REST Consumer has an operational Down state due to the empty host configuration until a usable host value is configured. The default value is `\"\"`.
  #[serde(rename = "remoteHost")]
  remote_host: Option<String>,
  /// The port associated with the host of the REST Consumer. The port can only be changed when the REST Consumer is disabled. The default value is `8080`.
  #[serde(rename = "remotePort")]
  remote_port: Option<i64>,
  /// The name of the REST Consumer.
  #[serde(rename = "restConsumerName")]
  rest_consumer_name: Option<String>,
  /// The name of the REST Delivery Point.
  #[serde(rename = "restDeliveryPointName")]
  rest_delivery_point_name: Option<String>,
  /// The number of seconds that must pass before retrying the remote REST Consumer connection. The default value is `3`.
  #[serde(rename = "retryDelay")]
  retry_delay: Option<i32>,
  /// The colon-separated list of cipher-suites the REST Consumer uses in its encrypted connection. All supported suites are included by default, from most-secure to least-secure. The REST Consumer should choose the first suite from this list that it supports. The cipher-suite list can only be changed when the REST Consumer is disabled. The default value is `\"default\"`.
  #[serde(rename = "tlsCipherSuiteList")]
  tls_cipher_suite_list: Option<String>,
  /// Enable or disable TLS for the REST Consumer. This may only be done when the REST Consumer is disabled. The default value is `false`.
  #[serde(rename = "tlsEnabled")]
  tls_enabled: Option<bool>
}

impl MsgVpnRestDeliveryPointRestConsumer {
  pub fn new() -> MsgVpnRestDeliveryPointRestConsumer {
    MsgVpnRestDeliveryPointRestConsumer {
      authentication_client_cert_content: None,
      authentication_client_cert_password: None,
      authentication_http_basic_password: None,
      authentication_http_basic_username: None,
      authentication_scheme: None,
      enabled: None,
      local_interface: None,
      max_post_wait_time: None,
      msg_vpn_name: None,
      outgoing_connection_count: None,
      remote_host: None,
      remote_port: None,
      rest_consumer_name: None,
      rest_delivery_point_name: None,
      retry_delay: None,
      tls_cipher_suite_list: None,
      tls_enabled: None
    }
  }

  pub fn set_authentication_client_cert_content(&mut self, authentication_client_cert_content: String) {
    self.authentication_client_cert_content = Some(authentication_client_cert_content);
  }

  pub fn with_authentication_client_cert_content(mut self, authentication_client_cert_content: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.authentication_client_cert_content = Some(authentication_client_cert_content);
    self
  }

  pub fn authentication_client_cert_content(&self) -> Option<&String> {
    self.authentication_client_cert_content.as_ref()
  }

  pub fn reset_authentication_client_cert_content(&mut self) {
    self.authentication_client_cert_content = None;
  }

  pub fn set_authentication_client_cert_password(&mut self, authentication_client_cert_password: String) {
    self.authentication_client_cert_password = Some(authentication_client_cert_password);
  }

  pub fn with_authentication_client_cert_password(mut self, authentication_client_cert_password: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.authentication_client_cert_password = Some(authentication_client_cert_password);
    self
  }

  pub fn authentication_client_cert_password(&self) -> Option<&String> {
    self.authentication_client_cert_password.as_ref()
  }

  pub fn reset_authentication_client_cert_password(&mut self) {
    self.authentication_client_cert_password = None;
  }

  pub fn set_authentication_http_basic_password(&mut self, authentication_http_basic_password: String) {
    self.authentication_http_basic_password = Some(authentication_http_basic_password);
  }

  pub fn with_authentication_http_basic_password(mut self, authentication_http_basic_password: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.authentication_http_basic_password = Some(authentication_http_basic_password);
    self
  }

  pub fn authentication_http_basic_password(&self) -> Option<&String> {
    self.authentication_http_basic_password.as_ref()
  }

  pub fn reset_authentication_http_basic_password(&mut self) {
    self.authentication_http_basic_password = None;
  }

  pub fn set_authentication_http_basic_username(&mut self, authentication_http_basic_username: String) {
    self.authentication_http_basic_username = Some(authentication_http_basic_username);
  }

  pub fn with_authentication_http_basic_username(mut self, authentication_http_basic_username: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.authentication_http_basic_username = Some(authentication_http_basic_username);
    self
  }

  pub fn authentication_http_basic_username(&self) -> Option<&String> {
    self.authentication_http_basic_username.as_ref()
  }

  pub fn reset_authentication_http_basic_username(&mut self) {
    self.authentication_http_basic_username = None;
  }

  pub fn set_authentication_scheme(&mut self, authentication_scheme: String) {
    self.authentication_scheme = Some(authentication_scheme);
  }

  pub fn with_authentication_scheme(mut self, authentication_scheme: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.authentication_scheme = Some(authentication_scheme);
    self
  }

  pub fn authentication_scheme(&self) -> Option<&String> {
    self.authentication_scheme.as_ref()
  }

  pub fn reset_authentication_scheme(&mut self) {
    self.authentication_scheme = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnRestDeliveryPointRestConsumer {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_local_interface(&mut self, local_interface: String) {
    self.local_interface = Some(local_interface);
  }

  pub fn with_local_interface(mut self, local_interface: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.local_interface = Some(local_interface);
    self
  }

  pub fn local_interface(&self) -> Option<&String> {
    self.local_interface.as_ref()
  }

  pub fn reset_local_interface(&mut self) {
    self.local_interface = None;
  }

  pub fn set_max_post_wait_time(&mut self, max_post_wait_time: i32) {
    self.max_post_wait_time = Some(max_post_wait_time);
  }

  pub fn with_max_post_wait_time(mut self, max_post_wait_time: i32) -> MsgVpnRestDeliveryPointRestConsumer {
    self.max_post_wait_time = Some(max_post_wait_time);
    self
  }

  pub fn max_post_wait_time(&self) -> Option<&i32> {
    self.max_post_wait_time.as_ref()
  }

  pub fn reset_max_post_wait_time(&mut self) {
    self.max_post_wait_time = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_outgoing_connection_count(&mut self, outgoing_connection_count: i32) {
    self.outgoing_connection_count = Some(outgoing_connection_count);
  }

  pub fn with_outgoing_connection_count(mut self, outgoing_connection_count: i32) -> MsgVpnRestDeliveryPointRestConsumer {
    self.outgoing_connection_count = Some(outgoing_connection_count);
    self
  }

  pub fn outgoing_connection_count(&self) -> Option<&i32> {
    self.outgoing_connection_count.as_ref()
  }

  pub fn reset_outgoing_connection_count(&mut self) {
    self.outgoing_connection_count = None;
  }

  pub fn set_remote_host(&mut self, remote_host: String) {
    self.remote_host = Some(remote_host);
  }

  pub fn with_remote_host(mut self, remote_host: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.remote_host = Some(remote_host);
    self
  }

  pub fn remote_host(&self) -> Option<&String> {
    self.remote_host.as_ref()
  }

  pub fn reset_remote_host(&mut self) {
    self.remote_host = None;
  }

  pub fn set_remote_port(&mut self, remote_port: i64) {
    self.remote_port = Some(remote_port);
  }

  pub fn with_remote_port(mut self, remote_port: i64) -> MsgVpnRestDeliveryPointRestConsumer {
    self.remote_port = Some(remote_port);
    self
  }

  pub fn remote_port(&self) -> Option<&i64> {
    self.remote_port.as_ref()
  }

  pub fn reset_remote_port(&mut self) {
    self.remote_port = None;
  }

  pub fn set_rest_consumer_name(&mut self, rest_consumer_name: String) {
    self.rest_consumer_name = Some(rest_consumer_name);
  }

  pub fn with_rest_consumer_name(mut self, rest_consumer_name: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.rest_consumer_name = Some(rest_consumer_name);
    self
  }

  pub fn rest_consumer_name(&self) -> Option<&String> {
    self.rest_consumer_name.as_ref()
  }

  pub fn reset_rest_consumer_name(&mut self) {
    self.rest_consumer_name = None;
  }

  pub fn set_rest_delivery_point_name(&mut self, rest_delivery_point_name: String) {
    self.rest_delivery_point_name = Some(rest_delivery_point_name);
  }

  pub fn with_rest_delivery_point_name(mut self, rest_delivery_point_name: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.rest_delivery_point_name = Some(rest_delivery_point_name);
    self
  }

  pub fn rest_delivery_point_name(&self) -> Option<&String> {
    self.rest_delivery_point_name.as_ref()
  }

  pub fn reset_rest_delivery_point_name(&mut self) {
    self.rest_delivery_point_name = None;
  }

  pub fn set_retry_delay(&mut self, retry_delay: i32) {
    self.retry_delay = Some(retry_delay);
  }

  pub fn with_retry_delay(mut self, retry_delay: i32) -> MsgVpnRestDeliveryPointRestConsumer {
    self.retry_delay = Some(retry_delay);
    self
  }

  pub fn retry_delay(&self) -> Option<&i32> {
    self.retry_delay.as_ref()
  }

  pub fn reset_retry_delay(&mut self) {
    self.retry_delay = None;
  }

  pub fn set_tls_cipher_suite_list(&mut self, tls_cipher_suite_list: String) {
    self.tls_cipher_suite_list = Some(tls_cipher_suite_list);
  }

  pub fn with_tls_cipher_suite_list(mut self, tls_cipher_suite_list: String) -> MsgVpnRestDeliveryPointRestConsumer {
    self.tls_cipher_suite_list = Some(tls_cipher_suite_list);
    self
  }

  pub fn tls_cipher_suite_list(&self) -> Option<&String> {
    self.tls_cipher_suite_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_list(&mut self) {
    self.tls_cipher_suite_list = None;
  }

  pub fn set_tls_enabled(&mut self, tls_enabled: bool) {
    self.tls_enabled = Some(tls_enabled);
  }

  pub fn with_tls_enabled(mut self, tls_enabled: bool) -> MsgVpnRestDeliveryPointRestConsumer {
    self.tls_enabled = Some(tls_enabled);
    self
  }

  pub fn tls_enabled(&self) -> Option<&bool> {
    self.tls_enabled.as_ref()
  }

  pub fn reset_tls_enabled(&mut self) {
    self.tls_enabled = None;
  }

}



