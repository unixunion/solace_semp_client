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
pub struct MsgVpnJndiConnectionFactory {
  /// Enable or disable whether new JMS connections can use the same Client identifier (ID) as an existing connection. The default value is `false`. Available since 2.3.
  #[serde(rename = "allowDuplicateClientIdEnabled")]
  allow_duplicate_client_id_enabled: Option<bool>,
  /// The description of the Client. The default value is `\"\"`.
  #[serde(rename = "clientDescription")]
  client_description: Option<String>,
  /// The Client identifier (ID). If not specified, a unique value for it will be generated. The default value is `\"\"`.
  #[serde(rename = "clientId")]
  client_id: Option<String>,
  /// The name of the JMS Connection Factory.
  #[serde(rename = "connectionFactoryName")]
  connection_factory_name: Option<String>,
  /// Enable or disable overriding by the Subscriber (Consumer) of the deliver-to-one (DTO) property on messages. When enabled, the Subscriber can receive all DTO tagged messages. The default value is `true`.
  #[serde(rename = "dtoReceiveOverrideEnabled")]
  dto_receive_override_enabled: Option<bool>,
  /// The priority for receiving deliver-to-one (DTO) messages by the Subscriber (Consumer) if the messages are published on the local Router that the Subscriber is directly connected to. The default value is `1`.
  #[serde(rename = "dtoReceiveSubscriberLocalPriority")]
  dto_receive_subscriber_local_priority: Option<i32>,
  /// The priority for receiving deliver-to-one (DTO) messages by the Subscriber (Consumer) if the messages are published on a remote Router. The default value is `1`.
  #[serde(rename = "dtoReceiveSubscriberNetworkPriority")]
  dto_receive_subscriber_network_priority: Option<i32>,
  /// Enable or disable the deliver-to-one (DTO) property on messages sent by the Publisher (Producer). The default value is `false`.
  #[serde(rename = "dtoSendEnabled")]
  dto_send_enabled: Option<bool>,
  /// Enable or disable whether a durable endpoint will be dynamically created on the Router when the client calls \"Session.createDurableSubscriber()\" or \"Session.createQueue()\". The created endpoint respects the message time-to-live (TTL) according to the \"dynamicEndpointRespectTtlEnabled\" property. The default value is `false`.
  #[serde(rename = "dynamicEndpointCreateDurableEnabled")]
  dynamic_endpoint_create_durable_enabled: Option<bool>,
  /// Enable or disable whether dynamically created durable and non-durable endpoints respect the message time-to-live (TTL) property. The default value is `true`.
  #[serde(rename = "dynamicEndpointRespectTtlEnabled")]
  dynamic_endpoint_respect_ttl_enabled: Option<bool>,
  /// The timeout for sending the acknowledgement (ACK) for guaranteed messages received by the Subscriber (Consumer), in milliseconds. The default value is `1000`.
  #[serde(rename = "guaranteedReceiveAckTimeout")]
  guaranteed_receive_ack_timeout: Option<i32>,
  /// The size of the window for guaranteed messages received by the Subscriber (Consumer), in messages. The default value is `18`.
  #[serde(rename = "guaranteedReceiveWindowSize")]
  guaranteed_receive_window_size: Option<i32>,
  /// The threshold for sending the acknowledgement (ACK) for guaranteed messages received by the Subscriber (Consumer) as a percentage of the \"guaranteedReceiveWindowSize\" value. The default value is `60`.
  #[serde(rename = "guaranteedReceiveWindowSizeAckThreshold")]
  guaranteed_receive_window_size_ack_threshold: Option<i32>,
  /// The timeout for receiving the acknowledgement (ACK) for guaranteed messages sent by the Publisher (Producer), in milliseconds. The default value is `2000`.
  #[serde(rename = "guaranteedSendAckTimeout")]
  guaranteed_send_ack_timeout: Option<i32>,
  /// The size of the window for non-persistent guaranteed messages sent by the Publisher (Producer), in messages. For persistent messages the window size is fixed at 1. The default value is `255`.
  #[serde(rename = "guaranteedSendWindowSize")]
  guaranteed_send_window_size: Option<i32>,
  /// The default delivery mode for messages sent by the Publisher (Producer). The default value is `\"persistent\"`. The allowed values and their meaning are:  <pre> \"persistent\" - Router spools messages (persists in the Message Spool) as part of the send operation. \"non-persistent\" - Router does not spool messages (does not persist in the Message Spool) as part of the send operation. </pre> 
  #[serde(rename = "messagingDefaultDeliveryMode")]
  messaging_default_delivery_mode: Option<String>,
  /// Enable or disable whether messages sent by the Publisher (Producer) are Dead Message Queue (DMQ) eligible by default. The default value is `false`.
  #[serde(rename = "messagingDefaultDmqEligibleEnabled")]
  messaging_default_dmq_eligible_enabled: Option<bool>,
  /// Enable or disable whether messages sent by the Publisher (Producer) are Eliding eligible by default. The default value is `false`.
  #[serde(rename = "messagingDefaultElidingEligibleEnabled")]
  messaging_default_eliding_eligible_enabled: Option<bool>,
  /// Enable or disable inclusion (adding or replacing) of the JMSXUserID property in messages sent by the Publisher (Producer). The default value is `false`.
  #[serde(rename = "messagingJmsxUserIdEnabled")]
  messaging_jmsx_user_id_enabled: Option<bool>,
  /// Enable or disable encoding of JMS text messages in Publisher (Producer) messages as XML payload. When disabled, JMS text messages are encoded as a binary attachment. The default value is `true`.
  #[serde(rename = "messagingTextInXmlPayloadEnabled")]
  messaging_text_in_xml_payload_enabled: Option<bool>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName")]
  msg_vpn_name: Option<String>,
  /// The ZLIB compression level for the connection to the Router. The value \"0\" means no compression, and the value \"-1\" means the compression level is specified in the JNDI Properties file. The default value is `-1`.
  #[serde(rename = "transportCompressionLevel")]
  transport_compression_level: Option<i32>,
  /// The maximum number of retry attempts to establish an initial connection to the host (Router) or list of hosts (Routers). The value \"0\" means a single attempt (no retries), and the value \"-1\" means to retry forever. The default value is `0`.
  #[serde(rename = "transportConnectRetryCount")]
  transport_connect_retry_count: Option<i32>,
  /// The maximum number of retry attempts to establish an initial connection to each host (Router) on the list of hosts (Routers). The value \"0\" means a single attempt (no retries), and the value \"-1\" means to retry forever. The default value is `0`.
  #[serde(rename = "transportConnectRetryPerHostCount")]
  transport_connect_retry_per_host_count: Option<i32>,
  /// The timeout for establishing an initial connection to the Router, in milliseconds. The default value is `30000`.
  #[serde(rename = "transportConnectTimeout")]
  transport_connect_timeout: Option<i32>,
  /// Enable or disable usage of the Direct Transport mode for sending non-persistent messages. When disabled, the Guaranteed Transport mode is used. The default value is `true`.
  #[serde(rename = "transportDirectTransportEnabled")]
  transport_direct_transport_enabled: Option<bool>,
  /// The maximum number of consecutive application-level keepalive messages sent without the Router response before the connection to the Router is closed. The default value is `3`.
  #[serde(rename = "transportKeepaliveCount")]
  transport_keepalive_count: Option<i32>,
  /// Enable or disable usage of application-level keepalive messages to maintain a connection with the Router. The default value is `true`.
  #[serde(rename = "transportKeepaliveEnabled")]
  transport_keepalive_enabled: Option<bool>,
  /// The interval between application-level keepalive messages, in milliseconds. The default value is `3000`.
  #[serde(rename = "transportKeepaliveInterval")]
  transport_keepalive_interval: Option<i32>,
  /// Enable or disable delivery of asynchronous messages directly from the I/O thread. Contact Solace Support before enabling this property. The default value is `false`.
  #[serde(rename = "transportMsgCallbackOnIoThreadEnabled")]
  transport_msg_callback_on_io_thread_enabled: Option<bool>,
  /// Enable or disable optimization for the Direct Transport delivery mode. If enabled, the client application is limited to one Publisher (Producer) and one non-durable Subscriber (Consumer). The default value is `false`.
  #[serde(rename = "transportOptimizeDirectEnabled")]
  transport_optimize_direct_enabled: Option<bool>,
  /// The connection port number on the Router for SMF clients. The value \"-1\" means the port is specified in the JNDI Properties file. The default value is `-1`.
  #[serde(rename = "transportPort")]
  transport_port: Option<i32>,
  /// The timeout for reading a reply from the Router, in milliseconds. The default value is `10000`.
  #[serde(rename = "transportReadTimeout")]
  transport_read_timeout: Option<i32>,
  /// The size of the receive socket buffer, in bytes. It corresponds to the SO_RCVBUF socket option. The default value is `65536`.
  #[serde(rename = "transportReceiveBufferSize")]
  transport_receive_buffer_size: Option<i32>,
  /// The maximum number of attempts to reconnect to the host (Router) or list of hosts (Routers) after the connection has been lost. The value \"-1\" means to retry forever. The default value is `3`.
  #[serde(rename = "transportReconnectRetryCount")]
  transport_reconnect_retry_count: Option<i32>,
  /// The amount of time before making another attempt to connect or reconnect to the host (Router) after the connection has been lost, in milliseconds. The default value is `3000`.
  #[serde(rename = "transportReconnectRetryWait")]
  transport_reconnect_retry_wait: Option<i32>,
  /// The size of the send socket buffer, in bytes. It corresponds to the SO_SNDBUF socket option. The default value is `65536`.
  #[serde(rename = "transportSendBufferSize")]
  transport_send_buffer_size: Option<i32>,
  /// Enable or disable the TCP_NODELAY option. When enabled, Nagle's algorithm for TCP/IP congestion control (RFC 896) is disabled. The default value is `true`.
  #[serde(rename = "transportTcpNoDelayEnabled")]
  transport_tcp_no_delay_enabled: Option<bool>,
  /// Enable or disable this as an XA Connection Factory. When enabled, the Connection Factory can be cast to \"XAConnectionFactory\", \"XAQueueConnectionFactory\" or \"XATopicConnectionFactory\". The default value is `false`.
  #[serde(rename = "xaEnabled")]
  xa_enabled: Option<bool>
}

impl MsgVpnJndiConnectionFactory {
  pub fn new() -> MsgVpnJndiConnectionFactory {
    MsgVpnJndiConnectionFactory {
      allow_duplicate_client_id_enabled: None,
      client_description: None,
      client_id: None,
      connection_factory_name: None,
      dto_receive_override_enabled: None,
      dto_receive_subscriber_local_priority: None,
      dto_receive_subscriber_network_priority: None,
      dto_send_enabled: None,
      dynamic_endpoint_create_durable_enabled: None,
      dynamic_endpoint_respect_ttl_enabled: None,
      guaranteed_receive_ack_timeout: None,
      guaranteed_receive_window_size: None,
      guaranteed_receive_window_size_ack_threshold: None,
      guaranteed_send_ack_timeout: None,
      guaranteed_send_window_size: None,
      messaging_default_delivery_mode: None,
      messaging_default_dmq_eligible_enabled: None,
      messaging_default_eliding_eligible_enabled: None,
      messaging_jmsx_user_id_enabled: None,
      messaging_text_in_xml_payload_enabled: None,
      msg_vpn_name: None,
      transport_compression_level: None,
      transport_connect_retry_count: None,
      transport_connect_retry_per_host_count: None,
      transport_connect_timeout: None,
      transport_direct_transport_enabled: None,
      transport_keepalive_count: None,
      transport_keepalive_enabled: None,
      transport_keepalive_interval: None,
      transport_msg_callback_on_io_thread_enabled: None,
      transport_optimize_direct_enabled: None,
      transport_port: None,
      transport_read_timeout: None,
      transport_receive_buffer_size: None,
      transport_reconnect_retry_count: None,
      transport_reconnect_retry_wait: None,
      transport_send_buffer_size: None,
      transport_tcp_no_delay_enabled: None,
      xa_enabled: None
    }
  }

  pub fn set_allow_duplicate_client_id_enabled(&mut self, allow_duplicate_client_id_enabled: bool) {
    self.allow_duplicate_client_id_enabled = Some(allow_duplicate_client_id_enabled);
  }

  pub fn with_allow_duplicate_client_id_enabled(mut self, allow_duplicate_client_id_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.allow_duplicate_client_id_enabled = Some(allow_duplicate_client_id_enabled);
    self
  }

  pub fn allow_duplicate_client_id_enabled(&self) -> Option<&bool> {
    self.allow_duplicate_client_id_enabled.as_ref()
  }

  pub fn reset_allow_duplicate_client_id_enabled(&mut self) {
    self.allow_duplicate_client_id_enabled = None;
  }

  pub fn set_client_description(&mut self, client_description: String) {
    self.client_description = Some(client_description);
  }

  pub fn with_client_description(mut self, client_description: String) -> MsgVpnJndiConnectionFactory {
    self.client_description = Some(client_description);
    self
  }

  pub fn client_description(&self) -> Option<&String> {
    self.client_description.as_ref()
  }

  pub fn reset_client_description(&mut self) {
    self.client_description = None;
  }

  pub fn set_client_id(&mut self, client_id: String) {
    self.client_id = Some(client_id);
  }

  pub fn with_client_id(mut self, client_id: String) -> MsgVpnJndiConnectionFactory {
    self.client_id = Some(client_id);
    self
  }

  pub fn client_id(&self) -> Option<&String> {
    self.client_id.as_ref()
  }

  pub fn reset_client_id(&mut self) {
    self.client_id = None;
  }

  pub fn set_connection_factory_name(&mut self, connection_factory_name: String) {
    self.connection_factory_name = Some(connection_factory_name);
  }

  pub fn with_connection_factory_name(mut self, connection_factory_name: String) -> MsgVpnJndiConnectionFactory {
    self.connection_factory_name = Some(connection_factory_name);
    self
  }

  pub fn connection_factory_name(&self) -> Option<&String> {
    self.connection_factory_name.as_ref()
  }

  pub fn reset_connection_factory_name(&mut self) {
    self.connection_factory_name = None;
  }

  pub fn set_dto_receive_override_enabled(&mut self, dto_receive_override_enabled: bool) {
    self.dto_receive_override_enabled = Some(dto_receive_override_enabled);
  }

  pub fn with_dto_receive_override_enabled(mut self, dto_receive_override_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.dto_receive_override_enabled = Some(dto_receive_override_enabled);
    self
  }

  pub fn dto_receive_override_enabled(&self) -> Option<&bool> {
    self.dto_receive_override_enabled.as_ref()
  }

  pub fn reset_dto_receive_override_enabled(&mut self) {
    self.dto_receive_override_enabled = None;
  }

  pub fn set_dto_receive_subscriber_local_priority(&mut self, dto_receive_subscriber_local_priority: i32) {
    self.dto_receive_subscriber_local_priority = Some(dto_receive_subscriber_local_priority);
  }

  pub fn with_dto_receive_subscriber_local_priority(mut self, dto_receive_subscriber_local_priority: i32) -> MsgVpnJndiConnectionFactory {
    self.dto_receive_subscriber_local_priority = Some(dto_receive_subscriber_local_priority);
    self
  }

  pub fn dto_receive_subscriber_local_priority(&self) -> Option<&i32> {
    self.dto_receive_subscriber_local_priority.as_ref()
  }

  pub fn reset_dto_receive_subscriber_local_priority(&mut self) {
    self.dto_receive_subscriber_local_priority = None;
  }

  pub fn set_dto_receive_subscriber_network_priority(&mut self, dto_receive_subscriber_network_priority: i32) {
    self.dto_receive_subscriber_network_priority = Some(dto_receive_subscriber_network_priority);
  }

  pub fn with_dto_receive_subscriber_network_priority(mut self, dto_receive_subscriber_network_priority: i32) -> MsgVpnJndiConnectionFactory {
    self.dto_receive_subscriber_network_priority = Some(dto_receive_subscriber_network_priority);
    self
  }

  pub fn dto_receive_subscriber_network_priority(&self) -> Option<&i32> {
    self.dto_receive_subscriber_network_priority.as_ref()
  }

  pub fn reset_dto_receive_subscriber_network_priority(&mut self) {
    self.dto_receive_subscriber_network_priority = None;
  }

  pub fn set_dto_send_enabled(&mut self, dto_send_enabled: bool) {
    self.dto_send_enabled = Some(dto_send_enabled);
  }

  pub fn with_dto_send_enabled(mut self, dto_send_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.dto_send_enabled = Some(dto_send_enabled);
    self
  }

  pub fn dto_send_enabled(&self) -> Option<&bool> {
    self.dto_send_enabled.as_ref()
  }

  pub fn reset_dto_send_enabled(&mut self) {
    self.dto_send_enabled = None;
  }

  pub fn set_dynamic_endpoint_create_durable_enabled(&mut self, dynamic_endpoint_create_durable_enabled: bool) {
    self.dynamic_endpoint_create_durable_enabled = Some(dynamic_endpoint_create_durable_enabled);
  }

  pub fn with_dynamic_endpoint_create_durable_enabled(mut self, dynamic_endpoint_create_durable_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.dynamic_endpoint_create_durable_enabled = Some(dynamic_endpoint_create_durable_enabled);
    self
  }

  pub fn dynamic_endpoint_create_durable_enabled(&self) -> Option<&bool> {
    self.dynamic_endpoint_create_durable_enabled.as_ref()
  }

  pub fn reset_dynamic_endpoint_create_durable_enabled(&mut self) {
    self.dynamic_endpoint_create_durable_enabled = None;
  }

  pub fn set_dynamic_endpoint_respect_ttl_enabled(&mut self, dynamic_endpoint_respect_ttl_enabled: bool) {
    self.dynamic_endpoint_respect_ttl_enabled = Some(dynamic_endpoint_respect_ttl_enabled);
  }

  pub fn with_dynamic_endpoint_respect_ttl_enabled(mut self, dynamic_endpoint_respect_ttl_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.dynamic_endpoint_respect_ttl_enabled = Some(dynamic_endpoint_respect_ttl_enabled);
    self
  }

  pub fn dynamic_endpoint_respect_ttl_enabled(&self) -> Option<&bool> {
    self.dynamic_endpoint_respect_ttl_enabled.as_ref()
  }

  pub fn reset_dynamic_endpoint_respect_ttl_enabled(&mut self) {
    self.dynamic_endpoint_respect_ttl_enabled = None;
  }

  pub fn set_guaranteed_receive_ack_timeout(&mut self, guaranteed_receive_ack_timeout: i32) {
    self.guaranteed_receive_ack_timeout = Some(guaranteed_receive_ack_timeout);
  }

  pub fn with_guaranteed_receive_ack_timeout(mut self, guaranteed_receive_ack_timeout: i32) -> MsgVpnJndiConnectionFactory {
    self.guaranteed_receive_ack_timeout = Some(guaranteed_receive_ack_timeout);
    self
  }

  pub fn guaranteed_receive_ack_timeout(&self) -> Option<&i32> {
    self.guaranteed_receive_ack_timeout.as_ref()
  }

  pub fn reset_guaranteed_receive_ack_timeout(&mut self) {
    self.guaranteed_receive_ack_timeout = None;
  }

  pub fn set_guaranteed_receive_window_size(&mut self, guaranteed_receive_window_size: i32) {
    self.guaranteed_receive_window_size = Some(guaranteed_receive_window_size);
  }

  pub fn with_guaranteed_receive_window_size(mut self, guaranteed_receive_window_size: i32) -> MsgVpnJndiConnectionFactory {
    self.guaranteed_receive_window_size = Some(guaranteed_receive_window_size);
    self
  }

  pub fn guaranteed_receive_window_size(&self) -> Option<&i32> {
    self.guaranteed_receive_window_size.as_ref()
  }

  pub fn reset_guaranteed_receive_window_size(&mut self) {
    self.guaranteed_receive_window_size = None;
  }

  pub fn set_guaranteed_receive_window_size_ack_threshold(&mut self, guaranteed_receive_window_size_ack_threshold: i32) {
    self.guaranteed_receive_window_size_ack_threshold = Some(guaranteed_receive_window_size_ack_threshold);
  }

  pub fn with_guaranteed_receive_window_size_ack_threshold(mut self, guaranteed_receive_window_size_ack_threshold: i32) -> MsgVpnJndiConnectionFactory {
    self.guaranteed_receive_window_size_ack_threshold = Some(guaranteed_receive_window_size_ack_threshold);
    self
  }

  pub fn guaranteed_receive_window_size_ack_threshold(&self) -> Option<&i32> {
    self.guaranteed_receive_window_size_ack_threshold.as_ref()
  }

  pub fn reset_guaranteed_receive_window_size_ack_threshold(&mut self) {
    self.guaranteed_receive_window_size_ack_threshold = None;
  }

  pub fn set_guaranteed_send_ack_timeout(&mut self, guaranteed_send_ack_timeout: i32) {
    self.guaranteed_send_ack_timeout = Some(guaranteed_send_ack_timeout);
  }

  pub fn with_guaranteed_send_ack_timeout(mut self, guaranteed_send_ack_timeout: i32) -> MsgVpnJndiConnectionFactory {
    self.guaranteed_send_ack_timeout = Some(guaranteed_send_ack_timeout);
    self
  }

  pub fn guaranteed_send_ack_timeout(&self) -> Option<&i32> {
    self.guaranteed_send_ack_timeout.as_ref()
  }

  pub fn reset_guaranteed_send_ack_timeout(&mut self) {
    self.guaranteed_send_ack_timeout = None;
  }

  pub fn set_guaranteed_send_window_size(&mut self, guaranteed_send_window_size: i32) {
    self.guaranteed_send_window_size = Some(guaranteed_send_window_size);
  }

  pub fn with_guaranteed_send_window_size(mut self, guaranteed_send_window_size: i32) -> MsgVpnJndiConnectionFactory {
    self.guaranteed_send_window_size = Some(guaranteed_send_window_size);
    self
  }

  pub fn guaranteed_send_window_size(&self) -> Option<&i32> {
    self.guaranteed_send_window_size.as_ref()
  }

  pub fn reset_guaranteed_send_window_size(&mut self) {
    self.guaranteed_send_window_size = None;
  }

  pub fn set_messaging_default_delivery_mode(&mut self, messaging_default_delivery_mode: String) {
    self.messaging_default_delivery_mode = Some(messaging_default_delivery_mode);
  }

  pub fn with_messaging_default_delivery_mode(mut self, messaging_default_delivery_mode: String) -> MsgVpnJndiConnectionFactory {
    self.messaging_default_delivery_mode = Some(messaging_default_delivery_mode);
    self
  }

  pub fn messaging_default_delivery_mode(&self) -> Option<&String> {
    self.messaging_default_delivery_mode.as_ref()
  }

  pub fn reset_messaging_default_delivery_mode(&mut self) {
    self.messaging_default_delivery_mode = None;
  }

  pub fn set_messaging_default_dmq_eligible_enabled(&mut self, messaging_default_dmq_eligible_enabled: bool) {
    self.messaging_default_dmq_eligible_enabled = Some(messaging_default_dmq_eligible_enabled);
  }

  pub fn with_messaging_default_dmq_eligible_enabled(mut self, messaging_default_dmq_eligible_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.messaging_default_dmq_eligible_enabled = Some(messaging_default_dmq_eligible_enabled);
    self
  }

  pub fn messaging_default_dmq_eligible_enabled(&self) -> Option<&bool> {
    self.messaging_default_dmq_eligible_enabled.as_ref()
  }

  pub fn reset_messaging_default_dmq_eligible_enabled(&mut self) {
    self.messaging_default_dmq_eligible_enabled = None;
  }

  pub fn set_messaging_default_eliding_eligible_enabled(&mut self, messaging_default_eliding_eligible_enabled: bool) {
    self.messaging_default_eliding_eligible_enabled = Some(messaging_default_eliding_eligible_enabled);
  }

  pub fn with_messaging_default_eliding_eligible_enabled(mut self, messaging_default_eliding_eligible_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.messaging_default_eliding_eligible_enabled = Some(messaging_default_eliding_eligible_enabled);
    self
  }

  pub fn messaging_default_eliding_eligible_enabled(&self) -> Option<&bool> {
    self.messaging_default_eliding_eligible_enabled.as_ref()
  }

  pub fn reset_messaging_default_eliding_eligible_enabled(&mut self) {
    self.messaging_default_eliding_eligible_enabled = None;
  }

  pub fn set_messaging_jmsx_user_id_enabled(&mut self, messaging_jmsx_user_id_enabled: bool) {
    self.messaging_jmsx_user_id_enabled = Some(messaging_jmsx_user_id_enabled);
  }

  pub fn with_messaging_jmsx_user_id_enabled(mut self, messaging_jmsx_user_id_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.messaging_jmsx_user_id_enabled = Some(messaging_jmsx_user_id_enabled);
    self
  }

  pub fn messaging_jmsx_user_id_enabled(&self) -> Option<&bool> {
    self.messaging_jmsx_user_id_enabled.as_ref()
  }

  pub fn reset_messaging_jmsx_user_id_enabled(&mut self) {
    self.messaging_jmsx_user_id_enabled = None;
  }

  pub fn set_messaging_text_in_xml_payload_enabled(&mut self, messaging_text_in_xml_payload_enabled: bool) {
    self.messaging_text_in_xml_payload_enabled = Some(messaging_text_in_xml_payload_enabled);
  }

  pub fn with_messaging_text_in_xml_payload_enabled(mut self, messaging_text_in_xml_payload_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.messaging_text_in_xml_payload_enabled = Some(messaging_text_in_xml_payload_enabled);
    self
  }

  pub fn messaging_text_in_xml_payload_enabled(&self) -> Option<&bool> {
    self.messaging_text_in_xml_payload_enabled.as_ref()
  }

  pub fn reset_messaging_text_in_xml_payload_enabled(&mut self) {
    self.messaging_text_in_xml_payload_enabled = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnJndiConnectionFactory {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_transport_compression_level(&mut self, transport_compression_level: i32) {
    self.transport_compression_level = Some(transport_compression_level);
  }

  pub fn with_transport_compression_level(mut self, transport_compression_level: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_compression_level = Some(transport_compression_level);
    self
  }

  pub fn transport_compression_level(&self) -> Option<&i32> {
    self.transport_compression_level.as_ref()
  }

  pub fn reset_transport_compression_level(&mut self) {
    self.transport_compression_level = None;
  }

  pub fn set_transport_connect_retry_count(&mut self, transport_connect_retry_count: i32) {
    self.transport_connect_retry_count = Some(transport_connect_retry_count);
  }

  pub fn with_transport_connect_retry_count(mut self, transport_connect_retry_count: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_connect_retry_count = Some(transport_connect_retry_count);
    self
  }

  pub fn transport_connect_retry_count(&self) -> Option<&i32> {
    self.transport_connect_retry_count.as_ref()
  }

  pub fn reset_transport_connect_retry_count(&mut self) {
    self.transport_connect_retry_count = None;
  }

  pub fn set_transport_connect_retry_per_host_count(&mut self, transport_connect_retry_per_host_count: i32) {
    self.transport_connect_retry_per_host_count = Some(transport_connect_retry_per_host_count);
  }

  pub fn with_transport_connect_retry_per_host_count(mut self, transport_connect_retry_per_host_count: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_connect_retry_per_host_count = Some(transport_connect_retry_per_host_count);
    self
  }

  pub fn transport_connect_retry_per_host_count(&self) -> Option<&i32> {
    self.transport_connect_retry_per_host_count.as_ref()
  }

  pub fn reset_transport_connect_retry_per_host_count(&mut self) {
    self.transport_connect_retry_per_host_count = None;
  }

  pub fn set_transport_connect_timeout(&mut self, transport_connect_timeout: i32) {
    self.transport_connect_timeout = Some(transport_connect_timeout);
  }

  pub fn with_transport_connect_timeout(mut self, transport_connect_timeout: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_connect_timeout = Some(transport_connect_timeout);
    self
  }

  pub fn transport_connect_timeout(&self) -> Option<&i32> {
    self.transport_connect_timeout.as_ref()
  }

  pub fn reset_transport_connect_timeout(&mut self) {
    self.transport_connect_timeout = None;
  }

  pub fn set_transport_direct_transport_enabled(&mut self, transport_direct_transport_enabled: bool) {
    self.transport_direct_transport_enabled = Some(transport_direct_transport_enabled);
  }

  pub fn with_transport_direct_transport_enabled(mut self, transport_direct_transport_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.transport_direct_transport_enabled = Some(transport_direct_transport_enabled);
    self
  }

  pub fn transport_direct_transport_enabled(&self) -> Option<&bool> {
    self.transport_direct_transport_enabled.as_ref()
  }

  pub fn reset_transport_direct_transport_enabled(&mut self) {
    self.transport_direct_transport_enabled = None;
  }

  pub fn set_transport_keepalive_count(&mut self, transport_keepalive_count: i32) {
    self.transport_keepalive_count = Some(transport_keepalive_count);
  }

  pub fn with_transport_keepalive_count(mut self, transport_keepalive_count: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_keepalive_count = Some(transport_keepalive_count);
    self
  }

  pub fn transport_keepalive_count(&self) -> Option<&i32> {
    self.transport_keepalive_count.as_ref()
  }

  pub fn reset_transport_keepalive_count(&mut self) {
    self.transport_keepalive_count = None;
  }

  pub fn set_transport_keepalive_enabled(&mut self, transport_keepalive_enabled: bool) {
    self.transport_keepalive_enabled = Some(transport_keepalive_enabled);
  }

  pub fn with_transport_keepalive_enabled(mut self, transport_keepalive_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.transport_keepalive_enabled = Some(transport_keepalive_enabled);
    self
  }

  pub fn transport_keepalive_enabled(&self) -> Option<&bool> {
    self.transport_keepalive_enabled.as_ref()
  }

  pub fn reset_transport_keepalive_enabled(&mut self) {
    self.transport_keepalive_enabled = None;
  }

  pub fn set_transport_keepalive_interval(&mut self, transport_keepalive_interval: i32) {
    self.transport_keepalive_interval = Some(transport_keepalive_interval);
  }

  pub fn with_transport_keepalive_interval(mut self, transport_keepalive_interval: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_keepalive_interval = Some(transport_keepalive_interval);
    self
  }

  pub fn transport_keepalive_interval(&self) -> Option<&i32> {
    self.transport_keepalive_interval.as_ref()
  }

  pub fn reset_transport_keepalive_interval(&mut self) {
    self.transport_keepalive_interval = None;
  }

  pub fn set_transport_msg_callback_on_io_thread_enabled(&mut self, transport_msg_callback_on_io_thread_enabled: bool) {
    self.transport_msg_callback_on_io_thread_enabled = Some(transport_msg_callback_on_io_thread_enabled);
  }

  pub fn with_transport_msg_callback_on_io_thread_enabled(mut self, transport_msg_callback_on_io_thread_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.transport_msg_callback_on_io_thread_enabled = Some(transport_msg_callback_on_io_thread_enabled);
    self
  }

  pub fn transport_msg_callback_on_io_thread_enabled(&self) -> Option<&bool> {
    self.transport_msg_callback_on_io_thread_enabled.as_ref()
  }

  pub fn reset_transport_msg_callback_on_io_thread_enabled(&mut self) {
    self.transport_msg_callback_on_io_thread_enabled = None;
  }

  pub fn set_transport_optimize_direct_enabled(&mut self, transport_optimize_direct_enabled: bool) {
    self.transport_optimize_direct_enabled = Some(transport_optimize_direct_enabled);
  }

  pub fn with_transport_optimize_direct_enabled(mut self, transport_optimize_direct_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.transport_optimize_direct_enabled = Some(transport_optimize_direct_enabled);
    self
  }

  pub fn transport_optimize_direct_enabled(&self) -> Option<&bool> {
    self.transport_optimize_direct_enabled.as_ref()
  }

  pub fn reset_transport_optimize_direct_enabled(&mut self) {
    self.transport_optimize_direct_enabled = None;
  }

  pub fn set_transport_port(&mut self, transport_port: i32) {
    self.transport_port = Some(transport_port);
  }

  pub fn with_transport_port(mut self, transport_port: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_port = Some(transport_port);
    self
  }

  pub fn transport_port(&self) -> Option<&i32> {
    self.transport_port.as_ref()
  }

  pub fn reset_transport_port(&mut self) {
    self.transport_port = None;
  }

  pub fn set_transport_read_timeout(&mut self, transport_read_timeout: i32) {
    self.transport_read_timeout = Some(transport_read_timeout);
  }

  pub fn with_transport_read_timeout(mut self, transport_read_timeout: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_read_timeout = Some(transport_read_timeout);
    self
  }

  pub fn transport_read_timeout(&self) -> Option<&i32> {
    self.transport_read_timeout.as_ref()
  }

  pub fn reset_transport_read_timeout(&mut self) {
    self.transport_read_timeout = None;
  }

  pub fn set_transport_receive_buffer_size(&mut self, transport_receive_buffer_size: i32) {
    self.transport_receive_buffer_size = Some(transport_receive_buffer_size);
  }

  pub fn with_transport_receive_buffer_size(mut self, transport_receive_buffer_size: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_receive_buffer_size = Some(transport_receive_buffer_size);
    self
  }

  pub fn transport_receive_buffer_size(&self) -> Option<&i32> {
    self.transport_receive_buffer_size.as_ref()
  }

  pub fn reset_transport_receive_buffer_size(&mut self) {
    self.transport_receive_buffer_size = None;
  }

  pub fn set_transport_reconnect_retry_count(&mut self, transport_reconnect_retry_count: i32) {
    self.transport_reconnect_retry_count = Some(transport_reconnect_retry_count);
  }

  pub fn with_transport_reconnect_retry_count(mut self, transport_reconnect_retry_count: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_reconnect_retry_count = Some(transport_reconnect_retry_count);
    self
  }

  pub fn transport_reconnect_retry_count(&self) -> Option<&i32> {
    self.transport_reconnect_retry_count.as_ref()
  }

  pub fn reset_transport_reconnect_retry_count(&mut self) {
    self.transport_reconnect_retry_count = None;
  }

  pub fn set_transport_reconnect_retry_wait(&mut self, transport_reconnect_retry_wait: i32) {
    self.transport_reconnect_retry_wait = Some(transport_reconnect_retry_wait);
  }

  pub fn with_transport_reconnect_retry_wait(mut self, transport_reconnect_retry_wait: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_reconnect_retry_wait = Some(transport_reconnect_retry_wait);
    self
  }

  pub fn transport_reconnect_retry_wait(&self) -> Option<&i32> {
    self.transport_reconnect_retry_wait.as_ref()
  }

  pub fn reset_transport_reconnect_retry_wait(&mut self) {
    self.transport_reconnect_retry_wait = None;
  }

  pub fn set_transport_send_buffer_size(&mut self, transport_send_buffer_size: i32) {
    self.transport_send_buffer_size = Some(transport_send_buffer_size);
  }

  pub fn with_transport_send_buffer_size(mut self, transport_send_buffer_size: i32) -> MsgVpnJndiConnectionFactory {
    self.transport_send_buffer_size = Some(transport_send_buffer_size);
    self
  }

  pub fn transport_send_buffer_size(&self) -> Option<&i32> {
    self.transport_send_buffer_size.as_ref()
  }

  pub fn reset_transport_send_buffer_size(&mut self) {
    self.transport_send_buffer_size = None;
  }

  pub fn set_transport_tcp_no_delay_enabled(&mut self, transport_tcp_no_delay_enabled: bool) {
    self.transport_tcp_no_delay_enabled = Some(transport_tcp_no_delay_enabled);
  }

  pub fn with_transport_tcp_no_delay_enabled(mut self, transport_tcp_no_delay_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.transport_tcp_no_delay_enabled = Some(transport_tcp_no_delay_enabled);
    self
  }

  pub fn transport_tcp_no_delay_enabled(&self) -> Option<&bool> {
    self.transport_tcp_no_delay_enabled.as_ref()
  }

  pub fn reset_transport_tcp_no_delay_enabled(&mut self) {
    self.transport_tcp_no_delay_enabled = None;
  }

  pub fn set_xa_enabled(&mut self, xa_enabled: bool) {
    self.xa_enabled = Some(xa_enabled);
  }

  pub fn with_xa_enabled(mut self, xa_enabled: bool) -> MsgVpnJndiConnectionFactory {
    self.xa_enabled = Some(xa_enabled);
    self
  }

  pub fn xa_enabled(&self) -> Option<&bool> {
    self.xa_enabled.as_ref()
  }

  pub fn reset_xa_enabled(&mut self) {
    self.xa_enabled = None;
  }

}



