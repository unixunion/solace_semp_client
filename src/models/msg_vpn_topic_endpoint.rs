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
pub struct MsgVpnTopicEndpoint {
  /// The Topic Endpoint access type of either \"exclusive\" or \"non-exclusive\". The default value is `\"exclusive\"`. The allowed values and their meaning are:  <pre> \"exclusive\" - Exclusive delivery of messages to first bound client. \"non-exclusive\" - Non-exclusive delivery of messages to all bound clients. </pre>  Available since 2.4.
  #[serde(rename = "accessType")]
  access_type: Option<String>,
  /// Enable or disable the propagation of Consumer ACKs received on the active replication Message VPN to the standby replication Message VPN. The default value is `true`.
  #[serde(rename = "consumerAckPropagationEnabled")]
  consumer_ack_propagation_enabled: Option<bool>,
  /// The name of the Dead Message Queue (DMQ) used by the Topic Endpoint. The default value is `\"#DEAD_MSG_QUEUE\"`. Available since 2.2.
  #[serde(rename = "deadMsgQueue")]
  dead_msg_queue: Option<String>,
  /// Enable or disable the egress flow of messages from the Topic Endpoint. The default value is `false`.
  #[serde(rename = "egressEnabled")]
  egress_enabled: Option<bool>,
  #[serde(rename = "eventBindCountThreshold")]
  event_bind_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventRejectLowPriorityMsgLimitThreshold")]
  event_reject_low_priority_msg_limit_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventSpoolUsageThreshold")]
  event_spool_usage_threshold: Option<::models::EventThreshold>,
  /// Enable or disable the ingress flow of messages to the Topic Endpoint. The default value is `false`.
  #[serde(rename = "ingressEnabled")]
  ingress_enabled: Option<bool>,
  /// The maximum number of simultaneous Consumers of the Topic Endpoint. The default value is `1`. Available since 2.4.
  #[serde(rename = "maxBindCount")]
  max_bind_count: Option<i64>,
  /// The maximum allowed number of messages delivered but not acknowledged per flow for the Topic Endpoint. The default is the maximum value supported by the hardware. The default value is `10000`.
  #[serde(rename = "maxDeliveredUnackedMsgsPerFlow")]
  max_delivered_unacked_msgs_per_flow: Option<i64>,
  /// The maximum message size allowed in the Topic Endpoint, in bytes. The default value is `10000000`.
  #[serde(rename = "maxMsgSize")]
  max_msg_size: Option<i32>,
  /// The maximum number of times the Topic Endpoint will attempt redelivery of a given message prior to it being discarded or moved to the #DEAD_MSG_QUEUE. A value of 0 means to retry forever. The default value is `0`.
  #[serde(rename = "maxRedeliveryCount")]
  max_redelivery_count: Option<i64>,
  /// The maximum Message Spool usage by the Topic Endpoint (quota), in megabytes. Setting the value to zero enables the \"last-value-queue\" feature and disables quota checking. The default varies by platform. The default varies by platform.
  #[serde(rename = "maxSpoolUsage")]
  max_spool_usage: Option<i64>,
  /// The maximum number of seconds that a message can stay in the Topic Endpoint when \"respectTtlEnabled\" is \"true\". A message will expire according to the lesser of the TTL in the message (assigned by the Publisher) and the \"maxTtl\" configured on the Topic Endpoint. \"maxTtl\" is a 32-bit integer value from 1 to 4294967295 representing the expiry time in seconds. A \"maxTtl\" of \"0\" disables this feature. The default value is `0`.
  #[serde(rename = "maxTtl")]
  max_ttl: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName")]
  msg_vpn_name: Option<String>,
  /// The Client Username which owns the Topic Endpoint. The default value is `\"\"`.
  #[serde(rename = "owner")]
  owner: Option<String>,
  /// Permission level for users of the Topic Endpoint, excluding the owner. The default value is `\"no-access\"`. The allowed values and their meaning are:  <pre> \"no-access\" - Disallows all access. \"read-only\" - Read-only access to the messages in the Topic Endpoint. \"consume\" - Consume (read and remove) messages in the Topic Endpoint. \"modify-topic\" - Consume messages or modify the topic/selector of the Topic Endpoint. \"delete\" - Consume messages, modify the topic/selector or delete the Topic Endpoint altogether. </pre> 
  #[serde(rename = "permission")]
  permission: Option<String>,
  /// Enable or disable if low priority messages are subject to \"rejectLowPriorityMsgLimit\" checking. This may only be enabled if \"rejectMsgToSenderOnDiscardBehavior\" does not have a value of \"never\". The default value is `false`.
  #[serde(rename = "rejectLowPriorityMsgEnabled")]
  reject_low_priority_msg_enabled: Option<bool>,
  /// The number of messages of any priority in the Topic Endpoint above which low priority messages are not admitted but higher priority messages are allowed. The default value is `0`.
  #[serde(rename = "rejectLowPriorityMsgLimit")]
  reject_low_priority_msg_limit: Option<i64>,
  /// The circumstances under which a negative acknowledgement (NACK) is sent to the client on discards. Note that NACKs cause the message to not be delivered to any destination and transacted-session commits to fail. This attribute may only have a value of \"never\" if \"rejectLowPriorityMsgEnabled\" is disabled. The default value is `\"never\"`. The allowed values and their meaning are:  <pre> \"always\" - Message discards always result in negative acknowledgments (NACKs) being returned to the sending client, even if the discard reason is that the topic-endpoint is disabled. \"when-topic-endpoint-enabled\" - Message discards result in negative acknowledgments (NACKs) being returned to the sending client, except if the discard reason is that the Topic Endpoint is disabled. \"never\" - Message discards never result in negative acknowledgments (NACKs) being returned to the sending client. </pre> 
  #[serde(rename = "rejectMsgToSenderOnDiscardBehavior")]
  reject_msg_to_sender_on_discard_behavior: Option<String>,
  /// Enable or disable the respecting of message priority. If enabled, messages contained in the Topic Endpoint are delivered in priority order, from 9 (highest) to 0 (lowest). The default value is `false`. Available since 2.8.
  #[serde(rename = "respectMsgPriorityEnabled")]
  respect_msg_priority_enabled: Option<bool>,
  /// Enable or disable the respecting of \"time to live\" (TTL). If enabled, then messages contained in the Topic Endpoint are checked for expiry. If expired, the message is removed from the Topic Endpoint and either discarded or a copy of the message placed in the #DEAD_MSG_QUEUE Endpoint. The default value is `false`.
  #[serde(rename = "respectTtlEnabled")]
  respect_ttl_enabled: Option<bool>,
  /// The name of the Topic Endpoint.
  #[serde(rename = "topicEndpointName")]
  topic_endpoint_name: Option<String>
}

impl MsgVpnTopicEndpoint {
  pub fn new() -> MsgVpnTopicEndpoint {
    MsgVpnTopicEndpoint {
      access_type: None,
      consumer_ack_propagation_enabled: None,
      dead_msg_queue: None,
      egress_enabled: None,
      event_bind_count_threshold: None,
      event_reject_low_priority_msg_limit_threshold: None,
      event_spool_usage_threshold: None,
      ingress_enabled: None,
      max_bind_count: None,
      max_delivered_unacked_msgs_per_flow: None,
      max_msg_size: None,
      max_redelivery_count: None,
      max_spool_usage: None,
      max_ttl: None,
      msg_vpn_name: None,
      owner: None,
      permission: None,
      reject_low_priority_msg_enabled: None,
      reject_low_priority_msg_limit: None,
      reject_msg_to_sender_on_discard_behavior: None,
      respect_msg_priority_enabled: None,
      respect_ttl_enabled: None,
      topic_endpoint_name: None
    }
  }

  pub fn set_access_type(&mut self, access_type: String) {
    self.access_type = Some(access_type);
  }

  pub fn with_access_type(mut self, access_type: String) -> MsgVpnTopicEndpoint {
    self.access_type = Some(access_type);
    self
  }

  pub fn access_type(&self) -> Option<&String> {
    self.access_type.as_ref()
  }

  pub fn reset_access_type(&mut self) {
    self.access_type = None;
  }

  pub fn set_consumer_ack_propagation_enabled(&mut self, consumer_ack_propagation_enabled: bool) {
    self.consumer_ack_propagation_enabled = Some(consumer_ack_propagation_enabled);
  }

  pub fn with_consumer_ack_propagation_enabled(mut self, consumer_ack_propagation_enabled: bool) -> MsgVpnTopicEndpoint {
    self.consumer_ack_propagation_enabled = Some(consumer_ack_propagation_enabled);
    self
  }

  pub fn consumer_ack_propagation_enabled(&self) -> Option<&bool> {
    self.consumer_ack_propagation_enabled.as_ref()
  }

  pub fn reset_consumer_ack_propagation_enabled(&mut self) {
    self.consumer_ack_propagation_enabled = None;
  }

  pub fn set_dead_msg_queue(&mut self, dead_msg_queue: String) {
    self.dead_msg_queue = Some(dead_msg_queue);
  }

  pub fn with_dead_msg_queue(mut self, dead_msg_queue: String) -> MsgVpnTopicEndpoint {
    self.dead_msg_queue = Some(dead_msg_queue);
    self
  }

  pub fn dead_msg_queue(&self) -> Option<&String> {
    self.dead_msg_queue.as_ref()
  }

  pub fn reset_dead_msg_queue(&mut self) {
    self.dead_msg_queue = None;
  }

  pub fn set_egress_enabled(&mut self, egress_enabled: bool) {
    self.egress_enabled = Some(egress_enabled);
  }

  pub fn with_egress_enabled(mut self, egress_enabled: bool) -> MsgVpnTopicEndpoint {
    self.egress_enabled = Some(egress_enabled);
    self
  }

  pub fn egress_enabled(&self) -> Option<&bool> {
    self.egress_enabled.as_ref()
  }

  pub fn reset_egress_enabled(&mut self) {
    self.egress_enabled = None;
  }

  pub fn set_event_bind_count_threshold(&mut self, event_bind_count_threshold: ::models::EventThreshold) {
    self.event_bind_count_threshold = Some(event_bind_count_threshold);
  }

  pub fn with_event_bind_count_threshold(mut self, event_bind_count_threshold: ::models::EventThreshold) -> MsgVpnTopicEndpoint {
    self.event_bind_count_threshold = Some(event_bind_count_threshold);
    self
  }

  pub fn event_bind_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_bind_count_threshold.as_ref()
  }

  pub fn reset_event_bind_count_threshold(&mut self) {
    self.event_bind_count_threshold = None;
  }

  pub fn set_event_reject_low_priority_msg_limit_threshold(&mut self, event_reject_low_priority_msg_limit_threshold: ::models::EventThreshold) {
    self.event_reject_low_priority_msg_limit_threshold = Some(event_reject_low_priority_msg_limit_threshold);
  }

  pub fn with_event_reject_low_priority_msg_limit_threshold(mut self, event_reject_low_priority_msg_limit_threshold: ::models::EventThreshold) -> MsgVpnTopicEndpoint {
    self.event_reject_low_priority_msg_limit_threshold = Some(event_reject_low_priority_msg_limit_threshold);
    self
  }

  pub fn event_reject_low_priority_msg_limit_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_reject_low_priority_msg_limit_threshold.as_ref()
  }

  pub fn reset_event_reject_low_priority_msg_limit_threshold(&mut self) {
    self.event_reject_low_priority_msg_limit_threshold = None;
  }

  pub fn set_event_spool_usage_threshold(&mut self, event_spool_usage_threshold: ::models::EventThreshold) {
    self.event_spool_usage_threshold = Some(event_spool_usage_threshold);
  }

  pub fn with_event_spool_usage_threshold(mut self, event_spool_usage_threshold: ::models::EventThreshold) -> MsgVpnTopicEndpoint {
    self.event_spool_usage_threshold = Some(event_spool_usage_threshold);
    self
  }

  pub fn event_spool_usage_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_spool_usage_threshold.as_ref()
  }

  pub fn reset_event_spool_usage_threshold(&mut self) {
    self.event_spool_usage_threshold = None;
  }

  pub fn set_ingress_enabled(&mut self, ingress_enabled: bool) {
    self.ingress_enabled = Some(ingress_enabled);
  }

  pub fn with_ingress_enabled(mut self, ingress_enabled: bool) -> MsgVpnTopicEndpoint {
    self.ingress_enabled = Some(ingress_enabled);
    self
  }

  pub fn ingress_enabled(&self) -> Option<&bool> {
    self.ingress_enabled.as_ref()
  }

  pub fn reset_ingress_enabled(&mut self) {
    self.ingress_enabled = None;
  }

  pub fn set_max_bind_count(&mut self, max_bind_count: i64) {
    self.max_bind_count = Some(max_bind_count);
  }

  pub fn with_max_bind_count(mut self, max_bind_count: i64) -> MsgVpnTopicEndpoint {
    self.max_bind_count = Some(max_bind_count);
    self
  }

  pub fn max_bind_count(&self) -> Option<&i64> {
    self.max_bind_count.as_ref()
  }

  pub fn reset_max_bind_count(&mut self) {
    self.max_bind_count = None;
  }

  pub fn set_max_delivered_unacked_msgs_per_flow(&mut self, max_delivered_unacked_msgs_per_flow: i64) {
    self.max_delivered_unacked_msgs_per_flow = Some(max_delivered_unacked_msgs_per_flow);
  }

  pub fn with_max_delivered_unacked_msgs_per_flow(mut self, max_delivered_unacked_msgs_per_flow: i64) -> MsgVpnTopicEndpoint {
    self.max_delivered_unacked_msgs_per_flow = Some(max_delivered_unacked_msgs_per_flow);
    self
  }

  pub fn max_delivered_unacked_msgs_per_flow(&self) -> Option<&i64> {
    self.max_delivered_unacked_msgs_per_flow.as_ref()
  }

  pub fn reset_max_delivered_unacked_msgs_per_flow(&mut self) {
    self.max_delivered_unacked_msgs_per_flow = None;
  }

  pub fn set_max_msg_size(&mut self, max_msg_size: i32) {
    self.max_msg_size = Some(max_msg_size);
  }

  pub fn with_max_msg_size(mut self, max_msg_size: i32) -> MsgVpnTopicEndpoint {
    self.max_msg_size = Some(max_msg_size);
    self
  }

  pub fn max_msg_size(&self) -> Option<&i32> {
    self.max_msg_size.as_ref()
  }

  pub fn reset_max_msg_size(&mut self) {
    self.max_msg_size = None;
  }

  pub fn set_max_redelivery_count(&mut self, max_redelivery_count: i64) {
    self.max_redelivery_count = Some(max_redelivery_count);
  }

  pub fn with_max_redelivery_count(mut self, max_redelivery_count: i64) -> MsgVpnTopicEndpoint {
    self.max_redelivery_count = Some(max_redelivery_count);
    self
  }

  pub fn max_redelivery_count(&self) -> Option<&i64> {
    self.max_redelivery_count.as_ref()
  }

  pub fn reset_max_redelivery_count(&mut self) {
    self.max_redelivery_count = None;
  }

  pub fn set_max_spool_usage(&mut self, max_spool_usage: i64) {
    self.max_spool_usage = Some(max_spool_usage);
  }

  pub fn with_max_spool_usage(mut self, max_spool_usage: i64) -> MsgVpnTopicEndpoint {
    self.max_spool_usage = Some(max_spool_usage);
    self
  }

  pub fn max_spool_usage(&self) -> Option<&i64> {
    self.max_spool_usage.as_ref()
  }

  pub fn reset_max_spool_usage(&mut self) {
    self.max_spool_usage = None;
  }

  pub fn set_max_ttl(&mut self, max_ttl: i64) {
    self.max_ttl = Some(max_ttl);
  }

  pub fn with_max_ttl(mut self, max_ttl: i64) -> MsgVpnTopicEndpoint {
    self.max_ttl = Some(max_ttl);
    self
  }

  pub fn max_ttl(&self) -> Option<&i64> {
    self.max_ttl.as_ref()
  }

  pub fn reset_max_ttl(&mut self) {
    self.max_ttl = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnTopicEndpoint {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_owner(&mut self, owner: String) {
    self.owner = Some(owner);
  }

  pub fn with_owner(mut self, owner: String) -> MsgVpnTopicEndpoint {
    self.owner = Some(owner);
    self
  }

  pub fn owner(&self) -> Option<&String> {
    self.owner.as_ref()
  }

  pub fn reset_owner(&mut self) {
    self.owner = None;
  }

  pub fn set_permission(&mut self, permission: String) {
    self.permission = Some(permission);
  }

  pub fn with_permission(mut self, permission: String) -> MsgVpnTopicEndpoint {
    self.permission = Some(permission);
    self
  }

  pub fn permission(&self) -> Option<&String> {
    self.permission.as_ref()
  }

  pub fn reset_permission(&mut self) {
    self.permission = None;
  }

  pub fn set_reject_low_priority_msg_enabled(&mut self, reject_low_priority_msg_enabled: bool) {
    self.reject_low_priority_msg_enabled = Some(reject_low_priority_msg_enabled);
  }

  pub fn with_reject_low_priority_msg_enabled(mut self, reject_low_priority_msg_enabled: bool) -> MsgVpnTopicEndpoint {
    self.reject_low_priority_msg_enabled = Some(reject_low_priority_msg_enabled);
    self
  }

  pub fn reject_low_priority_msg_enabled(&self) -> Option<&bool> {
    self.reject_low_priority_msg_enabled.as_ref()
  }

  pub fn reset_reject_low_priority_msg_enabled(&mut self) {
    self.reject_low_priority_msg_enabled = None;
  }

  pub fn set_reject_low_priority_msg_limit(&mut self, reject_low_priority_msg_limit: i64) {
    self.reject_low_priority_msg_limit = Some(reject_low_priority_msg_limit);
  }

  pub fn with_reject_low_priority_msg_limit(mut self, reject_low_priority_msg_limit: i64) -> MsgVpnTopicEndpoint {
    self.reject_low_priority_msg_limit = Some(reject_low_priority_msg_limit);
    self
  }

  pub fn reject_low_priority_msg_limit(&self) -> Option<&i64> {
    self.reject_low_priority_msg_limit.as_ref()
  }

  pub fn reset_reject_low_priority_msg_limit(&mut self) {
    self.reject_low_priority_msg_limit = None;
  }

  pub fn set_reject_msg_to_sender_on_discard_behavior(&mut self, reject_msg_to_sender_on_discard_behavior: String) {
    self.reject_msg_to_sender_on_discard_behavior = Some(reject_msg_to_sender_on_discard_behavior);
  }

  pub fn with_reject_msg_to_sender_on_discard_behavior(mut self, reject_msg_to_sender_on_discard_behavior: String) -> MsgVpnTopicEndpoint {
    self.reject_msg_to_sender_on_discard_behavior = Some(reject_msg_to_sender_on_discard_behavior);
    self
  }

  pub fn reject_msg_to_sender_on_discard_behavior(&self) -> Option<&String> {
    self.reject_msg_to_sender_on_discard_behavior.as_ref()
  }

  pub fn reset_reject_msg_to_sender_on_discard_behavior(&mut self) {
    self.reject_msg_to_sender_on_discard_behavior = None;
  }

  pub fn set_respect_msg_priority_enabled(&mut self, respect_msg_priority_enabled: bool) {
    self.respect_msg_priority_enabled = Some(respect_msg_priority_enabled);
  }

  pub fn with_respect_msg_priority_enabled(mut self, respect_msg_priority_enabled: bool) -> MsgVpnTopicEndpoint {
    self.respect_msg_priority_enabled = Some(respect_msg_priority_enabled);
    self
  }

  pub fn respect_msg_priority_enabled(&self) -> Option<&bool> {
    self.respect_msg_priority_enabled.as_ref()
  }

  pub fn reset_respect_msg_priority_enabled(&mut self) {
    self.respect_msg_priority_enabled = None;
  }

  pub fn set_respect_ttl_enabled(&mut self, respect_ttl_enabled: bool) {
    self.respect_ttl_enabled = Some(respect_ttl_enabled);
  }

  pub fn with_respect_ttl_enabled(mut self, respect_ttl_enabled: bool) -> MsgVpnTopicEndpoint {
    self.respect_ttl_enabled = Some(respect_ttl_enabled);
    self
  }

  pub fn respect_ttl_enabled(&self) -> Option<&bool> {
    self.respect_ttl_enabled.as_ref()
  }

  pub fn reset_respect_ttl_enabled(&mut self) {
    self.respect_ttl_enabled = None;
  }

  pub fn set_topic_endpoint_name(&mut self, topic_endpoint_name: String) {
    self.topic_endpoint_name = Some(topic_endpoint_name);
  }

  pub fn with_topic_endpoint_name(mut self, topic_endpoint_name: String) -> MsgVpnTopicEndpoint {
    self.topic_endpoint_name = Some(topic_endpoint_name);
    self
  }

  pub fn topic_endpoint_name(&self) -> Option<&String> {
    self.topic_endpoint_name.as_ref()
  }

  pub fn reset_topic_endpoint_name(&mut self) {
    self.topic_endpoint_name = None;
  }

}



