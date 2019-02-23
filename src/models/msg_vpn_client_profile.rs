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
pub struct MsgVpnClientProfile {
  /// Allow or deny Bridge clients to connect. Changing this setting does not affect existing Bridge client connections. The default value is `false`.
  #[serde(rename = "allowBridgeConnectionsEnabled")]
  allow_bridge_connections_enabled: Option<bool>,
  /// Allow or deny clients to bind to topic endpoints or queues with the cut-through delivery mode. Changing this setting does not affect existing client connections. The default value is `false`.
  #[serde(rename = "allowCutThroughForwardingEnabled")]
  allow_cut_through_forwarding_enabled: Option<bool>,
  /// Allow or deny clients to create topic endponts or queues. Changing this setting does not affect existing client connections. The default value is `false`.
  #[serde(rename = "allowGuaranteedEndpointCreateEnabled")]
  allow_guaranteed_endpoint_create_enabled: Option<bool>,
  /// Allow or deny clients to receive guaranteed messages. Changing this setting does not affect existing client connections. The default value is `false`.
  #[serde(rename = "allowGuaranteedMsgReceiveEnabled")]
  allow_guaranteed_msg_receive_enabled: Option<bool>,
  /// Allow or deny clients to send guaranteed messages. Changing this setting does not affect existing client connections. The default value is `false`.
  #[serde(rename = "allowGuaranteedMsgSendEnabled")]
  allow_guaranteed_msg_send_enabled: Option<bool>,
  /// Allow or deny clients to establish transacted sessions. Changing this setting does not affect existing client connections. The default value is `false`.
  #[serde(rename = "allowTransactedSessionsEnabled")]
  allow_transacted_sessions_enabled: Option<bool>,
  /// The name of a Queue to copy settings from when a new Queue is created by an API. The referenced Queue must exist on the Message VPN. The default value is `\"\"`.
  #[serde(rename = "apiQueueManagementCopyFromOnCreateName")]
  api_queue_management_copy_from_on_create_name: Option<String>,
  /// The name of a Topic Endpoint to copy settings from when a new Topic Endpoint is created by an API. The referenced Topic Endpoint must exist on the Message VPN. The default value is `\"\"`.
  #[serde(rename = "apiTopicEndpointManagementCopyFromOnCreateName")]
  api_topic_endpoint_management_copy_from_on_create_name: Option<String>,
  /// The Client Profile name.
  #[serde(rename = "clientProfileName")]
  client_profile_name: Option<String>,
  /// Enable or disable whether connected clients are allowed to use compression. The default value is `true`. Available since 2.10.
  #[serde(rename = "compressionEnabled")]
  compression_enabled: Option<bool>,
  /// The amount of time to delay the delivery of messages to clients after the initial message has been delivered (the eliding delay interval), in milliseconds. Zero value means there is no delay in delivering messages to clients. The default value is `0`.
  #[serde(rename = "elidingDelay")]
  eliding_delay: Option<i64>,
  /// Enable or disable the Message Eliding. The default value is `false`.
  #[serde(rename = "elidingEnabled")]
  eliding_enabled: Option<bool>,
  /// The maximum number of topics tracked for Message Eliding per one Client connection. The default value is `256`.
  #[serde(rename = "elidingMaxTopicCount")]
  eliding_max_topic_count: Option<i64>,
  #[serde(rename = "eventClientProvisionedEndpointSpoolUsageThreshold")]
  event_client_provisioned_endpoint_spool_usage_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "eventConnectionCountPerClientUsernameThreshold")]
  event_connection_count_per_client_username_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventEgressFlowCountThreshold")]
  event_egress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventEndpointCountPerClientUsernameThreshold")]
  event_endpoint_count_per_client_username_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventIngressFlowCountThreshold")]
  event_ingress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceSmfConnectionCountPerClientUsernameThreshold")]
  event_service_smf_connection_count_per_client_username_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceWebConnectionCountPerClientUsernameThreshold")]
  event_service_web_connection_count_per_client_username_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventSubscriptionCountThreshold")]
  event_subscription_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventTransactedSessionCountThreshold")]
  event_transacted_session_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventTransactionCountThreshold")]
  event_transaction_count_threshold: Option<::models::EventThreshold>,
  /// The maximum number of client connections that can be simultaneously connected with the same Client Username. The default is the max value supported by the hardware.
  #[serde(rename = "maxConnectionCountPerClientUsername")]
  max_connection_count_per_client_username: Option<i64>,
  /// The maximum number of egress flows that can be created by one client. The default is the max value supported by the hardware.
  #[serde(rename = "maxEgressFlowCount")]
  max_egress_flow_count: Option<i64>,
  /// The maximum number of queues and topic endpoints that can be created by clients with the same Client Username. The default is the max value supported by the hardware.
  #[serde(rename = "maxEndpointCountPerClientUsername")]
  max_endpoint_count_per_client_username: Option<i64>,
  /// The maximum number of ingress flows that can be created by one client. The default is the max value supported by the hardware.
  #[serde(rename = "maxIngressFlowCount")]
  max_ingress_flow_count: Option<i64>,
  /// The maximum number of subscriptions that can be created by one client. The default varies by platform.
  #[serde(rename = "maxSubscriptionCount")]
  max_subscription_count: Option<i64>,
  /// The maximum number of transacted sessions that can be created by one client. The default value is `10`.
  #[serde(rename = "maxTransactedSessionCount")]
  max_transacted_session_count: Option<i64>,
  /// The maximum number of transactions that can be created by one client. The default varies by platform.
  #[serde(rename = "maxTransactionCount")]
  max_transaction_count: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName")]
  msg_vpn_name: Option<String>,
  /// The maximum depth of the \"Control 1\" (C-1) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "queueControl1MaxDepth")]
  queue_control1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Control 1\" (C-1) priority queue, regardless of the \"queueControl1MaxDepth\" value. The default value is `4`.
  #[serde(rename = "queueControl1MinMsgBurst")]
  queue_control1_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 1\" (D-1) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "queueDirect1MaxDepth")]
  queue_direct1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 1\" (D-1) priority queue, regardless of the \"queueDirect1MaxDepth\" value. The default value is `4`.
  #[serde(rename = "queueDirect1MinMsgBurst")]
  queue_direct1_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 2\" (D-2) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "queueDirect2MaxDepth")]
  queue_direct2_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 2\" (D-2) priority queue, regardless of the \"queueDirect2MaxDepth\" value. The default value is `4`.
  #[serde(rename = "queueDirect2MinMsgBurst")]
  queue_direct2_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Direct 3\" (D-3) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "queueDirect3MaxDepth")]
  queue_direct3_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Direct 3\" (D-3) priority queue, regardless of the \"queueDirect3MaxDepth\" value. The default value is `4`.
  #[serde(rename = "queueDirect3MinMsgBurst")]
  queue_direct3_min_msg_burst: Option<i32>,
  /// The maximum depth of the \"Guaranteed 1\" (G-1) priority queue, in work units. Each work unit is 2048 bytes of message data. The default value is `20000`.
  #[serde(rename = "queueGuaranteed1MaxDepth")]
  queue_guaranteed1_max_depth: Option<i32>,
  /// The number of messages that are always allowed entry into the \"Guaranteed 1\" (G-3) priority queue, regardless of the \"queueGuaranteed1MaxDepth\" value. The default value is `255`.
  #[serde(rename = "queueGuaranteed1MinMsgBurst")]
  queue_guaranteed1_min_msg_burst: Option<i32>,
  /// Enable or disable sending of a negative acknowledgement (NACK) on the discard of a message because of a message subscription was not found. The default value is `false`. Available since 2.2.
  #[serde(rename = "rejectMsgToSenderOnNoSubscriptionMatchEnabled")]
  reject_msg_to_sender_on_no_subscription_match_enabled: Option<bool>,
  /// Allow or deny clients to connect to the Message VPN if its Replication state is standby. The default value is `false`.
  #[serde(rename = "replicationAllowClientConnectWhenStandbyEnabled")]
  replication_allow_client_connect_when_standby_enabled: Option<bool>,
  /// The maximum number of SMF client connections that can be simultaneously connected with the same Client Username. The default is the max value supported by the hardware.
  #[serde(rename = "serviceSmfMaxConnectionCountPerClientUsername")]
  service_smf_max_connection_count_per_client_username: Option<i64>,
  /// The timeout for inactive Web Transport client sessions, in seconds. The default value is `30`.
  #[serde(rename = "serviceWebInactiveTimeout")]
  service_web_inactive_timeout: Option<i64>,
  /// The maximum number of Web Transport client connections that can be simultaneously connected with the same Client Username. The default is the max value supported by the hardware.
  #[serde(rename = "serviceWebMaxConnectionCountPerClientUsername")]
  service_web_max_connection_count_per_client_username: Option<i64>,
  /// The maximum Web Transport payload size before its fragmentation occurs, in bytes. The size of the header is not included. The default value is `1000000`.
  #[serde(rename = "serviceWebMaxPayload")]
  service_web_max_payload: Option<i64>,
  /// The TCP initial congestion window size, in multiple of the TCP Maximum Segment Size (MSS). Changing the value from its default of 2 results in non-compliance with RFC 2581. Contact Solace Support before changing this value. The default value is `2`.
  #[serde(rename = "tcpCongestionWindowSize")]
  tcp_congestion_window_size: Option<i64>,
  /// The number of TCP keepalive retransmissions to be carried out before declaring that the remote end is not available. The default value is `5`.
  #[serde(rename = "tcpKeepaliveCount")]
  tcp_keepalive_count: Option<i64>,
  /// The amount of time a connection must remain idle before TCP begins sending keepalive probes, in seconds. The default value is `3`.
  #[serde(rename = "tcpKeepaliveIdleTime")]
  tcp_keepalive_idle_time: Option<i64>,
  /// The amount of time between TCP keepalive retransmissions when no acknowledgement is received, in seconds. The default value is `1`.
  #[serde(rename = "tcpKeepaliveInterval")]
  tcp_keepalive_interval: Option<i64>,
  /// The TCP maximum segment size, in kilobytes. Changes are applied to all existing connections. The default value is `1460`.
  #[serde(rename = "tcpMaxSegmentSize")]
  tcp_max_segment_size: Option<i64>,
  /// The TCP maximum window size, in kilobytes. Changes are applied to all existing connections. The default value is `256`.
  #[serde(rename = "tcpMaxWindowSize")]
  tcp_max_window_size: Option<i64>,
  /// Enable or disable allowing a client to downgrade an encrypted connection to plain text. The default value is `true`. Available since 2.8.
  #[serde(rename = "tlsAllowDowngradeToPlainTextEnabled")]
  tls_allow_downgrade_to_plain_text_enabled: Option<bool>
}

impl MsgVpnClientProfile {
  pub fn new() -> MsgVpnClientProfile {
    MsgVpnClientProfile {
      allow_bridge_connections_enabled: None,
      allow_cut_through_forwarding_enabled: None,
      allow_guaranteed_endpoint_create_enabled: None,
      allow_guaranteed_msg_receive_enabled: None,
      allow_guaranteed_msg_send_enabled: None,
      allow_transacted_sessions_enabled: None,
      api_queue_management_copy_from_on_create_name: None,
      api_topic_endpoint_management_copy_from_on_create_name: None,
      client_profile_name: None,
      compression_enabled: None,
      eliding_delay: None,
      eliding_enabled: None,
      eliding_max_topic_count: None,
      event_client_provisioned_endpoint_spool_usage_threshold: None,
      event_connection_count_per_client_username_threshold: None,
      event_egress_flow_count_threshold: None,
      event_endpoint_count_per_client_username_threshold: None,
      event_ingress_flow_count_threshold: None,
      event_service_smf_connection_count_per_client_username_threshold: None,
      event_service_web_connection_count_per_client_username_threshold: None,
      event_subscription_count_threshold: None,
      event_transacted_session_count_threshold: None,
      event_transaction_count_threshold: None,
      max_connection_count_per_client_username: None,
      max_egress_flow_count: None,
      max_endpoint_count_per_client_username: None,
      max_ingress_flow_count: None,
      max_subscription_count: None,
      max_transacted_session_count: None,
      max_transaction_count: None,
      msg_vpn_name: None,
      queue_control1_max_depth: None,
      queue_control1_min_msg_burst: None,
      queue_direct1_max_depth: None,
      queue_direct1_min_msg_burst: None,
      queue_direct2_max_depth: None,
      queue_direct2_min_msg_burst: None,
      queue_direct3_max_depth: None,
      queue_direct3_min_msg_burst: None,
      queue_guaranteed1_max_depth: None,
      queue_guaranteed1_min_msg_burst: None,
      reject_msg_to_sender_on_no_subscription_match_enabled: None,
      replication_allow_client_connect_when_standby_enabled: None,
      service_smf_max_connection_count_per_client_username: None,
      service_web_inactive_timeout: None,
      service_web_max_connection_count_per_client_username: None,
      service_web_max_payload: None,
      tcp_congestion_window_size: None,
      tcp_keepalive_count: None,
      tcp_keepalive_idle_time: None,
      tcp_keepalive_interval: None,
      tcp_max_segment_size: None,
      tcp_max_window_size: None,
      tls_allow_downgrade_to_plain_text_enabled: None
    }
  }

  pub fn set_allow_bridge_connections_enabled(&mut self, allow_bridge_connections_enabled: bool) {
    self.allow_bridge_connections_enabled = Some(allow_bridge_connections_enabled);
  }

  pub fn with_allow_bridge_connections_enabled(mut self, allow_bridge_connections_enabled: bool) -> MsgVpnClientProfile {
    self.allow_bridge_connections_enabled = Some(allow_bridge_connections_enabled);
    self
  }

  pub fn allow_bridge_connections_enabled(&self) -> Option<&bool> {
    self.allow_bridge_connections_enabled.as_ref()
  }

  pub fn reset_allow_bridge_connections_enabled(&mut self) {
    self.allow_bridge_connections_enabled = None;
  }

  pub fn set_allow_cut_through_forwarding_enabled(&mut self, allow_cut_through_forwarding_enabled: bool) {
    self.allow_cut_through_forwarding_enabled = Some(allow_cut_through_forwarding_enabled);
  }

  pub fn with_allow_cut_through_forwarding_enabled(mut self, allow_cut_through_forwarding_enabled: bool) -> MsgVpnClientProfile {
    self.allow_cut_through_forwarding_enabled = Some(allow_cut_through_forwarding_enabled);
    self
  }

  pub fn allow_cut_through_forwarding_enabled(&self) -> Option<&bool> {
    self.allow_cut_through_forwarding_enabled.as_ref()
  }

  pub fn reset_allow_cut_through_forwarding_enabled(&mut self) {
    self.allow_cut_through_forwarding_enabled = None;
  }

  pub fn set_allow_guaranteed_endpoint_create_enabled(&mut self, allow_guaranteed_endpoint_create_enabled: bool) {
    self.allow_guaranteed_endpoint_create_enabled = Some(allow_guaranteed_endpoint_create_enabled);
  }

  pub fn with_allow_guaranteed_endpoint_create_enabled(mut self, allow_guaranteed_endpoint_create_enabled: bool) -> MsgVpnClientProfile {
    self.allow_guaranteed_endpoint_create_enabled = Some(allow_guaranteed_endpoint_create_enabled);
    self
  }

  pub fn allow_guaranteed_endpoint_create_enabled(&self) -> Option<&bool> {
    self.allow_guaranteed_endpoint_create_enabled.as_ref()
  }

  pub fn reset_allow_guaranteed_endpoint_create_enabled(&mut self) {
    self.allow_guaranteed_endpoint_create_enabled = None;
  }

  pub fn set_allow_guaranteed_msg_receive_enabled(&mut self, allow_guaranteed_msg_receive_enabled: bool) {
    self.allow_guaranteed_msg_receive_enabled = Some(allow_guaranteed_msg_receive_enabled);
  }

  pub fn with_allow_guaranteed_msg_receive_enabled(mut self, allow_guaranteed_msg_receive_enabled: bool) -> MsgVpnClientProfile {
    self.allow_guaranteed_msg_receive_enabled = Some(allow_guaranteed_msg_receive_enabled);
    self
  }

  pub fn allow_guaranteed_msg_receive_enabled(&self) -> Option<&bool> {
    self.allow_guaranteed_msg_receive_enabled.as_ref()
  }

  pub fn reset_allow_guaranteed_msg_receive_enabled(&mut self) {
    self.allow_guaranteed_msg_receive_enabled = None;
  }

  pub fn set_allow_guaranteed_msg_send_enabled(&mut self, allow_guaranteed_msg_send_enabled: bool) {
    self.allow_guaranteed_msg_send_enabled = Some(allow_guaranteed_msg_send_enabled);
  }

  pub fn with_allow_guaranteed_msg_send_enabled(mut self, allow_guaranteed_msg_send_enabled: bool) -> MsgVpnClientProfile {
    self.allow_guaranteed_msg_send_enabled = Some(allow_guaranteed_msg_send_enabled);
    self
  }

  pub fn allow_guaranteed_msg_send_enabled(&self) -> Option<&bool> {
    self.allow_guaranteed_msg_send_enabled.as_ref()
  }

  pub fn reset_allow_guaranteed_msg_send_enabled(&mut self) {
    self.allow_guaranteed_msg_send_enabled = None;
  }

  pub fn set_allow_transacted_sessions_enabled(&mut self, allow_transacted_sessions_enabled: bool) {
    self.allow_transacted_sessions_enabled = Some(allow_transacted_sessions_enabled);
  }

  pub fn with_allow_transacted_sessions_enabled(mut self, allow_transacted_sessions_enabled: bool) -> MsgVpnClientProfile {
    self.allow_transacted_sessions_enabled = Some(allow_transacted_sessions_enabled);
    self
  }

  pub fn allow_transacted_sessions_enabled(&self) -> Option<&bool> {
    self.allow_transacted_sessions_enabled.as_ref()
  }

  pub fn reset_allow_transacted_sessions_enabled(&mut self) {
    self.allow_transacted_sessions_enabled = None;
  }

  pub fn set_api_queue_management_copy_from_on_create_name(&mut self, api_queue_management_copy_from_on_create_name: String) {
    self.api_queue_management_copy_from_on_create_name = Some(api_queue_management_copy_from_on_create_name);
  }

  pub fn with_api_queue_management_copy_from_on_create_name(mut self, api_queue_management_copy_from_on_create_name: String) -> MsgVpnClientProfile {
    self.api_queue_management_copy_from_on_create_name = Some(api_queue_management_copy_from_on_create_name);
    self
  }

  pub fn api_queue_management_copy_from_on_create_name(&self) -> Option<&String> {
    self.api_queue_management_copy_from_on_create_name.as_ref()
  }

  pub fn reset_api_queue_management_copy_from_on_create_name(&mut self) {
    self.api_queue_management_copy_from_on_create_name = None;
  }

  pub fn set_api_topic_endpoint_management_copy_from_on_create_name(&mut self, api_topic_endpoint_management_copy_from_on_create_name: String) {
    self.api_topic_endpoint_management_copy_from_on_create_name = Some(api_topic_endpoint_management_copy_from_on_create_name);
  }

  pub fn with_api_topic_endpoint_management_copy_from_on_create_name(mut self, api_topic_endpoint_management_copy_from_on_create_name: String) -> MsgVpnClientProfile {
    self.api_topic_endpoint_management_copy_from_on_create_name = Some(api_topic_endpoint_management_copy_from_on_create_name);
    self
  }

  pub fn api_topic_endpoint_management_copy_from_on_create_name(&self) -> Option<&String> {
    self.api_topic_endpoint_management_copy_from_on_create_name.as_ref()
  }

  pub fn reset_api_topic_endpoint_management_copy_from_on_create_name(&mut self) {
    self.api_topic_endpoint_management_copy_from_on_create_name = None;
  }

  pub fn set_client_profile_name(&mut self, client_profile_name: String) {
    self.client_profile_name = Some(client_profile_name);
  }

  pub fn with_client_profile_name(mut self, client_profile_name: String) -> MsgVpnClientProfile {
    self.client_profile_name = Some(client_profile_name);
    self
  }

  pub fn client_profile_name(&self) -> Option<&String> {
    self.client_profile_name.as_ref()
  }

  pub fn reset_client_profile_name(&mut self) {
    self.client_profile_name = None;
  }

  pub fn set_compression_enabled(&mut self, compression_enabled: bool) {
    self.compression_enabled = Some(compression_enabled);
  }

  pub fn with_compression_enabled(mut self, compression_enabled: bool) -> MsgVpnClientProfile {
    self.compression_enabled = Some(compression_enabled);
    self
  }

  pub fn compression_enabled(&self) -> Option<&bool> {
    self.compression_enabled.as_ref()
  }

  pub fn reset_compression_enabled(&mut self) {
    self.compression_enabled = None;
  }

  pub fn set_eliding_delay(&mut self, eliding_delay: i64) {
    self.eliding_delay = Some(eliding_delay);
  }

  pub fn with_eliding_delay(mut self, eliding_delay: i64) -> MsgVpnClientProfile {
    self.eliding_delay = Some(eliding_delay);
    self
  }

  pub fn eliding_delay(&self) -> Option<&i64> {
    self.eliding_delay.as_ref()
  }

  pub fn reset_eliding_delay(&mut self) {
    self.eliding_delay = None;
  }

  pub fn set_eliding_enabled(&mut self, eliding_enabled: bool) {
    self.eliding_enabled = Some(eliding_enabled);
  }

  pub fn with_eliding_enabled(mut self, eliding_enabled: bool) -> MsgVpnClientProfile {
    self.eliding_enabled = Some(eliding_enabled);
    self
  }

  pub fn eliding_enabled(&self) -> Option<&bool> {
    self.eliding_enabled.as_ref()
  }

  pub fn reset_eliding_enabled(&mut self) {
    self.eliding_enabled = None;
  }

  pub fn set_eliding_max_topic_count(&mut self, eliding_max_topic_count: i64) {
    self.eliding_max_topic_count = Some(eliding_max_topic_count);
  }

  pub fn with_eliding_max_topic_count(mut self, eliding_max_topic_count: i64) -> MsgVpnClientProfile {
    self.eliding_max_topic_count = Some(eliding_max_topic_count);
    self
  }

  pub fn eliding_max_topic_count(&self) -> Option<&i64> {
    self.eliding_max_topic_count.as_ref()
  }

  pub fn reset_eliding_max_topic_count(&mut self) {
    self.eliding_max_topic_count = None;
  }

  pub fn set_event_client_provisioned_endpoint_spool_usage_threshold(&mut self, event_client_provisioned_endpoint_spool_usage_threshold: ::models::EventThresholdByPercent) {
    self.event_client_provisioned_endpoint_spool_usage_threshold = Some(event_client_provisioned_endpoint_spool_usage_threshold);
  }

  pub fn with_event_client_provisioned_endpoint_spool_usage_threshold(mut self, event_client_provisioned_endpoint_spool_usage_threshold: ::models::EventThresholdByPercent) -> MsgVpnClientProfile {
    self.event_client_provisioned_endpoint_spool_usage_threshold = Some(event_client_provisioned_endpoint_spool_usage_threshold);
    self
  }

  pub fn event_client_provisioned_endpoint_spool_usage_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.event_client_provisioned_endpoint_spool_usage_threshold.as_ref()
  }

  pub fn reset_event_client_provisioned_endpoint_spool_usage_threshold(&mut self) {
    self.event_client_provisioned_endpoint_spool_usage_threshold = None;
  }

  pub fn set_event_connection_count_per_client_username_threshold(&mut self, event_connection_count_per_client_username_threshold: ::models::EventThreshold) {
    self.event_connection_count_per_client_username_threshold = Some(event_connection_count_per_client_username_threshold);
  }

  pub fn with_event_connection_count_per_client_username_threshold(mut self, event_connection_count_per_client_username_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_connection_count_per_client_username_threshold = Some(event_connection_count_per_client_username_threshold);
    self
  }

  pub fn event_connection_count_per_client_username_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_connection_count_per_client_username_threshold.as_ref()
  }

  pub fn reset_event_connection_count_per_client_username_threshold(&mut self) {
    self.event_connection_count_per_client_username_threshold = None;
  }

  pub fn set_event_egress_flow_count_threshold(&mut self, event_egress_flow_count_threshold: ::models::EventThreshold) {
    self.event_egress_flow_count_threshold = Some(event_egress_flow_count_threshold);
  }

  pub fn with_event_egress_flow_count_threshold(mut self, event_egress_flow_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_egress_flow_count_threshold = Some(event_egress_flow_count_threshold);
    self
  }

  pub fn event_egress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_egress_flow_count_threshold.as_ref()
  }

  pub fn reset_event_egress_flow_count_threshold(&mut self) {
    self.event_egress_flow_count_threshold = None;
  }

  pub fn set_event_endpoint_count_per_client_username_threshold(&mut self, event_endpoint_count_per_client_username_threshold: ::models::EventThreshold) {
    self.event_endpoint_count_per_client_username_threshold = Some(event_endpoint_count_per_client_username_threshold);
  }

  pub fn with_event_endpoint_count_per_client_username_threshold(mut self, event_endpoint_count_per_client_username_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_endpoint_count_per_client_username_threshold = Some(event_endpoint_count_per_client_username_threshold);
    self
  }

  pub fn event_endpoint_count_per_client_username_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_endpoint_count_per_client_username_threshold.as_ref()
  }

  pub fn reset_event_endpoint_count_per_client_username_threshold(&mut self) {
    self.event_endpoint_count_per_client_username_threshold = None;
  }

  pub fn set_event_ingress_flow_count_threshold(&mut self, event_ingress_flow_count_threshold: ::models::EventThreshold) {
    self.event_ingress_flow_count_threshold = Some(event_ingress_flow_count_threshold);
  }

  pub fn with_event_ingress_flow_count_threshold(mut self, event_ingress_flow_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_ingress_flow_count_threshold = Some(event_ingress_flow_count_threshold);
    self
  }

  pub fn event_ingress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_ingress_flow_count_threshold.as_ref()
  }

  pub fn reset_event_ingress_flow_count_threshold(&mut self) {
    self.event_ingress_flow_count_threshold = None;
  }

  pub fn set_event_service_smf_connection_count_per_client_username_threshold(&mut self, event_service_smf_connection_count_per_client_username_threshold: ::models::EventThreshold) {
    self.event_service_smf_connection_count_per_client_username_threshold = Some(event_service_smf_connection_count_per_client_username_threshold);
  }

  pub fn with_event_service_smf_connection_count_per_client_username_threshold(mut self, event_service_smf_connection_count_per_client_username_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_service_smf_connection_count_per_client_username_threshold = Some(event_service_smf_connection_count_per_client_username_threshold);
    self
  }

  pub fn event_service_smf_connection_count_per_client_username_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_smf_connection_count_per_client_username_threshold.as_ref()
  }

  pub fn reset_event_service_smf_connection_count_per_client_username_threshold(&mut self) {
    self.event_service_smf_connection_count_per_client_username_threshold = None;
  }

  pub fn set_event_service_web_connection_count_per_client_username_threshold(&mut self, event_service_web_connection_count_per_client_username_threshold: ::models::EventThreshold) {
    self.event_service_web_connection_count_per_client_username_threshold = Some(event_service_web_connection_count_per_client_username_threshold);
  }

  pub fn with_event_service_web_connection_count_per_client_username_threshold(mut self, event_service_web_connection_count_per_client_username_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_service_web_connection_count_per_client_username_threshold = Some(event_service_web_connection_count_per_client_username_threshold);
    self
  }

  pub fn event_service_web_connection_count_per_client_username_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_web_connection_count_per_client_username_threshold.as_ref()
  }

  pub fn reset_event_service_web_connection_count_per_client_username_threshold(&mut self) {
    self.event_service_web_connection_count_per_client_username_threshold = None;
  }

  pub fn set_event_subscription_count_threshold(&mut self, event_subscription_count_threshold: ::models::EventThreshold) {
    self.event_subscription_count_threshold = Some(event_subscription_count_threshold);
  }

  pub fn with_event_subscription_count_threshold(mut self, event_subscription_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_subscription_count_threshold = Some(event_subscription_count_threshold);
    self
  }

  pub fn event_subscription_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_subscription_count_threshold.as_ref()
  }

  pub fn reset_event_subscription_count_threshold(&mut self) {
    self.event_subscription_count_threshold = None;
  }

  pub fn set_event_transacted_session_count_threshold(&mut self, event_transacted_session_count_threshold: ::models::EventThreshold) {
    self.event_transacted_session_count_threshold = Some(event_transacted_session_count_threshold);
  }

  pub fn with_event_transacted_session_count_threshold(mut self, event_transacted_session_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_transacted_session_count_threshold = Some(event_transacted_session_count_threshold);
    self
  }

  pub fn event_transacted_session_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_transacted_session_count_threshold.as_ref()
  }

  pub fn reset_event_transacted_session_count_threshold(&mut self) {
    self.event_transacted_session_count_threshold = None;
  }

  pub fn set_event_transaction_count_threshold(&mut self, event_transaction_count_threshold: ::models::EventThreshold) {
    self.event_transaction_count_threshold = Some(event_transaction_count_threshold);
  }

  pub fn with_event_transaction_count_threshold(mut self, event_transaction_count_threshold: ::models::EventThreshold) -> MsgVpnClientProfile {
    self.event_transaction_count_threshold = Some(event_transaction_count_threshold);
    self
  }

  pub fn event_transaction_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_transaction_count_threshold.as_ref()
  }

  pub fn reset_event_transaction_count_threshold(&mut self) {
    self.event_transaction_count_threshold = None;
  }

  pub fn set_max_connection_count_per_client_username(&mut self, max_connection_count_per_client_username: i64) {
    self.max_connection_count_per_client_username = Some(max_connection_count_per_client_username);
  }

  pub fn with_max_connection_count_per_client_username(mut self, max_connection_count_per_client_username: i64) -> MsgVpnClientProfile {
    self.max_connection_count_per_client_username = Some(max_connection_count_per_client_username);
    self
  }

  pub fn max_connection_count_per_client_username(&self) -> Option<&i64> {
    self.max_connection_count_per_client_username.as_ref()
  }

  pub fn reset_max_connection_count_per_client_username(&mut self) {
    self.max_connection_count_per_client_username = None;
  }

  pub fn set_max_egress_flow_count(&mut self, max_egress_flow_count: i64) {
    self.max_egress_flow_count = Some(max_egress_flow_count);
  }

  pub fn with_max_egress_flow_count(mut self, max_egress_flow_count: i64) -> MsgVpnClientProfile {
    self.max_egress_flow_count = Some(max_egress_flow_count);
    self
  }

  pub fn max_egress_flow_count(&self) -> Option<&i64> {
    self.max_egress_flow_count.as_ref()
  }

  pub fn reset_max_egress_flow_count(&mut self) {
    self.max_egress_flow_count = None;
  }

  pub fn set_max_endpoint_count_per_client_username(&mut self, max_endpoint_count_per_client_username: i64) {
    self.max_endpoint_count_per_client_username = Some(max_endpoint_count_per_client_username);
  }

  pub fn with_max_endpoint_count_per_client_username(mut self, max_endpoint_count_per_client_username: i64) -> MsgVpnClientProfile {
    self.max_endpoint_count_per_client_username = Some(max_endpoint_count_per_client_username);
    self
  }

  pub fn max_endpoint_count_per_client_username(&self) -> Option<&i64> {
    self.max_endpoint_count_per_client_username.as_ref()
  }

  pub fn reset_max_endpoint_count_per_client_username(&mut self) {
    self.max_endpoint_count_per_client_username = None;
  }

  pub fn set_max_ingress_flow_count(&mut self, max_ingress_flow_count: i64) {
    self.max_ingress_flow_count = Some(max_ingress_flow_count);
  }

  pub fn with_max_ingress_flow_count(mut self, max_ingress_flow_count: i64) -> MsgVpnClientProfile {
    self.max_ingress_flow_count = Some(max_ingress_flow_count);
    self
  }

  pub fn max_ingress_flow_count(&self) -> Option<&i64> {
    self.max_ingress_flow_count.as_ref()
  }

  pub fn reset_max_ingress_flow_count(&mut self) {
    self.max_ingress_flow_count = None;
  }

  pub fn set_max_subscription_count(&mut self, max_subscription_count: i64) {
    self.max_subscription_count = Some(max_subscription_count);
  }

  pub fn with_max_subscription_count(mut self, max_subscription_count: i64) -> MsgVpnClientProfile {
    self.max_subscription_count = Some(max_subscription_count);
    self
  }

  pub fn max_subscription_count(&self) -> Option<&i64> {
    self.max_subscription_count.as_ref()
  }

  pub fn reset_max_subscription_count(&mut self) {
    self.max_subscription_count = None;
  }

  pub fn set_max_transacted_session_count(&mut self, max_transacted_session_count: i64) {
    self.max_transacted_session_count = Some(max_transacted_session_count);
  }

  pub fn with_max_transacted_session_count(mut self, max_transacted_session_count: i64) -> MsgVpnClientProfile {
    self.max_transacted_session_count = Some(max_transacted_session_count);
    self
  }

  pub fn max_transacted_session_count(&self) -> Option<&i64> {
    self.max_transacted_session_count.as_ref()
  }

  pub fn reset_max_transacted_session_count(&mut self) {
    self.max_transacted_session_count = None;
  }

  pub fn set_max_transaction_count(&mut self, max_transaction_count: i64) {
    self.max_transaction_count = Some(max_transaction_count);
  }

  pub fn with_max_transaction_count(mut self, max_transaction_count: i64) -> MsgVpnClientProfile {
    self.max_transaction_count = Some(max_transaction_count);
    self
  }

  pub fn max_transaction_count(&self) -> Option<&i64> {
    self.max_transaction_count.as_ref()
  }

  pub fn reset_max_transaction_count(&mut self) {
    self.max_transaction_count = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnClientProfile {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_queue_control1_max_depth(&mut self, queue_control1_max_depth: i32) {
    self.queue_control1_max_depth = Some(queue_control1_max_depth);
  }

  pub fn with_queue_control1_max_depth(mut self, queue_control1_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_control1_max_depth = Some(queue_control1_max_depth);
    self
  }

  pub fn queue_control1_max_depth(&self) -> Option<&i32> {
    self.queue_control1_max_depth.as_ref()
  }

  pub fn reset_queue_control1_max_depth(&mut self) {
    self.queue_control1_max_depth = None;
  }

  pub fn set_queue_control1_min_msg_burst(&mut self, queue_control1_min_msg_burst: i32) {
    self.queue_control1_min_msg_burst = Some(queue_control1_min_msg_burst);
  }

  pub fn with_queue_control1_min_msg_burst(mut self, queue_control1_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_control1_min_msg_burst = Some(queue_control1_min_msg_burst);
    self
  }

  pub fn queue_control1_min_msg_burst(&self) -> Option<&i32> {
    self.queue_control1_min_msg_burst.as_ref()
  }

  pub fn reset_queue_control1_min_msg_burst(&mut self) {
    self.queue_control1_min_msg_burst = None;
  }

  pub fn set_queue_direct1_max_depth(&mut self, queue_direct1_max_depth: i32) {
    self.queue_direct1_max_depth = Some(queue_direct1_max_depth);
  }

  pub fn with_queue_direct1_max_depth(mut self, queue_direct1_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_direct1_max_depth = Some(queue_direct1_max_depth);
    self
  }

  pub fn queue_direct1_max_depth(&self) -> Option<&i32> {
    self.queue_direct1_max_depth.as_ref()
  }

  pub fn reset_queue_direct1_max_depth(&mut self) {
    self.queue_direct1_max_depth = None;
  }

  pub fn set_queue_direct1_min_msg_burst(&mut self, queue_direct1_min_msg_burst: i32) {
    self.queue_direct1_min_msg_burst = Some(queue_direct1_min_msg_burst);
  }

  pub fn with_queue_direct1_min_msg_burst(mut self, queue_direct1_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_direct1_min_msg_burst = Some(queue_direct1_min_msg_burst);
    self
  }

  pub fn queue_direct1_min_msg_burst(&self) -> Option<&i32> {
    self.queue_direct1_min_msg_burst.as_ref()
  }

  pub fn reset_queue_direct1_min_msg_burst(&mut self) {
    self.queue_direct1_min_msg_burst = None;
  }

  pub fn set_queue_direct2_max_depth(&mut self, queue_direct2_max_depth: i32) {
    self.queue_direct2_max_depth = Some(queue_direct2_max_depth);
  }

  pub fn with_queue_direct2_max_depth(mut self, queue_direct2_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_direct2_max_depth = Some(queue_direct2_max_depth);
    self
  }

  pub fn queue_direct2_max_depth(&self) -> Option<&i32> {
    self.queue_direct2_max_depth.as_ref()
  }

  pub fn reset_queue_direct2_max_depth(&mut self) {
    self.queue_direct2_max_depth = None;
  }

  pub fn set_queue_direct2_min_msg_burst(&mut self, queue_direct2_min_msg_burst: i32) {
    self.queue_direct2_min_msg_burst = Some(queue_direct2_min_msg_burst);
  }

  pub fn with_queue_direct2_min_msg_burst(mut self, queue_direct2_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_direct2_min_msg_burst = Some(queue_direct2_min_msg_burst);
    self
  }

  pub fn queue_direct2_min_msg_burst(&self) -> Option<&i32> {
    self.queue_direct2_min_msg_burst.as_ref()
  }

  pub fn reset_queue_direct2_min_msg_burst(&mut self) {
    self.queue_direct2_min_msg_burst = None;
  }

  pub fn set_queue_direct3_max_depth(&mut self, queue_direct3_max_depth: i32) {
    self.queue_direct3_max_depth = Some(queue_direct3_max_depth);
  }

  pub fn with_queue_direct3_max_depth(mut self, queue_direct3_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_direct3_max_depth = Some(queue_direct3_max_depth);
    self
  }

  pub fn queue_direct3_max_depth(&self) -> Option<&i32> {
    self.queue_direct3_max_depth.as_ref()
  }

  pub fn reset_queue_direct3_max_depth(&mut self) {
    self.queue_direct3_max_depth = None;
  }

  pub fn set_queue_direct3_min_msg_burst(&mut self, queue_direct3_min_msg_burst: i32) {
    self.queue_direct3_min_msg_burst = Some(queue_direct3_min_msg_burst);
  }

  pub fn with_queue_direct3_min_msg_burst(mut self, queue_direct3_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_direct3_min_msg_burst = Some(queue_direct3_min_msg_burst);
    self
  }

  pub fn queue_direct3_min_msg_burst(&self) -> Option<&i32> {
    self.queue_direct3_min_msg_burst.as_ref()
  }

  pub fn reset_queue_direct3_min_msg_burst(&mut self) {
    self.queue_direct3_min_msg_burst = None;
  }

  pub fn set_queue_guaranteed1_max_depth(&mut self, queue_guaranteed1_max_depth: i32) {
    self.queue_guaranteed1_max_depth = Some(queue_guaranteed1_max_depth);
  }

  pub fn with_queue_guaranteed1_max_depth(mut self, queue_guaranteed1_max_depth: i32) -> MsgVpnClientProfile {
    self.queue_guaranteed1_max_depth = Some(queue_guaranteed1_max_depth);
    self
  }

  pub fn queue_guaranteed1_max_depth(&self) -> Option<&i32> {
    self.queue_guaranteed1_max_depth.as_ref()
  }

  pub fn reset_queue_guaranteed1_max_depth(&mut self) {
    self.queue_guaranteed1_max_depth = None;
  }

  pub fn set_queue_guaranteed1_min_msg_burst(&mut self, queue_guaranteed1_min_msg_burst: i32) {
    self.queue_guaranteed1_min_msg_burst = Some(queue_guaranteed1_min_msg_burst);
  }

  pub fn with_queue_guaranteed1_min_msg_burst(mut self, queue_guaranteed1_min_msg_burst: i32) -> MsgVpnClientProfile {
    self.queue_guaranteed1_min_msg_burst = Some(queue_guaranteed1_min_msg_burst);
    self
  }

  pub fn queue_guaranteed1_min_msg_burst(&self) -> Option<&i32> {
    self.queue_guaranteed1_min_msg_burst.as_ref()
  }

  pub fn reset_queue_guaranteed1_min_msg_burst(&mut self) {
    self.queue_guaranteed1_min_msg_burst = None;
  }

  pub fn set_reject_msg_to_sender_on_no_subscription_match_enabled(&mut self, reject_msg_to_sender_on_no_subscription_match_enabled: bool) {
    self.reject_msg_to_sender_on_no_subscription_match_enabled = Some(reject_msg_to_sender_on_no_subscription_match_enabled);
  }

  pub fn with_reject_msg_to_sender_on_no_subscription_match_enabled(mut self, reject_msg_to_sender_on_no_subscription_match_enabled: bool) -> MsgVpnClientProfile {
    self.reject_msg_to_sender_on_no_subscription_match_enabled = Some(reject_msg_to_sender_on_no_subscription_match_enabled);
    self
  }

  pub fn reject_msg_to_sender_on_no_subscription_match_enabled(&self) -> Option<&bool> {
    self.reject_msg_to_sender_on_no_subscription_match_enabled.as_ref()
  }

  pub fn reset_reject_msg_to_sender_on_no_subscription_match_enabled(&mut self) {
    self.reject_msg_to_sender_on_no_subscription_match_enabled = None;
  }

  pub fn set_replication_allow_client_connect_when_standby_enabled(&mut self, replication_allow_client_connect_when_standby_enabled: bool) {
    self.replication_allow_client_connect_when_standby_enabled = Some(replication_allow_client_connect_when_standby_enabled);
  }

  pub fn with_replication_allow_client_connect_when_standby_enabled(mut self, replication_allow_client_connect_when_standby_enabled: bool) -> MsgVpnClientProfile {
    self.replication_allow_client_connect_when_standby_enabled = Some(replication_allow_client_connect_when_standby_enabled);
    self
  }

  pub fn replication_allow_client_connect_when_standby_enabled(&self) -> Option<&bool> {
    self.replication_allow_client_connect_when_standby_enabled.as_ref()
  }

  pub fn reset_replication_allow_client_connect_when_standby_enabled(&mut self) {
    self.replication_allow_client_connect_when_standby_enabled = None;
  }

  pub fn set_service_smf_max_connection_count_per_client_username(&mut self, service_smf_max_connection_count_per_client_username: i64) {
    self.service_smf_max_connection_count_per_client_username = Some(service_smf_max_connection_count_per_client_username);
  }

  pub fn with_service_smf_max_connection_count_per_client_username(mut self, service_smf_max_connection_count_per_client_username: i64) -> MsgVpnClientProfile {
    self.service_smf_max_connection_count_per_client_username = Some(service_smf_max_connection_count_per_client_username);
    self
  }

  pub fn service_smf_max_connection_count_per_client_username(&self) -> Option<&i64> {
    self.service_smf_max_connection_count_per_client_username.as_ref()
  }

  pub fn reset_service_smf_max_connection_count_per_client_username(&mut self) {
    self.service_smf_max_connection_count_per_client_username = None;
  }

  pub fn set_service_web_inactive_timeout(&mut self, service_web_inactive_timeout: i64) {
    self.service_web_inactive_timeout = Some(service_web_inactive_timeout);
  }

  pub fn with_service_web_inactive_timeout(mut self, service_web_inactive_timeout: i64) -> MsgVpnClientProfile {
    self.service_web_inactive_timeout = Some(service_web_inactive_timeout);
    self
  }

  pub fn service_web_inactive_timeout(&self) -> Option<&i64> {
    self.service_web_inactive_timeout.as_ref()
  }

  pub fn reset_service_web_inactive_timeout(&mut self) {
    self.service_web_inactive_timeout = None;
  }

  pub fn set_service_web_max_connection_count_per_client_username(&mut self, service_web_max_connection_count_per_client_username: i64) {
    self.service_web_max_connection_count_per_client_username = Some(service_web_max_connection_count_per_client_username);
  }

  pub fn with_service_web_max_connection_count_per_client_username(mut self, service_web_max_connection_count_per_client_username: i64) -> MsgVpnClientProfile {
    self.service_web_max_connection_count_per_client_username = Some(service_web_max_connection_count_per_client_username);
    self
  }

  pub fn service_web_max_connection_count_per_client_username(&self) -> Option<&i64> {
    self.service_web_max_connection_count_per_client_username.as_ref()
  }

  pub fn reset_service_web_max_connection_count_per_client_username(&mut self) {
    self.service_web_max_connection_count_per_client_username = None;
  }

  pub fn set_service_web_max_payload(&mut self, service_web_max_payload: i64) {
    self.service_web_max_payload = Some(service_web_max_payload);
  }

  pub fn with_service_web_max_payload(mut self, service_web_max_payload: i64) -> MsgVpnClientProfile {
    self.service_web_max_payload = Some(service_web_max_payload);
    self
  }

  pub fn service_web_max_payload(&self) -> Option<&i64> {
    self.service_web_max_payload.as_ref()
  }

  pub fn reset_service_web_max_payload(&mut self) {
    self.service_web_max_payload = None;
  }

  pub fn set_tcp_congestion_window_size(&mut self, tcp_congestion_window_size: i64) {
    self.tcp_congestion_window_size = Some(tcp_congestion_window_size);
  }

  pub fn with_tcp_congestion_window_size(mut self, tcp_congestion_window_size: i64) -> MsgVpnClientProfile {
    self.tcp_congestion_window_size = Some(tcp_congestion_window_size);
    self
  }

  pub fn tcp_congestion_window_size(&self) -> Option<&i64> {
    self.tcp_congestion_window_size.as_ref()
  }

  pub fn reset_tcp_congestion_window_size(&mut self) {
    self.tcp_congestion_window_size = None;
  }

  pub fn set_tcp_keepalive_count(&mut self, tcp_keepalive_count: i64) {
    self.tcp_keepalive_count = Some(tcp_keepalive_count);
  }

  pub fn with_tcp_keepalive_count(mut self, tcp_keepalive_count: i64) -> MsgVpnClientProfile {
    self.tcp_keepalive_count = Some(tcp_keepalive_count);
    self
  }

  pub fn tcp_keepalive_count(&self) -> Option<&i64> {
    self.tcp_keepalive_count.as_ref()
  }

  pub fn reset_tcp_keepalive_count(&mut self) {
    self.tcp_keepalive_count = None;
  }

  pub fn set_tcp_keepalive_idle_time(&mut self, tcp_keepalive_idle_time: i64) {
    self.tcp_keepalive_idle_time = Some(tcp_keepalive_idle_time);
  }

  pub fn with_tcp_keepalive_idle_time(mut self, tcp_keepalive_idle_time: i64) -> MsgVpnClientProfile {
    self.tcp_keepalive_idle_time = Some(tcp_keepalive_idle_time);
    self
  }

  pub fn tcp_keepalive_idle_time(&self) -> Option<&i64> {
    self.tcp_keepalive_idle_time.as_ref()
  }

  pub fn reset_tcp_keepalive_idle_time(&mut self) {
    self.tcp_keepalive_idle_time = None;
  }

  pub fn set_tcp_keepalive_interval(&mut self, tcp_keepalive_interval: i64) {
    self.tcp_keepalive_interval = Some(tcp_keepalive_interval);
  }

  pub fn with_tcp_keepalive_interval(mut self, tcp_keepalive_interval: i64) -> MsgVpnClientProfile {
    self.tcp_keepalive_interval = Some(tcp_keepalive_interval);
    self
  }

  pub fn tcp_keepalive_interval(&self) -> Option<&i64> {
    self.tcp_keepalive_interval.as_ref()
  }

  pub fn reset_tcp_keepalive_interval(&mut self) {
    self.tcp_keepalive_interval = None;
  }

  pub fn set_tcp_max_segment_size(&mut self, tcp_max_segment_size: i64) {
    self.tcp_max_segment_size = Some(tcp_max_segment_size);
  }

  pub fn with_tcp_max_segment_size(mut self, tcp_max_segment_size: i64) -> MsgVpnClientProfile {
    self.tcp_max_segment_size = Some(tcp_max_segment_size);
    self
  }

  pub fn tcp_max_segment_size(&self) -> Option<&i64> {
    self.tcp_max_segment_size.as_ref()
  }

  pub fn reset_tcp_max_segment_size(&mut self) {
    self.tcp_max_segment_size = None;
  }

  pub fn set_tcp_max_window_size(&mut self, tcp_max_window_size: i64) {
    self.tcp_max_window_size = Some(tcp_max_window_size);
  }

  pub fn with_tcp_max_window_size(mut self, tcp_max_window_size: i64) -> MsgVpnClientProfile {
    self.tcp_max_window_size = Some(tcp_max_window_size);
    self
  }

  pub fn tcp_max_window_size(&self) -> Option<&i64> {
    self.tcp_max_window_size.as_ref()
  }

  pub fn reset_tcp_max_window_size(&mut self) {
    self.tcp_max_window_size = None;
  }

  pub fn set_tls_allow_downgrade_to_plain_text_enabled(&mut self, tls_allow_downgrade_to_plain_text_enabled: bool) {
    self.tls_allow_downgrade_to_plain_text_enabled = Some(tls_allow_downgrade_to_plain_text_enabled);
  }

  pub fn with_tls_allow_downgrade_to_plain_text_enabled(mut self, tls_allow_downgrade_to_plain_text_enabled: bool) -> MsgVpnClientProfile {
    self.tls_allow_downgrade_to_plain_text_enabled = Some(tls_allow_downgrade_to_plain_text_enabled);
    self
  }

  pub fn tls_allow_downgrade_to_plain_text_enabled(&self) -> Option<&bool> {
    self.tls_allow_downgrade_to_plain_text_enabled.as_ref()
  }

  pub fn reset_tls_allow_downgrade_to_plain_text_enabled(&mut self) {
    self.tls_allow_downgrade_to_plain_text_enabled = None;
  }

}



